use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::CreateWishlistCommand;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;

pub struct CreateWishlistUseCase;

impl CreateWishlistUseCase {
    pub async fn execute(
        _uow: &mut SqliteUnitOfWork<'_>,
        _cmd: CreateWishlistCommand,
    ) -> Result<WishlistPreview, DomainError> {
        // Implement business logic: create wishlist, fetch previews and return created preview.
        unimplemented!()
    }
}
