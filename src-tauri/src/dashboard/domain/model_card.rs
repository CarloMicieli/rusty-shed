use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::PurchaseCondition;
use serde::Serialize;

/// Compact view of a railway model for dashboard card display
#[derive(Debug, Clone, PartialEq, Eq, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ModelCard {
    /// Unique collection item identifier (format: "trn:collection-item:{uuid}")
    pub id: CollectionItemId,

    /// Path to thumbnail image (relative to data directory)
    pub thumbnail_path: Option<String>,

    /// Manufacturer name (e.g., "Roco", "Fleischmann")
    pub manufacturer: String,

    /// Product code from manufacturer
    pub product_code: String,

    /// Purchase condition status
    pub condition: PurchaseCondition,

    /// Model description or auto-generated title
    /// Frontend will truncate to ~100 characters
    pub description: String,
}
