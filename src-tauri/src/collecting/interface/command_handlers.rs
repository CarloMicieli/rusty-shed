use crate::catalog::application::{SaveRailwayModel, SaveRailwayModelInput};
use crate::catalog::domain::railway_model::Category;
use crate::collecting::application::AddCollectionItemInput as DomainAddCollectionItemInput;
use crate::collecting::application::{
    AddCollectionItem, GetCollection, GetDepot, RemoveCollectionItem,
    RemoveCollectionItemInput as DomainRemoveCollectionItemInput,
};
use crate::collecting::domain::{BoxCondition, ModelCondition, PurchaseCondition};
use crate::collecting::domain::{CollectionItemId, CollectionView, DepotView};
use crate::collecting::interface::command_args::AddRailwayModelToCollectionArgs;
use crate::collecting::interface::{AddCollectionItemArgs, RemoveCollectionItemArgs};
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::{Currency, MonetaryAmount};
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
use crate::sellers::domain::seller_id::SellerId;
use crate::state::AppState;
use chrono::NaiveDate;
use log::info;
use std::convert::TryFrom;

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
    info!("Fetching collection");

    let mut unit_of_work = state.unit_of_work().await?;

    let collection = GetCollection::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(collection)
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
    info!("Fetching depot view");

    let mut unit_of_work = state.unit_of_work().await?;

    let depot_view = GetDepot::execute(&mut unit_of_work).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(depot_view)
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
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(removed_id)
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

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(item_id)
}

/// Simplified flow: save (merge) the railway model and add it to the default collection.
#[tauri::command]
#[specta::specta]
pub async fn add_railway_model_to_collection(
    state: tauri::State<'_, AppState>,
    args: AddRailwayModelToCollectionArgs,
) -> Result<(), CommandError> {
    info!("add_railway_model_to_collection (collecting): {:?}", args);

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
        railway_model_id: railway_model_id.clone(),
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

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(())
}
