use crate::catalog::domain::railway_model::Category;
use crate::collecting::application::{
    AddCollectionItemCommand, GetCollectionQuery, GetDepotQuery, RemoveCollectionItemCommand,
};
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::RemoveCollectionItem;
use crate::collecting::domain::{AddCollectionItem, CollectionView, DepotView};
use crate::collecting::interface::{AddCollectionItemInput, RemoveCollectionItemInput};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use chrono::NaiveDate;
use log::{debug, info};
use std::convert::TryFrom;

/// Tauri command to retrieve the current collection.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `Collection` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(Collection)` when retrieval succeeds.
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

    debug!("Collection: {:?}", collection);
    
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
/// asynchronously and returns the updated `CollectionView` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: Input parameters for removing the collection item.
///
/// Returns:
/// - `Ok(CollectionView)` when removal succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn remove_collection_item(
    state: tauri::State<'_, AppState>,
    input: RemoveCollectionItemInput,
) -> Result<CollectionView, CommandError> {
    info!("Removing collection item: {:?}", input);

    let collection_item_id = CollectionItemId::try_from(input.collection_item_id)
        .map_err(|_| CommandError::validation_field("collection_item_id", "invalid"))?;

    let category = input
        .category
        .parse::<Category>()
        .map_err(|_| CommandError::validation_field("category", "invalid"))?;

    let removed_date = NaiveDate::parse_from_str(&input.removed_date, "%Y-%m-%d")
        .map_err(|_| CommandError::validation_field("removed_date", "invalid"))?;

    let domain_cmd = RemoveCollectionItem {
        collection_item_id,
        category,
        removed_date,
    };

    let mut unit_of_work = state.unit_of_work().await?;

    let collection_view =
        RemoveCollectionItemCommand::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(collection_view)
}

/// Tauri command to add a new item to the collection.
///
/// This handler constructs the repository and command handler, executes the command
/// asynchronously and returns the updated `CollectionView` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `input`: Input parameters for adding the collection item.
///
/// Returns:
/// - `Ok(CollectionView)` when addition succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn add_collection_item(
    state: tauri::State<'_, AppState>,
    input: AddCollectionItemInput,
) -> Result<CollectionView, CommandError> {
    info!("Adding collection item: {:?}", input);

    let domain_cmd = AddCollectionItem::try_from(input).map_err(CommandError::from)?;

    let mut unit_of_work = state.unit_of_work().await?;

    let collection_view = AddCollectionItemCommand::execute(&mut unit_of_work, domain_cmd).await?;
    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(collection_view)
}
