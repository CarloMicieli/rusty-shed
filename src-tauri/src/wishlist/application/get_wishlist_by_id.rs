use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::infrastructure::repository::WishlistUowExt;

/// Use case to fetch a wishlist by its identifier.
///
/// This use case retrieves a `Wishlist` from the provided `SqliteUnitOfWork`.
/// It returns `Ok(Some(wishlist))` when found, `Ok(None)` when no wishlist
/// exists for the given id, or an error if the repository operation fails.
pub struct GetWishlistUseCase;

impl GetWishlistUseCase {
    pub async fn execute(
        &self,
        uow: &mut SqliteUnitOfWork<'_>,
        id: String,
    ) -> anyhow::Result<Option<crate::wishlist::domain::wishlist::Wishlist>> {
        let mut repo = uow.wishlist_repo();
        let wishlist = repo.get_wishlist_by_id(&id).await?;
        Ok(wishlist)
    }
}

#[cfg(test)]
mod tests {
    use super::GetWishlistUseCase;
    use crate::core::domain::currency::Currency;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlist_returns_none(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;

        let use_case = GetWishlistUseCase;
        let result = use_case
            .execute(&mut unit_of_work, "non-existing-id".to_string())
            .await?;

        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_wishlist"))]
    async fn get_wishlist_returns_some(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;

        let use_case = GetWishlistUseCase;
        let result = use_case
            .execute(
                &mut unit_of_work,
                "58fb6f1d-d838-44b5-b65c-21e5388ca4c9".to_string(),
            )
            .await?;

        assert!(result.is_some());
        let wishlist = result.unwrap();
        assert_eq!(wishlist.id, "58fb6f1d-d838-44b5-b65c-21e5388ca4c9");
        assert_eq!(wishlist.items.len(), 1);

        let item = &wishlist.items[0];
        assert_eq!(item.id, "2af7578c-8857-4894-8c93-0be4b579ff25");
        assert_eq!(
            item.railway_model_id.to_string(),
            "trn:railway-model:acme:60100".to_string()
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.amount),
            Some(12345u64)
        );
        assert_eq!(
            item.desired_price.as_ref().map(|p| p.currency),
            Some(Currency::EUR)
        );
        assert_eq!(item.priority, WishlistPriority::Normal);
        assert_eq!(item.status, WishlistStatus::Wanted);
        assert_eq!(item.notes, Some("Fixture item notes".to_string()));
        assert_eq!(
            item.added_date,
            chrono::NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()
        );

        Ok(())
    }
}
