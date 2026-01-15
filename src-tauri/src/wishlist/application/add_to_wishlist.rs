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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::RailwayModelId;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;

    #[tokio::test]
    async fn it_should_add_items_wishlists() {
        let mut mock = MockWishlistRepository::new();

        let id = WishlistItemId::default();
        let railway_model_id = RailwayModelId::try_from("trn:railway-model:test:1234").unwrap();
        let test_id_provider = MockIdProvider::new(id.clone());

        mock.expect_add_item().times(1).returning(|_, _| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let cmd = AddToWishlistCommand {
            wishlist_id: WishlistId::default(),
            railway_model_id,
            priority: WishlistPriority::Normal,
            status: WishlistStatus::Wanted,
            desired_price: None,
            notes: None,
            added_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        let result = AddToWishlistUseCase::execute(&mut unit_of_work, test_id_provider, cmd).await;

        assert!(result.is_ok());
    }
}
