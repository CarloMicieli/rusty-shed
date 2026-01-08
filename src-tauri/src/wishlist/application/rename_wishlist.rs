use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::RenameWishlistCommand;

pub struct RenameWishlistUseCase;

impl RenameWishlistUseCase {
    pub async fn execute(
        &self,
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: RenameWishlistCommand,
    ) -> Result<(), DomainError> {
        // Implement rename logic
        unimplemented!()
    }
}
