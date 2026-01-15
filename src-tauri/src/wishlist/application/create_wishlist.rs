use crate::core::domain::IdProvider;
use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::commands::CreateWishlistCommand;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;

/// Use case responsible for creating a new wishlist aggregate.
///
/// This use case constructs a `Wishlist` from the provided command,
/// persists it via the repository exposed by the `unit_of_work`, and
/// returns a lightweight `WishlistPreview` representing the created
/// resource.
pub struct CreateWishlistUseCase;

impl CreateWishlistUseCase {
    /// Execute the wishlist creation use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `create_wishlist`: validated domain command describing the wishlist to create.
    ///
    /// # Returns
    /// * `WishlistPreview` on success.
    /// * `DomainError` on failure.
    pub async fn execute<U, P>(
        unit_of_work: &mut U,
        id_provider: P,
        create_wishlist: CreateWishlistCommand,
    ) -> Result<WishlistPreview, DomainError>
    where
        U: WishlistUowExt + Send,
        P: IdProvider<WishlistId>,
    {
        let mut repo = unit_of_work.wishlist_repository();

        let wishlist = Wishlist {
            id: id_provider.next_id(),
            name: create_wishlist.name,
            notes: create_wishlist.notes,
            is_default: create_wishlist.is_default,
            items: Vec::new(),
        };

        repo.create_wishlist(&wishlist).await?;

        // Return the freshly created preview by listing previews and finding by id
        let previews = repo.list_wishlist_previews().await?;
        let maybe = previews
            .into_iter()
            .find(|p| p.id.to_string() == wishlist.id.to_string());

        match maybe {
            Some(p) => Ok(p),
            None => Err(DomainError::NotFound {
                resource: "WishlistPreview".to_string(),
                identifier: wishlist.id.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::test_utils::MockIdProvider;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn it_should_create_wishlists() {
        let mut mock = MockWishlistRepository::new();

        let id = WishlistId::default();
        let test_id_provider = MockIdProvider::new(id.clone());

        mock.expect_create_wishlist().times(1).returning(|_| Ok(()));

        mock.expect_list_wishlist_previews()
            .times(1)
            .returning(move || {
                let wishlist = WishlistPreview {
                    id: id.clone(),
                    name: "New Wishlist".to_string(),
                    is_default: false,
                    count: 0,
                    notes: None,
                    total_value: std::collections::HashMap::new(),
                    updated_at: NaiveDate::from_ymd_opt(2016, 7, 8)
                        .unwrap()
                        .and_hms_opt(9, 10, 11)
                        .unwrap(),
                };

                Ok(vec![wishlist.clone()])
            });

        let mut unit_of_work = FakeUow::new(mock);

        let cmd = CreateWishlistCommand {
            name: "New Wishlist".to_string(),
            notes: Some("Some notes".to_string()),
            is_default: false,
        };

        let preview = CreateWishlistUseCase::execute(&mut unit_of_work, test_id_provider, cmd)
            .await
            .expect("Failed to create wishlist");

        assert_eq!(preview.name, "New Wishlist");
    }
}
