use crate::core::domain::domain_error::DomainError;
use crate::wishlist::domain::repository::WishlistUowExt;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use anyhow::Result;

/// Stateless use case to fetch wishlist previews.
pub struct GetWishlistsUseCase;

impl GetWishlistsUseCase {
    /// Execute the get wishlists use case.
    ///
    /// # Arguments
    /// - `unit_of_work`: transactional unit providing repository access.
    ///
    /// # Returns
    /// * `Vec<WishlistPreview>` on success
    /// * `DomainError` on failure.
    pub async fn execute<U>(unit_of_work: &mut U) -> Result<Vec<WishlistPreview>, DomainError>
    where
        U: WishlistUowExt + Send,
    {
        let mut repo = unit_of_work.wishlist_repository();
        let previews = repo.list_wishlist_previews().await?;
        Ok(previews)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn list_wishlists_empty(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let previews = GetWishlistsUseCase::execute(&mut unit_of_work)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        assert!(previews.is_empty());
        Ok(())
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_wishlists.sql")
    )]
    async fn list_wishlists_with_totals(conn: SqlitePool) -> Result<()> {
        let wishlist_id = "trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9";

        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let previews = GetWishlistsUseCase::execute(&mut unit_of_work)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        // find our wishlist
        let preview = previews
            .into_iter()
            .find(|p| p.id.to_string() == wishlist_id)
            .expect("preview present");

        // Fixture contains two items for this wishlist
        assert_eq!(preview.count, 2);
        let usd = preview
            .total_value
            .get(&Currency::USD)
            .cloned()
            .unwrap_or(0);
        let eur = preview
            .total_value
            .get(&Currency::EUR)
            .cloned()
            .unwrap_or(0);

        assert_eq!(usd, 0);
        assert_eq!(eur, 17500 + 15000); // 32500

        Ok(())
    }
}
