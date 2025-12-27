//! Command handlers exposed to the Tauri frontend for the `collecting` feature.
//!
//! These functions act as a thin adapter between the Tauri IPC layer and the
//! application/use-case layer. They translate incoming requests into use-case
//! invocations and map application errors into `CommandError` values suitable
//! for returning over the IPC boundary.

use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::domain::collection::Collection;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;

/// Tauri command to retrieve the current collection.
///
/// This handler constructs the repository and use-case, executes the use-case
/// asynchronously and returns the `Collection` on success. On failure, it
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
    // 1. Initialize the Unit of Work from the pool stored in AppState
    let mut uow = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

    // 2. Initialize the stateless Use Case
    let use_case = GetCollectionUseCase::new();

    // 3. Execute the Use Case within the transaction context
    match use_case.execute(&mut uow).await {
        Ok(collection) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            uow.commit()
                .await
                .map_err(|e| CommandError::DatabaseError(e.to_string()))?;

            Ok(collection)
        }
        Err(e) => Err(CommandError::Unknown(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::db::init_in_memory_db_pool;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn command_get_collection_returns_empty() {
        // 1. Setup: Create the isolated in-memory DB
        let pool = init_in_memory_db_pool().await.expect("init in-memory pool");

        // 2. Initialize the Use Case (now stateless)
        let use_case = GetCollectionUseCase::new();

        // 3. Start the Unit of Work (Transaction)
        let mut uow = SqliteUnitOfWork::new(&pool)
            .await
            .expect("Failed to begin unit of work");

        // 4. Execute the Use Case passing the UoW context
        let found_collection = use_case
            .execute(&mut uow)
            .await
            .expect("get_collection execution failed");

        // 5. Assertions
        assert_eq!(found_collection.name, "My Collection");
        assert_eq!(found_collection.items.len(), 0);

        // 6. Cleanup: Explicitly commit if changes were made (optional for read-only tests)
        uow.commit().await.expect("commit failed");
    }
}
