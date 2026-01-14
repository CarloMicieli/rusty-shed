use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::RenameWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that renames an existing wishlist.
///
/// The use case updates the wishlist name in persistence. If the wishlist
/// is not found the repository will return `DomainError::NotFound`.
pub struct RenameWishlistUseCase;

impl RenameWishlistUseCase {
    /// Execute the rename wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the wishlist id and new name.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: RenameWishlistCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.rename_wishlist(&cmd.id, &cmd.name).await?;
        Ok(())
    }
}
