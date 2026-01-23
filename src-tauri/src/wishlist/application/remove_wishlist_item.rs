use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::inputs::RemoveWishlistItemInput;
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
    /// - `input`: command carrying the `WishlistItemId` to remove.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: RemoveWishlistItemInput,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.remove_item(&input.item_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist_item_id::WishlistItemId;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn it_should_remove_wishlist_items() {
        let mut mock = MockWishlistRepository::new();

        let item_id = WishlistItemId::default();

        mock.expect_remove_item()
            .times(1)
            .with(eq(item_id.clone()))
            .returning(|_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let input = RemoveWishlistItemInput { item_id };

        let res = RemoveWishlistItemUseCase::execute(&mut unit_of_work, input).await;

        assert!(res.is_ok());
    }
}
