use crate::core::domain::domain_error::DomainError;
use crate::wishlist::application::queries::WishlistView;
use crate::wishlist::domain::repository::WishlistUowExt;

/// Query to fetch all wishlists with their previews.
pub struct GetWishlistsQuery;

impl GetWishlistsQuery {
    /// Execute the get wishlists query.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<WishlistView>` on success
    /// * `DomainError` on failure.
    ///
    /// # Type Parameters
    /// - `U`: Unit of work type implementing `WishlistUowExt` and `Send`.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<WishlistView>, DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let previews = unit_of_work.wishlist_repository().find_wishlists().await?;
        let views = previews.into_iter().map(WishlistView::from).collect();
        Ok(views)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wishlist::application::testing::FakeUow;
    use crate::wishlist::domain::MockWishlistRepository;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use crate::wishlist::domain::wishlist_preview::WishlistPreview;
    use chrono::NaiveDateTime;
    use std::collections::HashMap;

    #[tokio::test]
    async fn list_wishlists_empty() {
        let mut mock = MockWishlistRepository::new();
        mock.expect_find_wishlists()
            .times(1)
            .returning(|| Ok(vec![]));

        let mut uow = FakeUow::new(mock);
        let result = GetWishlistsQuery::execute(&mut uow).await.unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn list_wishlists_returns_views() {
        let id = WishlistId::default();
        let id_clone = id.clone();

        let mut mock = MockWishlistRepository::new();
        mock.expect_find_wishlists().times(1).returning(move || {
            Ok(vec![WishlistPreview {
                id: id_clone.clone(),
                name: "My List".to_string(),
                notes: None,
                is_default: true,
                count: 3,
                updated_at: NaiveDateTime::default(),
                total_value: HashMap::new(),
            }])
        });

        let mut uow = FakeUow::new(mock);
        let result = GetWishlistsQuery::execute(&mut uow).await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, id);
        assert_eq!(result[0].name, "My List");
        assert_eq!(result[0].count, 3);
        assert!(result[0].is_default);
    }
}
