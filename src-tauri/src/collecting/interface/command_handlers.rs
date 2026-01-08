use crate::catalog::domain::railway_model::Category;
use crate::collecting::application::GetCollectionQuery;
use crate::collecting::application::RemoveCollectionItemCommand;
use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::RemoveCollectionItem;
use crate::collecting::domain::{CollectionView, DepotView};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use chrono::NaiveDate;
use serde::Deserialize;

/// Tauri command to retrieve the current collection.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `Collection` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// - `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(Collection)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_collection(
    state: tauri::State<'_, AppState>,
) -> Result<CollectionView, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    match GetCollectionQuery::execute(&mut unit_of_work).await {
        Ok(collection) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(collection)
        }
        Err(e) => Err(e.into()),
    }
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
/// - `state`: Tauri-managed application state which provides a database pool.
///
/// Returns:
/// - `Ok(DepotView)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_depot(_state: tauri::State<'_, AppState>) -> Result<DepotView, CommandError> {
    let mut unit_of_work = _state.unit_of_work().await?;

    match crate::collecting::application::GetDepotQuery::execute(&mut unit_of_work).await {
        Ok(depot) => {
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(depot)
        }
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize, specta::Type)]
pub struct RemoveCollectionItemInput {
    pub collection_item_id: String,
    pub category: String,
    pub removed_date: String,
}

#[tauri::command]
#[specta::specta]
pub async fn remove_collection_item(
    state: tauri::State<'_, AppState>,
    input: RemoveCollectionItemInput,
) -> Result<CollectionView, CommandError> {
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

    match RemoveCollectionItemCommand::execute(&mut unit_of_work, domain_cmd).await {
        Ok(view) => {
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(view)
        }
        Err(e) => Err(e.into()),
    }
}
