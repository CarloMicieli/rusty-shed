use crate::core::domain::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::metadata::Metadata;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::domain::{
    TrackInventory, TrackInventoryId, TrackInventoryRepository, TrackProductRepository,
    TrackPurchase, TrackQuantity, TracksInventoryUowExt,
};
use crate::tracks_inventory::infrastructure::SqliteTrackProductRepository;
use crate::tracks_inventory::infrastructure::entities::{
    TrackInventoryItemRow, TrackInventoryRow, TrackPurchaseRow,
};
use chrono::{DateTime, Utc};
use sqlx::SqliteConnection;
use std::collections::HashMap;

pub struct SqliteTrackInventoryRepository<'conn> {
    executor: &'conn mut SqliteConnection,
}

impl<'conn> SqliteTrackInventoryRepository<'conn> {
    pub fn new(executor: &'conn mut SqliteConnection) -> Self {
        Self { executor }
    }

    async fn load_inventory(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventory>, DomainError> {
        let sql_inv = r#"
            SELECT id, created_at, updated_at, version, name, description
            FROM track_inventories
            WHERE id = ?1
            LIMIT 1
        "#;

        let header: Option<TrackInventoryRow> = sqlx::query_as(sql_inv)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let header = match header {
            Some(h) => h,
            None => return Ok(None),
        };

        let sql_items = r#"
            SELECT track_id, quantity, required 
            FROM track_inventory_items 
            WHERE inventory_id = ?1
        "#;

        let inventory_items: Vec<TrackInventoryItemRow> = sqlx::query_as(sql_items)
            .bind(id)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut inventory_map = HashMap::new();
        for inventory_item in inventory_items {
            inventory_map.insert(
                inventory_item.track_id.clone(),
                TrackQuantity {
                    track_id: inventory_item.track_id,
                    quantity: inventory_item.quantity,
                },
            );
        }

        // Load purchases using typed rows
        let sql_purchases = r#"
            SELECT id, track_id, quantity, price_amount, price_currency, seller_id, purchase_date
            FROM track_purchases
            WHERE inventory_id = ?1
            ORDER BY purchase_date ASC
        "#;

        let track_purchase_rows: Vec<TrackPurchaseRow> = sqlx::query_as(sql_purchases)
            .bind(id)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let mut track_purchases = Vec::new();
        for track_purchase_row in track_purchase_rows {
            let currency = Currency::from_code(&track_purchase_row.price_currency)
                .map_err(|e| DomainError::Validation(e.to_string()))?;
            let monetary = MonetaryAmount::new(track_purchase_row.price_amount, currency);

            let track_purchase = TrackPurchase {
                track_purchase_id: track_purchase_row.id,
                track_id: track_purchase_row.track_id,
                quantity: track_purchase_row.quantity,
                price: monetary,
                seller_id: track_purchase_row.seller_id,
                purchase_date: track_purchase_row.purchase_date,
            };

            track_purchases.push(track_purchase);
        }

        // Build metadata from header
        let created_at_dt: DateTime<Utc> = header.created_at;
        let updated_at_dt: DateTime<Utc> = header.updated_at;
        let version_u8: u8 = if header.version < 0 {
            0
        } else if header.version > (u8::MAX as i64) {
            u8::MAX
        } else {
            header.version as u8
        };

        let metadata = Metadata {
            version: version_u8,
            created_at: created_at_dt,
            updated_at: updated_at_dt,
        };

        // Construct TrackInventory with mapped metadata
        let inventory = TrackInventory {
            id: header.id,
            inventory: inventory_map,
            purchase_history: track_purchases,
            metadata,
            name: header.name.unwrap_or_default(),
            description: header.description,
            pending_events: Vec::new(),
        };

        Ok(Some(inventory))
    }
}

#[async_trait::async_trait]
impl<'conn> TrackInventoryRepository for SqliteTrackInventoryRepository<'conn> {
    async fn find_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventory>, DomainError> {
        self.load_inventory(id).await
    }

    async fn delete(&mut self, id: &TrackInventoryId) -> Result<(), DomainError> {
        let sql = "DELETE FROM track_inventories WHERE id = ?1";
        sqlx::query(sql)
            .bind(id)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;
        Ok(())
    }

    async fn set_item_required(
        &mut self,
        inventory_id: &TrackInventoryId,
        track_id: &crate::tracks_inventory::domain::TrackId,
        required: i64,
    ) -> Result<bool, DomainError> {
        let sql = r#"
            UPDATE track_inventory_items
            SET required = ?1
            WHERE inventory_id = ?2 AND track_id = ?3
        "#;
        let result = sqlx::query(sql)
            .bind(required)
            .bind(inventory_id)
            .bind(track_id)
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;
        Ok(result.rows_affected() > 0)
    }

    async fn save(&mut self, inventory: TrackInventory) -> Result<(), DomainError> {
        // Prefer event-driven persistence when the aggregate emitted events.
        let mut inventory = inventory;
        let events = inventory.pull_events();

        if !events.is_empty() {
            // Apply events incrementally. This keeps compatibility with the
            // existing schema while allowing producers to emit fine-grained
            // changes.
            for ev in events.into_iter() {
                match ev {
                    crate::tracks_inventory::domain::TrackInventoryEvent::Created => {
                        // Insert header with the aggregate's current name/description to satisfy NOT NULL constraints.
                        let sql_upsert = r#"
                            INSERT OR REPLACE INTO track_inventories (id, created_at, updated_at, version, name, description)
                            VALUES (?1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0, ?2, ?3)
                        "#;
                        sqlx::query(sql_upsert)
                            .bind(inventory.id.to_string())
                            .bind(inventory.name.clone())
                            .bind(inventory.description.clone())
                            .execute(&mut *self.executor)
                            .await
                            .map_err(DomainError::from)?;
                    }

                    crate::tracks_inventory::domain::TrackInventoryEvent::Renamed { name } => {
                        let sql = r#"UPDATE track_inventories SET name = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2"#;
                        sqlx::query(sql)
                            .bind(name)
                            .bind(inventory.id.to_string())
                            .execute(&mut *self.executor)
                            .await
                            .map_err(DomainError::from)?;
                    }

                    crate::tracks_inventory::domain::TrackInventoryEvent::DescriptionUpdated {
                        description,
                    } => {
                        let sql = r#"UPDATE track_inventories SET description = ?1, updated_at = CURRENT_TIMESTAMP WHERE id = ?2"#;
                        sqlx::query(sql)
                            .bind(description)
                            .bind(inventory.id.to_string())
                            .execute(&mut *self.executor)
                            .await
                            .map_err(DomainError::from)?;
                    }

                    crate::tracks_inventory::domain::TrackInventoryEvent::ItemQuantitySet {
                        track_id,
                        quantity,
                    } => {
                        if quantity <= 0 {
                            let sql_del = r#"DELETE FROM track_inventory_items WHERE inventory_id = ?1 AND track_id = ?2"#;
                            sqlx::query(sql_del)
                                .bind(inventory.id.to_string())
                                .bind(track_id.to_string())
                                .execute(&mut *self.executor)
                                .await
                                .map_err(DomainError::from)?;
                        } else {
                            let sql_upd = r#"INSERT OR REPLACE INTO track_inventory_items (inventory_id, track_id, quantity) VALUES (?1, ?2, ?3)"#;
                            sqlx::query(sql_upd)
                                .bind(inventory.id.to_string())
                                .bind(track_id.to_string())
                                .bind(quantity)
                                .execute(&mut *self.executor)
                                .await
                                .map_err(DomainError::from)?;
                        }
                    }

                    crate::tracks_inventory::domain::TrackInventoryEvent::PurchaseAdded {
                        purchase,
                    } => {
                        let insert_purchase = r#"
                            INSERT OR REPLACE INTO track_purchases (
                                id, inventory_id, track_id, quantity, price_amount,
                                price_currency, seller_id, purchase_date, created_at)
                            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, CURRENT_TIMESTAMP)
                        "#;

                        sqlx::query(insert_purchase)
                            .bind(purchase.track_purchase_id.to_string())
                            .bind(inventory.id.to_string())
                            .bind(purchase.track_id.to_string())
                            .bind(purchase.quantity)
                            .bind(purchase.price.amount)
                            .bind(purchase.price.currency.to_code())
                            .bind(purchase.seller_id.as_ref().map(|s| s.to_string()))
                            .bind(purchase.purchase_date.to_string())
                            .execute(&mut *self.executor)
                            .await
                            .map_err(DomainError::from)?;

                        // Increment the item quantity in the inventory without
                        // clobbering the `required` field set by the user.
                        let upsert_item = r#"
                            INSERT INTO track_inventory_items (inventory_id, track_id, quantity)
                            VALUES (?1, ?2, ?3)
                            ON CONFLICT (inventory_id, track_id)
                            DO UPDATE SET quantity = quantity + excluded.quantity
                        "#;

                        sqlx::query(upsert_item)
                            .bind(inventory.id.to_string())
                            .bind(purchase.track_id.to_string())
                            .bind(purchase.quantity)
                            .execute(&mut *self.executor)
                            .await
                            .map_err(DomainError::from)?;
                    }
                }
            }

            // Touch the inventory header's updated_at timestamp after applying events
            let _ = sqlx::query(
                "UPDATE track_inventories SET updated_at = CURRENT_TIMESTAMP WHERE id = ?1",
            )
            .bind(inventory.id.to_string())
            .execute(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

            return Ok(());
        }

        // No events: nothing to persist.
        Ok(())
    }
}

impl TracksInventoryUowExt for SqliteUnitOfWork {
    fn track_products_repo(&mut self) -> Box<dyn TrackProductRepository + '_> {
        Box::new(SqliteTrackProductRepository::new(&mut self.tx))
    }

    fn track_inventories_repo(&mut self) -> Box<dyn TrackInventoryRepository + '_> {
        Box::new(SqliteTrackInventoryRepository::new(&mut self.tx))
    }
}
