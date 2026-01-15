use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::AddToWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_item_id::WishlistItemId;

/// Use case that adds a new item to an existing wishlist.
///
/// It creates a `WishlistItem` from the command, persists it using the
/// repository provided by the `unit_of_work` and returns the persisted
/// `WishlistItem`.
pub struct AddToWishlistUseCase;

impl AddToWishlistUseCase {
    /// Execute the add-to-wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: domain command containing item details and target wishlist id.
    ///
    /// # Returns
    /// * `WishlistItem` on success
    /// * `DomainError` on failure.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        cmd: AddToWishlistCommand,
    ) -> Result<WishlistItem, DomainError>
    where
        U: WishlistUowExt + Send,
        P: IdProvider<WishlistItemId>,
    {
        let mut repo = unit_of_work.wishlist_repository();

        let item = WishlistItem {
            id: id_provider.next_id(),
            railway_model_id: cmd.railway_model_id,
            priority: cmd.priority,
            status: cmd.status,
            added_date: cmd.added_date,
            removed_date: None,
            notes: cmd.notes,
            desired_price: cmd.desired_price,
            purchased_price: None,
        };

        repo.add_item(&cmd.wishlist_id, &item).await?;
        Ok(item)
    }
}
