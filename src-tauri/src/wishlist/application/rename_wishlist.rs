use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::RenameWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Use case that renames an existing wishlist.
///
/// The use case updates the wishlist name in persistence. If the wishlist
/// is not found the repository will return `DomainError::NotFound`.
pub struct RenameWishlistUseCase;

impl RenameWishlistUseCase {
    /// Execute the rename wishlist use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `cmd`: command containing the wishlist id and new name.
    ///
    /// # Returns
    /// * `()` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: RenameWishlistCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        repo.rename_wishlist(&cmd.id, &cmd.name).await?;
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
    async fn it_should_rename_wishlists() {
        let mut mock = MockWishlistRepository::new();

        let id = WishlistId::default();

        mock.expect_rename_wishlist()
            .times(1)
            .with(eq(id.clone()), eq("New Wishlist Name".to_string()))
            .returning(|_, _| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let cmd = RenameWishlistCommand {
            id,
            name: "New Wishlist Name".to_string(),
        };

        let res = RenameWishlistUseCase::execute(&mut unit_of_work, cmd).await;

        assert!(res.is_ok());
    }
}
