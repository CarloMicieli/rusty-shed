//! SQLite-backed repository implementations for the tracks inventory feature.
//!
//! This module implements:
//! - [`SqliteTrackInventoryRepository`] – persists [`TrackInventory`] aggregates.
//! - [`SqliteTrackProductRepository`] – persists [`TrackProduct`] master data.
//! - [`TracksInventoryUowExt`] for [`SqliteUnitOfWork`] – provides both repos.
//! - [`TrackProductUowExt`] for [`SqliteUnitOfWork`] – focused product-only access.
//!
//! Repository methods delegate all raw SQL to [`super::database`] and all
//! row-to-domain conversions to [`super::mappers`].

use crate::catalog::domain::manufacturer::ManufacturerId;
use crate::core::domain::domain_error::DomainError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::domain::views::{
    TrackInventoryListItem, TrackInventoryView, TrackProductView,
};
use crate::tracks_inventory::domain::{
    TrackId, TrackInventory, TrackInventoryEvent, TrackInventoryId, TrackInventoryRepository,
    TrackProduct, TrackProductRepository, TrackProductUowExt, TracksInventoryUowExt,
};
use crate::tracks_inventory::infrastructure::{database, mappers};
use rust_decimal::prelude::ToPrimitive;
use sqlx::SqliteConnection;

// ---------------------------------------------------------------------------
// SqliteTrackInventoryRepository
// ---------------------------------------------------------------------------

/// SQLite-backed repository for [`TrackInventory`] aggregates.
///
/// Reads are performed with three coordinated queries (header, items,
/// purchases); writes are performed event-by-event using the domain event
/// log emitted by the aggregate.
pub struct SqliteTrackInventoryRepository<'conn> {
    /// Mutable reference to the active database connection/transaction.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteTrackInventoryRepository<'conn> {
    /// Creates a new repository bound to the given executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> TrackInventoryRepository for SqliteTrackInventoryRepository<'conn> {
    /// Loads a [`TrackInventory`] aggregate by its ID, including all inventory
    /// items and purchase history.
    ///
    /// Returns `Ok(None)` when no inventory with the given `id` exists.
    async fn find_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventory>, DomainError> {
        let header = database::find_track_inventory_by_id(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        let header = match header {
            Some(h) => h,
            None => return Ok(None),
        };

        let item_rows = database::find_track_inventory_items(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        let purchase_rows = database::find_track_purchases(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        let inventory = mappers::assemble_track_inventory(header, item_rows, purchase_rows)?;
        Ok(Some(inventory))
    }

    /// Applies the pending domain events on the given [`TrackInventory`] to
    /// the database.
    ///
    /// Each event variant maps to one or more targeted SQL operations.
    /// When there are no pending events the method is a no-op.
    async fn save(&mut self, inventory: TrackInventory) -> Result<(), DomainError> {
        let mut inventory = inventory;
        let events = inventory.pull_events();

        if events.is_empty() {
            return Ok(());
        }

        for ev in events {
            match ev {
                TrackInventoryEvent::Created => {
                    database::insert_track_inventory(
                        &mut *self.executor,
                        &inventory.id,
                        &inventory.name,
                        inventory.description.as_deref(),
                    )
                    .await
                    .map_err(DomainError::from)?;
                }

                TrackInventoryEvent::Renamed { name } => {
                    database::rename_track_inventory(&mut *self.executor, &inventory.id, &name)
                        .await
                        .map_err(DomainError::from)?;
                }

                TrackInventoryEvent::DescriptionUpdated { description } => {
                    database::update_track_inventory_description(
                        &mut *self.executor,
                        &inventory.id,
                        description.as_deref(),
                    )
                    .await
                    .map_err(DomainError::from)?;
                }

                TrackInventoryEvent::ItemQuantitySet { track_id, quantity } => {
                    if quantity <= 0 {
                        database::delete_track_inventory_item(
                            &mut *self.executor,
                            &inventory.id,
                            &track_id,
                        )
                        .await
                        .map_err(DomainError::from)?;
                    } else {
                        database::upsert_track_inventory_item(
                            &mut *self.executor,
                            &inventory.id,
                            &track_id,
                            quantity,
                        )
                        .await
                        .map_err(DomainError::from)?;
                    }
                }

                TrackInventoryEvent::PurchaseAdded { purchase } => {
                    database::insert_track_purchase(
                        &mut *self.executor,
                        &inventory.id,
                        &purchase.track_purchase_id.to_string(),
                        &purchase.track_id,
                        purchase.quantity,
                        purchase.price.amount,
                        purchase.price.currency.to_code(),
                        purchase.seller_id.as_ref().map(|s| s.to_string()),
                        &purchase.purchase_date.to_string(),
                    )
                    .await
                    .map_err(DomainError::from)?;

                    // Increment the item quantity without clobbering the
                    // user-managed `required` column.
                    database::increment_inventory_item_quantity(
                        &mut *self.executor,
                        &inventory.id,
                        &purchase.track_id,
                        purchase.quantity,
                    )
                    .await
                    .map_err(DomainError::from)?;
                }
            }
        }

        // Bump the header timestamp to reflect the latest mutations.
        database::touch_inventory_updated_at(&mut *self.executor, &inventory.id)
            .await
            .map_err(DomainError::from)?;

        Ok(())
    }

    /// Deletes the inventory identified by `id`.
    ///
    /// Cascading deletes on `track_inventory_items` and `track_purchases` are
    /// expected to be enforced by database foreign-key constraints.
    async fn delete(&mut self, id: &TrackInventoryId) -> Result<(), DomainError> {
        database::delete_track_inventory(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)
    }

    /// Updates the `required` quantity for a specific item within an inventory.
    ///
    /// Returns `true` if the row was found and updated, `false` otherwise.
    async fn set_item_required(
        &mut self,
        inventory_id: &TrackInventoryId,
        track_id: &TrackId,
        required: i64,
    ) -> Result<bool, DomainError> {
        let rows = database::set_inventory_item_required(
            &mut *self.executor,
            inventory_id,
            track_id,
            required,
        )
        .await
        .map_err(DomainError::from)?;

        Ok(rows > 0)
    }

    /// Returns summary list items for all inventories (item count + total
    /// quantity aggregated via SQL).
    async fn find_all_summaries(&mut self) -> Result<Vec<TrackInventoryListItem>, DomainError> {
        let rows = database::find_all_inventory_summaries(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows.into_iter().map(TrackInventoryListItem::from).collect())
    }

    /// Returns a fully-populated view model for a single inventory, or `None`
    /// when the inventory does not exist.
    async fn find_view_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventoryView>, DomainError> {
        let header = database::find_inventory_header_view(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        let header = match header {
            Some(h) => h,
            None => return Ok(None),
        };

        let item_rows = database::find_inventory_item_views(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        let purchase_rows = database::find_inventory_purchase_views(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        Ok(Some(mappers::assemble_inventory_view(
            header,
            item_rows,
            purchase_rows,
        )))
    }
}

// ---------------------------------------------------------------------------
// SqliteTrackProductRepository
// ---------------------------------------------------------------------------

/// SQLite-backed repository for [`TrackProduct`] master-data records.
pub struct SqliteTrackProductRepository<'conn> {
    /// Mutable reference to the active database connection/transaction.
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteTrackProductRepository<'conn> {
    /// Creates a new repository bound to the given executor.
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }
}

#[async_trait::async_trait]
impl<'conn> TrackProductRepository for SqliteTrackProductRepository<'conn> {
    /// Finds a [`TrackProduct`] by its canonical `TrackId`.
    ///
    /// Returns `Ok(None)` when no matching record exists.
    async fn find_by_id(&mut self, id: &TrackId) -> Result<Option<TrackProduct>, DomainError> {
        let row = database::find_track_product_by_id(&mut *self.executor, id)
            .await
            .map_err(DomainError::from)?;

        row.map(TrackProduct::try_from).transpose()
    }

    /// Finds a [`TrackProduct`] by manufacturer ID and product code.
    ///
    /// Returns `Ok(None)` when no matching record exists.
    async fn find_by_product_code(
        &mut self,
        manufacturer_id: &ManufacturerId,
        product_code: &str,
    ) -> Result<Option<TrackProduct>, DomainError> {
        let row = database::find_track_product_by_code(
            &mut *self.executor,
            &manufacturer_id.to_string(),
            product_code,
        )
        .await
        .map_err(DomainError::from)?;

        row.map(TrackProduct::try_from).transpose()
    }

    /// Returns all track products as display views joined with manufacturer names.
    async fn find_all_views(&mut self) -> Result<Vec<TrackProductView>, DomainError> {
        let rows = database::find_all_product_views(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows.into_iter().map(TrackProductView::from).collect())
    }

    /// Persists a [`TrackProduct`] master record.
    ///
    /// Uses an upsert strategy (`INSERT OR REPLACE`) so the method is safe
    /// to call for both new and existing products.
    async fn save(&mut self, track: TrackProduct) -> Result<(), DomainError> {
        database::upsert_track_product(
            &mut *self.executor,
            &track.track_id,
            &track.manufacturer_id.to_string(),
            &track.product_code,
            if track.with_roadbed { 1 } else { 0 },
            track.length.map(|l| l.quantity().to_i32().unwrap_or(0)),
            track.radius.map(|r| r.quantity().to_i32().unwrap_or(0)),
            track.track_code,
            track.track_type,
        )
        .await
        .map_err(DomainError::from)
    }
}

// ---------------------------------------------------------------------------
// Unit-of-Work extension impls
// ---------------------------------------------------------------------------

impl TracksInventoryUowExt for SqliteUnitOfWork {
    /// Returns a boxed [`TrackProductRepository`] bound to this unit of work's
    /// transaction lifetime.
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        Box::new(SqliteTrackProductRepository::new(&mut self.tx))
    }

    /// Returns a boxed [`TrackInventoryRepository`] bound to this unit of work's
    /// transaction lifetime.
    fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_> {
        Box::new(SqliteTrackInventoryRepository::new(&mut self.tx))
    }
}

impl TrackProductUowExt for SqliteUnitOfWork {
    /// Returns a boxed [`TrackProductRepository`] bound to this unit of work's
    /// transaction lifetime.
    ///
    /// This focused extension mirrors [`TracksInventoryUowExt::track_products_repo`]
    /// for callers that only need product master data.
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        Box::new(SqliteTrackProductRepository::new(&mut self.tx))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
    use crate::tracks_inventory::domain::TracksInventoryUowExt;

    #[sqlx::test(migrations = "./migrations")]
    async fn track_inventories_repo_find_by_id_returns_none(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool)
            .await
            .expect("unit of work creation should succeed");

        let missing_id = crate::tracks_inventory::domain::TrackInventoryId::try_from(
            "trn:track-inventory:00000000-0000-0000-0000-999999999999",
        )
        .unwrap();

        let mut repo = uow.track_inventories_repo();
        let result = repo.find_by_id(&missing_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn track_products_repo_find_by_id_returns_none(pool: sqlx::SqlitePool) {
        let mut uow = SqliteUnitOfWork::new(&pool)
            .await
            .expect("unit of work creation should succeed");

        let missing_id =
            crate::tracks_inventory::domain::TrackId::try_from("trn:track:acme:99999").unwrap();

        let mut repo = TracksInventoryUowExt::track_products_repo(&mut uow);
        let result: Result<Option<crate::tracks_inventory::domain::TrackProduct>, DomainError> =
            repo.find_by_id(&missing_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
