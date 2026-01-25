use crate::core::domain::metadata::Metadata;
use crate::wishlist::domain::wishlist_event::WishlistEvent;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use serde::{Deserialize, Serialize};

/// Domain model representing a user's wishlist.
///
/// A `Wishlist` is a named collection of `WishlistItem`s. It carries
/// optional notes, a flag indicating whether it is the default list, and
/// a stable identifier used across the application. Business logic should
/// operate on this aggregate root when mutating the contained items.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Wishlist {
    /// Unique identifier for the wishlist.
    pub id: WishlistId,
    /// Human-readable name for the wishlist.
    pub name: String,
    /// Optional free-form notes attached to the wishlist.
    pub notes: Option<String>,
    /// Whether this wishlist is the default for the user.
    pub is_default: bool,
    /// Items contained in this wishlist.
    pub items: Vec<WishlistItem>,
    /// Events produced by operations on the aggregate that have not yet been
    /// persisted/handled by a repository or unit of work.
    #[serde(skip)]
    pub pending_events: Vec<WishlistEvent>,
    /// Metadata about the wishlist (creation date, last modified, etc.).
    #[serde(skip)]
    pub metadata: Metadata,
}

impl Wishlist {
    /// Emit an `ItemAdded` event and apply it to the in-memory state.
    pub fn add_item(&mut self, item: WishlistItem) {
        let ev = WishlistEvent::ItemAdded { item: item.clone() };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
    }

    /// Emit a `Renamed` event and apply it to the aggregate state.
    pub fn rename(&mut self, name: &str) {
        let ev = WishlistEvent::Renamed {
            name: name.to_string(),
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
    }

    /// Apply an event to the aggregate's in-memory state.
    fn apply_event(&mut self, event: &WishlistEvent) {
        match event {
            WishlistEvent::Created {
                name,
                notes,
                is_default,
            } => {
                self.name = name.clone();
                self.notes = notes.clone();
                self.is_default = *is_default;
            }
            WishlistEvent::Renamed { name } => self.name = name.clone(),
            WishlistEvent::ItemAdded { item } => self.items.push(item.clone()),
            WishlistEvent::ItemRemoved { item_id } => {
                self.items.retain(|i| i.id != *item_id);
            }
            WishlistEvent::ItemMoved {
                item_id,
                destination: _,
            } => {
                // Movement between wishlists is primarily a repository concern
                // (it involves two aggregates/rows). At the aggregate level we
                // remove the item when moved out.
                self.items.retain(|i| i.id != *item_id);
            }
            WishlistEvent::MarkedDefault { is_default } => self.is_default = *is_default,
        }
    }

    /// Retrieve and clear pending events for persistence.
    pub fn drain_events(&mut self) -> Vec<WishlistEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
