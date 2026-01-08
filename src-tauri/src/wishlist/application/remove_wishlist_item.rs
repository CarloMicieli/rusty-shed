use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::RemoveWishlistItemCommand;

pub struct RemoveWishlistItemUseCase;

impl RemoveWishlistItemUseCase {
    pub async fn execute(
        &self,
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: RemoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        // Implement remove item logic
        unimplemented!()
    }
}
