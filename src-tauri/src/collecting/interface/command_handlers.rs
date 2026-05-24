use crate::catalog::application::{SaveRailwayModel, SaveRailwayModelInput};
use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{Category, Epoch, PowerMethod};
use crate::catalog::domain::scale::Scale;
use crate::collecting::application::AddCollectionItemInput as DomainAddCollectionItemInput;
use crate::collecting::application::{
    AcquisitionItemInput, AddCollectionItem, GetCollection, GetDepot, ReceivePreorder,
    ReceivePreorderInput, RecordAcquisition, RecordAcquisitionInput, RemoveCollectionItem,
    RemoveCollectionItemInput as DomainRemoveCollectionItemInput, SellCollectionItem,
    SellCollectionItemInput as DomainSellCollectionItemInput, UpdateCollectionItem,
};
use crate::collecting::domain::{
    BoxCondition, CollectionItemUpdate, CollectionStats, ModelCondition, PurchaseCondition,
    UpdateCollectionItemInput,
};
use crate::collecting::domain::{CollectionItemId, CollectionView, DepotView};
use crate::collecting::interface::command_args::{
    AcquisitionItemArgs, AddRailwayModelToCollectionArgs, ReceivePreorderArgs,
    RecordAcquisitionArgs, SellCollectionItemArgs,
};
use crate::collecting::interface::{
    AddCollectionItemArgs, CollectionItemUpdateArgs, RemoveCollectionItemArgs,
    UpdateCollectionItemArgs,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, Language, MonetaryAmount};
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::sellers::domain::seller_id::SellerId;
use crate::state::AppState;
use chrono::NaiveDate;
use garde::Validate;
use std::convert::TryFrom;
use tracing::info;

#[derive(serde::Serialize, specta::Type)]
pub struct DetailedCollectionItemView {
    pub model: Option<crate::catalog::domain::railway_model::RailwayModelView>,
    pub image: Option<crate::media::interface::RailwayModelImageResponse>,
    pub seller: Option<crate::sellers::application::seller_view::SellerView>,
}

pub async fn get_collection_item_details_inner(
    state: &AppState,
    railway_model_id: String,
    seller_id: Option<String>,
    lang: Language,
) -> Result<DetailedCollectionItemView, CommandError> {
    let r_id =
        crate::catalog::domain::railway_model::RailwayModelId::try_from(railway_model_id.as_str())
            .map_err(|_| CommandError::validation_field("railway_model_id", "invalid format"))?;

    let model = crate::catalog::interface::command_handlers::get_railway_model_by_id_inner(
        state,
        r_id.clone(),
        lang,
    )
    .await?;
    let image =
        crate::media::interface::command_handlers::get_railway_model_image_inner(state, r_id)
            .await
            .ok();

    let seller = match seller_id {
        Some(ref s) => {
            let s_id = SellerId::try_from(s.as_str())
                .map_err(|_| CommandError::validation_field("seller_id", "invalid format"))?;
            crate::sellers::interface::command_handlers::get_seller_by_id_inner(state, s_id).await?
        }
        None => None,
    };

    Ok(DetailedCollectionItemView {
        model,
        image,
        seller,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn get_collection_item_details(
    state: tauri::State<'_, AppState>,
    railway_model_id: String,
    seller_id: Option<String>,
    lang: Language,
) -> Result<DetailedCollectionItemView, CommandError> {
    get_collection_item_details_inner(&state, railway_model_id, seller_id, lang).await
}

pub async fn get_collection_inner(state: &AppState) -> Result<CollectionView, CommandError> {
    info!("Fetching collection");

    let mut unit_of_work = state.unit_of_work().await?;

    let collection = GetCollection::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;

    Ok(collection)
}

/// Tauri command to retrieve the default collection.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `CollectionView` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(CollectionView)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_collection(
    state: tauri::State<'_, AppState>,
) -> Result<CollectionView, CommandError> {
    get_collection_inner(&state).await
}

pub async fn get_depot_inner(state: &AppState) -> Result<DepotView, CommandError> {
    info!("Fetching depot view");

    let mut unit_of_work = state.unit_of_work().await?;

    let depot_view = GetDepot::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await?;

    Ok(depot_view)
}

/// Tauri command to retrieve the current depot view: which is the list
/// of rolling stocks part of the collection.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `DepotView` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(DepotView)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_depot(state: tauri::State<'_, AppState>) -> Result<DepotView, CommandError> {
    get_depot_inner(&state).await
}

fn parse_remove_collection_item_input(
    args: RemoveCollectionItemArgs,
) -> Result<DomainRemoveCollectionItemInput, CommandError> {
    args.validate().map_err(CommandError::from)?;

    let collection_item_id = CollectionItemId::try_from(args.collection_item_id)
        .map_err(|_| CommandError::validation_field("collection_item_id", "invalid"))?;

    let category = args
        .category
        .parse::<Category>()
        .map_err(|_| CommandError::validation_field("category", "invalid"))?;

    let removed_date = NaiveDate::parse_from_str(&args.removed_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("removed_date", "invalid"))?;

    Ok(DomainRemoveCollectionItemInput {
        collection_item_id,
        category,
        removed_date,
    })
}

pub async fn remove_collection_item_inner(
    state: &AppState,
    args: RemoveCollectionItemArgs,
) -> Result<CollectionItemId, CommandError> {
    info!("Removing collection item: {:?}", args);
    let domain_cmd = parse_remove_collection_item_input(args)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let removed_id = RemoveCollectionItem::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await?;

    Ok(removed_id)
}

/// Tauri command to remove an item from the collection.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the removed `CollectionItemId` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `args`: Input parameters for removing the collection item.
///
/// Returns:
/// - `Ok(CollectionItemId)` when removal succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn remove_collection_item(
    state: tauri::State<'_, AppState>,
    args: RemoveCollectionItemArgs,
) -> Result<CollectionItemId, CommandError> {
    remove_collection_item_inner(&state, args).await
}

pub async fn sell_collection_item_inner(
    state: &AppState,
    args: SellCollectionItemArgs,
) -> Result<(), CommandError> {
    info!("Selling collection item: {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let collection_item_id = CollectionItemId::try_from(args.item_id)
        .map_err(|_| CommandError::validation_field("item_id", "invalid"))?;

    let sale_date = NaiveDate::parse_from_str(&args.sale_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("sale_date", "invalid"))?;

    let currency = Currency::from_code(&args.currency)
        .map_err(|_| CommandError::validation_field("currency", "invalid"))?;

    let sale_price = MonetaryAmount::new(args.amount, currency);

    let domain_input = DomainSellCollectionItemInput {
        collection_item_id,
        sale_date,
        sale_price,
        buyer_id: args.buyer_id,
    };

    let mut unit_of_work = state.unit_of_work().await?;
    SellCollectionItem::execute(&mut unit_of_work, domain_input).await?;
    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to sell an item from the collection.
#[tauri::command]
#[specta::specta]
pub async fn sell_collection_item(
    state: tauri::State<'_, AppState>,
    args: SellCollectionItemArgs,
) -> Result<(), CommandError> {
    sell_collection_item_inner(&state, args).await
}

pub async fn update_collection_item_inner(
    state: &AppState,
    args: UpdateCollectionItemArgs,
) -> Result<(), CommandError> {
    info!("Updating collection item: {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let collection_item_id = parse_collection_item_id(args.collection_item_id)?;

    let update = match args.update {
        CollectionItemUpdateArgs::Seller { seller_id } => {
            CollectionItemUpdate::Seller(parse_optional_seller_id(seller_id)?)
        }
        CollectionItemUpdateArgs::Price { amount, currency } => {
            CollectionItemUpdate::Price(parse_optional_price(amount, currency)?)
        }
        CollectionItemUpdateArgs::PurchaseDate { purchase_date } => {
            CollectionItemUpdate::PurchaseDate(purchase_date)
        }
        CollectionItemUpdateArgs::AddedDate { added_date } => {
            CollectionItemUpdate::AddedDate(added_date)
        }
        CollectionItemUpdateArgs::Notes { notes } => CollectionItemUpdate::Notes(notes),
        CollectionItemUpdateArgs::PurchaseCondition { purchase_condition } => {
            CollectionItemUpdate::PurchaseCondition(parse_optional_purchase_condition(
                purchase_condition,
            )?)
        }
        CollectionItemUpdateArgs::ModelCondition { model_condition } => {
            CollectionItemUpdate::ModelCondition(parse_optional_model_condition(model_condition)?)
        }
        CollectionItemUpdateArgs::BoxCondition { box_condition } => {
            CollectionItemUpdate::BoxCondition(parse_optional_box_condition(box_condition)?)
        }
    };

    let mut unit_of_work = state.unit_of_work().await?;
    let input = UpdateCollectionItemInput {
        collection_item_id,
        update,
    };

    UpdateCollectionItem::execute(&mut unit_of_work, input).await?;
    unit_of_work.commit().await?;

    Ok(())
}

/// Tauri command to update mutable fields of an existing collection item.
#[tauri::command]
#[specta::specta]
pub async fn update_collection_item(
    state: tauri::State<'_, AppState>,
    args: UpdateCollectionItemArgs,
) -> Result<(), CommandError> {
    update_collection_item_inner(&state, args).await
}

pub async fn add_collection_item_inner(
    state: &AppState,
    args: AddCollectionItemArgs,
) -> Result<CollectionItemId, CommandError> {
    info!("Adding collection item: {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let domain_cmd = match DomainAddCollectionItemInput::try_from(args) {
        Ok(v) => v,
        Err(e) => return Err(CommandError::from(e)),
    };
    let mut unit_of_work = state.unit_of_work().await?;

    let id_provider = RuntimeIdProvider::new();
    let purchase_info_provider = RuntimeIdProvider::new();

    let item_id = AddCollectionItem::execute(
        &mut unit_of_work,
        id_provider,
        purchase_info_provider,
        domain_cmd,
    )
    .await?;

    unit_of_work.commit().await?;

    Ok(item_id)
}

/// Tauri command to add a new item to the collection.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the newly created `CollectionItemId` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `args`: Input parameters for adding the collection item.
///
/// Returns:
/// - `Ok(CollectionItemId)` when addition succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn add_collection_item(
    state: tauri::State<'_, AppState>,
    args: AddCollectionItemArgs,
) -> Result<CollectionItemId, CommandError> {
    add_collection_item_inner(&state, args).await
}

pub async fn add_railway_model_to_collection_inner(
    state: &AppState,
    args: AddRailwayModelToCollectionArgs,
) -> Result<(), CommandError> {
    info!("add_railway_model_to_collection (collecting): {:?}", args);

    args.validate().map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let save_input: SaveRailwayModelInput = args.railway_model.try_into()?;

    let railway_model_id = SaveRailwayModel::execute(&mut unit_of_work, save_input).await?;

    let price = parse_price(&args.price_currency, args.price_amount)?;
    let seller_id = parse_optional_seller_id(args.seller_id.clone())?;
    let purchase_condition = parse_optional_purchase_condition(args.purchase_condition.clone())?;
    let model_condition = parse_optional_model_condition(args.model_condition.clone())?;
    let box_condition = parse_optional_box_condition(args.box_condition.clone())?;

    let add_input = DomainAddCollectionItemInput {
        railway_model_id: railway_model_id.clone(), // Clone required: railway_model_id used in execute above and needed here
        price,
        seller_id,
        added_date: args.added_date,
        purchase_date: args.purchase_date,
        purchase_condition,
        model_condition,
        box_condition,
        notes: args.notes,
    };

    let id_provider = RuntimeIdProvider::new();
    let purchase_info_provider = RuntimeIdProvider::new();

    let item_id = AddCollectionItem::execute(
        &mut unit_of_work,
        id_provider,
        purchase_info_provider,
        add_input,
    )
    .await?;

    // If the user added a preorder, patch the purchase_info row with preorder details
    let purchase_type = args.purchase_type.as_deref().unwrap_or("STANDARD");
    if purchase_type == "PREORDER" {
        let deposit_amount = args.deposit_amount.unwrap_or(0);
        let deposit_currency = args
            .deposit_currency
            .as_deref()
            .unwrap_or(args.price_currency.as_str());
        let preorder_total_amount = args.preorder_total_amount.unwrap_or(args.price_amount);
        let preorder_total_currency = args
            .preorder_total_currency
            .as_deref()
            .unwrap_or(args.price_currency.as_str());

        unit_of_work
            .collections_repository()
            .convert_to_preorder(
                &item_id,
                deposit_amount,
                deposit_currency,
                preorder_total_amount,
                preorder_total_currency,
                args.expected_date,
            )
            .await?;
    }

    unit_of_work.commit().await?;

    Ok(())
}

fn parse_collection_item_id(raw: String) -> Result<CollectionItemId, CommandError> {
    CollectionItemId::try_from(raw)
        .map_err(|_| CommandError::validation_field("collection_item_id", "invalid"))
}

fn parse_optional_seller_id(raw: Option<String>) -> Result<Option<SellerId>, CommandError> {
    match raw {
        Some(value) => SellerId::try_from(value.as_str())
            .map(Some)
            .map_err(|_| CommandError::validation_field("seller_id", "invalid")),
        None => Ok(None),
    }
}

fn parse_price(price_currency: &str, price_amount: i64) -> Result<MonetaryAmount, CommandError> {
    let currency = Currency::from_code(price_currency)
        .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;
    Ok(MonetaryAmount::new(price_amount, currency))
}

fn parse_optional_price(
    amount: Option<i64>,
    currency: Option<String>,
) -> Result<Option<MonetaryAmount>, CommandError> {
    match (amount, currency) {
        (Some(raw_amount), Some(raw_currency)) => {
            if raw_amount < 0 {
                return Err(CommandError::validation_field("amount", "must_be_positive"));
            }
            let parsed_currency = Currency::from_code(&raw_currency)
                .map_err(|_| CommandError::validation_field("currency", "invalid"))?;
            Ok(Some(MonetaryAmount::new(raw_amount, parsed_currency)))
        }
        (None, None) => Ok(None),
        _ => Err(CommandError::validation_field(
            "price",
            "amount_and_currency_must_be_both_present_or_null",
        )),
    }
}

fn parse_optional_purchase_condition(
    raw: Option<String>,
) -> Result<Option<PurchaseCondition>, CommandError> {
    match raw {
        Some(value) => value
            .parse::<PurchaseCondition>()
            .map(Some)
            .map_err(|_| CommandError::validation_field("purchase_condition", "invalid")),
        None => Ok(None),
    }
}

fn parse_optional_model_condition(
    raw: Option<String>,
) -> Result<Option<ModelCondition>, CommandError> {
    match raw {
        Some(value) => value
            .parse::<ModelCondition>()
            .map(Some)
            .map_err(|_| CommandError::validation_field("model_condition", "invalid")),
        None => Ok(None),
    }
}

fn parse_optional_box_condition(raw: Option<String>) -> Result<Option<BoxCondition>, CommandError> {
    match raw {
        Some(value) => value
            .parse::<BoxCondition>()
            .map(Some)
            .map_err(|_| CommandError::validation_field("box_condition", "invalid")),
        None => Ok(None),
    }
}

/// Simplified flow: save (merge) the railway model and add it to the default collection.
#[tauri::command]
#[specta::specta]
pub async fn add_railway_model_to_collection(
    state: tauri::State<'_, AppState>,
    args: AddRailwayModelToCollectionArgs,
) -> Result<(), CommandError> {
    add_railway_model_to_collection_inner(&state, args).await
}

pub async fn get_collection_stats_inner(state: &AppState) -> Result<CollectionStats, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;
    let stats = unit_of_work.collections_repository().get_stats().await?;
    unit_of_work.commit().await?;
    Ok(stats)
}

#[tauri::command]
#[specta::specta]
pub async fn get_collection_stats(
    state: tauri::State<'_, AppState>,
) -> Result<CollectionStats, CommandError> {
    get_collection_stats_inner(&state).await
}

pub async fn receive_preorder_inner(
    state: &AppState,
    args: ReceivePreorderArgs,
) -> Result<(), CommandError> {
    info!("receive_preorder (collecting): {:?}", args);
    args.validate().map_err(CommandError::from)?;

    let item_id = CollectionItemId::try_from(args.item_id.as_str())
        .map_err(|_| CommandError::validation_field("item_id", "invalid"))?;
    let received_date = NaiveDate::parse_from_str(&args.received_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("received_date", "invalid date format"))?;

    let mut uow = state.unit_of_work().await?;
    ReceivePreorder::execute(
        &mut uow,
        ReceivePreorderInput {
            collection_item_id: item_id,
            received_date,
        },
    )
    .await?;
    uow.commit().await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn receive_preorder(
    state: tauri::State<'_, AppState>,
    args: ReceivePreorderArgs,
) -> Result<(), CommandError> {
    receive_preorder_inner(&state, args).await
}

pub async fn record_acquisition_inner(
    state: &AppState,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError> {
    info!("Recording acquisition: {} items", args.items.len());

    args.validate().map_err(CommandError::from)?;

    let purchase_date = parse_purchase_date(&args.purchase_date)?;
    let seller_id = parse_optional_acquisition_seller_id(args.seller_id)?;
    let items = parse_acquisition_items(args.items)?;

    let input = RecordAcquisitionInput {
        seller_id,
        purchase_date,
        items,
    };

    let mut unit_of_work = state.unit_of_work().await?;

    let collection_item_id_provider = RuntimeIdProvider::new();
    let purchase_info_id_provider = RuntimeIdProvider::new();

    let ids = RecordAcquisition::execute(
        &mut unit_of_work,
        collection_item_id_provider,
        purchase_info_id_provider,
        input,
    )
    .await?;

    unit_of_work.commit().await?;

    Ok(ids)
}

fn parse_purchase_date(raw: &str) -> Result<NaiveDate, CommandError> {
    let purchase_date = NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("purchase_date", "invalid date format"))?;
    let today = chrono::Local::now().date_naive();
    if purchase_date > today {
        return Err(CommandError::validation_field(
            "purchase_date",
            "purchase date cannot be in the future",
        ));
    }

    Ok(purchase_date)
}

fn parse_optional_acquisition_seller_id(
    raw: Option<String>,
) -> Result<Option<SellerId>, CommandError> {
    match raw {
        Some(value) => SellerId::try_from(value.as_str())
            .map(Some)
            .map_err(|_| CommandError::validation_field("seller_id", "invalid")),
        None => Ok(None),
    }
}

fn parse_acquisition_items(
    items: Vec<AcquisitionItemArgs>,
) -> Result<Vec<AcquisitionItemInput>, CommandError> {
    items.into_iter().map(parse_acquisition_item).collect()
}

fn parse_acquisition_item(item: AcquisitionItemArgs) -> Result<AcquisitionItemInput, CommandError> {
    let manufacturer_id = ManufacturerId::try_from(item.manufacturer_id.as_str())
        .map_err(|_| CommandError::validation_field("manufacturer_id", "invalid"))?;

    let category = item
        .category
        .parse::<Category>()
        .map_err(|_| CommandError::validation_field("category", "invalid"))?;

    let scale = Scale::try_from(item.scale.as_str())
        .map_err(|_| CommandError::validation_field("scale", "invalid"))?;

    let power_method = item
        .power_method
        .parse::<PowerMethod>()
        .map_err(|_| CommandError::validation_field("power_method", "invalid"))?;

    let currency = Currency::from_code(&item.price_currency)
        .map_err(|_| CommandError::validation_field("price_currency", "invalid"))?;

    Ok(AcquisitionItemInput {
        manufacturer_id,
        product_code: item.product_code,
        description: item.description,
        category,
        scale,
        epoch: Epoch(item.epoch),
        power_method,
        price: MonetaryAmount::new(item.price_amount, currency),
    })
}

/// Tauri command to record a batch acquisition: upsert catalog entries and add collection items.
#[tauri::command]
#[specta::specta]
pub async fn record_acquisition(
    state: tauri::State<'_, AppState>,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError> {
    record_acquisition_inner(&state, args).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::interface::{SimplifiedRailwayModelArgs, SimplifiedRollingStockArgs};
    use chrono::NaiveDate;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    // ── remove_collection_item_inner ─────────────────────────────────────

    #[sqlx::test(migrations = "./migrations")]
    async fn remove_collection_item_empty_id_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = RemoveCollectionItemArgs {
            collection_item_id: String::new(),
            category: "LOCOMOTIVES".to_string(),
            removed_date: "2025-01-01".to_string(),
        };
        let result = remove_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn remove_collection_item_invalid_category_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = RemoveCollectionItemArgs {
            collection_item_id: "trn:collection-item:some-id".to_string(),
            category: "NOT_A_CATEGORY".to_string(),
            removed_date: "2025-01-01".to_string(),
        };
        let result = remove_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    // ── update_collection_item_inner ────────────────────────────────────

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_empty_id_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: String::new(),
            update: CollectionItemUpdateArgs::Notes { notes: None },
        };
        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_invalid_seller_id_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:some-id".to_string(),
            update: CollectionItemUpdateArgs::Seller {
                seller_id: Some("not-a-valid-trn".to_string()),
            },
        };
        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn update_collection_item_price_amount_without_currency_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .to_string(),
            update: CollectionItemUpdateArgs::Price {
                amount: Some(1000),
                currency: None,
            },
        };
        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn update_collection_item_price_currency_without_amount_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .to_string(),
            update: CollectionItemUpdateArgs::Price {
                amount: None,
                currency: Some("EUR".to_string()),
            },
        };
        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn update_collection_item_notes_persists(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730"
                .to_string(),
            update: CollectionItemUpdateArgs::Notes {
                notes: Some("Updated notes from test".to_string()),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(result.is_ok(), "Expected success, got: {:?}", result);

        let notes: Option<String> =
            sqlx::query_scalar("SELECT notes FROM collection_items WHERE id = ?1")
                .bind("trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730")
                .fetch_one(&pool)
                .await
                .expect("notes should be queryable");

        assert_eq!(notes.as_deref(), Some("Updated notes from test"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_purchase_date_with_missing_item_returns_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:11111111-1111-1111-1111-111111111111"
                .to_string(),
            update: CollectionItemUpdateArgs::PurchaseDate {
                purchase_date: NaiveDate::from_ymd_opt(2025, 2, 1),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(
            result.is_err(),
            "Expected error for missing item, got success"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_added_date_with_missing_item_returns_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:22222222-2222-2222-2222-222222222222"
                .to_string(),
            update: CollectionItemUpdateArgs::AddedDate {
                added_date: NaiveDate::from_ymd_opt(2025, 2, 2),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(
            result.is_err(),
            "Expected error for missing item, got success"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_invalid_purchase_condition_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:33333333-3333-3333-3333-333333333333"
                .to_string(),
            update: CollectionItemUpdateArgs::PurchaseCondition {
                purchase_condition: Some("UNKNOWN".to_string()),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_invalid_model_condition_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:44444444-4444-4444-4444-444444444444"
                .to_string(),
            update: CollectionItemUpdateArgs::ModelCondition {
                model_condition: Some("BROKEN".to_string()),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn update_collection_item_invalid_box_condition_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);
        let args = UpdateCollectionItemArgs {
            collection_item_id: "trn:collection-item:55555555-5555-5555-5555-555555555555"
                .to_string(),
            update: CollectionItemUpdateArgs::BoxCondition {
                box_condition: Some("WORN".to_string()),
            },
        };

        let result = update_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    // ── add_collection_item_inner ────────────────────────────────────────

    #[sqlx::test(migrations = "./migrations")]
    async fn add_collection_item_bad_currency_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = AddCollectionItemArgs {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            price_amount: 1000,
            price_currency: "TOOSHORT".to_string(), // Must be exactly 3 chars — "TOO SHORT"
            seller_id: None,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: None,
        };
        let result = add_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn add_collection_item_negative_price_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let args = AddCollectionItemArgs {
            railway_model_id: "trn:railway-model:acme:60100".to_string(),
            price_amount: -1,
            price_currency: "EUR".to_string(),
            seller_id: None,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            purchase_condition: None,
            model_condition: None,
            box_condition: None,
            notes: None,
        };
        let result = add_collection_item_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            "../../../fixtures/test_collection.sql",
            "../../../fixtures/test_seller.sql"
        )
    )]
    async fn add_railway_model_to_collection_happy_path_inserts_item(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let args = AddRailwayModelToCollectionArgs {
            railway_model: SimplifiedRailwayModelArgs {
                manufacturer_id: "trn:manufacturer:acme".to_string(),
                product_code: "NEW-100".to_string(),
                description: "New model from handler test".to_string(),
                category: "LOCOMOTIVES".to_string(),
                scale: "H0".to_string(),
                epoch: "IV".to_string(),
                power_method: "DC".to_string(),
                rolling_stocks: vec![SimplifiedRollingStockArgs {
                    railway_company_id: "trn:railway-company:fs".to_string(),
                    series_code: "E.999".to_string(),
                    road_number: Some("E.999.001".to_string()),
                    subcategory: Some("ELECTRIC_LOCOMOTIVE".to_string()),
                    category: "LOCOMOTIVE".to_string(),
                }],
            },
            price_amount: 25000,
            price_currency: "EUR".to_string(),
            seller_id: Some("trn:seller:model-train-shop".to_string()),
            added_date: NaiveDate::from_ymd_opt(2025, 2, 1).expect("valid date"),
            purchase_date: NaiveDate::from_ymd_opt(2025, 2, 1).expect("valid date"),
            purchase_condition: Some("NEW".to_string()),
            model_condition: Some("MINT".to_string()),
            box_condition: Some("ORIGINAL_MINT".to_string()),
            notes: Some("Created from add_railway_model_to_collection_inner test".to_string()),
            purchase_type: None,
            deposit_amount: None,
            deposit_currency: None,
            preorder_total_amount: None,
            preorder_total_currency: None,
            expected_date: None,
        };

        let result = add_railway_model_to_collection_inner(&state, args).await;
        assert!(result.is_ok(), "Expected success, got: {:?}", result);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(1) FROM collection_items WHERE added_date = ?1")
                .bind("2025-02-01")
                .fetch_one(&pool)
                .await
                .expect("collection_items should be queryable");

        assert!(count >= 1, "expected at least one inserted collection item");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures(
            "../../../fixtures/test_collection.sql",
            "../../../fixtures/test_seller.sql"
        )
    )]
    async fn add_railway_model_to_collection_preorder_updates_purchase_type(pool: SqlitePool) {
        let state = app_state(pool.clone());
        let args = AddRailwayModelToCollectionArgs {
            railway_model: SimplifiedRailwayModelArgs {
                manufacturer_id: "trn:manufacturer:acme".to_string(),
                product_code: "NEW-101".to_string(),
                description: "Preorder model from handler test".to_string(),
                category: "LOCOMOTIVES".to_string(),
                scale: "H0".to_string(),
                epoch: "IV".to_string(),
                power_method: "DC".to_string(),
                rolling_stocks: vec![SimplifiedRollingStockArgs {
                    railway_company_id: "trn:railway-company:fs".to_string(),
                    series_code: "E.998".to_string(),
                    road_number: Some("E.998.001".to_string()),
                    subcategory: Some("ELECTRIC_LOCOMOTIVE".to_string()),
                    category: "LOCOMOTIVE".to_string(),
                }],
            },
            price_amount: 1000,
            price_currency: "EUR".to_string(),
            seller_id: Some("trn:seller:model-train-shop".to_string()),
            added_date: NaiveDate::from_ymd_opt(2025, 3, 1).expect("valid date"),
            purchase_date: NaiveDate::from_ymd_opt(2025, 3, 1).expect("valid date"),
            purchase_condition: Some("NEW".to_string()),
            model_condition: Some("MINT".to_string()),
            box_condition: Some("ORIGINAL_MINT".to_string()),
            notes: Some("Preorder test".to_string()),
            purchase_type: Some("PREORDER".to_string()),
            deposit_amount: Some(200),
            deposit_currency: Some("EUR".to_string()),
            preorder_total_amount: Some(1000),
            preorder_total_currency: Some("EUR".to_string()),
            expected_date: Some(NaiveDate::from_ymd_opt(2026, 1, 1).expect("valid date")),
        };

        let result = add_railway_model_to_collection_inner(&state, args).await;
        assert!(result.is_ok(), "Expected success, got: {:?}", result);

        let purchase_type: Option<String> = sqlx::query_scalar(
            "SELECT purchase_type FROM purchase_infos ORDER BY ROWID DESC LIMIT 1",
        )
        .fetch_one(&pool)
        .await
        .expect("purchase_infos should be queryable");

        assert_eq!(purchase_type.as_deref(), Some("PREORDER"));
    }

    // ── record_acquisition_inner ───────────────────────────────────────

    fn valid_record_acquisition_args() -> RecordAcquisitionArgs {
        RecordAcquisitionArgs {
            seller_id: None,
            purchase_date: "2025-06-01".to_string(),
            items: vec![AcquisitionItemArgs {
                manufacturer_id: "trn:manufacturer:acme".to_string(),
                product_code: "60100".to_string(),
                description: "Steam locomotive".to_string(),
                category: "LOCOMOTIVES".to_string(),
                scale: "H0".to_string(),
                epoch: "IV".to_string(),
                power_method: "DC".to_string(),
                price_amount: 5000,
                price_currency: "EUR".to_string(),
            }],
        }
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn get_collection_item_details_invalid_model_id_returns_validation_error(
        pool: SqlitePool,
    ) {
        let state = app_state(pool);

        let result = get_collection_item_details_inner(
            &state,
            "not-a-railway-model-id".to_string(),
            None,
            Language::English,
        )
        .await;

        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn sell_collection_item_valid_args_reach_domain_layer(pool: SqlitePool) {
        let state = app_state(pool);
        let args = SellCollectionItemArgs {
            item_id: "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730".to_string(),
            sale_date: "2025-06-01".to_string(),
            amount: 5000,
            currency: "EUR".to_string(),
            buyer_id: None,
        };

        let result = sell_collection_item_inner(&state, args).await;
        assert!(
            !matches!(result, Err(CommandError::ValidationError(_))),
            "Expected non-validation result, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_collection.sql")
    )]
    async fn receive_preorder_valid_args_reach_domain_layer(pool: SqlitePool) {
        let state = app_state(pool);
        let args = ReceivePreorderArgs {
            item_id: "trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730".to_string(),
            received_date: "2025-06-01".to_string(),
        };

        let result = receive_preorder_inner(&state, args).await;
        assert!(
            !matches!(result, Err(CommandError::ValidationError(_))),
            "Expected non-validation result, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn record_acquisition_invalid_date_format_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let mut args = valid_record_acquisition_args();
        args.purchase_date = "06-01-2025".to_string();

        let result = record_acquisition_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn record_acquisition_future_date_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let mut args = valid_record_acquisition_args();
        args.purchase_date = "2099-12-31".to_string();

        let result = record_acquisition_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn record_acquisition_invalid_seller_id_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let mut args = valid_record_acquisition_args();
        args.seller_id = Some("not-a-trn".to_string());

        let result = record_acquisition_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn record_acquisition_invalid_category_returns_validation_error(pool: SqlitePool) {
        let state = app_state(pool);
        let mut args = valid_record_acquisition_args();
        args.items[0].category = "NOT_A_CATEGORY".to_string();

        let result = record_acquisition_inner(&state, args).await;
        assert!(
            matches!(result, Err(CommandError::ValidationError(_))),
            "Expected ValidationError, got: {:?}",
            result
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_railway_model.sql")
    )]
    async fn record_acquisition_happy_path_returns_created_ids(pool: SqlitePool) {
        let state = app_state(pool);
        let args = valid_record_acquisition_args();

        let result = record_acquisition_inner(&state, args)
            .await
            .expect("record acquisition should succeed");

        assert_eq!(result.len(), 1);
    }
}
