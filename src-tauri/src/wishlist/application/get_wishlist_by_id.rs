use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

pub struct GetWishlistUseCase;

impl GetWishlistUseCase {
    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        id: String,
    ) -> anyhow::Result<Option<Wishlist>> {
        let mut repo = uow.wishlist_repo();
        let wishlist = repo.get_wishlist_by_id(&id).await?;
        Ok(wishlist)
    }
}
