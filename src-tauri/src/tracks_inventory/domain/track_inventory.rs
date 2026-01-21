use super::track_id::TrackId;
use super::track_inventory_event::TrackInventoryEvent;
use super::track_inventory_id::TrackInventoryId;
use super::track_purchase::TrackPurchase;
use super::track_quantity::TrackQuantity;
use crate::core::domain::metadata::Metadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem;

/// Aggregate representing a collection of track products owned or managed by
/// the application/user.
///
/// The `TrackInventory` holds the current quantities of track items (`inventory`),
/// a chronological `purchase_history` of acquisitions, a stable identifier
/// (`id`) and auxiliary `metadata` (for example timestamps or ownership info).
/// Use domain repositories and use-cases to construct and persist instances
/// rather than manipulating the struct directly.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackInventory {
    /// Stable TRN identifier for this inventory aggregate.
    pub id: TrackInventoryId,

    /// Human-readable name for this inventory.
    pub name: String,

    /// Optional detailed description for this inventory.
    pub description: Option<String>,

    /// Current quantities indexed by `TrackId`.
    ///
    /// The map holds the canonical `TrackId` key and the corresponding
    /// `TrackQuantity` value which contains the available amount for that
    /// track product.
    pub inventory: HashMap<TrackId, TrackQuantity>,

    /// Chronological list of purchases that contributed to this inventory.
    ///
    /// Use this to display acquisition history or to compute provenance and
    /// cost statistics.
    pub purchase_history: Vec<TrackPurchase>,

    /// Additional auxiliary metadata associated with the inventory record
    /// (for example timestamps, owner id or audit information).
    pub metadata: Metadata,

    /// Pending domain events emitted by this aggregate which infrastructure
    /// can consume to persist changes in an event-driven manner.
    ///
    /// Not serialized for persistence of the snapshot/state itself.
    #[serde(skip)]
    pub pending_events: Vec<TrackInventoryEvent>,
}

impl TrackInventory {
    /// Pulls all pending events from the aggregate and clears the local queue.
    /// Use by repositories or dispatchers to consume and persist emitted events.
    pub fn pull_events(&mut self) -> Vec<TrackInventoryEvent> {
        // Move the events out without cloning.
        mem::take(&mut self.pending_events)
    }

    /// Append a domain event to the aggregate's pending queue.
    pub fn push_event(&mut self, ev: TrackInventoryEvent) {
        self.pending_events.push(ev);
    }
}
