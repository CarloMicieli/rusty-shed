use crate::wishlist::domain::wishlist_item::WishlistItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub notes: Option<String>,
    pub is_default: bool,
    pub items: Vec<WishlistItem>,
}

impl Wishlist {
    pub fn add_item(&mut self, item: WishlistItem) {
        self.items.push(item);
    }
}
