use crate::data_management::application::ports::{AllDuplicates, ImportRepository, PersistResult};
use crate::data_management::domain::{
    DataContainerDto, DataManagementError, ImageFailure, RecordCounts,
};
use crate::data_management::infrastructure::ArchiveExtractor;
use crate::data_management::infrastructure::DuplicateChecker;
use crate::data_management::infrastructure::schema_mapper::{
    model_category_to_rolling_stock_category, schema_category_to_db, schema_maintenance_type_to_db,
    schema_power_method_to_db, schema_seller_type_to_db,
};
use async_trait::async_trait;
use log::warn;
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

const DEFAULT_COLLECTION_ID: &str = "trn:collection:1";
const DEFAULT_LANGUAGE_CODE: &str = "en";

/// SQLite-backed implementation of the `ImportRepository` port.
///
/// Handles all raw SQL, duplicate checking, and transactional writes for the import pipeline.
pub struct SqliteImportRepository {
    pool: SqlitePool,
}

impl SqliteImportRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ImportRepository for SqliteImportRepository {
    async fn check_duplicates(
        &self,
        data: &DataContainerDto,
    ) -> Result<AllDuplicates, DataManagementError> {
        let checker = DuplicateChecker::new(self.pool.clone());

        // All six checks are independent reads — run them concurrently
        let (
            manufacturer_dupes,
            railway_model_dupes,
            collection_item_dupes,
            seller_dupes,
            track_product_dupes,
            track_inventory_dupes,
        ) = tokio::try_join!(
            checker.check_manufacturers(&data.manufacturers),
            checker.check_railway_models(&data.railway_models),
            checker.check_collection_items(&data.collection_items),
            checker.check_sellers(&data.sellers),
            checker.check_track_products(&data.track_products),
            checker.check_track_inventories(&data.track_inventories),
        )
        .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        Ok(AllDuplicates {
            manufacturer_dupes,
            railway_model_dupes,
            collection_item_dupes,
            seller_dupes,
            track_product_dupes,
            track_inventory_dupes,
        })
    }

    async fn persist(
        &self,
        data: &DataContainerDto,
        duplicates: &AllDuplicates,
        archive_path: &Path,
        media_dir: &Path,
    ) -> Result<PersistResult, DataManagementError> {
        // Build HashSets for fast duplicate filtering
        let new_manufacturer_ids: HashSet<&str> = duplicates
            .manufacturer_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_model_ids: HashSet<&str> = duplicates
            .railway_model_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_item_ids: HashSet<&str> = duplicates
            .collection_item_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_seller_ids: HashSet<&str> = duplicates
            .seller_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_track_product_ids: HashSet<&str> = duplicates
            .track_product_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_track_inventory_ids: HashSet<&str> = duplicates
            .track_inventory_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();

        let mut added = RecordCounts::default();
        let mut skipped = RecordCounts::default();
        let mut pending_images: Vec<String> = vec![];

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        // Ensure the default collection exists
        sqlx::query("INSERT OR IGNORE INTO collections (id, name) VALUES (?, ?)")
            .bind(DEFAULT_COLLECTION_ID)
            .bind("My Collection")
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        // 1. Insert new manufacturers
        for m in data
            .manufacturers
            .iter()
            .filter(|m| new_manufacturer_ids.contains(m.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO manufacturers \
                 (id, name, registered_company_name, country_code, website_url, status) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&m.id)
            .bind(&m.name)
            .bind(&m.registered_company_name)
            .bind(&m.country_code)
            .bind(&m.website_url)
            .bind(m.status.as_deref().unwrap_or("ACTIVE"))
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.manufacturers = duplicates.manufacturer_dupes.new_count() as u32;
        skipped.manufacturers = duplicates.manufacturer_dupes.duplicate_count() as u32;

        // 2. Insert new railway companies (INSERT OR IGNORE — no explicit duplicate check)
        for rc in &data.railway_companies {
            sqlx::query(
                "INSERT OR IGNORE INTO railway_companies \
                 (id, name, country_code, status) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(&rc.id)
            .bind(&rc.name)
            .bind(&rc.country_code)
            .bind(rc.status.as_deref().unwrap_or("ACTIVE"))
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.railway_companies = data.railway_companies.len() as u32;

        // 3. Insert new railway models + translations + rolling stocks
        for model in data
            .railway_models
            .iter()
            .filter(|m| new_model_ids.contains(m.id.as_str()))
        {
            let power_method = schema_power_method_to_db(&model.power_method)?;
            let category = schema_category_to_db(&model.category.r#type)?;

            sqlx::query(
                "INSERT OR IGNORE INTO railway_models \
                 (id, manufacturer_id, product_code, power_method, scale, epoch, \
                  category, delivery_date, availability_status) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&model.id)
            .bind(&model.manufacturer_id)
            .bind(&model.product_code)
            .bind(power_method)
            .bind(&model.scale)
            .bind(&model.epoch)
            .bind(category)
            .bind(&model.delivery_date)
            .bind(&model.availability_status)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            if !model.description.is_empty() || model.details.is_some() {
                sqlx::query(
                    "INSERT OR IGNORE INTO railway_model_translations \
                     (railway_model_id, language_code, description, details) \
                     VALUES (?, ?, ?, ?)",
                )
                .bind(&model.id)
                .bind(DEFAULT_LANGUAGE_CODE)
                .bind(&model.description)
                .bind(&model.details)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }

            let rolling_stock_category = model_category_to_rolling_stock_category(category);
            for rs in &model.rolling_stocks {
                let rs_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT OR IGNORE INTO rolling_stocks \
                     (id, railway_model_id, category, railway_company_id, series_code, \
                      road_number, livery, friendly_name, is_dummy) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&rs_id)
                .bind(&model.id)
                .bind(rolling_stock_category)
                .bind(&rs.railway_company_id)
                .bind(&rs.series_code)
                .bind(&rs.road_number)
                .bind(&rs.livery)
                .bind(&rs.friendly_name)
                .bind(rs.is_dummy.unwrap_or(false) as i64)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }
        }

        added.railway_models = duplicates.railway_model_dupes.new_count() as u32;
        skipped.railway_models = duplicates.railway_model_dupes.duplicate_count() as u32;

        // 4. Insert new sellers
        for seller in data
            .sellers
            .iter()
            .filter(|s| new_seller_ids.contains(s.id.as_str()))
        {
            let seller_type = schema_seller_type_to_db(&seller.seller_type)?;
            sqlx::query(
                "INSERT OR IGNORE INTO sellers \
                 (id, name, type, email, phone, website_url) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&seller.id)
            .bind(&seller.name)
            .bind(seller_type)
            .bind(&seller.email)
            .bind(&seller.phone)
            .bind(&seller.website_url)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.sellers = duplicates.seller_dupes.new_count() as u32;
        skipped.sellers = duplicates.seller_dupes.duplicate_count() as u32;

        // 5. Insert new collection items + purchase infos
        for item in data
            .collection_items
            .iter()
            .filter(|i| new_item_ids.contains(i.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO collection_items \
                 (id, collection_id, railway_model_id, added_date, removed_date, \
                  purchase_condition, model_condition, box_condition, notes) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&item.id)
            .bind(DEFAULT_COLLECTION_ID)
            .bind(&item.railway_model_id)
            .bind(&item.added_date)
            .bind(&item.removed_date)
            .bind(&item.purchase_condition)
            .bind(&item.model_condition)
            .bind(&item.box_condition)
            .bind(&item.notes)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            // Collect image filename for post-commit copy (never inside the transaction)
            if let Some(ref image_filename) = item.image {
                pending_images.push(image_filename.clone());
            }

            if let Some(ref purchase) = item.purchase {
                let purchase_date = purchase
                    .purchase_date
                    .as_deref()
                    .unwrap_or(item.added_date.as_str());
                let purchase_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT OR IGNORE INTO purchase_infos \
                     (id, collection_item_id, purchase_type, purchase_date, seller_id, \
                      purchased_price_amount, purchased_price_currency, \
                      sale_date, sale_price_amount, sale_price_currency, \
                      deposit_amount, deposit_currency, expected_date) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&purchase_id)
                .bind(&item.id)
                .bind(&purchase.r#type)
                .bind(purchase_date)
                .bind(&purchase.seller_id)
                .bind(purchase.price.as_ref().map(|p| p.amount as i64))
                .bind(purchase.price.as_ref().map(|p| p.currency.as_str()))
                .bind(&purchase.sale_date)
                .bind(purchase.sale_price.as_ref().map(|p| p.amount as i64))
                .bind(purchase.sale_price.as_ref().map(|p| p.currency.as_str()))
                .bind(purchase.deposit_amount.as_ref().map(|p| p.amount as i64))
                .bind(
                    purchase
                        .deposit_amount
                        .as_ref()
                        .map(|p| p.currency.as_str()),
                )
                .bind(&purchase.expected_delivery)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }
        }

        added.collection_items = duplicates.collection_item_dupes.new_count() as u32;
        skipped.collection_items = duplicates.collection_item_dupes.duplicate_count() as u32;

        // 6. Insert new track products
        for product in data
            .track_products
            .iter()
            .filter(|p| new_track_product_ids.contains(p.track_id.as_str()))
        {
            let product_db_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT OR IGNORE INTO track_products \
                 (id, track_id, manufacturer_id, product_code, description, \
                  track_type, track_code, with_roadbed, length_mm, radius_mm, \
                  created_at, updated_at, version) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)",
            )
            .bind(&product_db_id)
            .bind(&product.track_id)
            .bind(&product.manufacturer_id)
            .bind(&product.product_code)
            .bind(&product.description)
            .bind(&product.track_type)
            .bind(&product.track_code)
            .bind(product.with_roadbed as i64)
            .bind(product.length)
            .bind(product.radius)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.track_products = duplicates.track_product_dupes.new_count() as u32;
        skipped.track_products = duplicates.track_product_dupes.duplicate_count() as u32;

        // 7. Insert new track inventories with items and purchases
        for inventory in data
            .track_inventories
            .iter()
            .filter(|inv| new_track_inventory_ids.contains(inv.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO track_inventories \
                 (id, name, description, created_at, updated_at, version) \
                 VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 0)",
            )
            .bind(&inventory.id)
            .bind(&inventory.name)
            .bind(&inventory.description)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            for item in &inventory.items {
                sqlx::query(
                    "INSERT OR IGNORE INTO track_inventory_items \
                     (inventory_id, track_id, quantity, required) \
                     VALUES (?, ?, ?, ?)",
                )
                .bind(&inventory.id)
                .bind(&item.track_id)
                .bind(item.quantity)
                .bind(item.required)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }

            for purchase in &inventory.purchases {
                sqlx::query(
                    "INSERT OR IGNORE INTO track_purchases \
                     (id, inventory_id, track_id, quantity, \
                      price_amount, price_currency, seller_id, purchase_date, created_at) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)",
                )
                .bind(&purchase.id)
                .bind(&inventory.id)
                .bind(&purchase.track_id)
                .bind(purchase.quantity)
                .bind(purchase.price.amount as i64)
                .bind(&purchase.price.currency)
                .bind(&purchase.seller_id)
                .bind(&purchase.purchase_date)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }
        }

        added.track_inventories = duplicates.track_inventory_dupes.new_count() as u32;
        skipped.track_inventories = duplicates.track_inventory_dupes.duplicate_count() as u32;

        // 8. Import maintenance cards via owned_rolling_stocks bridge
        for card in &data.maintenance_cards {
            let card_exists: bool =
                sqlx::query_scalar("SELECT COUNT(1) FROM maintenance_cards WHERE id = ?")
                    .bind(&card.id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|count: i64| count > 0)
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            if card_exists {
                skipped.maintenance_cards += 1;
                continue;
            }

            let item_exists: bool =
                sqlx::query_scalar("SELECT COUNT(1) FROM collection_items WHERE id = ?")
                    .bind(&card.collection_item_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|count: i64| count > 0)
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            if !item_exists {
                warn!(
                    "Skipping maintenance card '{}': collection item '{}' not found",
                    card.id, card.collection_item_id
                );
                skipped.maintenance_cards += 1;
                continue;
            }

            let ors_id = Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO owned_rolling_stocks (id, collection_item_id) VALUES (?, ?)")
                .bind(&ors_id)
                .bind(&card.collection_item_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            sqlx::query(
                "INSERT INTO maintenance_cards \
                 (id, owned_rolling_stock_id, last_maintenance_date, next_maintenance_date) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(&card.id)
            .bind(&ors_id)
            .bind(&card.last_maintenance_date)
            .bind(&card.next_maintenance_date)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            for event in &card.events {
                sqlx::query(
                    "INSERT OR IGNORE INTO maintenance_events \
                     (id, maintenance_card_id, date_performed, maintenance_type, notes) \
                     VALUES (?, ?, ?, ?, ?)",
                )
                .bind(&event.id)
                .bind(&card.id)
                .bind(&event.date)
                .bind(schema_maintenance_type_to_db(&event.r#type)?)
                .bind(&event.description)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }

            added.maintenance_cards += 1;
        }

        // Commit the transaction — all DB work done before any file I/O
        tx.commit()
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        // Extract all images from the archive in a single blocking task, then write them
        // asynchronously. This keeps the tokio runtime unblocked during file I/O.
        let archive_path_owned = archive_path.to_path_buf();
        let entry_paths: Vec<String> = pending_images
            .iter()
            .map(|f| format!("images/{}", f))
            .collect();
        let extracted =
            ArchiveExtractor::extract_files_batch_async(archive_path_owned, entry_paths)
                .await
                .map_err(|e| DataManagementError::IoError(e.to_string()))?;

        let mut images_imported: u32 = 0;
        let mut images_failed: Vec<ImageFailure> = vec![];
        for (image_filename, extract_result) in pending_images.iter().zip(extracted.into_iter()) {
            let (_, bytes_result) = extract_result;
            match bytes_result {
                Ok(bytes) => {
                    let dest = media_dir.join(image_filename);
                    if let Err(e) = tokio::fs::write(&dest, &bytes).await {
                        warn!("Failed to write image '{}': {}", image_filename, e);
                        images_failed
                            .push(ImageFailure::new(image_filename.clone(), e.to_string()));
                    } else {
                        images_imported += 1;
                    }
                }
                Err(e) => {
                    warn!("Image '{}' not found in archive: {}", image_filename, e);
                    images_failed.push(ImageFailure::new(image_filename.clone(), e.to_string()));
                }
            }
        }

        Ok(PersistResult {
            added,
            skipped,
            images_imported,
            images_failed,
        })
    }
}
