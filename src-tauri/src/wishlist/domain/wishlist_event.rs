use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Domain events emitted by `Wishlist` aggregate.
///
/// Events are intentionally simple, serializable messages that describe
/// state changes on the aggregate. They are used to persist changes in a
/// durable store and to drive side-effects in repositories or handlers.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum WishlistEvent {
    Created {
        name: String,
        notes: Option<String>,
        is_default: bool,
    },
    Renamed {
        name: String,
    },
    ItemAdded {
        item: WishlistItem,
    },
    ItemRemoved {
        item_id: WishlistItemId,
    },
    ItemMoved {
        item_id: WishlistItemId,
        destination: WishlistId,
    },
    MarkedDefault {
        is_default: bool,
    },
    /// Emitted when a wishlist item is successfully marked as purchased
    /// and the item has been moved to the collection.
    ItemPurchased {
        item_id: WishlistItemId,
        purchased_price: MonetaryAmount,
    },
    /// Emitted when one or more editable fields on a wishlist item are updated.
    ///
    /// Only the fields wrapped in `Some` are changed; `None` means "leave unchanged".
    /// For `desired_price`: `None` = unchanged, `Some(None)` = clear, `Some(Some(v))` = set.
    ItemUpdated {
        item_id: WishlistItemId,
        priority: Option<WishlistPriority>,
        status: Option<WishlistStatus>,
        desired_price: Option<Option<MonetaryAmount>>,
        added_date: Option<NaiveDate>,
    },
}
