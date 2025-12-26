use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::infrastructure::sqlite_repo::SqliteCollectionRepository;
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use std::sync::Arc;

#[tauri::command]
pub async fn get_collection(
    state: tauri::State<'_, AppState>,
) -> Result<crate::collecting::domain::collection::Collection, CommandError> {
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
