use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;

#[async_trait::async_trait]
pub trait WishlistRepository: Send + Sync {
    async fn get_wishlist_by_id(&mut self, id: &str) -> anyhow::Result<Option<Wishlist>>;
    async fn list_wishlist_previews(&mut self) -> anyhow::Result<Vec<WishlistPreview>>;
}
