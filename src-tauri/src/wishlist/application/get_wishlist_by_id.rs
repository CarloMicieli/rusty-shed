use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_id::WishlistId;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case to fetch a wishlist by its identifier.
///
/// This use case retrieves a `Wishlist` from the provided `SqliteUnitOfWork`.
/// It returns `Ok(Some(wishlist))` when found, `Ok(None)` when no wishlist
/// exists for the given id, or an error if the repository operation fails.
pub struct GetWishlistUseCase;

impl GetWishlistUseCase {
    pub async fn execute(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        id: &WishlistId,
    ) -> Result<Option<Wishlist>, DomainError> {
        let mut repo = unit_of_work.wishlist_repo();
        let wishlist = repo.get_wishlist_by_id(id).await?;
        Ok(wishlist)
    }
}

#[cfg(test)]
mod tests {
    use super::GetWishlistUseCase;
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
        let res = GetWishlistUseCase::execute(&mut unit_of_work, &id)
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
        let res = GetWishlistUseCase::execute(&mut unit_of_work, &id)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        assert!(res.is_some());
        let wishlist = res.unwrap();
        assert_eq!(
            wishlist.id.to_string(),
            "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9"
        );
        assert_eq!(wishlist.items.len(), 1);

        // Check the item's desired price mapping
        let item = &wishlist.items[0];
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
