use serde::Serialize;

/// Lifecycle statistics for the model railway collection.
///
/// Provides three distinct counts reflecting each lifecycle state, plus
/// financial aggregates useful for dashboard summaries.
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CollectionStats {
    /// Number of items in 'PREORDER' state (not yet physically received).
    pub preordered_count: i64,
    /// Number of physically owned active items (removed_date IS NULL, purchase_type = 'PURCHASED').
    pub active_count: i64,
    /// Number of items that have been sold (purchase_type = 'SOLD').
    pub sold_count: i64,
    /// Sum of all deposit amounts for open preorders (in minor units).
    pub investment_at_risk_amount: i64,
    /// Currency code for `investment_at_risk_amount` (e.g. "EUR"). Null when no preorders.
    pub investment_at_risk_currency: Option<String>,
    /// Realized profit/loss for all sold items: sum(sale_price - purchase_price) in minor units.
    pub realized_profit_amount: i64,
    /// Currency code for `realized_profit_amount`. Null when nothing has been sold.
    pub realized_profit_currency: Option<String>,
}
