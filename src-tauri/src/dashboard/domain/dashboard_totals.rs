use crate::core::domain::MonetaryAmount;
use serde::Serialize;

/// Aggregated totals for the user's dashboard.
///
/// Fields represent various summary statistics about the user's collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DashboardTotals {
    /// Total number of unique items in the collection.
    pub collection_items: u32,
    /// Total number of unique items marked as wishlisted.
    pub wishlists: u32,
    /// Total number of unique items that are due for maintenance.
    pub maintenance_due: u32,
    /// Total monetary value of the entire collection.
    pub total_value: Option<MonetaryAmount>,
}
