use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::DeleteWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that deletes a wishlist and its items.
///
/// This operation removes the wishlist aggregate from persistence; the
/// repository is expected to cascade-delete associated items. A
/// `DomainError::NotFound` is returned if the wishlist does not exist.
pub struct DeleteWishlistUseCase;

impl DeleteWishlistUseCase {
    /// Execute the delete-wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the wishlist id to delete.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: DeleteWishlistCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.delete_wishlist(&cmd.id).await?;
        Ok(())
    }
}
