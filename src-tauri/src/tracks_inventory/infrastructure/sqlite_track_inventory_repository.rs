use crate::core::domain::Currency;
use crate::core::domain::domain_error::DomainError;
use crate::core::domain::length::Length;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::domain::metadata::Metadata;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::tracks_inventory::domain::views::{
    TrackInventoryItemView, TrackInventoryListItem, TrackInventoryView, TrackProductView,
    TrackPurchaseView,
};
use crate::tracks_inventory::domain::{
    TrackCode, TrackInventory, TrackInventoryId, TrackInventoryRepository, TrackProductRepository,
    TrackPurchase, TrackQuantity, TrackType, TracksInventoryUowExt,
};
use crate::tracks_inventory::infrastructure::SqliteTrackProductRepository;
use crate::tracks_inventory::infrastructure::entities::{
    TrackInventoryHeaderViewRow, TrackInventoryItemRow, TrackInventoryItemViewRow,
    TrackInventoryRow, TrackInventorySummaryRow, TrackPurchaseRow, TrackPurchaseViewRow,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
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

    /// Fetch all inventories as summary list items (COUNT + SUM aggregates).
    async fn find_all_summaries_impl(
        &mut self,
    ) -> Result<Vec<TrackInventoryListItem>, DomainError> {
        let sql = r#"
            SELECT 
                ti.id,
                ti.name,
                ti.description,
                COUNT(DISTINCT tii.track_id) as total_items,
                COALESCE(SUM(tii.quantity), 0) as total_quantity
            FROM track_inventories ti
            LEFT JOIN track_inventory_items tii ON ti.id = tii.inventory_id
            GROUP BY ti.id, ti.name, ti.description
            ORDER BY ti.created_at DESC
        "#;

        let rows: Vec<TrackInventorySummaryRow> = sqlx::query_as(sql)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        Ok(rows
            .into_iter()
            .map(|r| TrackInventoryListItem {
                id: r.id,
                name: r.name.unwrap_or_default(),
                description: r.description,
                total_items: r.total_items,
                total_quantity: r.total_quantity,
            })
            .collect())
    }

    /// Fetch a single inventory with items and purchases as a view.
    async fn find_view_by_id_impl(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventoryView>, DomainError> {
        let header_sql = r#"
            SELECT id, name, description
            FROM track_inventories
            WHERE id = ?1
        "#;

        let header: Option<TrackInventoryHeaderViewRow> = sqlx::query_as(header_sql)
            .bind(id)
            .fetch_optional(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let header = match header {
            Some(h) => h,
            None => return Ok(None),
        };

        let items_sql = r#"
            SELECT 
                tii.track_id,
                tii.quantity,
                tii.required,
                tp.product_code,
                tp.description,
                tp.track_type,
                tp.track_code,
                tp.with_roadbed,
                tp.length_mm,
                tp.radius_mm,
                m.name as manufacturer_name
            FROM track_inventory_items tii
            INNER JOIN track_products tp ON tii.track_id = tp.track_id
            INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
            WHERE tii.inventory_id = ?1
            ORDER BY tp.product_code
        "#;

        let item_rows: Vec<TrackInventoryItemViewRow> = sqlx::query_as(items_sql)
            .bind(id)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let items: Vec<TrackInventoryItemView> = item_rows
            .into_iter()
            .map(|row| {
                let track_product = map_track_product_view(TrackProductFields {
                    track_id: row.track_id.clone(),
                    manufacturer_name: row.manufacturer_name,
                    product_code: row.product_code,
                    description: row.description,
                    track_type: row.track_type,
                    track_code: row.track_code,
                    with_roadbed: row.with_roadbed,
                    length_mm: row.length_mm,
                    radius_mm: row.radius_mm,
                });
                TrackInventoryItemView {
                    track_id: row.track_id,
                    track_product,
                    quantity: row.quantity,
                    required: row.required,
                }
            })
            .collect();

        let purchases_sql = r#"
            SELECT 
                tp_hist.id,
                tp_hist.track_id,
                tp_hist.quantity,
                tp_hist.price_amount,
                tp_hist.price_currency,
                tp_hist.purchase_date,
                s.name as seller_name,
                tp.product_code,
                tp.description,
                tp.track_type,
                tp.track_code,
                tp.with_roadbed,
                tp.length_mm,
                tp.radius_mm,
                m.name as manufacturer_name
            FROM track_purchases tp_hist
            INNER JOIN track_products tp ON tp_hist.track_id = tp.track_id
            INNER JOIN manufacturers m ON tp.manufacturer_id = m.id
            LEFT JOIN sellers s ON tp_hist.seller_id = s.id
            WHERE tp_hist.inventory_id = ?1
            ORDER BY tp_hist.purchase_date DESC
        "#;

        let purchase_rows: Vec<TrackPurchaseViewRow> = sqlx::query_as(purchases_sql)
            .bind(id)
            .fetch_all(&mut *self.executor)
            .await
            .map_err(DomainError::from)?;

        let purchases: Vec<TrackPurchaseView> = purchase_rows
            .into_iter()
            .map(|row| {
                let currency = Currency::from_code(&row.price_currency).unwrap_or(Currency::USD);
                let track_product = map_track_product_view(TrackProductFields {
                    track_id: row.track_id.clone(),
                    manufacturer_name: row.manufacturer_name,
                    product_code: row.product_code,
                    description: row.description,
                    track_type: row.track_type,
                    track_code: row.track_code,
                    with_roadbed: row.with_roadbed,
                    length_mm: row.length_mm,
                    radius_mm: row.radius_mm,
                });
                TrackPurchaseView {
                    id: row.id,
                    track_product,
                    quantity: row.quantity,
                    price: MonetaryAmount::new(row.price_amount, currency),
                    seller_name: row.seller_name,
                    purchase_date: row.purchase_date,
                }
            })
            .collect();

        Ok(Some(TrackInventoryView {
            id: header.id,
            name: header.name.unwrap_or_default(),
            description: header.description,
            items,
            purchases,
        }))
    }
}

/// Bundles the product-related columns that appear in both item and purchase view rows.
struct TrackProductFields {
    track_id: crate::tracks_inventory::domain::TrackId,
    manufacturer_name: String,
    product_code: String,
    description: Option<String>,
    track_type: Option<String>,
    track_code: Option<TrackCode>,
    with_roadbed: i64,
    length_mm: Option<i32>,
    radius_mm: Option<i32>,
}

/// Maps a [`TrackProductFields`] into a [`TrackProductView`].
fn map_track_product_view(f: TrackProductFields) -> TrackProductView {
    let mm_to_length = |mm: i32| {
        Decimal::from_i32(mm).and_then(|d| Length::try_new(d, MeasureUnit::Millimeters).ok())
    };
    TrackProductView {
        track_id: f.track_id,
        manufacturer_name: f.manufacturer_name,
        product_code: f.product_code,
        description: f.description.unwrap_or_default(),
        track_type: f
            .track_type
            .and_then(|t| t.parse::<TrackType>().ok())
            .unwrap_or(TrackType::Straight),
        track_code: f.track_code.unwrap_or(TrackCode::Code83),
        with_roadbed: f.with_roadbed == 1,
        length: f.length_mm.and_then(mm_to_length),
        radius: f.radius_mm.and_then(mm_to_length),
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

    async fn find_all_summaries(&mut self) -> Result<Vec<TrackInventoryListItem>, DomainError> {
        self.find_all_summaries_impl().await
    }

    async fn find_view_by_id(
        &mut self,
        id: &TrackInventoryId,
    ) -> Result<Option<TrackInventoryView>, DomainError> {
        self.find_view_by_id_impl(id).await
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
