use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::length::Length;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackType};
use chrono::NaiveDate;

/// Input DTOs for the tracks inventory application layer.
#[derive(Debug, Clone)]
pub struct NewTrackInventoryInput {
    /// The name of the new track inventory.
    pub name: String,
    /// An optional description for the new track inventory.
    pub description: Option<String>,
}

/// Input used to add a purchase to an existing `TrackInventory`.
#[derive(Debug, Clone)]
pub struct AddTrackPurchaseInput {
    pub id: TrackInventoryId,

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

/// Input used to rename an existing `TrackInventory`.
#[derive(Debug, Clone)]
pub struct RenameTrackInventoryInput {
    /// The identifier of the track inventory to be renamed.
    pub id: TrackInventoryId,
    /// The new name for the track inventory.
    pub new_name: String,
}

/// Input used to set the quantity for a specific track in an inventory.
#[derive(Debug, Clone)]
pub struct SetTrackItemQuantityInput {
    /// The identifier of the track inventory to update.
    pub inventory_id: TrackInventoryId,

    /// The canonical identifier of the track product.
    pub track_id: TrackId,

    /// The desired quantity for the track product. Values <= 0 remove the item.
    pub quantity: i64,
}

/// Input used to create a new track product.
#[derive(Debug, Clone)]
pub struct CreateTrackProductInput {
    /// Manufacturer that produces this track product.
    pub manufacturer_id: ManufacturerId,
    /// Manufacturer's product code or name.
    pub product_code: String,
    /// Human-readable description of the track piece.
    pub description: String,
    /// Geometric type of the track piece.
    pub track_type: TrackType,
    /// Rail profile code describing the rail height.
    pub track_code: TrackCode,
    /// Whether this track piece includes an integrated roadbed.
    pub with_roadbed: bool,
    /// Length for straight track pieces, when applicable.
    pub length: Option<Length>,
    /// Radius for curved track elements, when applicable.
    pub radius: Option<Length>,
}
