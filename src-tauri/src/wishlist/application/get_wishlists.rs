use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::infrastructure::repository::WishlistUowExt;
use anyhow::Result;

use crate::wishlist::domain::wishlist_preview::WishlistPreview;

/// Stateless use case to fetch wishlist previews.
pub struct GetWishlistsUseCase;

impl GetWishlistsUseCase {
    pub async fn execute(&self, uow: &mut SqliteUnitOfWork<'_>) -> Result<Vec<WishlistPreview>> {
        let mut repo = uow.wishlist_repo();
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
        let mut uow = SqliteUnitOfWork::new(&conn).await?;
        let use_case = GetWishlistsUseCase;
        let previews = use_case.execute(&mut uow).await?;
        assert!(previews.is_empty());
        Ok(())
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn list_wishlists_with_totals(conn: SqlitePool) -> Result<()> {
        // seed a wishlist and items in USD and EUR
        let wl_id = "test-wl-1";
        sqlx::query(
            "INSERT INTO wishlists (id, name, notes, is_default, created_at, updated_at) VALUES (?, ?, ?, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
        )
        .bind(wl_id)
        .bind("Seeded WL")
        .bind("notes")
        .execute(&conn)
        .await?;

        // items: USD 1000, USD 2500, EUR 750
        sqlx::query("INSERT INTO wishlist_items (id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date) VALUES (?, ?, ?, 'NORMAL', 'WANTED', ?, ?, '2025-12-26')")
            .bind("it-1")
            .bind(wl_id)
            .bind(Option::<String>::None)
            .bind(1000i64)
            .bind("USD")
            .execute(&conn)
            .await?;

        sqlx::query("INSERT INTO wishlist_items (id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date) VALUES (?, ?, ?, 'NORMAL', 'WANTED', ?, ?, '2025-12-26')")
            .bind("it-2")
            .bind(wl_id)
            .bind(Option::<String>::None)
            .bind(2500i64)
            .bind("USD")
            .execute(&conn)
            .await?;

        sqlx::query("INSERT INTO wishlist_items (id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date) VALUES (?, ?, ?, 'NORMAL', 'WANTED', ?, ?, '2025-12-26')")
            .bind("it-3")
            .bind(wl_id)
            .bind(Option::<String>::None)
            .bind(750i64)
            .bind("EUR")
            .execute(&conn)
            .await?;

        let mut uow = SqliteUnitOfWork::new(&conn).await?;
        let use_case = GetWishlistsUseCase;
        let previews = use_case.execute(&mut uow).await?;

        // find our wishlist
        let preview = previews
            .into_iter()
            .find(|p| p.id == wl_id)
            .expect("preview present");
        assert_eq!(preview.count, 3);
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
        assert_eq!(usd, 3500);
        assert_eq!(eur, 750);

        Ok(())
    }
}
