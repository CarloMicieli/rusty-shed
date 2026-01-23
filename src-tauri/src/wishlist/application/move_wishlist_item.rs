use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::inputs::MoveWishlistItemInput;
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
    /// - `input`: command containing the item id and destination wishlist id.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: MoveWishlistItemInput,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.move_item(&input.item_id, &input.destination_wishlist_id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn it_should_move_wishlist_items() {
        let mut mock = MockWishlistRepository::new();

        let item_id = WishlistItemId::default();
        let destination_wishlist_id = WishlistId::default();
        let wishlist_id = WishlistId::default();

        mock.expect_move_item()
            .times(1)
            .with(eq(item_id.clone()), eq(destination_wishlist_id.clone()))
            .returning(|_, _| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let input = MoveWishlistItemInput {
            item_id,
            destination_wishlist_id,
            wishlist_id,
        };

        let res = MoveWishlistItemUseCase::execute(&mut unit_of_work, input).await;

        assert!(res.is_ok());
    }
}
