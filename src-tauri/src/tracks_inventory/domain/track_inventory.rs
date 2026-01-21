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
    /// Stable unique identifier for this inventory.
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

    /// Pending domain events emitted by this aggregate which infrastructure
    /// can consume to persist changes in an event-driven manner.
    ///
    /// Not serialized for persistence of the snapshot/state itself.
    #[serde(skip)]
    pub(crate) pending_events: Vec<TrackInventoryEvent>,

    /// Additional auxiliary metadata associated with the inventory record
    /// (for example timestamps, owner id or audit information).
    pub metadata: Metadata,
}

impl TrackInventory {
    /// Pulls all pending events from the aggregate and clears the local queue.
    /// Use by repositories or dispatchers to consume and persist emitted events.
    pub fn pull_events(&mut self) -> Vec<TrackInventoryEvent> {
        // Move the events out without cloning.
        mem::take(&mut self.pending_events)
    }

    /// Append a domain event to the aggregate's pending queue.
    fn push_event(&mut self, ev: TrackInventoryEvent) {
        self.pending_events.push(ev);
    }

    /// Add a purchase to the aggregate's history and emit the corresponding
    /// domain event. The provided `purchase` is pushed into the
    /// `purchase_history` and a `PurchaseAdded` event is enqueued.
    pub fn add_purchase(&mut self, purchase: TrackPurchase) {
        // Clone once so we can both store the purchase in history and move
        // the owned purchase into the event payload.
        self.purchase_history.push(purchase.clone());
        self.push_event(TrackInventoryEvent::PurchaseAdded { purchase });
    }

    /// Rename the inventory: update the `name` field and emit a `Renamed` event.
    pub fn rename(&mut self, new_name: String) {
        self.name = new_name.clone();
        self.push_event(TrackInventoryEvent::Renamed { name: new_name });
    }

    /// Set the quantity for a given `track_id`. If `quantity` is less than
    /// or equal to zero the entry is removed. The method updates the
    /// in-memory `inventory` map and emits an `ItemQuantitySet` event.
    pub fn set_item_quantity(&mut self, track_id: TrackId, quantity: i64) {
        if quantity <= 0 {
            self.inventory.remove(&track_id);
        } else {
            self.inventory.insert(
                track_id.clone(),
                TrackQuantity {
                    track_id: track_id.clone(),
                    quantity,
                },
            );
        }

        self.push_event(TrackInventoryEvent::ItemQuantitySet { track_id, quantity });
    }

    /// Construct a new `TrackInventory` aggregate with the provided identity,
    /// name and optional description. The returned aggregate will have a
    /// `Created` domain event enqueued in `pending_events` so repositories
    /// can apply an event-driven persistence strategy for new aggregates.
    pub fn new(id: TrackInventoryId, name: String, description: Option<String>) -> Self {
        let mut inventory = TrackInventory {
            id,
            name,
            description,
            inventory: HashMap::new(),
            purchase_history: Vec::new(),
            metadata: Metadata::default(),
            pending_events: Vec::new(),
        };

        inventory.push_event(TrackInventoryEvent::Created);

        inventory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;
    use crate::core::domain::monetary_amount::MonetaryAmount;
    use crate::sellers::domain::seller_id::SellerId;
    use crate::tracks_inventory::domain::track_id::TrackId;
    use crate::tracks_inventory::domain::track_purchase::TrackPurchase;
    use crate::tracks_inventory::domain::track_purchase_id::TrackPurchaseId;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[test]
    fn it_should_new_emits_created_and_clears_on_pull() {
        let id = TrackInventoryId::default();
        let mut inv = TrackInventory::new(id, "Initial".to_string(), None);

        let events = inv.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrackInventoryEvent::Created => {}
            other => panic!("expected Created event, got: {:?}", other),
        }

        // subsequent pulls return empty
        let again = inv.pull_events();
        assert!(again.is_empty());
    }

    #[test]
    fn it_should_rename_updates_name_and_emits_event() {
        let id = TrackInventoryId::default();
        let mut inv = TrackInventory::new(id, "Old".to_string(), None);

        inv.pull_events(); // consume Created

        inv.rename("New Name".to_string());
        assert_eq!(inv.name, "New Name");

        let events = inv.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrackInventoryEvent::Renamed { name } => assert_eq!(name, "New Name"),
            other => panic!("expected Renamed event, got: {:?}", other),
        }
    }

    #[test]
    fn it_should_add_purchase_appends_and_emits() {
        let id = TrackInventoryId::default();
        let mut inv = TrackInventory::new(id, "Inv".to_string(), None);
        inv.pull_events(); // consume Created

        let purchase = TrackPurchase {
            track_purchase_id: TrackPurchaseId::default(),
            track_id: TrackId::try_from("trn:track:acme:60100").unwrap(),
            quantity: 3,
            price: MonetaryAmount::new(1000, Currency::EUR),
            seller_id: Some(SellerId::try_from("trn:seller:model-train-shop").unwrap()),
            purchase_date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
        };

        inv.add_purchase(purchase.clone());

        assert!(
            inv.purchase_history
                .iter()
                .any(|p| p.track_id.to_string() == "trn:track:acme:60100" && p.quantity == 3)
        );

        let events = inv.pull_events();
        assert_eq!(events.len(), 1);
        match &events[0] {
            TrackInventoryEvent::PurchaseAdded { purchase: p } => assert_eq!(p.quantity, 3),
            other => panic!("expected PurchaseAdded event, got: {:?}", other),
        }
    }
}
