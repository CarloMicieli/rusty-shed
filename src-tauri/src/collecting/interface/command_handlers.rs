use crate::collecting::application::GetCollectionQuery;
use crate::collecting::domain::CollectionView;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;

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
    let mut unit_of_work = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

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

/// Tauri command to retrieve depot data (alias of `get_collection`).
#[tauri::command]
#[specta::specta]
pub async fn get_depot(_state: tauri::State<'_, AppState>) -> Result<(), CommandError> {
    todo!()
}
