use crate::import::domain::{ImportResult, ImportSession, ManifestDto, RecordCounts};
use crate::import::infrastructure::DuplicateChecker;
use sqlx::SqlitePool;
use std::collections::HashSet;
use uuid::Uuid;

/// Executes the import of validated package data into the database.
///
/// The use case:
/// 1. Checks for duplicates using DuplicateChecker
/// 2. Filters out duplicate records
/// 3. Inserts only new records into the database
/// 4. Tracks added and skipped counts for reporting
pub struct ExecuteImportUseCase {
    pool: SqlitePool,
}

impl ExecuteImportUseCase {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Execute the import use case.
    ///
    /// # Arguments
    /// * `session` - The import session with validated data
    /// * `manifest` - The validated manifest data
    ///
    /// # Returns
    /// An ImportResult with counts of added records and skipped duplicates
    ///
    /// # Transaction Safety
    /// All database writes are performed in a single transaction. If any write fails,
    /// the entire transaction is rolled back, ensuring atomicity.
    pub async fn execute(
        &self,
        _session: &ImportSession,
        manifest: &ManifestDto,
    ) -> Result<ImportResult, String> {
        let start = std::time::Instant::now();

        // Start a database transaction for atomic writes
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| format!("Failed to begin transaction: {}", e))?;

        // Check for duplicates
        let duplicate_checker = DuplicateChecker::new(self.pool.clone());

        let manufacturer_dupes = duplicate_checker
            .check_manufacturers(&manifest.data.manufacturers)
            .await
            .map_err(|e| format!("Failed to check manufacturer duplicates: {}", e))?;

        let railway_model_dupes = duplicate_checker
            .check_railway_models(&manifest.data.railway_models)
            .await
            .map_err(|e| format!("Failed to check railway model duplicates: {}", e))?;

        let collection_item_dupes = duplicate_checker
            .check_collection_items(&manifest.data.collection_items)
            .await
            .map_err(|e| format!("Failed to check collection item duplicates: {}", e))?;

        let seller_dupes = duplicate_checker
            .check_sellers(&manifest.data.sellers)
            .await
            .map_err(|e| format!("Failed to check seller duplicates: {}", e))?;

        let track_product_dupes = duplicate_checker
            .check_track_products(&manifest.data.track_products)
            .await
            .map_err(|e| format!("Failed to check track product duplicates: {}", e))?;

        let track_inventory_dupes = duplicate_checker
            .check_track_inventories(&manifest.data.track_inventories)
            .await
            .map_err(|e| format!("Failed to check track inventory duplicates: {}", e))?;

        // Build HashSets for fast lookups
        let new_manufacturer_ids: HashSet<&str> = manufacturer_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_model_ids: HashSet<&str> = railway_model_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_item_ids: HashSet<&str> = collection_item_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_seller_ids: HashSet<&str> =
            seller_dupes.new_ids.iter().map(|s| s.as_str()).collect();
        let new_track_product_ids: HashSet<&str> = track_product_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();
        let new_track_inventory_ids: HashSet<&str> = track_inventory_dupes
            .new_ids
            .iter()
            .map(|s| s.as_str())
            .collect();

        let mut added = RecordCounts::default();
        let mut skipped = RecordCounts::default();

        // Ensure the default collection exists
        sqlx::query("INSERT OR IGNORE INTO collections (id, name) VALUES (?, ?)")
            .bind("trn:collection:1")
            .bind("My Collection")
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to ensure default collection: {}", e))?;

        // 1. Insert new manufacturers
        for m in manifest
            .data
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
            .map_err(|e| format!("Failed to insert manufacturer '{}': {}", m.name, e))?;
        }

        added.manufacturers = manufacturer_dupes.new_count() as u32;
        skipped.manufacturers = manufacturer_dupes.duplicate_count() as u32;

        // 2. Insert new railway companies (INSERT OR IGNORE — no duplicate check currently)
        for rc in &manifest.data.railway_companies {
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
            .map_err(|e| format!("Failed to insert railway company '{}': {}", rc.name, e))?;
        }

        added.railway_companies = manifest.data.railway_companies.len() as u32;

        // 3. Insert new railway models + translations + rolling stocks
        for model in manifest
            .data
            .railway_models
            .iter()
            .filter(|m| new_model_ids.contains(m.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO railway_models \
                 (id, manufacturer_id, product_code, power_method, scale, epoch, \
                  category, delivery_date, availability_status) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&model.id)
            .bind(&model.manufacturer_id)
            .bind(&model.product_code)
            .bind(schema_power_method_to_db(&model.power_method))
            .bind(&model.scale)
            .bind(&model.epoch)
            .bind(schema_category_to_db(&model.category.r#type))
            .bind(&model.delivery_date)
            .bind(&model.availability_status)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert railway model '{}': {}", model.id, e))?;

            // Insert translation for description/details
            if !model.description.is_empty() || model.details.is_some() {
                sqlx::query(
                    "INSERT OR IGNORE INTO railway_model_translations \
                     (railway_model_id, language_code, description, details) \
                     VALUES (?, 'en', ?, ?)",
                )
                .bind(&model.id)
                .bind(&model.description)
                .bind(&model.details)
                .execute(&mut *tx)
                .await
                .map_err(|e| format!("Failed to insert translation for '{}': {}", model.id, e))?;
            }

            // Insert rolling stocks (generate new UUIDs — no ID in manifest)
            let rolling_stock_category = model_category_to_rolling_stock_category(
                schema_category_to_db(&model.category.r#type),
            );
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
                .map_err(|e| format!("Failed to insert rolling stock for '{}': {}", model.id, e))?;
            }
        }

        added.railway_models = railway_model_dupes.new_count() as u32;
        skipped.railway_models = railway_model_dupes.duplicate_count() as u32;

        // 4. Insert new sellers
        for seller in manifest
            .data
            .sellers
            .iter()
            .filter(|s| new_seller_ids.contains(s.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO sellers \
                 (id, name, type, email, phone, website_url) \
                 VALUES (?, ?, ?, ?, ?, ?)",
            )
            .bind(&seller.id)
            .bind(&seller.name)
            .bind(&seller.seller_type)
            .bind(&seller.email)
            .bind(&seller.phone)
            .bind(&seller.website_url)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert seller '{}': {}", seller.name, e))?;
        }

        added.sellers = seller_dupes.new_count() as u32;
        skipped.sellers = seller_dupes.duplicate_count() as u32;

        // 5. Insert new collection items + purchase infos
        for item in manifest
            .data
            .collection_items
            .iter()
            .filter(|i| new_item_ids.contains(i.id.as_str()))
        {
            sqlx::query(
                "INSERT OR IGNORE INTO collection_items \
                 (id, collection_id, railway_model_id, added_date, removed_date, \
                  purchase_condition, model_condition, box_condition, notes) \
                 VALUES (?, 'trn:collection:1', ?, ?, ?, ?, ?, ?, ?)",
            )
            .bind(&item.id)
            .bind(&item.railway_model_id)
            .bind(&item.added_date)
            .bind(&item.removed_date)
            .bind(&item.purchase_condition)
            .bind(&item.model_condition)
            .bind(&item.box_condition)
            .bind(&item.notes)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to insert collection item '{}': {}", item.id, e))?;

            // Insert purchase info if present
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
                .map_err(|e| format!("Failed to insert purchase for '{}': {}", item.id, e))?;
            }
        }

        added.collection_items = collection_item_dupes.new_count() as u32;
        skipped.collection_items = collection_item_dupes.duplicate_count() as u32;

        // 6. Insert new track products
        for product in manifest
            .data
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
            .map_err(|e| {
                format!(
                    "Failed to insert track product '{}': {}",
                    product.track_id, e
                )
            })?;
        }

        added.track_products = track_product_dupes.new_count() as u32;
        skipped.track_products = track_product_dupes.duplicate_count() as u32;

        // 7. Insert new track inventories with items and purchases
        for inventory in manifest
            .data
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
            .map_err(|e| format!("Failed to insert track inventory '{}': {}", inventory.id, e))?;

            // Insert inventory items
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
                .map_err(|e| {
                    format!(
                        "Failed to insert track item '{}' in inventory '{}': {}",
                        item.track_id, inventory.id, e
                    )
                })?;
            }

            // Insert purchase history
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
                .map_err(|e| format!("Failed to insert track purchase '{}': {}", purchase.id, e))?;
            }
        }

        added.track_inventories = track_inventory_dupes.new_count() as u32;
        skipped.track_inventories = track_inventory_dupes.duplicate_count() as u32;

        // Maintenance cards require the owned_rolling_stocks FK chain — skipped in MVP
        added.maintenance_cards = 0;

        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| format!("Failed to commit transaction: {}", e))?;

        let result = ImportResult {
            session_id: _session.id.clone(),
            added,
            skipped,
            images_imported: 0,
            images_failed: vec![],
            duration_ms: start.elapsed().as_millis() as u64,
            warnings: vec![],
        };

        Ok(result)
    }
}

/// Convert schema category value (camelCase) to DB value (SCREAMING_SNAKE_CASE).
fn schema_category_to_db(schema_value: &str) -> &'static str {
    match schema_value {
        "locomotive" => "LOCOMOTIVES",
        "trainSet" => "TRAIN_SETS",
        "freightCar" => "FREIGHT_CARS",
        "passengerCar" => "PASSENGER_CARS",
        "electricMultipleUnit" => "ELECTRIC_MULTIPLE_UNITS",
        "railcar" => "RAILCARS",
        _ => "LOCOMOTIVES",
    }
}

/// Convert schema power method value (lowercase) to DB value (SCREAMING_SNAKE_CASE).
fn schema_power_method_to_db(schema_value: &str) -> &'static str {
    match schema_value {
        "ac" => "AC",
        "dc" => "DC",
        "dcc" => "DCC",
        "none" => "NONE",
        _ => "DC",
    }
}

/// Map railway model DB category (plural) to rolling stock DB category (singular).
fn model_category_to_rolling_stock_category(db_category: &str) -> &'static str {
    match db_category {
        "FREIGHT_CARS" => "FREIGHT_CAR",
        "PASSENGER_CARS" => "PASSENGER_CAR",
        "ELECTRIC_MULTIPLE_UNITS" => "ELECTRIC_MULTIPLE_UNIT",
        "RAILCARS" => "RAILCAR",
        _ => "LOCOMOTIVE", // LOCOMOTIVES, TRAIN_SETS, STARTER_SETS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_import_with_no_duplicates() {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create in-memory database");

        sqlx::query(
            "CREATE TABLE manufacturers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                registered_company_name TEXT,
                country_code TEXT,
                website_url TEXT,
                status TEXT NOT NULL DEFAULT 'ACTIVE',
                created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                version INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create manufacturers table");

        sqlx::query(
            "CREATE TABLE railway_companies (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                country_code TEXT,
                status TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create railway_companies table");

        sqlx::query(
            "CREATE TABLE railway_models (
                id TEXT PRIMARY KEY,
                manufacturer_id TEXT NOT NULL,
                product_code TEXT NOT NULL,
                power_method TEXT NOT NULL,
                scale TEXT NOT NULL,
                epoch TEXT NOT NULL,
                category TEXT NOT NULL,
                delivery_date TEXT,
                availability_status TEXT,
                UNIQUE(manufacturer_id, product_code)
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create railway_models table");

        sqlx::query(
            "CREATE TABLE railway_model_translations (
                railway_model_id TEXT NOT NULL,
                language_code TEXT NOT NULL,
                description TEXT,
                details TEXT,
                PRIMARY KEY (railway_model_id, language_code)
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create railway_model_translations table");

        sqlx::query(
            "CREATE TABLE rolling_stocks (
                id TEXT PRIMARY KEY,
                railway_model_id TEXT NOT NULL,
                category TEXT NOT NULL,
                railway_company_id TEXT NOT NULL,
                series_code TEXT NOT NULL,
                road_number TEXT,
                livery TEXT,
                friendly_name TEXT,
                is_dummy INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create rolling_stocks table");

        sqlx::query(
            "CREATE TABLE sellers (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                type TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                website_url TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create sellers table");

        sqlx::query("CREATE TABLE collections (id TEXT PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("Failed to create collections table");

        sqlx::query(
            "CREATE TABLE collection_items (
                id TEXT PRIMARY KEY,
                collection_id TEXT NOT NULL,
                railway_model_id TEXT NOT NULL,
                added_date TEXT NOT NULL,
                removed_date TEXT,
                purchase_condition TEXT,
                model_condition TEXT,
                box_condition TEXT,
                notes TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create collection_items table");

        sqlx::query(
            "CREATE TABLE purchase_infos (
                id TEXT PRIMARY KEY,
                collection_item_id TEXT NOT NULL,
                purchase_type TEXT,
                purchase_date TEXT NOT NULL,
                seller_id TEXT,
                purchased_price_amount INTEGER,
                purchased_price_currency TEXT,
                sale_date TEXT,
                sale_price_amount INTEGER,
                sale_price_currency TEXT,
                deposit_amount INTEGER,
                deposit_currency TEXT,
                expected_date TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create purchase_infos table");

        let session = ImportSession::new(
            std::path::PathBuf::from("/tmp/test.zip"),
            crate::import::domain::ArchiveFormat::Zip,
        );
        let manifest = ManifestDto {
            schema: None,
            version: "1.0".to_string(),
            exported_at: None,
            source: None,
            data: crate::import::domain::DataContainerDto::default(),
        };

        let use_case = ExecuteImportUseCase::new(pool);
        let result = use_case.execute(&session, &manifest).await;

        assert!(result.is_ok());
        let import_result = result.unwrap();
        assert_eq!(import_result.added.total(), 0);
        assert_eq!(import_result.skipped.total(), 0);
    }

    #[test]
    fn test_schema_category_to_db() {
        assert_eq!(schema_category_to_db("locomotive"), "LOCOMOTIVES");
        assert_eq!(schema_category_to_db("freightCar"), "FREIGHT_CARS");
        assert_eq!(schema_category_to_db("passengerCar"), "PASSENGER_CARS");
        assert_eq!(
            schema_category_to_db("electricMultipleUnit"),
            "ELECTRIC_MULTIPLE_UNITS"
        );
        assert_eq!(schema_category_to_db("railcar"), "RAILCARS");
        assert_eq!(schema_category_to_db("trainSet"), "TRAIN_SETS");
        assert_eq!(schema_category_to_db("unknown"), "LOCOMOTIVES");
    }

    #[test]
    fn test_schema_power_method_to_db() {
        assert_eq!(schema_power_method_to_db("ac"), "AC");
        assert_eq!(schema_power_method_to_db("dc"), "DC");
        assert_eq!(schema_power_method_to_db("dcc"), "DCC");
        assert_eq!(schema_power_method_to_db("none"), "NONE");
        assert_eq!(schema_power_method_to_db("unknown"), "DC");
    }

    #[test]
    fn test_model_category_to_rolling_stock_category() {
        assert_eq!(
            model_category_to_rolling_stock_category("LOCOMOTIVES"),
            "LOCOMOTIVE"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("FREIGHT_CARS"),
            "FREIGHT_CAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("PASSENGER_CARS"),
            "PASSENGER_CAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("ELECTRIC_MULTIPLE_UNITS"),
            "ELECTRIC_MULTIPLE_UNIT"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("RAILCARS"),
            "RAILCAR"
        );
        assert_eq!(
            model_category_to_rolling_stock_category("TRAIN_SETS"),
            "LOCOMOTIVE"
        );
    }
}
