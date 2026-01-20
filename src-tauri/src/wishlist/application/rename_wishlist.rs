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
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        cmd: RenameWishlistCommand,
    ) -> Result<(), DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();

        // Load aggregate, apply rename which emits an event, then persist
        // the aggregate via the repository which will process events.
        let maybe = repo.find_by_id(&cmd.id).await?;
        let mut wishlist = maybe.ok_or(DomainError::NotFound {
            resource: "Wishlist".to_string(),
            identifier: cmd.id.to_string(),
        })?;

        wishlist.rename(&cmd.name);
        repo.save_wishlist(&wishlist).await?;
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
        let id_for_expect = id.clone();
        let id_for_return = id.clone();

        mock.expect_find_by_id()
            .times(1)
            .with(eq(id_for_expect))
            .returning(move |_| {
                Ok(Some(crate::wishlist::domain::wishlist::Wishlist {
                    id: id_for_return.clone(),
                    name: "Old Name".to_string(),
                    notes: None,
                    is_default: false,
                    items: vec![],
                    pending_events: vec![],
                    metadata: crate::core::domain::metadata::Metadata::default(),
                }))
            });

        mock.expect_save_wishlist()
            .times(1)
            .withf(move |w| w.name == "New Wishlist Name")
            .returning(|_| Ok(()));

        let mut unit_of_work = FakeUow::new(mock);

        let cmd = RenameWishlistCommand {
            id,
            name: "New Wishlist Name".to_string(),
        };

        let res = RenameWishlistUseCase::execute(&mut unit_of_work, cmd).await;

        assert!(res.is_ok());
    }
}
