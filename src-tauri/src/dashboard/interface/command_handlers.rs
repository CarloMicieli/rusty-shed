use crate::core::infrastructure::error::CommandError;
use crate::dashboard::application::GetDashboardSummaryQuery;
use crate::dashboard::domain::DashboardSummary;
use crate::state::AppState;
use log::info;
use serde::Deserialize;

/// Tauri command to retrieve the dashboard summary.
///
/// This handler constructs the repository and query handler, executes the query
/// asynchronously and returns the `DashboardSummary` on success. On failure, it
/// converts the error into a `CommandError` preserving the error
/// message for logging/debugging.
///
/// Parameters:
/// * `state`: Tauri-managed application state which provides a database pool.
/// * `criteria`: Optional query criteria to customize the summary retrieval.
///
/// Returns:
/// - `Ok(DashboardSummary)` when retrieval succeeds.
/// - `Err(CommandError)` when the use-case returns an error.
#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_summary(
    state: tauri::State<'_, AppState>,
    criteria: Option<QueryCriteria>,
) -> Result<DashboardSummary, CommandError> {
    info!("Fetching dashboard summary");

    let mut unit_of_work = state.unit_of_work().await?;

    let criteria = criteria.unwrap_or_default();
    let number_of_recent_items = criteria
        .number_of_recent_items
        .unwrap_or(DEFAULT_RECENT_ITEMS);
    let number_of_depot_entries = criteria
        .number_of_depot_entries
        .unwrap_or(DEFAULT_DEPOT_ENTRIES);

    let dashboard_summary = GetDashboardSummaryQuery::execute(
        &mut unit_of_work,
        number_of_recent_items,
        number_of_depot_entries,
    )
    .await?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    Ok(dashboard_summary)
}

const DEFAULT_RECENT_ITEMS: u8 = 4;
const DEFAULT_DEPOT_ENTRIES: u8 = 10;

/// Query criteria for retrieving the dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QueryCriteria {
    /// Number of recent items to retrieve for the dashboard.
    pub number_of_recent_items: Option<u8>,
    /// Number of depot entries to retrieve for the dashboard.
    pub number_of_depot_entries: Option<u8>,
}

/// Default values for QueryCriteria.
impl Default for QueryCriteria {
    fn default() -> Self {
        Self {
            number_of_recent_items: Some(DEFAULT_RECENT_ITEMS),
            number_of_depot_entries: Some(DEFAULT_DEPOT_ENTRIES),
        }
    }
}
