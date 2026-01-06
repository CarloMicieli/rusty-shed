use crate::core::infrastructure::error::CommandError;
use crate::dashboard::application::GetDashboardSummaryQuery;
use crate::dashboard::domain::DashboardSummary;
use crate::state::AppState;
use serde::Deserialize;
use tauri::State;

#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_summary(
    state: State<'_, AppState>,
    params: Option<QueryParams>,
) -> Result<DashboardSummary, CommandError> {
    let mut unit_of_work = state.unit_of_work().await?;

    let params = params.unwrap_or_default();
    let number_of_recent_items = params
        .number_of_recent_items
        .unwrap_or(DEFAULT_RECENT_ITEMS);
    let number_of_depot_entries = params
        .number_of_depot_entries
        .unwrap_or(DEFAULT_DEPOT_ENTRIES);

    match GetDashboardSummaryQuery::execute(
        &mut unit_of_work,
        number_of_recent_items,
        number_of_depot_entries,
    )
    .await
    {
        Ok(dashboard_summary) => {
            // Since this is a 'get' operation, committing is technically optional,
            // but calling it ensures the transaction is closed cleanly.
            unit_of_work
                .commit()
                .await
                .map_err(|err| CommandError::DatabaseError(err.to_string()))?;

            Ok(dashboard_summary)
        }
        Err(e) => Err(e.into()),
    }
}

const DEFAULT_RECENT_ITEMS: u8 = 4;
const DEFAULT_DEPOT_ENTRIES: u8 = 10;

/// Query parameters for retrieving the dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
pub struct QueryParams {
    /// Number of recent items to retrieve for the dashboard.
    pub number_of_recent_items: Option<u8>,
    /// Number of depot entries to retrieve for the dashboard.
    pub number_of_depot_entries: Option<u8>,
}

/// Default values for QueryParams.
impl Default for QueryParams {
    fn default() -> Self {
        Self {
            number_of_recent_items: Some(DEFAULT_RECENT_ITEMS),
            number_of_depot_entries: Some(DEFAULT_DEPOT_ENTRIES),
        }
    }
}
