use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::MoveWishlistItemCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that moves a wishlist item to a different wishlist.
///
/// This operation updates the owning wishlist reference for the item. If
/// the item does not exist the repository will return a `DomainError::NotFound`.
pub struct MoveWishlistItemUseCase;

impl MoveWishlistItemUseCase {
    /// Execute the move wishlist item use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the item id and destination wishlist id.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: MoveWishlistItemCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.move_item(&cmd.item_id, &cmd.destination_wishlist_id)
            .await?;
        Ok(())
    }
}
