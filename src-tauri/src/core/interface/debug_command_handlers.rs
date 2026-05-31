use crate::core::infrastructure::debug::{
    DatabaseTableStat, load_database_table_stats, tail_recent_logs,
};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;

const DEFAULT_LOG_LIMIT: u32 = 100;
const MAX_LOG_LIMIT: u32 = 100;

/// Load read-only SQLite table statistics for the developer tools page.
pub async fn get_db_stats_inner(state: &AppState) -> Result<Vec<DatabaseTableStat>, CommandError> {
    load_database_table_stats(&state.db_pool()).await
}

/// Return row counts and best-effort size estimates for application tables.
#[tauri::command]
#[specta::specta]
pub async fn get_db_stats(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<DatabaseTableStat>, CommandError> {
    get_db_stats_inner(&state).await
}

/// Load the latest persisted application log lines for the developer tools page.
pub async fn get_recent_logs_inner(
    state: &AppState,
    limit: Option<u32>,
) -> Result<Vec<String>, CommandError> {
    let limit = limit.unwrap_or(DEFAULT_LOG_LIMIT).clamp(1, MAX_LOG_LIMIT) as usize;
    tail_recent_logs(state.log_dir(), limit).await
}

/// Return the latest persisted application log lines.
#[tauri::command]
#[specta::specta]
pub async fn get_recent_logs(
    state: tauri::State<'_, AppState>,
    limit: Option<u32>,
) -> Result<Vec<String>, CommandError> {
    get_recent_logs_inner(&state, limit).await
}
