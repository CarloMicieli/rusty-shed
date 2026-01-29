use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::sellers::domain::seller_id::SellerId;
use crate::tracks_inventory::domain::TrackId;
use crate::tracks_inventory::domain::TrackInventoryId;
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
