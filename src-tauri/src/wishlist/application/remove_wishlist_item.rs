use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::RemoveWishlistItemCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that removes a wishlist item by its identifier.
///
/// This operation deletes the item from persistence. If the item does not
/// exist the repository returns a `DomainError::NotFound` which is
/// propagated to the caller.
pub struct RemoveWishlistItemUseCase;

impl RemoveWishlistItemUseCase {
    /// Execute the remove wishlist item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command carrying the `WishlistItemId` to remove.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: RemoveWishlistItemCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.remove_item(&cmd.item_id).await?;
        Ok(())
    }
}
