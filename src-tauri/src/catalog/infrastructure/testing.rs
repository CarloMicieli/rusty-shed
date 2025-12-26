//! Utilities to help tests insert catalog data into the application's SQLite database.
//!
//! These helpers are small async convenience wrappers around `sqlx` that insert
//! minimal required columns for common catalog tables (manufacturers,
//! railway_companies, railway_models, rolling_stocks). They return the supplied
//! `id` on success (the migrations use `TEXT` primary keys), and wrap errors in
//! `anyhow::Error` with added context.

use anyhow::{Context, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

/// Collected ids for test data created by `CatalogTestDb::setup_railway_model`.
#[derive(Debug)]
pub struct CatalogTestData {
    /// Inserted manufacturer id
    pub manufacturer_id: String,
    /// Inserted railway company id
    pub railway_company_id: String,
    /// Inserted railway model id
    pub railway_model_id: String,
    /// Inserted rolling stock ids (may contain multiple ids)
    pub rolling_stock_ids: Vec<String>,
}

/// Helper for inserting test records into catalog tables.
///
/// Construct this with an existing `SqlitePool` (for example an in-memory
/// database used by tests). The methods on this type perform simple INSERTs of
/// the minimal NOT NULL columns for each table and return the provided `id` if
/// the insertion succeeds.
///
/// Errors are returned as `anyhow::Error` with contextual messages.
pub struct CatalogTestDb {
    db_pool: SqlitePool,
}

impl CatalogTestDb {
    /// Create a new test db helper from an existing connection pool.
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }

    /// Insert a manufacturer record.
    ///
    /// Parameters:
    /// - `id`: the TEXT primary key to insert for the manufacturer (migrations use TEXT ids)
    /// - `name`: the manufacturer name (NOT NULL)
    ///
    /// Returns: `Ok(id.to_string())` on success, or an `anyhow::Error` with
    /// context on failure.
    pub async fn insert_manufacturer(&self, id: &str, name: &str) -> Result<String> {
        let sql = format!("INSERT INTO {} (id, name) VALUES (?1, ?2)", "manufacturers");
        sqlx::query(&sql)
            .bind(id)
            .bind(name)
            .execute(&self.db_pool)
            .await
            .with_context(|| format!("inserting manufacturer id={} name={}", id, name))?;

        Ok(id.to_string())
    }

    /// Insert a railway company record.
    ///
    /// Parameters:
    /// - `id`: TEXT primary key for the railway company
    /// - `name`: company name (NOT NULL)
    ///
    /// Returns: `Ok(id)` on success.
    pub async fn insert_railway_company(&self, id: &str, name: &str) -> Result<String> {
        let sql = format!(
            "INSERT INTO {} (id, name) VALUES (?1, ?2)",
            "railway_companies"
        );
        sqlx::query(&sql)
            .bind(id)
            .bind(name)
            .execute(&self.db_pool)
            .await
            .with_context(|| format!("inserting railway_company id={} name={}", id, name))?;

        Ok(id.to_string())
    }

    /// Insert a railway model record.
    ///
    /// This inserts the minimal NOT NULL columns for `railway_models` as defined
    /// in the project's migrations.
    ///
    /// Parameters:
    /// - `id`: TEXT primary key for the model
    /// - `manufacturer_id`: TEXT foreign key referencing `manufacturers(id)` (NOT NULL)
    /// - `product_code`: product code (NOT NULL)
    /// - `description`: textual description (NOT NULL)
    /// - `power_method`: e.g. "electric", "diesel" (NOT NULL)
    /// - `scale`: model scale (NOT NULL)
    /// - `epoch`: era/epoch (NOT NULL)
    /// - `category`: model category (NOT NULL)
    ///
    /// Returns: `Ok(id)` on success.
    #[allow(clippy::too_many_arguments)]
    pub async fn insert_railway_model(
        &self,
        id: &str,
        manufacturer_id: &str,
        product_code: &str,
        description: &str,
        power_method: &str,
        scale: &str,
        epoch: &str,
        category: &str,
    ) -> Result<String> {
        let sql = format!(
            "INSERT INTO {} (id, manufacturer_id, product_code, description, power_method, scale, epoch, category) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            "railway_models"
        );
        sqlx::query(&sql)
            .bind(id)
            .bind(manufacturer_id)
            .bind(product_code)
            .bind(description)
            .bind(power_method)
            .bind(scale)
            .bind(epoch)
            .bind(category)
            .execute(&self.db_pool)
            .await
            .with_context(|| {
                format!(
                    "inserting railway_model id={} product_code={}",
                    id, product_code
                )
            })?;

        Ok(id.to_string())
    }

    /// Insert a rolling stock record.
    ///
    /// Inserts the NOT NULL columns required by the `rolling_stocks` table. Many
    /// rolling stock columns are nullable; this helper only requires the
    /// smallest set needed for creating a valid row.
    ///
    /// Parameters:
    /// - `id`: TEXT primary key for the rolling stock
    /// - `railway_model_id`: TEXT foreign key referencing `railway_models(id)` (NOT NULL)
    /// - `category`: rolling stock category (NOT NULL)
    /// - `railway_company_id`: TEXT foreign key referencing `railway_companies(id)` (NOT NULL)
    /// - `is_dummy`: INTEGER flag (0 or 1) — column is NOT NULL but has a default, the helper accepts an explicit value
    ///
    /// Returns: `Ok(id)` on success.
    pub async fn insert_rolling_stock(
        &self,
        id: &str,
        railway_model_id: &str,
        category: &str,
        railway_company_id: &str,
        is_dummy: i32,
    ) -> Result<String> {
        let sql = format!(
            "INSERT INTO {} (id, railway_model_id, category, railway_company_id, is_dummy) VALUES (?1, ?2, ?3, ?4, ?5)",
            "rolling_stocks"
        );
        sqlx::query(&sql)
            .bind(id)
            .bind(railway_model_id)
            .bind(category)
            .bind(railway_company_id)
            .bind(is_dummy)
            .execute(&self.db_pool)
            .await
            .with_context(|| {
                format!(
                    "inserting rolling_stock id={} railway_model_id={}",
                    id, railway_model_id
                )
            })?;

        Ok(id.to_string())
    }

    /// Create a manufacturer, railway company, railway model and one rolling
    /// stock using reasonable Italian electric locomotive test data.
    ///
    /// Returns the generated ids collected in `CatalogTestData`.
    pub async fn setup_railway_model(&self) -> Result<CatalogTestData> {
        // Generate ids
        let manufacturer_id = Uuid::new_v4().to_string();
        let railway_company_id = Uuid::new_v4().to_string();
        let railway_model_id = Uuid::new_v4().to_string();
        let rolling_stock_id = Uuid::new_v4().to_string();

        // Insert manufacturer and company
        self.insert_manufacturer(&manufacturer_id, "ACME").await?;
        self.insert_railway_company(&railway_company_id, "FS")
            .await?;

        // Insert a railway model describing an Italian electric locomotive
        let product_code = "E656";
        let description = "FS Class E656 electric locomotive";
        let power_method = "electric";
        let scale = "HO";
        let epoch = "VI";
        let category = "locomotive";

        self.insert_railway_model(
            &railway_model_id,
            &manufacturer_id,
            product_code,
            description,
            power_method,
            scale,
            epoch,
            category,
        )
        .await?;

        // Insert one rolling stock instance for the model
        self.insert_rolling_stock(
            &rolling_stock_id,
            &railway_model_id,
            category,
            &railway_company_id,
            0,
        )
        .await?;

        Ok(CatalogTestData {
            manufacturer_id,
            railway_company_id,
            railway_model_id,
            rolling_stock_ids: vec![rolling_stock_id],
        })
    }
}
