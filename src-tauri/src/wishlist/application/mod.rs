mod add_to_wishlist;
mod create_wishlist;
mod delete_wishlist;
mod get_wishlist_by_id;
mod get_wishlists;
mod move_wishlist_item;
mod remove_wishlist_item;
mod rename_wishlist;
mod set_default_wishlist;

pub use self::get_wishlist_by_id::GetWishlistUseCase;
pub use self::get_wishlists::GetWishlistsUseCase;

pub use self::add_to_wishlist::AddToWishlistUseCase;
pub use self::create_wishlist::CreateWishlistUseCase;
pub use self::delete_wishlist::DeleteWishlistUseCase;
pub use self::move_wishlist_item::MoveWishlistItemUseCase;
pub use self::remove_wishlist_item::RemoveWishlistItemUseCase;
pub use self::rename_wishlist::RenameWishlistUseCase;
pub use self::set_default_wishlist::SetDefaultWishlistUseCase;
