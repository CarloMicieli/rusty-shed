use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::domain::currency::Currency;
use crate::wishlist::domain::wishlist_id::WishlistId;

/// Lightweight preview information for a wishlist.
///
/// Primarily used for list views where a full wishlist and its items are
/// not required. `total_value` maps a `Currency` to the summed amount
/// (stored in the DB as integer amounts, typically the smallest unit, e.g.
/// cents) for the wishlist items in that currency.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistPreview {
    pub id: WishlistId,
    pub name: String,
    pub notes: Option<String>,
    pub is_default: bool,
    pub count: i64,
    pub updated_at: NaiveDateTime,
    pub total_value: HashMap<Currency, i64>,
}
