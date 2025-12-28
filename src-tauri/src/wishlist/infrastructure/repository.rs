use crate::core::domain::currency::Currency;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::repository::WishlistRepository;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::domain::wishlist_preview::WishlistPreview;
use crate::wishlist::infrastructure::database;
use crate::wishlist::infrastructure::entities::WishlistPreviewRow;
use std::collections::HashMap;

pub struct SqliteWishlistRepository<'conn> {
    /// A mutable reference to the database connection/executor.
    executor: &'conn mut sqlx::SqliteConnection,
}

impl<'conn> SqliteWishlistRepository<'conn> {
    /// Creates a new repository instance using the provided executor.
    pub fn new(executor: &'conn mut sqlx::SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> WishlistRepository for SqliteWishlistRepository<'conn> {
    /// Executes the SQLite-specific logic to fetch a wishlist by its ID.
    async fn get_wishlist_by_id(&mut self, id: &str) -> anyhow::Result<Option<Wishlist>> {
        let wishlist_row = database::find_wishlist_by_id(&mut *self.executor, id).await?;

        if wishlist_row.is_none() {
            return Ok(None);
        }

        let wishlist_item_rows =
            database::find_wishlist_items_by_id(&mut *self.executor, id).await?;

        let mut wishlist = Wishlist::try_from(wishlist_row.unwrap())?;

        for item_row in wishlist_item_rows {
            let item = WishlistItem::try_from(item_row)?;
            wishlist.add_item(item);
        }

        Ok(Some(wishlist))
    }

    async fn list_wishlist_previews(&mut self) -> anyhow::Result<Vec<WishlistPreview>> {
        let rows: Vec<WishlistPreviewRow> =
            database::find_wishlist_previews(&mut *self.executor).await?;

        let mut map: HashMap<String, WishlistPreview> = HashMap::with_capacity(rows.len());

        for row in rows.into_iter() {
            let entry = map.entry(row.wishlist_id.clone()).or_insert_with(|| {
                WishlistPreview {
                    id: row.wishlist_id,
                    name: row.name,
                    notes: row.notes,
                    is_default: row.is_default != 0,
                    count: 0,
                    updated_at: row.updated_at,
                    total_value: HashMap::with_capacity(2), // Most wishlists use 1-2 currencies
                }
            });

            entry.count += row.item_count;

            if let (Some(total), Some(curr_str)) = (row.total_amount, row.currency)
                && let Ok(currency) = Currency::from_code(&curr_str)
            {
                *entry.total_value.entry(currency).or_insert(0) += total;
            }
        }

        let mut previews: Vec<WishlistPreview> = Vec::with_capacity(map.len());
        previews.extend(map.into_values());

        previews.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(previews)
    }
}

pub trait WishlistUowExt {
    fn wishlist_repo(&mut self) -> Box<dyn WishlistRepository + '_>;
}

impl<'conn> WishlistUowExt for SqliteUnitOfWork<'conn> {
    /// Links the SQLite-specific repository to the Unit of Work.
    ///
    /// It re-borrows the internal transaction (`&mut *self.tx`) to provide
    /// the repository with a mutable executor without transferring ownership.
    fn wishlist_repo(&mut self) -> Box<dyn WishlistRepository + '_> {
        Box::new(SqliteWishlistRepository::new(&mut self.tx))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Currency;
    use crate::wishlist::domain::wishlist_priority::WishlistPriority;
    use crate::wishlist::domain::wishlist_status::WishlistStatus;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn get_wishlist_repo_returns_none(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repo();

        let result = repo.get_wishlist_by_id("non-existing-id").await?;
        assert!(result.is_none());

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_wishlist"))]
    async fn get_wishlist_repo_returns_some(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repo();

        let result = repo
            .get_wishlist_by_id("58fb6f1d-d838-44b5-b65c-21e5388ca4c9")
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

    #[sqlx::test(migrations = "./migrations")]
    async fn list_wishlist_previews_returns_empty(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repo();

        let previews = repo.list_wishlist_previews().await?;
        assert_eq!(previews.len(), 0);

        Ok(())
    }

    #[sqlx::test(migrations = "./migrations", fixtures("test_wishlists"))]
    async fn list_wishlist_previews_returns_preview(conn: SqlitePool) -> Result<()> {
        let mut unit_of_work = SqliteUnitOfWork::new(&conn).await?;
        let mut repo = unit_of_work.wishlist_repo();

        let wishlist_previews = repo.list_wishlist_previews().await?;

        assert_eq!(wishlist_previews.len(), 2);

        let first_wishlist = &wishlist_previews[0];
        assert_eq!(first_wishlist.name, "Test Wishlist 1");
        assert_eq!(first_wishlist.id, "58fb6f1d-d838-44b5-b65c-21e5388ca4c9");
        assert_eq!(first_wishlist.count, 2);
        assert_eq!(first_wishlist.notes, Some("Notes".to_string()));
        assert_eq!(first_wishlist.is_default, false);
        assert_eq!(first_wishlist.total_value.get(&Currency::EUR), Some(&32500));

        let second_wishlist = &wishlist_previews[1];
        assert_eq!(second_wishlist.name, "Test Wishlist 2");
        assert_eq!(second_wishlist.id, "c9950910-96e1-47ae-8097-cd0ebbaa83f5");
        assert_eq!(second_wishlist.count, 2);
        assert_eq!(second_wishlist.notes, Some("Notes".to_string()));
        assert_eq!(second_wishlist.is_default, true);
        assert_eq!(
            second_wishlist.total_value.get(&Currency::EUR),
            Some(&15000)
        );
        assert_eq!(
            second_wishlist.total_value.get(&Currency::USD),
            Some(&17500)
        );

        Ok(())
    }
}
