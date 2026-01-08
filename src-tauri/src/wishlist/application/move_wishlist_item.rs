use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::commands::MoveWishlistItemCommand;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case that moves a wishlist item to a different wishlist.
///
/// This operation updates the owning wishlist reference for the item. If
/// the item does not exist the repository will return a `DomainError::NotFound`.
pub struct MoveWishlistItemUseCase;

impl MoveWishlistItemUseCase {
    /// Execute the move-wishlist-item use case.
    ///
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the item id and destination wishlist id.
    ///
    /// Returns `()` on success or a `DomainError` on failure.
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        cmd: MoveWishlistItemCommand,
    ) -> Result<(), DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        repo.move_item(&cmd.item_id, &cmd.destination_wishlist_id)
            .await?;
        Ok(())
    }
}
