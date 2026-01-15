mod add_to_wishlist;
mod create_wishlist;
mod delete_wishlist;
mod get_wishlist_by_id;
mod get_wishlists;
mod move_wishlist_item;
pub mod queries;
mod remove_wishlist_item;
mod rename_wishlist;
mod set_default_wishlist;
#[cfg(test)]
mod testing;

pub use self::get_wishlist_by_id::GetWishlistByIdQuery;
pub use self::get_wishlists::GetWishlistsQuery;
pub use self::queries::{WishlistItemView, WishlistView};

pub use self::add_to_wishlist::AddToWishlistUseCase;
pub use self::create_wishlist::CreateWishlistUseCase;
pub use self::delete_wishlist::DeleteWishlistUseCase;
pub use self::move_wishlist_item::MoveWishlistItemUseCase;
pub use self::remove_wishlist_item::RemoveWishlistItemUseCase;
pub use self::rename_wishlist::RenameWishlistUseCase;
pub use self::set_default_wishlist::SetDefaultWishlistUseCase;
