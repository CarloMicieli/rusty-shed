use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::wishlist::domain::wishlist_event::WishlistEvent;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
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

    /// Validate and transition a wishlist item to the `Purchased` status.
    ///
    /// Returns `DomainError::BusinessRule` if the item is not found or is already
    /// in a terminal status (`Purchased` or `Ignored`).
    pub fn purchase_item(
        &mut self,
        item_id: &WishlistItemId,
        purchased_price: MonetaryAmount,
    ) -> Result<(), DomainError> {
        let item = self
            .items
            .iter()
            .find(|i| i.id == *item_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "WishlistItem".to_string(),
                identifier: item_id.to_string(),
            })?;

        if item.status != WishlistStatus::Wanted && item.status != WishlistStatus::OnOrder {
            return Err(DomainError::BusinessRule(
                "Item is not available for purchase".to_string(),
            ));
        }

        let ev = WishlistEvent::ItemPurchased {
            item_id: item_id.clone(),
            purchased_price,
        };
        self.pending_events.push(ev.clone());
        self.apply_event(&ev);
        Ok(())
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
            WishlistEvent::ItemPurchased {
                item_id,
                purchased_price,
            } => {
                if let Some(item) = self.items.iter_mut().find(|i| i.id == *item_id) {
                    item.status = WishlistStatus::Purchased;
                    item.purchased_price = Some(purchased_price.clone());
                }
            }
        }
    }

    /// Retrieve and clear pending events for persistence.
    pub fn drain_events(&mut self) -> Vec<WishlistEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
