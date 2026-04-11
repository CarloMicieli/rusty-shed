use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::queries::WishlistView;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_id::WishlistId;

/// Query to fetch a wishlist by its identifier.
pub struct GetWishlistByIdQuery;

impl GetWishlistByIdQuery {
    /// Execute the get wishlist by id query.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    /// - `id`: identifier of the wishlist to retrieve.
    ///
    /// # Returns
    /// * `Ok(Some(WishlistView))` if the wishlist is found.
    /// * `Ok(None)` if not found.
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(
        unit_of_work: &mut U,
        id: &WishlistId,
    ) -> Result<Option<WishlistView>, DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let wishlist = unit_of_work.wishlist_repository().find_by_id(id).await?;
        Ok(wishlist.map(WishlistView::from))
    }
}

#[cfg(test)]
mod tests {
    use super::GetWishlistByIdQuery;
    use crate::core::domain::metadata::Metadata;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist::Wishlist;
    use crate::wishlist::domain::wishlist_id::WishlistId;

    #[tokio::test]
    async fn get_wishlist_returns_none() {
        let mut mock = MockWishlistRepository::new();
        let id = WishlistId::default();
        let id_clone = id.clone();
        mock.expect_find_by_id()
            .times(1)
            .returning(move |_| Ok(None));

        let mut uow = FakeUow::new(mock);
        let res = GetWishlistByIdQuery::execute(&mut uow, &id_clone)
            .await
            .unwrap();

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn get_wishlist_returns_some() {
        let id = WishlistId::default();
        let id_for_mock = id.clone();

        let mut mock = MockWishlistRepository::new();
        mock.expect_find_by_id().times(1).returning(move |_| {
            Ok(Some(Wishlist {
                id: id_for_mock.clone(),
                name: "My Wishlist".to_string(),
                notes: None,
                is_default: false,
                items: vec![],
                pending_events: vec![],
                metadata: Metadata::default(),
            }))
        });

        let mut uow = FakeUow::new(mock);
        let res = GetWishlistByIdQuery::execute(&mut uow, &id).await.unwrap();

        assert!(res.is_some());
        let view = res.unwrap();
        assert_eq!(view.id, id);
        assert_eq!(view.name, "My Wishlist");
    }
}
