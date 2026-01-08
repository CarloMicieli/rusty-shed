use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::RemoveWishlistItemCommand;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case that removes a wishlist item by its identifier.
///
/// This operation deletes the item from persistence. If the item does not
/// exist the repository returns a `DomainError::NotFound` which is
/// propagated to the caller.
pub struct RemoveWishlistItemUseCase;

impl RemoveWishlistItemUseCase {
    /// Execute the remove-wishlist-item use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command carrying the `WishlistItemId` to remove.
    ///
    /// Returns `()` on success or a `DomainError` on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: RemoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        repo.remove_item(&cmd.item_id).await?;
        Ok(())
    }
}
