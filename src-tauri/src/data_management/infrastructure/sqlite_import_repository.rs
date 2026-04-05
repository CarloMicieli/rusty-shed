use crate::data_management::application::ports::{AllDuplicates, ImportRepository, PersistResult};
use crate::data_management::domain::{
    DataContainerDto, DataManagementError, ImageFailure, RecordCounts,
};
use crate::data_management::infrastructure::ArchiveExtractor;
use crate::data_management::infrastructure::DuplicateChecker;
use crate::data_management::infrastructure::schema_mapper::{
    model_category_to_rolling_stock_category, schema_box_condition_to_db, schema_category_to_db,
    schema_maintenance_type_to_db, schema_manufacturer_status_to_db, schema_model_condition_to_db,
    schema_power_method_to_db, schema_purchase_condition_to_db, schema_purchase_type_to_db,
    schema_railway_company_status_to_db, schema_seller_type_to_db,
};
use crate::search::infrastructure::sqlite_global_search_repository::rebuild_search_index;
use async_trait::async_trait;
use log::warn;
use sqlx::SqlitePool;
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

const DEFAULT_COLLECTION_ID: &str = "trn:collection:1";

fn format_decimal_for_text(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        let mut text = value.to_string();
        while text.contains('.') && text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        text
    }
}

fn synthesize_rolling_stock_id(
    model_id: &str,
    railway_company_id: &str,
    series_code: &str,
    road_number: Option<&str>,
    index: usize,
) -> String {
    format!(
        "rs::{model_id}::{railway_company_id}::{series_code}::{}::{index}",
        road_number.unwrap_or_default()
    )
}

/// Map a sqlx error to a `DataManagementError`, producing a human-readable message for
/// SQLite foreign key violations (error code 787) instead of the raw SQLite error string.
fn map_db_error(e: sqlx::Error, context: &str) -> DataManagementError {
    if let sqlx::Error::Database(ref db_err) = e
        && db_err.code().as_deref() == Some("787")
    {
        return DataManagementError::DatabaseError(format!(
            "Foreign key constraint failed while inserting {context}: \
             a referenced record does not exist in the database. \
             Ensure all manufacturers, railway companies, and sellers \
             in the manifest are present and consistent.",
        ));
    }
    DataManagementError::DatabaseError(e.to_string())
}

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

        // All checks are independent reads — run them concurrently
        let (
            manufacturer_dupes,
            railway_model_dupes,
            collection_item_dupes,
            seller_dupes,
            track_product_dupes,
            track_inventory_dupes,
            formation_category_dupes,
            train_formation_dupes,
            prototype_dupes,
            wishlist_dupes,
            decoder_dupes,
            digital_roster_dupes,
        ) = tokio::try_join!(
            checker.check_manufacturers(&data.manufacturers),
            checker.check_railway_models(&data.railway_models),
            checker.check_collection_items(&data.collection_items),
            checker.check_sellers(&data.sellers),
            checker.check_track_products(&data.track_products),
            checker.check_track_inventories(&data.track_inventories),
            checker.check_formation_categories(&data.formation_categories),
            checker.check_train_formations(&data.train_formations),
            checker.check_prototypes(&data.prototypes),
            checker.check_wishlists(&data.wishlists),
            checker.check_decoders(&data.decoders),
            checker.check_digital_roster(&data.digital_rolling_stocks),
        )
        .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        Ok(AllDuplicates {
            manufacturer_dupes,
            railway_model_dupes,
            collection_item_dupes,
            seller_dupes,
            track_product_dupes,
            track_inventory_dupes,
            formation_category_dupes,
            train_formation_dupes,
            prototype_dupes,
            wishlist_dupes,
            decoder_dupes,
            digital_roster_dupes,
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
        let new_formation_category_ids: HashSet<&str> = duplicates
            .formation_category_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_train_formation_ids: HashSet<&str> = duplicates
            .train_formation_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_prototype_ids: HashSet<&str> = duplicates
            .prototype_dupes
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
            let status = schema_manufacturer_status_to_db(m.status.as_deref())?;
            sqlx::query(
                "INSERT OR IGNORE INTO manufacturers \
                 (id, name, registered_company_name, country_code, website_url, status, \
                  street_address, extended_address, city, state_region, postal_code) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&m.id)
            .bind(&m.name)
            .bind(&m.registered_company_name)
            .bind(&m.country_code)
            .bind(&m.website_url)
            .bind(status)
            .bind(&m.street_address)
            .bind(&m.extended_address)
            .bind(&m.city)
            .bind(&m.state_region)
            .bind(&m.postal_code)
            .execute(&mut *tx)
            .await
            .map_err(|e| map_db_error(e, "manufacturer"))?;
        }

        added.manufacturers = duplicates.manufacturer_dupes.new_count() as u32;
        skipped.manufacturers = duplicates.manufacturer_dupes.duplicate_count() as u32;

        // 2. Insert new railway companies (INSERT OR IGNORE — no explicit duplicate check)
        for rc in &data.railway_companies {
            let status = schema_railway_company_status_to_db(rc.status.as_deref())?;
            sqlx::query(
                "INSERT OR IGNORE INTO railway_companies \
                 (id, name, country_code, status, operating_since, operating_until) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&rc.id)
            .bind(&rc.name)
            .bind(&rc.country_code)
            .bind(status)
            .bind(&rc.operating_since)
            .bind(&rc.operating_until)
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
            let category = schema_category_to_db(&model.category)?;

            let model_result = sqlx::query(
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
            .map_err(|e| map_db_error(e, "railway model"))?;

            if model_result.rows_affected() == 0 {
                warn!(
                    "Skipping model '{}': manufacturer_id '{}' not found in database",
                    model.id, model.manufacturer_id
                );
                continue;
            }

            for language_code in ["en", "it"] {
                let description = match language_code {
                    "en" => model.description.en.as_deref(),
                    _ => model.description.it.as_deref(),
                };
                let details = model.details.as_ref().and_then(|d| match language_code {
                    "en" => d.en.as_deref(),
                    _ => d.it.as_deref(),
                });

                if description.is_some() || details.is_some() {
                    sqlx::query(
                        "INSERT OR IGNORE INTO railway_model_translations \
                         (railway_model_id, language_code, description, details) \
                         VALUES (?, ?, ?, ?)",
                    )
                    .bind(&model.id)
                    .bind(language_code)
                    .bind(description)
                    .bind(details)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
                }
            }

            let rolling_stock_category = model_category_to_rolling_stock_category(category);
            for (index, rs) in model.rolling_stocks.iter().enumerate() {
                let rs_id = rs.id.clone().unwrap_or_else(|| {
                    synthesize_rolling_stock_id(
                        &model.id,
                        &rs.railway_company_id,
                        &rs.series_code,
                        rs.road_number.as_deref(),
                        index,
                    )
                });
                sqlx::query(
                    "INSERT OR IGNORE INTO rolling_stocks \
                     (id, railway_model_id, category, railway_company_id, series_code, \
                      series, road_number, friendly_name, depot, livery, \
                      electric_multiple_unit_type, freight_car_type, locomotive_type, \
                      passenger_car_type, railcar_type, service_level, length_inches, \
                      length_millimeters, technical_minimum_radius_mm, technical_coupling_socket, \
                      technical_coupling_close_couplers, technical_coupling_digital_shunting, \
                      technical_flywheel_fitted, technical_body_shell, technical_chassis, \
                      technical_interior_lights, technical_lights, technical_sprung_buffers, \
                      dcc_interface, control, is_dummy) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                )
                .bind(&rs_id)
                .bind(&model.id)
                .bind(rolling_stock_category)
                .bind(&rs.railway_company_id)
                .bind(&rs.series_code)
                .bind(&rs.series)
                .bind(&rs.road_number)
                .bind(&rs.friendly_name)
                .bind(&rs.depot)
                .bind(&rs.livery)
                .bind(&rs.electric_multiple_unit_type)
                .bind(&rs.freight_car_type)
                .bind(&rs.locomotive_type)
                .bind(&rs.passenger_car_type)
                .bind(&rs.railcar_type)
                .bind(&rs.service_level)
                .bind(rs.length_inches.map(format_decimal_for_text))
                .bind(rs.length_millimeters.map(format_decimal_for_text))
                .bind(rs.technical_minimum_radius_mm.map(format_decimal_for_text))
                .bind(&rs.technical_coupling_socket)
                .bind(&rs.technical_coupling_close_couplers)
                .bind(&rs.technical_coupling_digital_shunting)
                .bind(&rs.technical_flywheel_fitted)
                .bind(&rs.technical_body_shell)
                .bind(&rs.technical_chassis)
                .bind(&rs.technical_interior_lights)
                .bind(&rs.technical_lights)
                .bind(&rs.technical_sprung_buffers)
                .bind(&rs.dcc_interface)
                .bind(&rs.control)
                .bind(rs.is_dummy.unwrap_or(false) as i64)
                .execute(&mut *tx)
                .await
                .map_err(|e| map_db_error(e, "rolling stock"))?;
            }
        }

        added.railway_models = duplicates.railway_model_dupes.new_count() as u32;
        skipped.railway_models = duplicates.railway_model_dupes.duplicate_count() as u32;

        // Rebuild FTS search index for all newly inserted models.
        // Must run after translations and rolling_stocks are inserted (they feed the index).
        for model_id in &duplicates.railway_model_dupes.new_ids {
            rebuild_search_index(model_id, &mut tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

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
            .map_err(|e| map_db_error(e, "seller"))?;
        }

        added.sellers = duplicates.seller_dupes.new_count() as u32;
        skipped.sellers = duplicates.seller_dupes.duplicate_count() as u32;

        // 5. Insert new collection items + purchase infos
        for item in data
            .collection_items
            .iter()
            .filter(|i| new_item_ids.contains(i.id.as_str()))
        {
            let purchase_condition =
                schema_purchase_condition_to_db(item.purchase_condition.as_deref())?;
            let model_condition = schema_model_condition_to_db(item.model_condition.as_deref())?;
            let box_condition = schema_box_condition_to_db(item.box_condition.as_deref())?;
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
            .bind(purchase_condition)
            .bind(model_condition)
            .bind(box_condition)
            .bind(&item.notes)
            .execute(&mut *tx)
            .await
            .map_err(|e| map_db_error(e, "collection item"))?;

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
                let purchase_type = schema_purchase_type_to_db(&purchase.r#type)?;
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
                .bind(purchase_type)
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
                .map_err(|e| map_db_error(e, "purchase record"))?;
            }
        }

        added.collection_items = duplicates.collection_item_dupes.new_count() as u32;
        skipped.collection_items = duplicates.collection_item_dupes.duplicate_count() as u32;

        // 5.5 Insert owned_rolling_stocks — bridge rows linking collection_items to rolling_stocks.
        // Uses INSERT OR IGNORE to be idempotent on re-import.
        let mut added_ors = 0u32;
        for ors in &data.owned_rolling_stocks {
            let item_exists: bool =
                sqlx::query_scalar("SELECT COUNT(1) FROM collection_items WHERE id = ?")
                    .bind(&ors.collection_item_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|count: i64| count > 0)
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            if !item_exists {
                warn!(
                    "Skipping owned_rolling_stock '{}': collection item '{}' not found",
                    ors.id, ors.collection_item_id
                );
                continue;
            }

            sqlx::query(
                "INSERT OR IGNORE INTO owned_rolling_stocks \
                 (id, collection_item_id, rolling_stock_id, notes, dcc_address, \
                  installed_decoder_id, current_coupler_id) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&ors.id)
            .bind(&ors.collection_item_id)
            .bind(&ors.rolling_stock_id)
            .bind(&ors.notes)
            .bind(ors.dcc_address)
            .bind(&ors.installed_decoder_id)
            .bind(&ors.current_coupler_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            added_ors += 1;
        }
        added.owned_rolling_stocks = added_ors;

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

            // Resolve the owned_rolling_stock id for this maintenance card.
            // New-format archives carry the original `owned_rolling_stock_id`; when that row
            // was already inserted by step 5.5 we reuse it directly.  For old-format archives
            // (field absent) or when the ORS row is missing, fall back to creating a minimal
            // owned_rolling_stocks row so the maintenance card FK is satisfied.
            let ors_id = match &card.owned_rolling_stock_id {
                Some(existing_id) => {
                    let ors_exists: bool = sqlx::query_scalar(
                        "SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?",
                    )
                    .bind(existing_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|count: i64| count > 0)
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

                    if ors_exists {
                        // Step 5.5 already inserted the full row — nothing more to do.
                        existing_id.clone()
                    } else {
                        // ORS not present (e.g. maintenance-only export without collection items).
                        // Insert a minimal bridge row to satisfy the FK.
                        sqlx::query(
                            "INSERT OR IGNORE INTO owned_rolling_stocks \
                             (id, collection_item_id) VALUES (?, ?)",
                        )
                        .bind(existing_id)
                        .bind(&card.collection_item_id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
                        existing_id.clone()
                    }
                }
                None => {
                    // Old-format archive: generate a fresh id and create a minimal ORS row.
                    let new_id = Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO owned_rolling_stocks (id, collection_item_id) VALUES (?, ?)",
                    )
                    .bind(&new_id)
                    .bind(&card.collection_item_id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
                    new_id
                }
            };

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

        // 9. Insert new formation categories (INSERT OR IGNORE by unique name)
        for cat in data
            .formation_categories
            .iter()
            .filter(|c| new_formation_category_ids.contains(c.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO formation_categories \
                 (id, name, is_custom) \
                 VALUES (?, ?, ?)",
            )
            .bind(&cat.id)
            .bind(&cat.name)
            .bind(cat.is_custom as i64)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.formation_categories = duplicates.formation_category_dupes.new_count() as u32;
        skipped.formation_categories = duplicates.formation_category_dupes.duplicate_count() as u32;

        // 10. Insert new prototypes (INSERT OR IGNORE by PK; FK may already be satisfied)
        // Railway companies are inserted earlier (step 2 via INSERT OR IGNORE).
        for proto in data
            .prototypes
            .iter()
            .filter(|p| new_prototype_ids.contains(p.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO prototypes \
                 (id, railway_company_id, series_code, friendly_name, \
                  specification_type, \
                  locomotive_type, locomotive_series, \
                  service_level, passenger_car_type, \
                  freight_car_type, railcar_type, \
                  electric_multiple_unit_type, elements_count, is_permanently_coupled, \
                  is_motorized, is_custom) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&proto.id)
            .bind(&proto.railway_company_id)
            .bind(&proto.series_code)
            .bind(&proto.friendly_name)
            .bind(&proto.specification_type)
            .bind(&proto.locomotive_type)
            .bind(&proto.locomotive_series)
            .bind(&proto.service_level)
            .bind(&proto.passenger_car_type)
            .bind(&proto.freight_car_type)
            .bind(&proto.railcar_type)
            .bind(&proto.electric_multiple_unit_type)
            .bind(proto.elements_count)
            .bind(proto.is_permanently_coupled.map(i64::from))
            .bind(proto.is_motorized as i64)
            .bind(proto.is_custom as i64)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.prototypes = duplicates.prototype_dupes.new_count() as u32;
        skipped.prototypes = duplicates.prototype_dupes.duplicate_count() as u32;

        // 11. Insert new train formations + elements
        for formation in data
            .train_formations
            .iter()
            .filter(|f| new_train_formation_ids.contains(f.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO train_formations \
                 (id, name, category_id, start_year, end_year, epoch, notes) \
                 VALUES (?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&formation.id)
            .bind(&formation.name)
            .bind(&formation.category_id)
            .bind(formation.start_year)
            .bind(formation.end_year)
            .bind(&formation.epoch)
            .bind(&formation.notes)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            for element in &formation.elements {
                // owned_rolling_stock_id is intentionally NOT restored — it links to
                // collection items in the source DB which may not exist here.
                sqlx::query(
                    "INSERT OR IGNORE INTO formation_elements \
                     (id, formation_id, prototype_id, owned_rolling_stock_id, \
                      position_order, traction_override) \
                     VALUES (?, ?, ?, NULL, ?, ?)",
                )
                .bind(&element.id)
                .bind(&formation.id)
                .bind(&element.prototype_id)
                .bind(element.position_order)
                .bind(element.traction_override)
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }
        }

        added.train_formations = duplicates.train_formation_dupes.new_count() as u32;
        skipped.train_formations = duplicates.train_formation_dupes.duplicate_count() as u32;

        // 12. Insert new wishlists + items
        let new_wishlist_ids: HashSet<&str> = duplicates
            .wishlist_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();

        for wishlist in data
            .wishlists
            .iter()
            .filter(|w| new_wishlist_ids.contains(w.id.as_str()))
        {
            // Always import with is_default = 0 to avoid overriding the user's existing default
            sqlx::query(
                "INSERT OR IGNORE INTO wishlists \
                 (id, name, notes, is_default, version, created_at, updated_at) \
                 VALUES (?, ?, ?, 0, 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)",
            )
            .bind(&wishlist.id)
            .bind(&wishlist.name)
            .bind(&wishlist.notes)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            for item in &wishlist.items {
                sqlx::query(
                    "INSERT OR IGNORE INTO wishlist_items \
                     (id, wishlist_id, railway_model_id, priority, status, \
                      desired_price_amount, desired_price_currency, \
                      added_date, removed_date, notes, \
                      purchased_at, purchased_price_amount, purchased_price_currency) \
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, ?, ?)",
                )
                .bind(&item.id)
                .bind(&wishlist.id)
                .bind(&item.railway_model_id)
                .bind(&item.priority)
                .bind(&item.status)
                .bind(item.desired_price.as_ref().map(|p| p.amount as i64))
                .bind(item.desired_price.as_ref().map(|p| p.currency.as_str()))
                .bind(&item.added_date)
                .bind(&item.removed_date)
                .bind(&item.notes)
                .bind(item.purchased_price.as_ref().map(|p| p.amount as i64))
                .bind(item.purchased_price.as_ref().map(|p| p.currency.as_str()))
                .execute(&mut *tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
            }
        }

        added.wishlists = duplicates.wishlist_dupes.new_count() as u32;
        skipped.wishlists = duplicates.wishlist_dupes.duplicate_count() as u32;

        // 13. Insert new decoders
        let new_decoder_ids: HashSet<&str> = duplicates
            .decoder_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();

        for decoder in data
            .decoders
            .iter()
            .filter(|d| new_decoder_ids.contains(d.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO decoders \
                 (id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&decoder.id)
            .bind(&decoder.manufacturer_id)
            .bind(&decoder.product_code)
            .bind(&decoder.decoder_type)
            .bind(&decoder.protocol)
            .bind(&decoder.decoder_interface)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
        }

        added.decoders = duplicates.decoder_dupes.new_count() as u32;
        skipped.decoders = duplicates.decoder_dupes.duplicate_count() as u32;

        // 14. Insert new digital roster entries
        let new_roster_ids: HashSet<&str> = duplicates
            .digital_roster_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();

        for item in data
            .digital_rolling_stocks
            .iter()
            .filter(|i| new_roster_ids.contains(i.id.as_str()))
        {
            // owned_rolling_stock_id is a DB-internal FK; skip entries whose reference
            // doesn't exist in the target database (same pattern as maintenance card import).
            let ors_exists: bool =
                sqlx::query_scalar("SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?")
                    .bind(&item.owned_rolling_stock_id)
                    .fetch_one(&mut *tx)
                    .await
                    .map(|count: i64| count > 0)
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            if !ors_exists {
                warn!(
                    "Skipping digital roster entry '{}': owned rolling stock '{}' not found",
                    item.id, item.owned_rolling_stock_id
                );
                skipped.digital_rolling_stocks += 1;
                continue;
            }

            sqlx::query(
                "INSERT OR IGNORE INTO digital_rolling_stocks \
                 (id, owned_rolling_stock_id, dcc_address, installed_decoder_id) \
                 VALUES (?, ?, ?, ?)",
            )
            .bind(&item.id)
            .bind(&item.owned_rolling_stock_id)
            .bind(item.dcc_address)
            .bind(&item.decoder_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            added.digital_rolling_stocks += 1;
        }

        // Skipped count already accumulated above; also add deduplication skips
        skipped.digital_rolling_stocks += duplicates.digital_roster_dupes.duplicate_count() as u32;

        // Recalculate the collection summary from live data.
        // The import writes directly to collection_items / purchase_infos without going
        // through the domain's save() path, so the denormalized counts and total_value
        // in the collections row must be refreshed here.
        sqlx::query(
            r#"UPDATE collections SET
                locomotives_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'LOCOMOTIVES'
                ),
                passenger_cars_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'PASSENGER_CARS'
                ),
                freight_cars_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'FREIGHT_CARS'
                ),
                railcars_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'RAILCARS'
                ),
                train_sets_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'TRAIN_SETS'
                ),
                starter_sets_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'STARTER_SETS'
                ),
                electric_multiple_units_count = (
                    SELECT COUNT(*) FROM collection_items ci
                    JOIN railway_models rm ON rm.id = ci.railway_model_id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL AND rm.category = 'ELECTRIC_MULTIPLE_UNITS'
                ),
                total_value_amount = (
                    SELECT COALESCE(SUM(pi.purchased_price_amount), 0)
                    FROM collection_items ci
                    JOIN purchase_infos pi ON pi.collection_item_id = ci.id
                    WHERE ci.collection_id = ? AND ci.removed_date IS NULL
                )
            WHERE id = ?"#,
        )
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .bind(DEFAULT_COLLECTION_ID)
        .execute(&mut *tx)
        .await
        .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

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
