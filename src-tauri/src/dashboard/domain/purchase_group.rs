use crate::dashboard::domain::ModelCard;
use serde::Serialize;

/// A group of models acquired together (same purchase date + seller)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseGroup {
    /// Unique identifier for display purposes (format: "purchase-YYYY-MM-DD-{seller_id}")
    pub id: String,

    /// Date when the models were purchased (ISO 8601 date string)
    pub purchase_date: String,

    /// Name of the seller/shop (optional)
    pub seller_name: Option<String>,

    /// User notes about this purchase transaction
    pub notes: Option<String>,

    /// List of model cards in this purchase (max 3 for display)
    pub model_cards: Vec<ModelCard>,

    /// Total number of models in this purchase (for "+N more" indicator)
    pub total_count: usize,
}
