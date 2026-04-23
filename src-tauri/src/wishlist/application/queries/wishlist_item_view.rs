use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;
use serde::Serialize;

/// View model representing a single wishlist item.
///
/// This is a lightweight, serializable representation of `WishlistItem` tailored
/// for use in application queries and for returning over the interface layer.
/// Fields mirror the domain `WishlistItem` but intentionally do not include
/// any references to the aggregate root.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct WishlistItemView {
    /// Stable identifier for this wishlist item (e.g. `trn:wishlist-item:{uuid}`).
    pub id: WishlistItemId,
    /// Identifier of the referenced railway model (TRN form).
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
    /// Desired price the user is willing to pay for the item (monetary amount).
    pub desired_price: Option<MonetaryAmount>,
    /// Actual purchased price if available (monetary amount).
    pub purchased_price: Option<MonetaryAmount>,
}
