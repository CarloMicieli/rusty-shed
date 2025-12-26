//! Command handlers exposed to the Tauri frontend for the `collecting` feature.
//!
//! These functions act as a thin adapter between the Tauri IPC layer and the
//! application/use-case layer. They translate incoming requests into use-case
//! invocations and map application errors into `CommandError` values suitable
//! for returning over the IPC boundary.

use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::domain::collection::Collection;
use crate::collecting::infrastructure::sqlite_repo::SqliteCollectionRepository;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use std::sync::Arc;

/// Tauri command to retrieve the current collection.
///
/// This handler constructs the repository and use-case, executes the use-case
/// asynchronously and returns the `Collection` on success. On failure it
/// converts the error into a `CommandError::Unknown` preserving the error
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
pub async fn get_collection(state: tauri::State<'_, AppState>) -> Result<Collection, CommandError> {
    let repo = SqliteCollectionRepository::new(state.db_pool());
    let use_case = GetCollectionUseCase::new(Arc::new(repo));

    match use_case.execute().await {
        Ok(collection) => Ok(collection),
        Err(e) => Err(CommandError::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_in_memory_db_pool;
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    #[tokio::test]
    async fn command_get_collection_returns_empty() {
        // Create an isolated in-memory DB and run migrations
        let pool = init_in_memory_db_pool().await.expect("init in-memory pool");

        // Create repository and use case directly (bypass tauri::State wrapper)
        let repo = SqliteCollectionRepository::new(pool.clone());
        let use_case = GetCollectionUseCase::new(Arc::new(repo));

        let found_collection = use_case.execute().await.expect("get_collection");

        assert_eq!(found_collection.name, "My Collection");
        assert_eq!(found_collection.items.len(), 0);
    }
}
