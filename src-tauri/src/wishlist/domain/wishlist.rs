use crate::core::domain::MonetaryAmount;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::wishlist::domain::wishlist_event::WishlistEvent;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
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

    /// Update one or more editable fields on a wishlist item.
    ///
    /// Returns `DomainError::Validation` if all patch fields are `None` (no-op).
    /// Returns `DomainError::Validation` if `added_date` is in the future.
    /// Returns `DomainError::NotFound` if the item does not exist in this wishlist.
    pub fn update_item(
        &mut self,
        item_id: &WishlistItemId,
        priority: Option<WishlistPriority>,
        status: Option<WishlistStatus>,
        desired_price: Option<Option<MonetaryAmount>>,
        added_date: Option<NaiveDate>,
    ) -> Result<(), DomainError> {
        // At least one field must be provided.
        if priority.is_none() && status.is_none() && desired_price.is_none() && added_date.is_none() {
            return Err(DomainError::Validation(
                "At least one field must be provided to update".to_string(),
            ));
        }

        // Reject future dates.
        if let Some(date) = added_date {
            let today = chrono::Local::now().date_naive();
            if date > today {
                return Err(DomainError::Validation(
                    "Added date must not be in the future".to_string(),
                ));
            }
        }

        // Verify item exists.
        self.items
            .iter()
            .find(|i| i.id == *item_id)
            .ok_or_else(|| DomainError::NotFound {
                resource: "WishlistItem".to_string(),
                identifier: item_id.to_string(),
            })?;

        let ev = WishlistEvent::ItemUpdated {
            item_id: item_id.clone(),
            priority,
            status,
            desired_price,
            added_date,
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
            WishlistEvent::ItemUpdated {
                item_id,
                priority,
                status,
                desired_price,
                added_date,
            } => {
                if let Some(item) = self.items.iter_mut().find(|i| i.id == *item_id) {
                    if let Some(p) = priority {
                        item.priority = p.clone();
                    }
                    if let Some(s) = status {
                        item.status = s.clone();
                    }
                    match desired_price {
                        Some(Some(price)) => item.desired_price = Some(price.clone()),
                        Some(None) => item.desired_price = None,
                        None => {}
                    }
                    if let Some(d) = added_date {
                        item.added_date = *d;
                    }
                }
            }
        }
    }

    /// Retrieve and clear pending events for persistence.
    pub fn drain_events(&mut self) -> Vec<WishlistEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use chrono::NaiveDate;

    fn make_item(id: &str) -> WishlistItem {
        WishlistItem {
            id: WishlistItemId::try_from(id).unwrap(),
            railway_model_id: crate::catalog::domain::railway_model::RailwayModelId::try_from(
                "trn:railway-model:test",
            )
            .unwrap(),
            priority: WishlistPriority::Normal,
            status: WishlistStatus::Wanted,
            added_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            removed_date: None,
            notes: None,
            desired_price: None,
            purchased_price: None,
        }
    }

    fn make_wishlist(item: WishlistItem) -> Wishlist {
        Wishlist {
            id: WishlistId::try_from("trn:wishlist:11111111-1111-1111-1111-111111111111").unwrap(),
            name: "Test".to_string(),
            notes: None,
            is_default: false,
            items: vec![item],
            pending_events: vec![],
            metadata: crate::core::domain::metadata::Metadata::default(),
        }
    }

    #[test]
    fn update_item_valid_priority_change() {
        let item_id_str = "trn:wishlist-item:22222222-2222-2222-2222-222222222222";
        let item = make_item(item_id_str);
        let item_id = item.id.clone();
        let mut wishlist = make_wishlist(item);

        wishlist
            .update_item(&item_id, Some(WishlistPriority::High), None, None, None)
            .expect("update should succeed");

        let updated = wishlist.items.iter().find(|i| i.id == item_id).unwrap();
        assert_eq!(updated.priority, WishlistPriority::High);
        assert_eq!(wishlist.pending_events.len(), 1);
    }

    #[test]
    fn update_item_all_none_returns_validation_error() {
        let item_id_str = "trn:wishlist-item:22222222-2222-2222-2222-222222222222";
        let item = make_item(item_id_str);
        let item_id = item.id.clone();
        let mut wishlist = make_wishlist(item);

        let result = wishlist.update_item(&item_id, None, None, None, None);
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn update_item_future_date_rejected() {
        let item_id_str = "trn:wishlist-item:22222222-2222-2222-2222-222222222222";
        let item = make_item(item_id_str);
        let item_id = item.id.clone();
        let mut wishlist = make_wishlist(item);

        // Use a date far in the future
        let future_date = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();
        let result = wishlist.update_item(&item_id, None, None, None, Some(future_date));
        assert!(matches!(result, Err(DomainError::Validation(_))));
    }

    #[test]
    fn update_item_clear_desired_price() {
        let item_id_str = "trn:wishlist-item:22222222-2222-2222-2222-222222222222";
        let mut item = make_item(item_id_str);
        item.desired_price = Some(MonetaryAmount::new(1000, Currency::from_code("EUR").unwrap()));
        let item_id = item.id.clone();
        let mut wishlist = make_wishlist(item);

        wishlist
            .update_item(&item_id, None, None, Some(None), None)
            .expect("clearing price should succeed");

        let updated = wishlist.items.iter().find(|i| i.id == item_id).unwrap();
        assert!(updated.desired_price.is_none());
    }
}
