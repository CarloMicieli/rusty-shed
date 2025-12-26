use crate::collecting::application::get_collection::GetCollectionUseCase;
use crate::collecting::infrastructure::sqlite_repo::SqliteCollectionRepository;
use crate::state::AppState;
use std::sync::Arc;

#[tauri::command]
pub async fn get_collection(
    state: tauri::State<'_, AppState>,
) -> Result<crate::collecting::domain::collection::Collection, String> {
    let repo = SqliteCollectionRepository::new(state.db_pool());
    let use_case = GetCollectionUseCase::new(Arc::new(repo));

    match use_case.execute().await {
        Ok(collection) => Ok(collection),
        Err(e) => Err(e.to_string()),
    }
}
