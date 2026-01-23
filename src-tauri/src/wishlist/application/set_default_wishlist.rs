use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::inputs::SetDefaultWishlistInput;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that marks a wishlist as the default.
///
/// The use case ensures the provided wishlist is recorded as the single
/// default list for the user (repository handles exclusivity).
pub struct SetDefaultWishlistUseCase;

impl SetDefaultWishlistUseCase {
    /// Execute the set default wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `input`: command carrying the wishlist id to mark default.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        input: SetDefaultWishlistInput,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.set_default_wishlist(&input.id).await?;
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
    async fn it_should_set_wishlist_as_default() {
        let mut mock = MockWishlistRepository::new();

        let id = WishlistId::default();

        mock.expect_set_default_wishlist()
            .times(1)
            .with(eq(id.clone()))
            .returning(|_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let input = SetDefaultWishlistInput { id };

        let res = SetDefaultWishlistUseCase::execute(&mut unit_of_work, input).await;

        assert!(res.is_ok());
    }
}
