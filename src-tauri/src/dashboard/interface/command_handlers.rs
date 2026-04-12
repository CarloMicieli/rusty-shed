use crate::core::infrastructure::error::CommandError;
use crate::dashboard::application::GetDashboardSummary;
use crate::dashboard::domain::DashboardSummary;
use crate::state::AppState;
use serde::Deserialize;
use tracing::info;

const DEFAULT_RECENT_ITEMS: u8 = 4;

/// Query criteria for retrieving the dashboard summary.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QueryCriteria {
    /// Number of recent items to retrieve for the dashboard.
    pub number_of_recent_items: Option<u8>,
}

/// Default values for QueryCriteria.
impl Default for QueryCriteria {
    fn default() -> Self {
        Self {
            number_of_recent_items: Some(DEFAULT_RECENT_ITEMS),
        }
    }
}

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Retrieve the dashboard summary.
pub async fn get_dashboard_summary_inner(
    state: &AppState,
    criteria: Option<QueryCriteria>,
) -> Result<DashboardSummary, CommandError> {
    info!("Fetching dashboard summary");
    let criteria = criteria.unwrap_or_default();
    let number_of_recent_items = criteria
        .number_of_recent_items
        .unwrap_or(DEFAULT_RECENT_ITEMS);
    let mut uow = state.unit_of_work().await?;
    let dashboard_summary = GetDashboardSummary::execute(
        &mut uow,
        number_of_recent_items,
        state.models_dir().to_path_buf(),
    )
    .await?;
    uow.commit().await?;
    Ok(dashboard_summary)
}

/// Tauri command to retrieve the dashboard summary.
#[tauri::command]
#[specta::specta]
pub async fn get_dashboard_summary(
    state: tauri::State<'_, AppState>,
    criteria: Option<QueryCriteria>,
) -> Result<DashboardSummary, CommandError> {
    get_dashboard_summary_inner(&state, criteria).await
}
