use crate::state::AppState;

#[tauri::command]
#[specta::specta]
pub fn is_db_initialized(state: tauri::State<'_, AppState>) -> bool {
    state.is_initialized()
}

#[tauri::command]
#[specta::specta]
pub fn get_app_version() -> String {
    // Use the crate package version set at compile time
    env!("CARGO_PKG_VERSION").to_string()
}
