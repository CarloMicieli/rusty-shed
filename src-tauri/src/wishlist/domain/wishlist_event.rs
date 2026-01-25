use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
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
}
