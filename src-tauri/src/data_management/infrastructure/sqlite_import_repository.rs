use crate::data_management::application::ports::{AllDuplicates, ImportRepository, PersistResult};
use crate::data_management::domain::{
    DataContainerDto, DataManagementError, ImageFailure, MaintenanceCardRecord, RailwayModelRecord,
    RecordCounts,
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
use sqlx::{Sqlite, SqlitePool, Transaction};
use std::collections::HashSet;
use std::path::Path;
use tracing::warn;
use uuid::Uuid;

const DEFAULT_COLLECTION_ID: &str = "trn:collection:1";

#[derive(Debug, Default)]
struct NewIdSets {
    manufacturer_ids: HashSet<String>,
    model_ids: HashSet<String>,
    item_ids: HashSet<String>,
    seller_ids: HashSet<String>,
    track_product_ids: HashSet<String>,
    track_inventory_ids: HashSet<String>,
    formation_category_ids: HashSet<String>,
    train_formation_ids: HashSet<String>,
    prototype_ids: HashSet<String>,
    wishlist_ids: HashSet<String>,
    decoder_ids: HashSet<String>,
    digital_roster_ids: HashSet<String>,
}

fn build_new_id_sets(duplicates: &AllDuplicates) -> NewIdSets {
    NewIdSets {
        manufacturer_ids: duplicates
            .manufacturer_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        model_ids: duplicates
            .railway_model_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        item_ids: duplicates
            .collection_item_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        seller_ids: duplicates.seller_dupes.new_ids.iter().cloned().collect(),
        track_product_ids: duplicates
            .track_product_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        track_inventory_ids: duplicates
            .track_inventory_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        formation_category_ids: duplicates
            .formation_category_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        train_formation_ids: duplicates
            .train_formation_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
        prototype_ids: duplicates.prototype_dupes.new_ids.iter().cloned().collect(),
        wishlist_ids: duplicates.wishlist_dupes.new_ids.iter().cloned().collect(),
        decoder_ids: duplicates.decoder_dupes.new_ids.iter().cloned().collect(),
        digital_roster_ids: duplicates
            .digital_roster_dupes
            .new_ids
            .iter()
            .cloned()
            .collect(),
    }
}

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

    async fn exists_by_id(
        tx: &mut Transaction<'_, Sqlite>,
        query: &str,
        id: &str,
    ) -> Result<bool, DataManagementError> {
        sqlx::query_scalar(query)
            .bind(id)
            .fetch_one(&mut **tx)
            .await
            .map(|count: i64| count > 0)
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))
    }

    async fn insert_rolling_stocks_for_model(
        tx: &mut Transaction<'_, Sqlite>,
        model: &RailwayModelRecord,
        rolling_stock_category: &str,
    ) -> Result<(), DataManagementError> {
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
            .execute(&mut **tx)
            .await
            .map_err(|e| map_db_error(e, "rolling stock"))?;
        }

        Ok(())
    }

    async fn resolve_maintenance_ors_id(
        tx: &mut Transaction<'_, Sqlite>,
        card: &MaintenanceCardRecord,
    ) -> Result<String, DataManagementError> {
        match &card.owned_rolling_stock_id {
            Some(existing_id) => {
                let ors_exists = Self::exists_by_id(
                    tx,
                    "SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?",
                    existing_id,
                )
                .await?;

                if ors_exists {
                    return Ok(existing_id.clone());
                }

                sqlx::query(
                    "INSERT OR IGNORE INTO owned_rolling_stocks (id, collection_item_id) VALUES (?, ?)",
                )
                .bind(existing_id)
                .bind(&card.collection_item_id)
                .execute(&mut **tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

                Ok(existing_id.clone())
            }
            None => {
                let new_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO owned_rolling_stocks (id, collection_item_id) VALUES (?, ?)",
                )
                .bind(&new_id)
                .bind(&card.collection_item_id)
                .execute(&mut **tx)
                .await
                .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;
                Ok(new_id)
            }
        }
    }

    async fn ensure_default_collection(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DataManagementError> {
        sqlx::query("INSERT OR IGNORE INTO collections (id, name) VALUES (?, ?)")
            .bind(DEFAULT_COLLECTION_ID)
            .bind("My Collection")
            .execute(&mut **tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn persist_collection_items(
        tx: &mut Transaction<'_, Sqlite>,
        data: &DataContainerDto,
        new_ids: &NewIdSets,
        pending_images: &mut Vec<String>,
    ) -> Result<(), DataManagementError> {
        for item in data
            .collection_items
            .iter()
            .filter(|i| new_ids.item_ids.contains(&i.id))
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
            .execute(&mut **tx)
            .await
            .map_err(|e| map_db_error(e, "collection item"))?;

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
                .execute(&mut **tx)
                .await
                .map_err(|e| map_db_error(e, "purchase record"))?;
            }
        }

        Ok(())
    }

    async fn persist_owned_rolling_stocks(
        tx: &mut Transaction<'_, Sqlite>,
        data: &DataContainerDto,
    ) -> Result<u32, DataManagementError> {
        let mut added_ors = 0u32;

        for ors in &data.owned_rolling_stocks {
            let item_exists = Self::exists_by_id(
                tx,
                "SELECT COUNT(1) FROM collection_items WHERE id = ?",
                &ors.collection_item_id,
            )
            .await?;

            if !item_exists {
                warn!(
                    "Skipping owned_rolling_stock '{}': collection item '{}' not found",
                    ors.id, ors.collection_item_id
                );
                continue;
            }

            if let Some(ref rs_id) = ors.rolling_stock_id {
                let rs_exists = Self::exists_by_id(
                    tx,
                    "SELECT COUNT(1) FROM rolling_stocks WHERE id = ?",
                    rs_id,
                )
                .await?;

                if !rs_exists {
                    warn!(
                        "Skipping owned_rolling_stock '{}': rolling stock '{}' not found",
                        ors.id, rs_id
                    );
                    continue;
                }
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
            .execute(&mut **tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            added_ors += 1;
        }

        Ok(added_ors)
    }

    async fn persist_digital_roster(
        tx: &mut Transaction<'_, Sqlite>,
        data: &DataContainerDto,
        new_ids: &NewIdSets,
    ) -> Result<(u32, u32), DataManagementError> {
        let mut added = 0u32;
        let mut skipped_missing_owned_stock = 0u32;

        for item in data
            .digital_rolling_stocks
            .iter()
            .filter(|i| new_ids.digital_roster_ids.contains(&i.id))
        {
            let ors_exists = Self::exists_by_id(
                tx,
                "SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?",
                &item.owned_rolling_stock_id,
            )
            .await?;

            if !ors_exists {
                warn!(
                    "Skipping digital roster entry '{}': owned rolling stock '{}' not found",
                    item.id, item.owned_rolling_stock_id
                );
                skipped_missing_owned_stock += 1;
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
            .execute(&mut **tx)
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

            added += 1;
        }

        Ok((added, skipped_missing_owned_stock))
    }

    async fn refresh_collection_summary(
        tx: &mut Transaction<'_, Sqlite>,
    ) -> Result<(), DataManagementError> {
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
        .execute(&mut **tx)
        .await
        .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn import_pending_images(
        archive_path: &Path,
        media_dir: &Path,
        pending_images: &[String],
    ) -> Result<(u32, Vec<ImageFailure>), DataManagementError> {
        let archive_path_owned = archive_path.to_path_buf();
        let entry_paths: Vec<String> = pending_images
            .iter()
            .map(|f| format!("images/{f}"))
            .collect();
        let extracted =
            ArchiveExtractor::extract_files_batch_async(archive_path_owned, entry_paths)
                .await
                .map_err(|e| DataManagementError::IoError(e.to_string()))?;

        let mut images_imported: u32 = 0;
        let mut images_failed: Vec<ImageFailure> = vec![];

        for (image_filename, extract_result) in pending_images.iter().zip(extracted) {
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

        Ok((images_imported, images_failed))
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
        // Build HashSets for fast duplicate filtering.
        let new_ids = build_new_id_sets(duplicates);

        let mut added = RecordCounts::default();
        let mut skipped = RecordCounts::default();
        let mut pending_images: Vec<String> = vec![];

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        Self::ensure_default_collection(&mut tx).await?;

        // 1. Insert new manufacturers
        for m in data
            .manufacturers
            .iter()
            .filter(|m| new_ids.manufacturer_ids.contains(&m.id))
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
            .filter(|m| new_ids.model_ids.contains(&m.id))
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
            Self::insert_rolling_stocks_for_model(&mut tx, model, rolling_stock_category).await?;
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

        // 3b. Insert rolling stocks for duplicate railway models.
        //
        // check_railway_models marks a model as duplicate by (manufacturer_id, product_code),
        // so a model UUID that already exists in the DB never enters new_model_ids and its
        // rolling stocks are entirely skipped by step 3.  If the manifest contains new
        // owned_rolling_stocks that reference rolling stock IDs belonging to one of those
        // "duplicate" models, the INSERT at step 5.5 would fail with FK error 787.
        //
        // To fix this we collect the manifest-declared "duplicate" model IDs, verify which of
        // them actually exist in railway_models (handles the edge case where the manifest has
        // the same product_code under a different UUID), and then INSERT OR IGNORE their rolling
        // stocks — merging any new rows without disturbing existing ones.
        {
            let dup_model_ids: Vec<&str> = data
                .railway_models
                .iter()
                .filter(|m| !new_ids.model_ids.contains(&m.id))
                .map(|m| m.id.as_str())
                .collect();

            if !dup_model_ids.is_empty() {
                let placeholders = dup_model_ids
                    .iter()
                    .map(|_| "?")
                    .collect::<Vec<_>>()
                    .join(", ");
                let q = format!("SELECT id FROM railway_models WHERE id IN ({placeholders})");
                let mut qb = sqlx::query_scalar::<_, String>(&q);
                for id in &dup_model_ids {
                    qb = qb.bind(*id);
                }
                let confirmed_dup_ids: HashSet<String> = qb
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?
                    .into_iter()
                    .collect();

                for model in data
                    .railway_models
                    .iter()
                    .filter(|m| confirmed_dup_ids.contains(&m.id))
                {
                    let category = schema_category_to_db(&model.category)?;
                    let rolling_stock_category = model_category_to_rolling_stock_category(category);
                    Self::insert_rolling_stocks_for_model(&mut tx, model, rolling_stock_category)
                        .await?;
                }
            }
        }

        // 4. Insert new sellers
        for seller in data
            .sellers
            .iter()
            .filter(|s| new_ids.seller_ids.contains(&s.id))
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

        Self::persist_collection_items(&mut tx, data, &new_ids, &mut pending_images).await?;

        added.collection_items = duplicates.collection_item_dupes.new_count() as u32;
        skipped.collection_items = duplicates.collection_item_dupes.duplicate_count() as u32;

        added.owned_rolling_stocks = Self::persist_owned_rolling_stocks(&mut tx, data).await?;

        // 6. Insert new track products
        for product in data
            .track_products
            .iter()
            .filter(|p| new_ids.track_product_ids.contains(&p.track_id))
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
            .filter(|inv| new_ids.track_inventory_ids.contains(&inv.id))
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
            let card_exists = Self::exists_by_id(
                &mut tx,
                "SELECT COUNT(1) FROM maintenance_cards WHERE id = ?",
                &card.id,
            )
            .await?;

            if card_exists {
                skipped.maintenance_cards += 1;
                continue;
            }

            let item_exists = Self::exists_by_id(
                &mut tx,
                "SELECT COUNT(1) FROM collection_items WHERE id = ?",
                &card.collection_item_id,
            )
            .await?;

            if !item_exists {
                warn!(
                    "Skipping maintenance card '{}': collection item '{}' not found",
                    card.id, card.collection_item_id
                );
                skipped.maintenance_cards += 1;
                continue;
            }

            let ors_id = Self::resolve_maintenance_ors_id(&mut tx, card).await?;

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
            .filter(|c| new_ids.formation_category_ids.contains(&c.id))
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
            .filter(|p| new_ids.prototype_ids.contains(&p.id))
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
            .filter(|f| new_ids.train_formation_ids.contains(&f.id))
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
        for wishlist in data
            .wishlists
            .iter()
            .filter(|w| new_ids.wishlist_ids.contains(&w.id))
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
        for decoder in data
            .decoders
            .iter()
            .filter(|d| new_ids.decoder_ids.contains(&d.id))
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

        let (added_digital_roster, skipped_missing_owned_stock) =
            Self::persist_digital_roster(&mut tx, data, &new_ids).await?;
        added.digital_rolling_stocks = added_digital_roster;
        skipped.digital_rolling_stocks =
            skipped_missing_owned_stock + duplicates.digital_roster_dupes.duplicate_count() as u32;

        Self::refresh_collection_summary(&mut tx).await?;

        // Commit the transaction — all DB work done before any file I/O
        tx.commit()
            .await
            .map_err(|e| DataManagementError::DatabaseError(e.to_string()))?;

        let (images_imported, images_failed) =
            Self::import_pending_images(archive_path, media_dir, &pending_images).await?;

        Ok(PersistResult {
            added,
            skipped,
            images_imported,
            images_failed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::{
        CollectionItemRecord, LocalizedTextRecord, MaintenanceCardRecord, MoneyRecord,
        PurchaseRecord, RailwayCompanyRecord, RailwayModelRecord,
    };
    use crate::data_management::infrastructure::DuplicateCheckResult;
    use sqlx::SqlitePool;

    fn empty_duplicates() -> AllDuplicates {
        AllDuplicates {
            manufacturer_dupes: DuplicateCheckResult::default(),
            railway_model_dupes: DuplicateCheckResult::default(),
            collection_item_dupes: DuplicateCheckResult::default(),
            seller_dupes: DuplicateCheckResult::default(),
            track_product_dupes: DuplicateCheckResult::default(),
            track_inventory_dupes: DuplicateCheckResult::default(),
            formation_category_dupes: DuplicateCheckResult::default(),
            train_formation_dupes: DuplicateCheckResult::default(),
            prototype_dupes: DuplicateCheckResult::default(),
            wishlist_dupes: DuplicateCheckResult::default(),
            decoder_dupes: DuplicateCheckResult::default(),
            digital_roster_dupes: DuplicateCheckResult::default(),
        }
    }

    fn app_repo(pool: SqlitePool) -> SqliteImportRepository {
        SqliteImportRepository::new(pool)
    }

    async fn seed_minimal_catalog(pool: &SqlitePool) {
        sqlx::query("INSERT INTO manufacturers (id, name, status) VALUES (?, ?, ?)")
            .bind("trn:manufacturer:test")
            .bind("Test Manufacturer")
            .bind("ACTIVE")
            .execute(pool)
            .await
            .expect("manufacturer seed should succeed");

        sqlx::query("INSERT INTO railway_companies (id, name, status) VALUES (?, ?, ?)")
            .bind("trn:railway-company:test")
            .bind("Test Railway")
            .bind("ACTIVE")
            .execute(pool)
            .await
            .expect("railway company seed should succeed");

        sqlx::query(
            "INSERT INTO railway_models \
             (id, manufacturer_id, product_code, power_method, scale, epoch, category) \
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind("trn:railway-model:test:001")
        .bind("trn:manufacturer:test")
        .bind("T-001")
        .bind("DC")
        .bind("H0")
        .bind("VI")
        .bind("LOCOMOTIVES")
        .execute(pool)
        .await
        .expect("railway model seed should succeed");
    }

    #[test]
    fn format_decimal_for_text_strips_trailing_zeroes() {
        assert_eq!(format_decimal_for_text(12.0), "12");
        assert_eq!(format_decimal_for_text(12.50), "12.5");
        assert_eq!(format_decimal_for_text(12.125), "12.125");
    }

    #[test]
    fn synthesize_rolling_stock_id_includes_optional_road_number() {
        let with_road = synthesize_rolling_stock_id("m1", "rc1", "s1", Some("r1"), 3);
        let without_road = synthesize_rolling_stock_id("m1", "rc1", "s1", None, 3);

        assert_eq!(with_road, "rs::m1::rc1::s1::r1::3");
        assert_eq!(without_road, "rs::m1::rc1::s1::::3");
    }

    #[test]
    fn build_new_id_sets_maps_every_duplicate_bucket() {
        let mut duplicates = empty_duplicates();
        duplicates.manufacturer_dupes.new_ids = vec!["m1".to_string()];
        duplicates.railway_model_dupes.new_ids = vec!["rm1".to_string()];
        duplicates.collection_item_dupes.new_ids = vec!["ci1".to_string()];
        duplicates.seller_dupes.new_ids = vec!["s1".to_string()];
        duplicates.track_product_dupes.new_ids = vec!["tp1".to_string()];
        duplicates.track_inventory_dupes.new_ids = vec!["ti1".to_string()];
        duplicates.formation_category_dupes.new_ids = vec!["fc1".to_string()];
        duplicates.train_formation_dupes.new_ids = vec!["tf1".to_string()];
        duplicates.prototype_dupes.new_ids = vec!["p1".to_string()];
        duplicates.wishlist_dupes.new_ids = vec!["w1".to_string()];
        duplicates.decoder_dupes.new_ids = vec!["d1".to_string()];
        duplicates.digital_roster_dupes.new_ids = vec!["dr1".to_string()];

        let sets = build_new_id_sets(&duplicates);

        assert!(sets.manufacturer_ids.contains("m1"));
        assert!(sets.model_ids.contains("rm1"));
        assert!(sets.item_ids.contains("ci1"));
        assert!(sets.seller_ids.contains("s1"));
        assert!(sets.track_product_ids.contains("tp1"));
        assert!(sets.track_inventory_ids.contains("ti1"));
        assert!(sets.formation_category_ids.contains("fc1"));
        assert!(sets.train_formation_ids.contains("tf1"));
        assert!(sets.prototype_ids.contains("p1"));
        assert!(sets.wishlist_ids.contains("w1"));
        assert!(sets.decoder_ids.contains("d1"));
        assert!(sets.digital_roster_ids.contains("dr1"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_empty_payload_creates_default_collection(pool: SqlitePool) {
        let repo = app_repo(pool.clone());
        let data = DataContainerDto::default();
        let duplicates = empty_duplicates();

        let media_dir = tempfile::tempdir().expect("temp dir should be created");

        let result = repo
            .persist(
                &data,
                &duplicates,
                Path::new("unused.zip"),
                media_dir.path(),
            )
            .await
            .expect("persist should succeed for an empty payload");

        assert_eq!(result.images_imported, 0);
        assert!(result.images_failed.is_empty());
        assert_eq!(result.added.railway_models, 0);
        assert_eq!(result.added.collection_items, 0);
        assert_eq!(result.skipped.railway_models, 0);

        let count: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM collections WHERE id = ?")
            .bind(DEFAULT_COLLECTION_ID)
            .fetch_one(&pool)
            .await
            .expect("collections should be queryable");

        assert_eq!(count, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_skips_digital_roster_without_owned_stock(pool: SqlitePool) {
        let repo = app_repo(pool);
        let mut data = DataContainerDto::default();
        data.digital_rolling_stocks.push(
            crate::data_management::domain::DigitalRollingStockRecord {
                id: "trn:digital-rolling-stock:1".to_string(),
                owned_rolling_stock_id: "trn:owned-rolling-stock:missing".to_string(),
                dcc_address: 3,
                decoder_id: None,
            },
        );

        let mut duplicates = empty_duplicates();
        duplicates.digital_roster_dupes.new_ids = vec!["trn:digital-rolling-stock:1".to_string()];

        let media_dir = tempfile::tempdir().expect("temp dir should be created");

        let result = repo
            .persist(
                &data,
                &duplicates,
                Path::new("unused.zip"),
                media_dir.path(),
            )
            .await
            .expect("persist should succeed and skip dangling digital roster rows");

        assert_eq!(result.added.digital_rolling_stocks, 0);
        assert_eq!(result.skipped.digital_rolling_stocks, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_skips_owned_rolling_stock_when_collection_item_missing(pool: SqlitePool) {
        let repo = app_repo(pool.clone());
        let mut data = DataContainerDto::default();
        data.owned_rolling_stocks
            .push(crate::data_management::domain::OwnedRollingStockRecord {
                id: "ors-missing-item".to_string(),
                collection_item_id: "trn:collection-item:missing".to_string(),
                rolling_stock_id: None,
                notes: None,
                dcc_address: None,
                installed_decoder_id: None,
                current_coupler_id: None,
            });

        let media_dir = tempfile::tempdir().expect("temp dir should be created");
        let result = repo
            .persist(
                &data,
                &empty_duplicates(),
                Path::new("unused.zip"),
                media_dir.path(),
            )
            .await
            .expect("persist should succeed and skip dangling owned rolling stock");

        assert_eq!(result.added.owned_rolling_stocks, 0);

        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?")
                .bind("ors-missing-item")
                .fetch_one(&pool)
                .await
                .expect("owned_rolling_stocks should be queryable");

        assert_eq!(count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_creates_minimal_ors_for_maintenance_card_when_missing(pool: SqlitePool) {
        seed_minimal_catalog(&pool).await;
        sqlx::query("INSERT INTO collections (id, name) VALUES (?, ?)")
            .bind(DEFAULT_COLLECTION_ID)
            .bind("My Collection")
            .execute(&pool)
            .await
            .expect("collection seed should succeed");

        sqlx::query(
            "INSERT INTO collection_items \
             (id, collection_id, railway_model_id, added_date) VALUES (?, ?, ?, ?)",
        )
        .bind("trn:collection-item:test:001")
        .bind(DEFAULT_COLLECTION_ID)
        .bind("trn:railway-model:test:001")
        .bind("2025-01-10")
        .execute(&pool)
        .await
        .expect("collection item seed should succeed");

        let repo = app_repo(pool.clone());
        let mut data = DataContainerDto::default();
        data.maintenance_cards.push(MaintenanceCardRecord {
            id: "trn:maintenance-card:test:001".to_string(),
            collection_item_id: "trn:collection-item:test:001".to_string(),
            owned_rolling_stock_id: Some("ors-generated-for-card".to_string()),
            last_maintenance_date: Some("2025-01-11".to_string()),
            next_maintenance_date: None,
            events: vec![],
        });

        let media_dir = tempfile::tempdir().expect("temp dir should be created");
        let result = repo
            .persist(
                &data,
                &empty_duplicates(),
                Path::new("unused.zip"),
                media_dir.path(),
            )
            .await
            .expect("persist should create minimal ORS and maintenance card");

        assert_eq!(result.added.maintenance_cards, 1);

        let ors_count: i64 =
            sqlx::query_scalar("SELECT COUNT(1) FROM owned_rolling_stocks WHERE id = ?")
                .bind("ors-generated-for-card")
                .fetch_one(&pool)
                .await
                .expect("owned_rolling_stocks should be queryable");
        assert_eq!(ors_count, 1);

        let card_count: i64 =
            sqlx::query_scalar("SELECT COUNT(1) FROM maintenance_cards WHERE id = ?")
                .bind("trn:maintenance-card:test:001")
                .fetch_one(&pool)
                .await
                .expect("maintenance_cards should be queryable");
        assert_eq!(card_count, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn persist_recomputes_collection_summary_from_imported_rows(pool: SqlitePool) {
        let repo = app_repo(pool.clone());
        let mut data = DataContainerDto::default();

        data.manufacturers
            .push(crate::data_management::domain::ManufacturerRecord {
                id: "trn:manufacturer:sum".to_string(),
                name: "Summary Manufacturer".to_string(),
                registered_company_name: None,
                country_code: None,
                status: Some("ACTIVE".to_string()),
                website_url: None,
                street_address: None,
                extended_address: None,
                city: None,
                state_region: None,
                postal_code: None,
            });

        data.railway_companies.push(RailwayCompanyRecord {
            id: "trn:railway-company:sum".to_string(),
            name: "Summary Railway".to_string(),
            country_code: None,
            status: Some("ACTIVE".to_string()),
            operating_since: None,
            operating_until: None,
        });

        data.railway_models.push(RailwayModelRecord {
            id: "trn:railway-model:sum:001".to_string(),
            manufacturer_id: "trn:manufacturer:sum".to_string(),
            product_code: "SUM-001".to_string(),
            description: LocalizedTextRecord {
                en: Some("Summary model".to_string()),
                it: None,
            },
            scale: "H0".to_string(),
            epoch: "VI".to_string(),
            category: "LOCOMOTIVES".to_string(),
            power_method: "DC".to_string(),
            details: None,
            delivery_date: None,
            availability_status: None,
            image: None,
            rolling_stocks: vec![],
        });

        data.collection_items.push(CollectionItemRecord {
            id: "trn:collection-item:sum:001".to_string(),
            railway_model_id: "trn:railway-model:sum:001".to_string(),
            added_date: "2025-02-10".to_string(),
            removed_date: None,
            purchase_condition: Some("NEW".to_string()),
            model_condition: Some("MINT".to_string()),
            box_condition: Some("ORIGINAL_MINT".to_string()),
            notes: None,
            image: None,
            purchase: Some(PurchaseRecord {
                r#type: "PURCHASED".to_string(),
                purchase_date: Some("2025-02-10".to_string()),
                price: Some(MoneyRecord {
                    amount: 1234,
                    currency: "EUR".to_string(),
                }),
                seller_id: None,
                sale_date: None,
                sale_price: None,
                deposit_amount: None,
                expected_delivery: None,
            }),
        });

        let mut duplicates = empty_duplicates();
        duplicates.manufacturer_dupes.new_ids = vec!["trn:manufacturer:sum".to_string()];
        duplicates.railway_model_dupes.new_ids = vec!["trn:railway-model:sum:001".to_string()];
        duplicates.collection_item_dupes.new_ids = vec!["trn:collection-item:sum:001".to_string()];

        let media_dir = tempfile::tempdir().expect("temp dir should be created");
        repo.persist(
            &data,
            &duplicates,
            Path::new("unused.zip"),
            media_dir.path(),
        )
        .await
        .expect("persist should succeed for summary recompute scenario");

        let locomotives_count: i64 =
            sqlx::query_scalar("SELECT locomotives_count FROM collections WHERE id = ?")
                .bind(DEFAULT_COLLECTION_ID)
                .fetch_one(&pool)
                .await
                .expect("collections summary should be queryable");
        assert_eq!(locomotives_count, 1);

        let total_value_amount: i64 =
            sqlx::query_scalar("SELECT total_value_amount FROM collections WHERE id = ?")
                .bind(DEFAULT_COLLECTION_ID)
                .fetch_one(&pool)
                .await
                .expect("collections total value should be queryable");
        assert_eq!(total_value_amount, 1234);
    }
}
