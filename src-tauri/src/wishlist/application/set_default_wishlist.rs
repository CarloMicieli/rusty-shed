use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::SetDefaultWishlistCommand;

pub struct SetDefaultWishlistUseCase;

impl SetDefaultWishlistUseCase {
    pub async fn execute(
        &self,
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: SetDefaultWishlistCommand,
    ) -> Result<(), DomainError> {
        // Implement set-default logic
        unimplemented!()
    }
}
