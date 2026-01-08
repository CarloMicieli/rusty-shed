use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::RenameWishlistCommand;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case that renames an existing wishlist.
///
/// The use case updates the wishlist name in persistence. If the wishlist
/// is not found the repository will return `DomainError::NotFound`.
pub struct RenameWishlistUseCase;

impl RenameWishlistUseCase {
    /// Execute the rename-wishlist use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the wishlist id and new name.
    ///
    /// Returns `()` on success or a `DomainError` on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: RenameWishlistCommand,
    ) -> Result<(), DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        repo.rename_wishlist(&cmd.id, &cmd.name).await?;
        Ok(())
    }
}
