use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::DeleteWishlistCommand;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case that deletes a wishlist and its items.
///
/// This operation removes the wishlist aggregate from persistence; the
/// repository is expected to cascade-delete associated items. A
/// `DomainError::NotFound` is returned if the wishlist does not exist.
pub struct DeleteWishlistUseCase;

impl DeleteWishlistUseCase {
    /// Execute the delete-wishlist use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the wishlist id to delete.
    ///
    /// Returns `()` on success or a `DomainError` on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: DeleteWishlistCommand,
    ) -> Result<(), DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        repo.delete_wishlist(&cmd.id).await?;
        Ok(())
    }
}
