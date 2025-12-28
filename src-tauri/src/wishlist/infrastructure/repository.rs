use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::wishlist::domain::repository::WishlistRepository;
use crate::wishlist::domain::wishlist::Wishlist;
use crate::wishlist::domain::wishlist_item::WishlistItem;
use crate::wishlist::infrastructure::database;

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
        pretty_assertions::assert_eq!(item.id, "2af7578c-8857-4894-8c93-0be4b579ff25");
        pretty_assertions::assert_eq!(
            item.railway_model_id.to_string(),
            "trn:railway-model:acme:60100".to_string()
        );
        pretty_assertions::assert_eq!(
            item.desired_price.as_ref().map(|p| p.amount),
            Some(12345u64)
        );
        pretty_assertions::assert_eq!(
            item.desired_price.as_ref().map(|p| p.currency),
            Some(Currency::EUR)
        );
        pretty_assertions::assert_eq!(item.priority, WishlistPriority::Normal);
        pretty_assertions::assert_eq!(item.status, WishlistStatus::Wanted);
        pretty_assertions::assert_eq!(item.notes, Some("Fixture item notes".to_string()));
        pretty_assertions::assert_eq!(
            item.added_date,
            chrono::NaiveDate::from_ymd_opt(2025, 12, 26).unwrap()
        );

        Ok(())
    }
}
