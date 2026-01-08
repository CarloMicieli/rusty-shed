use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::AddToWishlistCommand;
use crate::wishlist::domain::wishlist_item::WishlistItem;

pub struct AddToWishlistUseCase;

impl AddToWishlistUseCase {
    pub async fn execute(
        _uow: &mut SqliteUnitOfWork<'_>,
        _cmd: AddToWishlistCommand,
    ) -> Result<WishlistItem, DomainError> {
        // Implement add item logic
        unimplemented!()
    }
}
