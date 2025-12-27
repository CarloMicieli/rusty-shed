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
