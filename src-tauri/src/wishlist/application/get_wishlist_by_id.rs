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
    use crate::core::domain::currency::Currency;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::wishlist::domain::wishlist_id::WishlistId;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlist_returns_none(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;

        let id = WishlistId::default();
        let res = GetWishlistByIdQuery::execute(&mut unit_of_work, &id)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        assert!(res.is_none());

        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlist.sql")
    )]
    async fn get_wishlist_returns_some(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;

        let id = WishlistId::try_from("trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9")?;
        let res = GetWishlistByIdQuery::execute(&mut unit_of_work, &id)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        assert!(res.is_some());
        let wishlist = res.unwrap();
        assert_eq!(
            wishlist.id.to_string(),
            "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9"
        );
        let items = wishlist.items.expect("items present");
        assert_eq!(items.len(), 1);

        // Check the item's desired price mapping
        let item = &items[0];
        // Assert the railway model id was mapped correctly from the DB row
        assert_eq!(
            item.railway_model_id.to_string(),
            "trn:railway-model:acme:60100".to_string()
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.amount),
            Some(12345i64)
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.currency),
            Some(Currency::EUR)
        );

        Ok(())
    }
}
