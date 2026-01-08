use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::MoveWishlistItemCommand;

pub struct MoveWishlistItemUseCase;

impl MoveWishlistItemUseCase {
    pub async fn execute(
        &self,
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: MoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        // Implement move item logic
        unimplemented!()
    }
}
