use crate::state::AppState;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

pub fn is_db_initialized_inner(state: &AppState) -> bool {
    state.is_initialized()
}

pub fn get_app_version_inner() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ---------------------------------------------------------------------------
// Tauri command wrappers – thin shims that delegate to inner functions
// ---------------------------------------------------------------------------

#[tauri::command]
#[specta::specta]
pub fn is_db_initialized(state: tauri::State<'_, AppState>) -> bool {
    is_db_initialized_inner(&state)
}

#[tauri::command]
#[specta::specta]
pub fn get_app_version() -> String {
    get_app_version_inner()
}
