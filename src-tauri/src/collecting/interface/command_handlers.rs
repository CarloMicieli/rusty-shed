use crate::catalog::domain::railway_model::Category;
use crate::collecting::application::AddCollectionItemInput as DomainAddCollectionItemInput;
use crate::collecting::application::{
    AddCollectionItemUseCase, GetCollectionQuery, GetDepotQuery,
    RemoveCollectionItemInput as DomainRemoveCollectionItemInput, RemoveCollectionItemUseCase,
};
use crate::collecting::domain::{CollectionItemId, CollectionView, DepotView};
use crate::collecting::interface::{AddCollectionItemArgs, RemoveCollectionItemArgs};
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::runtime_id_provider::RuntimeIdProvider;
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

    let collection = GetCollectionQuery::execute(&mut unit_of_work).await?;
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

    let depot_view = GetDepotQuery::execute(&mut unit_of_work).await?;
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

    let removed_id = RemoveCollectionItemUseCase::execute(&mut unit_of_work, domain_cmd).await?;
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

    let domain_cmd = DomainAddCollectionItemInput::try_from(args).map_err(CommandError::from)?;
    let mut unit_of_work = state.unit_of_work().await?;

    let id_provider = RuntimeIdProvider::new();
    let purchase_info_provider = RuntimeIdProvider::new();

    let item_id = AddCollectionItemUseCase::execute(
        &mut unit_of_work,
        id_provider,
        purchase_info_provider,
        domain_cmd,
    )
    .await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(item_id)
}
