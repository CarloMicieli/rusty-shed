use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;

/// Input structure for creating a new wishlist.
#[derive(Debug, Clone)]
pub struct CreateWishlistInput {
    /// The name of the wishlist.
    pub name: String,
    /// Optional notes for the wishlist.
    pub notes: Option<String>,
    /// Indicates if this wishlist should be set as the default.
    pub is_default: bool,
}

/// Input structure for renaming an existing wishlist.
#[derive(Debug, Clone)]
pub struct RenameWishlistInput {
    /// The unique identifier of the wishlist to be renamed.
    pub id: WishlistId,
    /// The new name for the wishlist.
    pub name: String,
}

/// Input structure for deleting a wishlist.
#[derive(Debug, Clone)]
pub struct DeleteWishlistInput {
    /// The unique identifier of the wishlist to be deleted.
    pub id: WishlistId,
}

/// Input structure for setting a wishlist as the default.
#[derive(Debug, Clone)]
pub struct SetDefaultWishlistInput {
    /// The unique identifier of the wishlist to be set as default.
    pub id: WishlistId,
}

/// Input structure for adding an item to a wishlist.
#[derive(Debug, Clone)]
pub struct AddToWishlistInput {
    /// The unique identifier of the wishlist to which the item will be added.
    pub wishlist_id: WishlistId,
    /// The unique identifier of the railway model to be added to the wishlist.
    pub railway_model_id: RailwayModelId,
    /// The priority level of the wishlist item.
    pub priority: WishlistPriority,
    /// The status of the wishlist item.
    pub status: WishlistStatus,
    /// The desired price for the wishlist item, if any.
    pub desired_price: Option<MonetaryAmount>,
    /// Optional notes for the wishlist item.
    pub notes: Option<String>,
    /// The date the item was added to the wishlist.
    pub added_date: NaiveDate,
}

/// Input structure for removing an item from a wishlist.
#[derive(Debug, Clone)]
pub struct RemoveWishlistItemInput {
    /// The unique identifier of the wishlist from which the item will be removed.
    pub item_id: WishlistItemId,
}

/// Input structure for moving an item from one wishlist to another.
#[derive(Debug, Clone)]
pub struct MoveWishlistItemInput {
    /// The unique identifier of the wishlist item to be moved.
    pub item_id: WishlistItemId,
    /// The unique identifier of the destination wishlist.
    pub destination_wishlist_id: WishlistId,
    /// The unique identifier of the source wishlist.
    pub wishlist_id: WishlistId,
}

/// Input structure for updating editable fields on a wishlist item.
///
/// Only fields wrapped in `Some` are changed; `None` means "leave unchanged".
/// For `desired_price`: `None` = unchanged, `Some(None)` = clear, `Some(Some(v))` = set.
#[derive(Debug, Clone)]
pub struct UpdateWishlistItemInput {
    /// The unique identifier of the parent wishlist.
    pub wishlist_id: WishlistId,
    /// The unique identifier of the wishlist item to update.
    pub item_id: WishlistItemId,
    /// New priority, or `None` to leave unchanged.
    pub priority: Option<WishlistPriority>,
    /// New status, or `None` to leave unchanged.
    pub status: Option<WishlistStatus>,
    /// Double-option desired price: `None` = unchanged, `Some(None)` = clear, `Some(Some(v))` = set.
    pub desired_price: Option<Option<MonetaryAmount>>,
    /// New added date, or `None` to leave unchanged.
    pub added_date: Option<NaiveDate>,
}
