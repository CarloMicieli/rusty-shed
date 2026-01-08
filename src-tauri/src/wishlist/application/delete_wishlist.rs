use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::DeleteWishlistCommand;

pub struct DeleteWishlistUseCase;

impl DeleteWishlistUseCase {
    pub async fn execute(
        _uow: &mut SqliteUnitOfWork<'_>,
        _cmd: DeleteWishlistCommand,
    ) -> Result<(), DomainError> {
        // Implement delete logic
        unimplemented!()
    }
}
