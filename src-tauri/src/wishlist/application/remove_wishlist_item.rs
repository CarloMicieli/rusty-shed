use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::RemoveWishlistItemCommand;

pub struct RemoveWishlistItemUseCase;

impl RemoveWishlistItemUseCase {
    pub async fn execute(
        _uow: &mut SqliteUnitOfWork<'_>,
        _cmd: RemoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        // Implement remove item logic
        unimplemented!()
    }
}
