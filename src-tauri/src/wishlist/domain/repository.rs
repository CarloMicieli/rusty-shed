use crate::wishlist::domain::wishlist::Wishlist;

#[async_trait::async_trait]
pub trait WishlistRepository: Send + Sync {
    async fn get_wishlist_by_id(&mut self, id: &str) -> anyhow::Result<Option<Wishlist>>;
}
