# Data Model: Acquisition Flow (038)

**Date**: 2026-03-12

---

## Overview

The Acquisition Flow introduces no new database tables. It maps to three existing tables via a new
use case and command that orchestrate the upsert + purchase-recording logic atomically.

```
[ AcquisitionDrawer (frontend) ]
         │
         │  record_acquisition(args)  [new Tauri command]
         ▼
[ RecordAcquisition use case (Rust) ]
         │
         ├── For each item:
         │     ├── RailwayModelId::new(mfr_id, product_code)
         │     ├── if not found → CatalogRepository::create(...)
         │     └── CollectionRepository::add_item(...)
         │
         ▼
[ SQLite via sqlx ]
   railway_models ← (conditional upsert per item)
   collection_items ← (one row per item)
   purchase_infos ← (one row per item, type = "Purchased")
```

---

## Existing Tables Used (no changes)

### `railway_models`

| Column            | Type    | Notes                                        |
| ----------------- | ------- | -------------------------------------------- |
| `id`              | TEXT PK | `trn:railway-model:{mfr_nss}:{product_code}` |
| `manufacturer_id` | TEXT FK | → `manufacturers.id`                         |
| `product_code`    | TEXT    | e.g., `"43870"`                              |
| `power_method`    | TEXT    | `"AC"`, `"DC"`, `"TRIX_EXPRESS"`             |
| `scale`           | TEXT    | `"H0"`, `"N"`, etc.                          |
| `epoch`           | TEXT    | `"III"`, `"IV"`, etc.                        |
| `category`        | TEXT    | `"LOCOMOTIVES"`, `"FREIGHT_CARS"`, etc.      |
| `created_at`      | TEXT    | Auto-set                                     |
| `updated_at`      | TEXT    | Auto-set                                     |
| `version`         | INTEGER | Optimistic lock                              |

**Upsert logic**: `RailwayModelId::new(manufacturer_id, product_code)` → probe `find_by_id`. Create
only if absent. Description is stored in `railway_model_translations` (language = `"en"`).

### `collection_items`

| Column               | Type    | Notes                        |
| -------------------- | ------- | ---------------------------- |
| `id`                 | TEXT PK | Generated `CollectionItemId` |
| `collection_id`      | TEXT FK | → default collection         |
| `railway_model_id`   | TEXT FK | → `railway_models.id`        |
| `added_date`         | TEXT    | Today (YYYY-MM-DD)           |
| `removed_date`       | TEXT    | NULL                         |
| `purchase_condition` | TEXT    | NULL (enrichable later)      |
| `model_condition`    | TEXT    | NULL (enrichable later)      |
| `box_condition`      | TEXT    | NULL (enrichable later)      |
| `notes`              | TEXT    | NULL                         |

### `purchase_infos`

| Column                     | Type    | Notes                            |
| -------------------------- | ------- | -------------------------------- |
| `id`                       | TEXT PK | Generated `PurchaseInfoId`       |
| `collection_item_id`       | TEXT FK | → `collection_items.id`          |
| `purchase_type`            | TEXT    | `"Purchased"`                    |
| `purchase_date`            | TEXT    | User-selected date (YYYY-MM-DD)  |
| `seller_id`                | TEXT FK | → `sellers.id` (optional)        |
| `purchased_price_amount`   | INTEGER | cents (0 if not provided)        |
| `purchased_price_currency` | TEXT    | e.g., `"EUR"`, `"USD"`           |
| all other columns          | NULL    | Not used for "Purchased" variant |

---

## New Rust Types

### Transport Layer (command_args.rs — collecting module)

```rust
/// Top-level args for the record_acquisition command.
/// Derives: Debug, Clone, Validate, specta::Type, serde::Deserialize
pub struct RecordAcquisitionArgs {
    pub seller_id: Option<String>,
    pub purchase_date: String,            // YYYY-MM-DD; validated as NaiveDate
    pub items: Vec<AcquisitionItemArgs>,  // at least one required
}

/// Per-item args within a single acquisition batch.
pub struct AcquisitionItemArgs {
    pub manufacturer_id: String,
    pub product_code: String,             // non-empty; min 1 char
    pub description: String,
    pub category: String,                 // enum variant name
    pub scale: String,                    // enum variant name
    pub epoch: String,
    pub power_method: String,             // enum variant name
    pub price_amount: i64,                // cents; 0 = no price recorded
    pub price_currency: String,           // ISO 4217 code
}
```

**Validation rules** (validator crate):

- `RecordAcquisitionArgs.purchase_date`: valid `NaiveDate`, not in the future.
- `RecordAcquisitionArgs.items`: `#[validate(length(min = 1))]`.
- `AcquisitionItemArgs.manufacturer_id`: non-empty.
- `AcquisitionItemArgs.product_code`: non-empty, min 1 char.
- `AcquisitionItemArgs.category`: must parse to `Category` enum.
- `AcquisitionItemArgs.scale`: must parse to `Scale` enum.
- `AcquisitionItemArgs.power_method`: must parse to `PowerMethod` enum.

### Use Case Layer (application layer — collecting module)

```rust
pub struct RecordAcquisitionInput {
    pub seller_id: Option<SellerId>,
    pub purchase_date: NaiveDate,
    pub items: Vec<AcquisitionItemInput>,
}

pub struct AcquisitionItemInput {
    pub manufacturer_id: ManufacturerId,
    pub product_code: String,
    pub description: String,
    pub category: Category,
    pub scale: Scale,
    pub epoch: Epoch,
    pub power_method: PowerMethod,
    pub price: MonetaryAmount,
}
```

**Use case: `RecordAcquisition::execute`**

```
For each item in input.items:
  1. id = RailwayModelId::new(&item.manufacturer_id, &item.product_code)?
  2. existing = catalog_repo.find_by_id(&id, "en")
  3. if existing.is_none():
       catalog_repo.create(RailwayModelParams { id, manufacturer_id, product_code, ... })
  4. collection_item_id = id_provider.next()
  5. purchase_info_id = id_provider.next()
  6. collection.add_item(NewCollectionItem { railway_model_id: id, price, seller_id, purchase_date, ... })
  7. collection_repo.save(&mut collection)
Return: Vec<CollectionItemId>
```

---

## New Frontend Types

### `src/lib/features/acquisition/types.ts`

```typescript
/** Session state for the entire acquisition drawer. */
export interface AcquisitionFormState {
  sellerId: string | null;
  purchaseDate: string; // YYYY-MM-DD, defaults to today
  batchDefaults: BatchDefaults;
  items: AcquisitionItemEntry[];
}

/** Shared defaults auto-applied to new item cards. */
export interface BatchDefaults {
  scale: string | null;
  powerMethod: string | null;
}

/** State for a single item card in the acquisition list. */
export interface AcquisitionItemEntry {
  uid: string; // crypto.randomUUID() — for list keying only
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: string | null;
  scale: string | null; // inherits from batchDefaults, overridable
  epoch: string | null;
  powerMethod: string | null; // inherits from batchDefaults, overridable
  priceAmount: number | null; // decimal entered by user (e.g. 29.99)
}
```

### Default state factory

```typescript
function createDefaultItem(defaults: BatchDefaults): AcquisitionItemEntry {
  return {
    uid: crypto.randomUUID(),
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    scale: defaults.scale,
    epoch: null,
    powerMethod: defaults.powerMethod,
    priceAmount: null
  };
}

function createDefaultFormState(): AcquisitionFormState {
  return {
    sellerId: null,
    purchaseDate: new Date().toISOString().split('T')[0],
    batchDefaults: { scale: null, powerMethod: null },
    items: [createDefaultItem({ scale: null, powerMethod: null })]
  };
}
```

---

## Validation Rules (Frontend)

Validation fires on "Finalize Purchase" click (set `touched = true`). Errors clear as user types.

| Field                 | Rule                           |
| --------------------- | ------------------------------ |
| `items` (array)       | length ≥ 1                     |
| `item.manufacturerId` | required (not null)            |
| `item.productCode`    | required, non-empty after trim |
| `item.category`       | required (not null)            |
| `item.scale`          | optional                       |
| `item.powerMethod`    | optional                       |
| `item.epoch`          | optional                       |
| `item.priceAmount`    | optional; if set, must be ≥ 0  |
| `sellerId`            | optional                       |
| `purchaseDate`        | required; not in future        |
