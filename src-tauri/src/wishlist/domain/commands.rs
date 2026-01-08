// Domain command/value objects for wishlist feature
// These are lightweight DTOs representing validated intent for use-cases.

#[derive(Debug, Clone)]
pub struct CreateWishlistCommand {
    pub name: String,
    pub notes: Option<String>,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct RenameWishlistCommand {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DeleteWishlistCommand {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct SetDefaultWishlistCommand {
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct AddToWishlistCommand {
    pub wishlist_id: String,
    pub railway_model_id: String,
    pub priority: Option<crate::wishlist::domain::wishlist_priority::WishlistPriority>,
    pub status: Option<crate::wishlist::domain::wishlist_status::WishlistStatus>,
    pub desired_price_amount: Option<i64>,
    pub desired_price_currency: Option<String>,
    pub notes: Option<String>,
    pub added_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RemoveWishlistItemCommand {
    pub item_id: String,
}

#[derive(Debug, Clone)]
pub struct MoveWishlistItemCommand {
    pub item_id: String,
    pub destination_wishlist_id: String,
}

// Future: implement TryFrom<AdapterInput> for these commands to centralize validation.
