use super::track_purchase_id::TrackPurchaseId;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::domain::TrackId;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Represents a single purchase of track products.
///
/// `TrackPurchase` records a single acquisition event for track items and
/// stores the unique purchase identifier, the quantity purchased, the
/// monetary price paid, an optional seller reference, and the date of
/// purchase. Use this record to populate inventory history and costing
/// calculations.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct TrackPurchase {
    /// Unique TRN identifier for this purchase event.
    pub track_purchase_id: TrackPurchaseId,

    /// Reference to the purchased track product (canonical `TrackId`).
    pub track_id: TrackId,

    /// Quantity of track items acquired in this purchase.
    pub quantity: i64,

    /// Total monetary amount paid for this purchase.
    pub price: MonetaryAmount,

    /// Optional seller reference for where the items were bought.
    pub seller_id: Option<SellerId>,

    /// Date when the purchase occurred.
    pub purchase_date: NaiveDate,
}
