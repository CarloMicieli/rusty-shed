use crate::catalog::application::{SaveRailwayModel, SaveRailwayModelInput};
use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::catalog::domain::railway_model::{Category, Epoch, PowerMethod};
use crate::catalog::domain::scale::Scale;
use crate::collecting::application::AddCollectionItemInput as DomainAddCollectionItemInput;
use crate::collecting::application::{
    AcquisitionItemInput, AddCollectionItem, GetCollection, GetDepot, RecordAcquisition,
    RecordAcquisitionInput, RemoveCollectionItem,
    RemoveCollectionItemInput as DomainRemoveCollectionItemInput, UpdateCollectionItem,
};
use crate::collecting::domain::{
    BoxCondition, CollectionItemUpdate, ModelCondition, PurchaseCondition,
    UpdateCollectionItemInput,
};
use crate::collecting::domain::{CollectionItemId, CollectionView, DepotView};
use crate::collecting::interface::command_args::{
    AddRailwayModelToCollectionArgs, RecordAcquisitionArgs,
};
use crate::collecting::interface::{
    AddCollectionItemArgs, CollectionItemUpdateArgs, RemoveCollectionItemArgs,
    UpdateCollectionItemArgs,
};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::sellers::domain::seller_id::SellerId;
use crate::state::AppState;
use chrono::NaiveDate;
use garde::Validate;
use log::info;
use std::convert::TryFrom;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

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

pub async fn remove_collection_item_inner(
    state: &AppState,
    args: RemoveCollectionItemArgs,
) -> Result<CollectionItemId, CommandError> {
    info!("Removing collection item: {:?}", args);

    let collection_item_id = CollectionItemId::try_from(args.collection_item_id)
        .map_err(|_| CommandError::validation_field("collection_item_id", "invalid"))?;

    let category = args
        .category
        .parse::<Category>()
        .map_err(|_| CommandError::validation_field("category", "invalid"))?;

    let removed_date = NaiveDate::parse_from_str(&args.removed_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("removed_date", "invalid"))?;

    let domain_cmd = DomainRemoveCollectionItemInput {
        collection_item_id,
        category,
        removed_date,
    };

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

pub async fn update_collection_item_inner(
    state: &AppState,
    args: UpdateCollectionItemArgs,
) -> Result<(), CommandError> {
    info!("Updating collection item: {:?}", args);

    let collection_item_id = CollectionItemId::try_from(args.collection_item_id)
        .map_err(|_| CommandError::validation_field("collection_item_id", "invalid"))?;

    let update = match args.update {
        CollectionItemUpdateArgs::Seller { seller_id } => {
            let parsed_seller = match seller_id {
                Some(raw) => Some(
                    SellerId::try_from(raw.as_str())
                        .map_err(|_| CommandError::validation_field("seller_id", "invalid"))?,
                ),
                None => None,
            };
            CollectionItemUpdate::Seller(parsed_seller)
        }
        CollectionItemUpdateArgs::Price { amount, currency } => {
            let price = match (amount, currency) {
                (Some(raw_amount), Some(raw_currency)) => {
                    if raw_amount < 0 {
                        return Err(CommandError::validation_field("amount", "must_be_positive"));
                    }
                    let parsed_currency = Currency::from_code(&raw_currency)
                        .map_err(|_| CommandError::validation_field("currency", "invalid"))?;
                    Some(MonetaryAmount::new(raw_amount, parsed_currency))
                }
                (None, None) => None,
                _ => {
                    return Err(CommandError::validation_field(
                        "price",
                        "amount_and_currency_must_be_both_present_or_null",
                    ));
                }
            };
            CollectionItemUpdate::Price(price)
        }
        CollectionItemUpdateArgs::PurchaseDate { purchase_date } => {
            CollectionItemUpdate::PurchaseDate(purchase_date)
        }
        CollectionItemUpdateArgs::AddedDate { added_date } => {
            CollectionItemUpdate::AddedDate(added_date)
        }
        CollectionItemUpdateArgs::Notes { notes } => CollectionItemUpdate::Notes(notes),
        CollectionItemUpdateArgs::PurchaseCondition { purchase_condition } => {
            let parsed = match purchase_condition {
                Some(value) => Some(value.parse::<PurchaseCondition>().map_err(|_| {
                    CommandError::validation_field("purchase_condition", "invalid")
                })?),
                None => None,
            };
            CollectionItemUpdate::PurchaseCondition(parsed)
        }
        CollectionItemUpdateArgs::ModelCondition { model_condition } => {
            let parsed =
                match model_condition {
                    Some(value) => Some(value.parse::<ModelCondition>().map_err(|_| {
                        CommandError::validation_field("model_condition", "invalid")
                    })?),
                    None => None,
                };
            CollectionItemUpdate::ModelCondition(parsed)
        }
        CollectionItemUpdateArgs::BoxCondition { box_condition } => {
            let parsed = match box_condition {
                Some(value) => Some(
                    value
                        .parse::<BoxCondition>()
                        .map_err(|_| CommandError::validation_field("box_condition", "invalid"))?,
                ),
                None => None,
            };
            CollectionItemUpdate::BoxCondition(parsed)
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

    // Convert price
    let currency = Currency::from_code(&args.price_currency)
        .map_err(|e| CommandError::from(DomainError::Validation(e.to_string())))?;
    let price = MonetaryAmount::new(args.price_amount, currency);

    // Seller id
    let seller_id: Option<SellerId> = match args.seller_id {
        Some(s) => Some(
            SellerId::try_from(s.as_str())
                .map_err(|_| CommandError::validation_field("seller_id", "invalid"))?,
        ),
        None => None,
    };

    // Parse enums
    let purchase_condition = match args.purchase_condition {
        Some(s) => Some(
            s.parse::<PurchaseCondition>()
                .map_err(|_| CommandError::validation_field("purchase_condition", "invalid"))?,
        ),
        None => None,
    };

    let model_condition = match args.model_condition {
        Some(s) => Some(
            s.parse::<ModelCondition>()
                .map_err(|_| CommandError::validation_field("model_condition", "invalid"))?,
        ),
        None => None,
    };

    let box_condition = match args.box_condition {
        Some(s) => Some(
            s.parse::<BoxCondition>()
                .map_err(|_| CommandError::validation_field("box_condition", "invalid"))?,
        ),
        None => None,
    };

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

    AddCollectionItem::execute(
        &mut unit_of_work,
        id_provider,
        purchase_info_provider,
        add_input,
    )
    .await?;

    unit_of_work.commit().await?;

    Ok(())
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

pub async fn record_acquisition_inner(
    state: &AppState,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError> {
    info!("Recording acquisition: {} items", args.items.len());

    args.validate().map_err(CommandError::from)?;

    let purchase_date = NaiveDate::parse_from_str(&args.purchase_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("purchase_date", "invalid date format"))?;
    let today = chrono::Local::now().date_naive();
    if purchase_date > today {
        return Err(CommandError::validation_field(
            "purchase_date",
            "purchase date cannot be in the future",
        ));
    }

    let seller_id: Option<SellerId> = match args.seller_id {
        Some(s) => Some(
            SellerId::try_from(s.as_str())
                .map_err(|_| CommandError::validation_field("seller_id", "invalid"))?,
        ),
        None => None,
    };

    let items = args
        .items
        .into_iter()
        .map(|item| {
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
        })
        .collect::<Result<Vec<_>, CommandError>>()?;

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

/// Tauri command to record a batch acquisition: upsert catalog entries and add collection items.
#[tauri::command]
#[specta::specta]
pub async fn record_acquisition(
    state: tauri::State<'_, AppState>,
    args: RecordAcquisitionArgs,
) -> Result<Vec<CollectionItemId>, CommandError> {
    record_acquisition_inner(&state, args).await
}
