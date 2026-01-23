use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::inputs::DeleteWishlistInput;
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
    /// - `input`: command containing the wishlist id to delete.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: DeleteWishlistInput,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.delete_wishlist(&input.id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use mockall::predicate::eq;

    #[tokio::test]
    async fn it_should_delete_wishlists() {
        let mut mock = MockWishlistRepository::new();

        let id = WishlistId::default();

        mock.expect_delete_wishlist()
            .times(1)
            .with(eq(id.clone()))
            .returning(|_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let input = DeleteWishlistInput { id };

        let res = DeleteWishlistUseCase::execute(&mut unit_of_work, input).await;

        assert!(res.is_ok());
    }
}
