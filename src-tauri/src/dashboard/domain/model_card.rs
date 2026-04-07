use crate::collecting::domain::CollectionItemId;
use crate::collecting::domain::PurchaseCondition;
use crate::core::domain::MonetaryAmount;
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

    /// Scale (e.g., "H0", "N") — from railway_models.scale
    pub scale: Option<String>,

    /// Era/epoch (e.g., "V", "IV") — from railway_models.epoch
    pub era: Option<String>,

    /// Purchase price — from purchase_infos.purchased_price_amount/currency
    pub price: Option<MonetaryAmount>,
}
