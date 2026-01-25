use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A single item within a `Wishlist`.
///
/// `WishlistItem` models the user-facing properties of an item the user
/// wants to track or acquire. It intentionally does not carry a reference
/// to its parent `Wishlist` as it is used as a value object inside the
/// aggregate. Business operations that need the wishlist context should
/// operate on the `Wishlist` aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct WishlistItem {
    /// Stable identifier for this wishlist item.
    pub id: WishlistItemId,
    /// Identifier of the referenced railway model.
    pub railway_model_id: RailwayModelId,
    /// The user's priority for this item.
    pub priority: WishlistPriority,
    /// The current procurement/status lifecycle state for the item.
    pub status: WishlistStatus,
    /// Date the item was added to the wishlist (YYYY-MM-DD).
    pub added_date: NaiveDate,
    /// Optional date when the item was removed from the wishlist.
    pub removed_date: Option<NaiveDate>,
    /// Optional free-form notes attached to the item.
    pub notes: Option<String>,
    /// Desired price the user is willing to pay for the item (in cents).
    pub desired_price: Option<MonetaryAmount>,
    /// Actual purchased price if available (in cents).
    pub purchased_price: Option<MonetaryAmount>,
}
