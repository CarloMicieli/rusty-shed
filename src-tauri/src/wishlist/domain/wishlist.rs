use crate::core::domain::metadata::Metadata;
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
    /// Metadata about the wishlist (creation date, last modified, etc.).
    pub metadata: Metadata,
}

impl Wishlist {
    pub fn add_item(&mut self, item: WishlistItem) {
        self.items.push(item);
    }
}
