use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::domain::MonetaryAmount;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
use crate::wishlist::domain::wishlist_priority::WishlistPriority;
use crate::wishlist::domain::wishlist_status::WishlistStatus;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct CreateWishlistInput {
    pub name: String,
    pub notes: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct RenameWishlistInput {
    pub id: WishlistId,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteWishlistInput {
    pub id: WishlistId,
}

#[derive(Debug, Clone)]
pub struct SetDefaultWishlistInput {
    pub id: WishlistId,
}

#[derive(Debug, Clone)]
pub struct AddToWishlistInput {
    pub wishlist_id: WishlistId,
    pub railway_model_id: RailwayModelId,
    pub priority: WishlistPriority,
    pub status: WishlistStatus,
    pub desired_price: Option<MonetaryAmount>,
    pub notes: Option<String>,
    pub added_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct RemoveWishlistItemInput {
    pub item_id: WishlistItemId,
}

#[derive(Debug, Clone)]
pub struct MoveWishlistItemInput {
    pub item_id: WishlistItemId,
    pub destination_wishlist_id: WishlistId,
    pub wishlist_id: WishlistId,
}
