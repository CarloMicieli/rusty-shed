use crate::dashboard::domain::{DashboardDepotEntry, DashboardRecentItem, DashboardTotals};
use serde::Serialize;

/// Comprehensive summary data for the user's dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardSummary {
    /// Aggregated totals for the dashboard.
    pub totals: DashboardTotals,
    /// List of recent items for quick access.
    pub recent_items: Vec<DashboardRecentItem>,
    /// List of depot items in the user's collection.
    pub depot_items: Vec<DashboardDepotEntry>,
}
