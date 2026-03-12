# Contract: Rust Backend — record_acquisition

**Module**: `src-tauri/src/collecting/`
**Command name**: `record_acquisition`

---

## Command Signature

```rust
#[tauri::command]
#[specta::specta]
pub async fn record_acquisition(
    state: tauri::State<'_, AppState>,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError>
```

---

## Args (Transport DTO)

File: `src-tauri/src/collecting/interface/command_args.rs`

```rust
#[derive(Debug, Clone, validator::Validate, specta::Type, serde::Deserialize)]
pub struct RecordAcquisitionArgs {
    pub seller_id: Option<String>,

    #[validate(custom(function = "validate_not_future_date"))]
    pub purchase_date: String,            // YYYY-MM-DD

    #[validate(length(min = 1, message = "at least one item is required"))]
    #[validate(nested)]
    pub items: Vec<AcquisitionItemArgs>,
}

#[derive(Debug, Clone, validator::Validate, specta::Type, serde::Deserialize)]
pub struct AcquisitionItemArgs {
    #[validate(length(min = 1, message = "manufacturer_id is required"))]
    pub manufacturer_id: String,

    #[validate(length(min = 1, message = "product_code is required"))]
    pub product_code: String,

    pub description: String,

    #[validate(length(min = 1, message = "category is required"))]
    pub category: String,

    #[validate(length(min = 1, message = "scale is required"))]
    pub scale: String,

    pub epoch: String,

    #[validate(length(min = 1, message = "power_method is required"))]
    pub power_method: String,

    pub price_amount: i64,    // cents; 0 = no price
    pub price_currency: String,
}
```

---

## Mapping to Use Case Input

File: `src-tauri/src/collecting/interface/command_handlers.rs`

```rust
pub async fn record_acquisition(
    state: tauri::State<'_, AppState>,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError> {
    args.validate().map_err(CommandError::validation)?;

    let input = RecordAcquisitionInput {
        seller_id: args.seller_id.map(SellerId::from_string_unchecked),
        purchase_date: NaiveDate::parse_from_str(&args.purchase_date, "%Y-%m-%d")
            .map_err(|_| CommandError::invalid_input("invalid purchase_date"))?,
        items: args.items.into_iter().map(|item| {
            Ok(AcquisitionItemInput {
                manufacturer_id: ManufacturerId::try_from(item.manufacturer_id.as_str())
                    .map_err(|e| CommandError::invalid_input(e.to_string()))?,
                product_code: item.product_code,
                description: item.description,
                category: item.category.parse::<Category>()
                    .map_err(|_| CommandError::invalid_input("invalid category"))?,
                scale: item.scale.parse::<Scale>()
                    .map_err(|_| CommandError::invalid_input("invalid scale"))?,
                epoch: Epoch(item.epoch),
                power_method: item.power_method.parse::<PowerMethod>()
                    .map_err(|_| CommandError::invalid_input("invalid power_method"))?,
                price: MonetaryAmount::new(item.price_amount, item.price_currency),
            })
        }).collect::<Result<Vec<_>, CommandError>>()?,
    };

    let mut uow = state.unit_of_work().await?;
    let ids = RecordAcquisition::execute(
        &mut uow,
        state.collection_item_id_provider(),
        state.purchase_info_id_provider(),
        input,
    ).await?;
    uow.commit().await?;

    Ok(ids)
}
```

---

## Use Case

File: `src-tauri/src/collecting/application/record_acquisition.rs` (new)

```rust
pub struct RecordAcquisition;

impl RecordAcquisition {
    pub async fn execute<U, P, Q>(
        unit_of_work: &mut U,
        collection_item_id_provider: P,
        purchase_info_id_provider: Q,
        input: RecordAcquisitionInput,
    ) -> Result<Vec<CollectionItemId>, DomainError>
    where
        U: CollectionUowExt + RailwayModelUowExt + Send,
        P: IdProvider<CollectionItemId> + Clone,
        Q: IdProvider<PurchaseInfoId> + Clone,
    {
        let collection = unit_of_work
            .collection_repository()
            .find_default()
            .await?;

        let mut ids = Vec::with_capacity(input.items.len());

        for item in input.items {
            // 1. Derive deterministic ID
            let model_id = RailwayModelId::new(&item.manufacturer_id, &item.product_code)
                .map_err(DomainError::from)?;

            // 2. Upsert catalog entry
            let existing = unit_of_work
                .railway_model_repository()
                .find_by_id(&model_id, "en")
                .await?;

            if existing.is_none() {
                unit_of_work
                    .railway_model_repository()
                    .create(&RailwayModelParams {
                        id: model_id.clone(),
                        manufacturer_id: item.manufacturer_id.clone(),
                        product_code: item.product_code.clone(),
                        description: item.description.clone(),
                        category: item.category,
                        scale: item.scale,
                        epoch: item.epoch.clone(),
                        power_method: item.power_method,
                        ..Default::default()
                    })
                    .await?;
            }

            // 3. Record purchase
            let collection_item_id = collection_item_id_provider.next();
            let purchase_info_id = purchase_info_id_provider.next();

            let new_item = NewCollectionItem {
                collection_item_id: collection_item_id.clone(),
                purchase_info_id,
                railway_model_id: model_id,
                price: item.price,
                seller_id: input.seller_id.clone(),
                added_date: Local::now().date_naive(),
                purchase_date: input.purchase_date,
                purchase_condition: None,
                model_condition: None,
                box_condition: None,
                notes: None,
            };

            collection.add_item(new_item)?;
            ids.push(collection_item_id);
        }

        unit_of_work
            .collection_repository()
            .save(&mut collection)
            .await?;

        Ok(ids)
    }
}
```

---

## Registration

File: `src-tauri/src/lib.rs` — add to `collect_commands!`:

```rust
collecting::interface::command_handlers::record_acquisition,
```

After registration, run `pnpm tauri dev` to regenerate `src/lib/bindings.ts`.

---

## Error Cases

| Condition                                          | Error returned                                |
| -------------------------------------------------- | --------------------------------------------- |
| `items` array is empty                             | `CommandError::Validation` (validator)        |
| `purchase_date` is a future date                   | `CommandError::Validation` (custom validator) |
| `manufacturer_id` invalid TRN format               | `CommandError::InvalidInput`                  |
| `category` / `scale` / `power_method` unrecognized | `CommandError::InvalidInput`                  |
| DB write failure                                   | `CommandError::Internal`                      |

---

## Global Shortcut (Ctrl+N) — REQUIRES DEPENDENCY APPROVAL

**Status**: Blocked pending user approval to add `tauri-plugin-global-shortcut`.

Once approved:

```toml
# src-tauri/Cargo.toml
[dependencies]
tauri-plugin-global-shortcut = "2"
```

```rust
// src-tauri/src/lib.rs — in setup()
.plugin(tauri_plugin_global_shortcut::Builder::new().build())
// in run():
app.global_shortcut().register("CommandOrControl+N", |app, _shortcut, _event| {
    app.emit("open-acquisition-drawer", ()).ok();
})?;
```

```json
// src-tauri/capabilities/default.json
{ "identifier": "global-shortcut:allow-register" }
```
