use crate::tracks_inventory::domain::{TrackId, TrackPurchase};
use serde::{Deserialize, Serialize};

/// Domain events emitted by the `TrackInventory` aggregate.
///
/// These events are intended to capture state changes that the
/// infrastructure layer can consume to persist changes in an
/// event-driven manner.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", content = "payload")]
pub enum TrackInventoryEvent {
    /// Inventory aggregate was created.
    Created,

    /// Inventory was renamed.
    Renamed { name: String },

    /// Inventory description was updated (may be `None` to clear).
    DescriptionUpdated { description: Option<String> },

    /// A specific track item's quantity was set (insert/update/delete).
    ItemQuantitySet { track_id: TrackId, quantity: i64 },

    /// A purchase record was added to the inventory's history.
    PurchaseAdded { purchase: TrackPurchase },
}
