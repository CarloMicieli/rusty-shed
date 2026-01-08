use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::MoveWishlistItemCommand;

pub struct MoveWishlistItemUseCase;

impl MoveWishlistItemUseCase {
    pub async fn execute(
        _uow: &mut SqliteUnitOfWork<'_>,
        _cmd: MoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        // Implement move item logic
        unimplemented!()
    }
}
