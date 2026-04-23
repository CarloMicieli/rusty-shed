use crate::core::domain::length::Length;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::tracks_inventory::domain::{
    TrackCode, TrackId, TrackInventoryId, TrackPurchaseId, TrackType,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Summary view of a track inventory for list display.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackInventoryListItem {
    /// Inventory identifier.
    pub id: TrackInventoryId,
    /// Inventory name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Count of distinct track types in this inventory.
    pub total_items: i64,
    /// Sum of all quantities across all track types.
    pub total_quantity: i64,
}

/// Detailed view of a track inventory with items and purchase history.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackInventoryView {
    /// Inventory identifier.
    pub id: TrackInventoryId,
    /// Inventory name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Track items with quantities.
    pub items: Vec<TrackInventoryItemView>,
    /// Purchase history.
    pub purchases: Vec<TrackPurchaseView>,
}

/// View of a single inventory item (track product + quantities).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackInventoryItemView {
    /// Track product identifier.
    pub track_id: TrackId,
    /// Product details.
    pub track_product: TrackProductView,
    /// Current stock quantity.
    pub quantity: i64,
    /// Required quantity for planning (defaults to 0).
    pub required: i64,
}

/// Track product view for display.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackProductView {
    /// Track product identifier.
    pub track_id: TrackId,
    /// Manufacturer name (denormalised for display).
    pub manufacturer_name: String,
    /// Manufacturer's product code.
    pub product_code: String,
    /// Human-readable description.
    pub description: String,
    /// Geometric type of the track piece.
    pub track_type: TrackType,
    /// Rail profile code.
    pub track_code: TrackCode,
    /// Whether this track piece includes an integrated roadbed.
    pub with_roadbed: bool,
    /// Length for straight track pieces.
    pub length: Option<Length>,
    /// Radius for curved track elements.
    pub radius: Option<Length>,
}

/// Purchase history view.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackPurchaseView {
    /// Purchase identifier.
    pub id: TrackPurchaseId,
    /// Product purchased.
    pub track_product: TrackProductView,
    /// Quantity purchased.
    pub quantity: i64,
    /// Total price.
    pub price: MonetaryAmount,
    /// Seller name (denormalised, optional).
    pub seller_name: Option<String>,
    /// Purchase date.
    pub purchase_date: NaiveDate,
}
