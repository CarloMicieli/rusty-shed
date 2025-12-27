//! Helpers to create collecting-related test data in the application's SQLite database.
//!
//! These utilities provide small async helpers that insert minimal valid rows
//! into the collecting schema (collections, collection_items, owned_rolling_stocks,
//! purchase_infos). They are intended for use in tests where quickly setting up
//! a collection and its items is useful.

use crate::collecting::domain::collection::DEFAULT_COLLECTION_ID;
use anyhow::{Context, Result};
use chrono::Local;
use sqlx::SqlitePool;
use uuid::Uuid;

/// Test helper for inserting collecting-related rows.
///
/// Construct with an existing `SqlitePool` (for example an in-memory database
/// used by tests). The methods on this type generate TEXT primary keys (UUIDs)
/// and insert the minimal NOT NULL columns required by the migrations. All
/// methods return the inserted id as `Ok(String)` on success, or an
/// `anyhow::Error` with context on failure.
pub struct CollectingTestDb {
    db_pool: SqlitePool,
}

impl CollectingTestDb {
    /// Create a new test db helper from an existing connection pool.
    pub fn new(db_pool: SqlitePool) -> Self {
        Self { db_pool }
    }

    /// Insert a collection and return the generated id.
    ///
    /// Creates a row in `collections` with a generated TEXT id and the provided name.
    pub async fn insert_collection(&self, name: &str) -> Result<String> {
        let id = Uuid::parse_str(DEFAULT_COLLECTION_ID)
            .unwrap_or_default()
            .to_string();
        let sql = "INSERT INTO collections (id, name, total_value_amount, total_value_currency) VALUES (?1, ?2, 0, 'EUR')";
        sqlx::query(sql)
            .bind(&id)
            .bind(name)
            .execute(&self.db_pool)
            .await
            .with_context(|| format!("inserting collection id={} name={}", id, name))?;
        Ok(id)
    }

    /// Insert a collection item for `collection_id` referencing `railway_model_id`.
    ///
    /// Returns the generated collection_item id.
    pub async fn insert_collection_item(
        &self,
        collection_id: &str,
        railway_model_id: &str,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let sql = "INSERT INTO collection_items (id, collection_id, railway_model_id) VALUES (?1, ?2, ?3)";
        sqlx::query(sql)
            .bind(&id)
            .bind(collection_id)
            .bind(railway_model_id)
            .execute(&self.db_pool)
            .await
            .with_context(|| {
                format!(
                    "inserting collection_item id={} collection_id={}",
                    id, collection_id
                )
            })?;
        Ok(id)
    }

    /// Insert an owned rolling stock row referencing a collection item and rolling stock.
    ///
    /// Returns the generated owned_rolling_stocks id.
    pub async fn insert_owned_rolling_stock(
        &self,
        collection_item_id: &str,
        rolling_stock_id: &str,
    ) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        let sql = "INSERT INTO owned_rolling_stocks (id, collection_item_id, rolling_stock_id) VALUES (?1, ?2, ?3)";
        sqlx::query(sql)
            .bind(&id)
            .bind(collection_item_id)
            .bind(rolling_stock_id)
            .execute(&self.db_pool)
            .await
            .with_context(|| {
                format!(
                    "inserting owned_rolling_stock id={} collection_item_id={}",
                    id, collection_item_id
                )
            })?;
        Ok(id)
    }

    /// Insert a purchase_info row for a collection item with sensible defaults.
    ///
    /// The purchase_id is generated. The helper sets `purchase_type` to "purchased",
    /// `purchase_date` to today (YYYY-MM-DD), and a default purchased_price_amount of 0
    /// with currency "EUR". Adjust as needed in tests.
    pub async fn insert_purchase_info(&self, collection_item_id: &str) -> Result<String> {
        let purchase_id = Uuid::new_v4().to_string();
        let purchase_type = "purchased";
        let purchase_date = Local::now().format("%Y-%m-%d").to_string();
        let purchased_price_amount: i64 = 0;
        let purchased_price_currency: &str = "EUR";

        let sql = "INSERT INTO purchase_infos (purchase_id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        sqlx::query(sql)
            .bind(&purchase_id)
            .bind(collection_item_id)
            .bind(purchase_type)
            .bind(&purchase_date)
            .bind(purchased_price_amount)
            .bind(purchased_price_currency)
            .execute(&self.db_pool)
            .await
            .with_context(|| {
                format!(
                    "inserting purchase_info purchase_id={} collection_item_id={}",
                    purchase_id, collection_item_id
                )
            })?;

        Ok(purchase_id)
    }

    /// Create a minimal collection containing one railway model and optional rolling stocks.
    ///
    /// - `railway_model_id`: id of the railway model to add to the collection
    /// - `rolling_stock_ids`: list of rolling stock ids to attach to the created collection item
    ///
    /// Returns the generated ids collected in `CollectingTestData`.
    pub async fn setup_minimal_collection(
        &self,
        railway_model_id: &str,
        rolling_stock_ids: Vec<&str>,
    ) -> Result<CollectingTestData> {
        // Create collection
        let collection_name = "Test Collection";
        let collection_id = self.insert_collection(collection_name).await?;

        // Create collection item referencing the provided railway model
        let collection_item_id = self
            .insert_collection_item(&collection_id, railway_model_id)
            .await?;

        // Attach provided rolling stocks (if any)
        let mut owned_rolling_stock_ids = Vec::new();
        for rs_id in rolling_stock_ids {
            // Note: ignore individual insert errors propagate up to caller
            let owned_rolling_stock_id = self
                .insert_owned_rolling_stock(&collection_item_id, rs_id)
                .await?;
            owned_rolling_stock_ids.push(owned_rolling_stock_id);
        }

        let purchase_info_id = self.insert_purchase_info(&collection_item_id).await?;

        Ok(CollectingTestData {
            collection_id,
            collection_item_id,
            owned_rolling_stock_ids,
            purchase_info_id,
        })
    }
}

/// Collected ids produced by `CollectingTestDb::setup_minimal_collection`.
///
/// - `collection_id`: id of the created collection
/// - `collection_item_id`: id of the single collection item linking the collection to a railway model
/// - `owned_rolling_stock_ids`: ids of any owned rolling stocks attached to the collection item
/// - `purchase_info_id`: id of the created purchase_info row associated with the collection item
#[derive(Debug)]
pub struct CollectingTestData {
    /// Inserted collection id
    pub collection_id: String,
    /// Inserted collection_item id
    pub collection_item_id: String,
    /// Inserted owned_rolling_stocks ids
    pub owned_rolling_stock_ids: Vec<String>,
    /// Inserted purchase_infos id
    pub purchase_info_id: String,
}
