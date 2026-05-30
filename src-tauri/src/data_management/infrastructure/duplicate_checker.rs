use crate::data_management::domain::{
    CollectionItemRecord, DecoderRecord, DigitalRollingStockRecord, FormationCategoryRecord,
    ManufacturerRecord, PrototypeRecord, RailwayModelRecord, SellerRecord, TrackInventoryRecord,
    TrackProductRecord, TrainFormationRecord, WishlistRecord,
};
use sqlx::SqlitePool;
use std::collections::HashSet;

/// Service for detecting duplicate records in the database before import.
///
/// Uses batch key loading to efficiently check for existing records.
#[derive(Debug, Clone)]
pub struct DuplicateChecker {
    pool: SqlitePool,
}

/// Result of duplicate checking for a single entity type.
#[derive(Debug, Clone, Default)]
pub struct DuplicateCheckResult {
    /// IDs from the manifest that already exist in the database
    pub duplicate_ids: Vec<String>,
    /// IDs from the manifest that are new (not in database)
    pub new_ids: Vec<String>,
}

impl DuplicateCheckResult {
    /// Count of duplicates found
    pub fn duplicate_count(&self) -> usize {
        self.duplicate_ids.len()
    }

    /// Count of new records
    pub fn new_count(&self) -> usize {
        self.new_ids.len()
    }

    /// Total count (duplicates + new)
    pub fn total_count(&self) -> usize {
        self.duplicate_ids.len() + self.new_ids.len()
    }
}

impl DuplicateChecker {
    /// Create a new duplicate checker with a database connection pool.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Check for duplicate manufacturers by id.
    ///
    /// Returns which manufacturer IDs already exist in the database.
    pub async fn check_manufacturers(
        &self,
        manufacturers: &[ManufacturerRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if manufacturers.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Extract ids for lookup
        let ids: Vec<String> = manufacturers.iter().map(|m| m.id.clone()).collect();

        // Query existing manufacturers by id
        let query = format!(
            "SELECT id FROM manufacturers WHERE id IN ({})",
            ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for id in &ids {
            query_builder = query_builder.bind(id);
        }

        let existing_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        // Partition into duplicates and new records
        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for manufacturer in manufacturers {
            if existing_ids.contains(&manufacturer.id) {
                duplicate_ids.push(manufacturer.id.clone());
            } else {
                new_ids.push(manufacturer.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate railway models by manufacturer_id + product_code.
    ///
    /// Uses a single batch query with composite key concatenation to avoid
    /// the N+1 query problem.
    pub async fn check_railway_models(
        &self,
        models: &[RailwayModelRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if models.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Build composite keys: "manufacturer_id|product_code"
        // The separator '|' must not appear in valid manufacturer IDs or product codes.
        let composite_keys: Vec<String> = models
            .iter()
            .map(|m| format!("{}|{}", m.manufacturer_id, m.product_code))
            .collect();

        let query = format!(
            "SELECT (manufacturer_id || '|' || product_code) \
             FROM railway_models \
             WHERE (manufacturer_id || '|' || product_code) IN ({})",
            composite_keys
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for key in &composite_keys {
            query_builder = query_builder.bind(key);
        }

        let existing_keys: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for model in models {
            let key = format!("{}|{}", model.manufacturer_id, model.product_code);
            if existing_keys.contains(&key) {
                duplicate_ids.push(model.id.clone());
            } else {
                new_ids.push(model.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate collection items by railway_model_id + added_date.
    ///
    /// Uses a single batch query with composite key concatenation to avoid
    /// the N+1 query problem.
    pub async fn check_collection_items(
        &self,
        items: &[CollectionItemRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if items.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Build composite keys using added_date (matches the DB column used in the query).
        // Separator '|' must not appear in valid railway_model_id or date values.
        let composite_keys: Vec<String> = items
            .iter()
            .map(|item| format!("{}|{}", item.railway_model_id, item.added_date))
            .collect();

        let query = format!(
            "SELECT (railway_model_id || '|' || added_date) \
             FROM collection_items \
             WHERE (railway_model_id || '|' || added_date) IN ({})",
            composite_keys
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for key in &composite_keys {
            query_builder = query_builder.bind(key);
        }

        let existing_keys: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for item in items {
            let key = format!("{}|{}", item.railway_model_id, item.added_date);
            if existing_keys.contains(&key) {
                duplicate_ids.push(item.id.clone());
            } else {
                new_ids.push(item.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate sellers by id.
    ///
    /// Returns which seller IDs already exist in the database.
    pub async fn check_sellers(
        &self,
        sellers: &[SellerRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if sellers.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Extract ids for lookup
        let ids: Vec<String> = sellers.iter().map(|s| s.id.clone()).collect();

        // Query existing sellers by id
        let query = format!(
            "SELECT id FROM sellers WHERE id IN ({})",
            ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for id in &ids {
            query_builder = query_builder.bind(id);
        }

        let existing_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        // Partition into duplicates and new records
        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for seller in sellers {
            if existing_ids.contains(&seller.id) {
                duplicate_ids.push(seller.id.clone());
            } else {
                new_ids.push(seller.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate track products by track_id (canonical TRN identifier).
    pub async fn check_track_products(
        &self,
        products: &[TrackProductRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if products.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let track_ids: Vec<String> = products.iter().map(|p| p.track_id.clone()).collect();

        let query = format!(
            "SELECT track_id FROM track_products WHERE track_id IN ({})",
            track_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for id in &track_ids {
            query_builder = query_builder.bind(id);
        }

        let existing_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for product in products {
            if existing_ids.contains(&product.track_id) {
                duplicate_ids.push(product.track_id.clone());
            } else {
                new_ids.push(product.track_id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate track inventories by id.
    pub async fn check_track_inventories(
        &self,
        inventories: &[TrackInventoryRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if inventories.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let ids: Vec<String> = inventories.iter().map(|inv| inv.id.clone()).collect();

        let query = format!(
            "SELECT id FROM track_inventories WHERE id IN ({})",
            ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for id in &ids {
            query_builder = query_builder.bind(id);
        }

        let existing_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for inv in inventories {
            if existing_ids.contains(&inv.id) {
                duplicate_ids.push(inv.id.clone());
            } else {
                new_ids.push(inv.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate formation categories by name (case-sensitive).
    pub async fn check_formation_categories(
        &self,
        categories: &[FormationCategoryRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if categories.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let names: Vec<String> = categories.iter().map(|c| c.name.clone()).collect();

        let query = format!(
            "SELECT name FROM formation_categories WHERE name IN ({})",
            names.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for name in &names {
            query_builder = query_builder.bind(name);
        }

        let existing_names: std::collections::HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for cat in categories {
            if existing_names.contains(&cat.name) {
                duplicate_ids.push(cat.id.clone());
            } else {
                new_ids.push(cat.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate train formations by name.
    pub async fn check_train_formations(
        &self,
        formations: &[TrainFormationRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if formations.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let names: Vec<String> = formations.iter().map(|f| f.name.clone()).collect();

        let query = format!(
            "SELECT name FROM train_formations WHERE name IN ({})",
            names.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for name in &names {
            query_builder = query_builder.bind(name);
        }

        let existing_names: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for formation in formations {
            if existing_names.contains(&formation.name) {
                duplicate_ids.push(formation.id.clone());
            } else {
                new_ids.push(formation.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate wishlists by name (case-sensitive).
    pub async fn check_wishlists(
        &self,
        wishlists: &[WishlistRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if wishlists.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let names: Vec<String> = wishlists.iter().map(|w| w.name.clone()).collect();

        let query = format!(
            "SELECT name FROM wishlists WHERE name IN ({})",
            names.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for name in &names {
            query_builder = query_builder.bind(name);
        }

        let existing_names: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for wishlist in wishlists {
            if existing_names.contains(&wishlist.name) {
                duplicate_ids.push(wishlist.id.clone());
            } else {
                new_ids.push(wishlist.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate prototypes by `railway_company_id + series_code + specification_type`.
    pub async fn check_prototypes(
        &self,
        prototypes: &[PrototypeRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if prototypes.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let composite_keys: Vec<String> = prototypes
            .iter()
            .map(|p| {
                format!(
                    "{}|{}|{}",
                    p.railway_company_id, p.series_code, p.specification_type
                )
            })
            .collect();

        let query = format!(
            "SELECT (railway_company_id || '|' || series_code || '|' || specification_type) \
             FROM prototypes \
             WHERE (railway_company_id || '|' || series_code || '|' || specification_type) IN ({})",
            composite_keys
                .iter()
                .map(|_| "?")
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for key in &composite_keys {
            query_builder = query_builder.bind(key);
        }

        let existing_keys: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for prototype in prototypes {
            let key = format!(
                "{}|{}|{}",
                prototype.railway_company_id, prototype.series_code, prototype.specification_type
            );
            if existing_keys.contains(&key) {
                duplicate_ids.push(prototype.id.clone());
            } else {
                new_ids.push(prototype.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate decoders by primary key (URN id).
    pub async fn check_decoders(
        &self,
        decoders: &[DecoderRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if decoders.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let ids: Vec<String> = decoders.iter().map(|d| d.id.clone()).collect();

        let query = format!(
            "SELECT id FROM decoders WHERE id IN ({})",
            ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for id in &ids {
            query_builder = query_builder.bind(id);
        }

        let existing_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for decoder in decoders {
            if existing_ids.contains(&decoder.id) {
                duplicate_ids.push(decoder.id.clone());
            } else {
                new_ids.push(decoder.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate digital roster entries by `owned_rolling_stock_id`.
    ///
    /// A collection item can have at most one roster entry, so `owned_rolling_stock_id`
    /// is the natural uniqueness key.
    pub async fn check_digital_roster(
        &self,
        items: &[DigitalRollingStockRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if items.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        let owned_ids: Vec<String> = items
            .iter()
            .map(|i| i.owned_rolling_stock_id.clone())
            .collect();

        let query = format!(
            "SELECT id FROM owned_rolling_stocks \
             WHERE id IN ({}) \
               AND (dcc_address IS NOT NULL OR installed_decoder_id IS NOT NULL)",
            owned_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ")
        );

        let mut query_builder = sqlx::query_scalar::<_, String>(&query);
        for oid in &owned_ids {
            query_builder = query_builder.bind(oid);
        }

        let existing_owned_ids: HashSet<String> = query_builder
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .collect();

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for item in items {
            if existing_owned_ids.contains(&item.owned_rolling_stock_id) {
                duplicate_ids.push(item.id.clone());
            } else {
                new_ids.push(item.id.clone());
            }
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::LocalizedTextRecord;

    async fn test_pool() -> sqlx::SqlitePool {
        let pool = sqlx::SqlitePool::connect(":memory:").await.expect("pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        pool
    }

    #[test]
    fn test_duplicate_check_result_counts() {
        let result = DuplicateCheckResult {
            duplicate_ids: vec!["dup1".to_string(), "dup2".to_string()],
            new_ids: vec!["new1".to_string(), "new2".to_string(), "new3".to_string()],
        };

        assert_eq!(result.duplicate_count(), 2);
        assert_eq!(result.new_count(), 3);
        assert_eq!(result.total_count(), 5);
    }

    #[test]
    fn test_duplicate_check_result_empty() {
        let result = DuplicateCheckResult::default();

        assert_eq!(result.duplicate_count(), 0);
        assert_eq!(result.new_count(), 0);
        assert_eq!(result.total_count(), 0);
    }

    #[tokio::test]
    async fn test_check_formation_categories_empty_input() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE formation_categories (id TEXT PRIMARY KEY, name TEXT NOT NULL UNIQUE, is_custom INTEGER NOT NULL DEFAULT 0)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        let checker = DuplicateChecker::new(pool);
        let result = checker
            .check_formation_categories(&[])
            .await
            .expect("check");
        assert_eq!(result.total_count(), 0);
    }

    #[tokio::test]
    async fn test_check_train_formations_detects_duplicate_name() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE train_formations (id TEXT PRIMARY KEY, name TEXT NOT NULL UNIQUE)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query("INSERT INTO train_formations (id, name) VALUES ('existing-id', 'Express')")
            .execute(&pool)
            .await
            .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let formations = vec![
            TrainFormationRecord {
                id: "existing-id".to_string(),
                name: "Express".to_string(),
                ..Default::default()
            },
            TrainFormationRecord {
                id: "new-id".to_string(),
                name: "Local".to_string(),
                ..Default::default()
            },
        ];

        let result = checker
            .check_train_formations(&formations)
            .await
            .expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(result.duplicate_ids.contains(&"existing-id".to_string()));
        assert!(result.new_ids.contains(&"new-id".to_string()));
    }

    #[tokio::test]
    async fn test_check_wishlists_detects_duplicate_name() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE wishlists (id TEXT PRIMARY KEY, name TEXT NOT NULL UNIQUE, \
             notes TEXT, is_default INTEGER NOT NULL DEFAULT 0, version INTEGER NOT NULL DEFAULT 0, \
             created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query("INSERT INTO wishlists (id, name) VALUES ('w1', 'My Wishlist')")
            .execute(&pool)
            .await
            .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let wishlists = vec![
            WishlistRecord {
                id: "w1".to_string(),
                name: "My Wishlist".to_string(),
                ..Default::default()
            },
            WishlistRecord {
                id: "w2".to_string(),
                name: "New Wishlist".to_string(),
                ..Default::default()
            },
        ];

        let result = checker.check_wishlists(&wishlists).await.expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(result.duplicate_ids.contains(&"w1".to_string()));
        assert!(result.new_ids.contains(&"w2".to_string()));
    }

    #[tokio::test]
    async fn test_check_track_products_empty_input() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE track_products (track_id TEXT PRIMARY KEY, product_name TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .expect("create table");

        let checker = DuplicateChecker::new(pool);
        let result = checker.check_track_products(&[]).await.expect("check");

        assert_eq!(result.total_count(), 0);
        assert_eq!(result.duplicate_count(), 0);
        assert_eq!(result.new_count(), 0);
    }

    #[tokio::test]
    async fn test_check_track_products_detects_duplicate_track_id() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE track_products (track_id TEXT PRIMARY KEY, product_name TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query(
            "INSERT INTO track_products (track_id, product_name) VALUES ('trn:track:r1', 'R1 Curve')",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let products = vec![
            TrackProductRecord {
                track_id: "trn:track:r1".to_string(),
                manufacturer_id: "trn:manufacturer:acme".to_string(),
                product_code: "R1".to_string(),
                description: LocalizedTextRecord {
                    en: Some("R1 Curve".to_string()),
                    it: None,
                },
                details: None,
                track_type: "CURVE".to_string(),
                track_code: "CODE_100".to_string(),
                with_roadbed: false,
                length: None,
                radius: Some(360),
            },
            TrackProductRecord {
                track_id: "trn:track:r2".to_string(),
                manufacturer_id: "trn:manufacturer:acme".to_string(),
                product_code: "R2".to_string(),
                description: LocalizedTextRecord {
                    en: Some("R2 Curve".to_string()),
                    it: None,
                },
                details: None,
                track_type: "CURVE".to_string(),
                track_code: "CODE_100".to_string(),
                with_roadbed: false,
                length: None,
                radius: Some(420),
            },
        ];

        let result = checker
            .check_track_products(&products)
            .await
            .expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(result.duplicate_ids.contains(&"trn:track:r1".to_string()));
        assert!(result.new_ids.contains(&"trn:track:r2".to_string()));
    }

    #[tokio::test]
    async fn test_check_track_inventories_empty_input() {
        let pool = test_pool().await;
        sqlx::query("CREATE TABLE track_inventories (id TEXT PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("create table");

        let checker = DuplicateChecker::new(pool);
        let result = checker.check_track_inventories(&[]).await.expect("check");

        assert_eq!(result.total_count(), 0);
        assert_eq!(result.duplicate_count(), 0);
        assert_eq!(result.new_count(), 0);
    }

    #[tokio::test]
    async fn test_check_track_inventories_detects_duplicate_id() {
        let pool = test_pool().await;
        sqlx::query("CREATE TABLE track_inventories (id TEXT PRIMARY KEY, name TEXT NOT NULL)")
            .execute(&pool)
            .await
            .expect("create table");
        sqlx::query(
            "INSERT INTO track_inventories (id, name) VALUES ('trn:track-inventory:main', 'Main Layout')",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let inventories = vec![
            TrackInventoryRecord {
                id: "trn:track-inventory:main".to_string(),
                name: "Main Layout".to_string(),
                description: None,
                items: Vec::new(),
                purchases: Vec::new(),
            },
            TrackInventoryRecord {
                id: "trn:track-inventory:branch".to_string(),
                name: "Branch Layout".to_string(),
                description: None,
                items: Vec::new(),
                purchases: Vec::new(),
            },
        ];

        let result = checker
            .check_track_inventories(&inventories)
            .await
            .expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(
            result
                .duplicate_ids
                .contains(&"trn:track-inventory:main".to_string())
        );
        assert!(
            result
                .new_ids
                .contains(&"trn:track-inventory:branch".to_string())
        );
    }

    #[tokio::test]
    async fn test_check_prototypes_uses_composite_key() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE prototypes (id TEXT PRIMARY KEY, railway_company_id TEXT NOT NULL, series_code TEXT NOT NULL, specification_type TEXT NOT NULL, is_custom INTEGER NOT NULL DEFAULT 0)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query(
            "INSERT INTO prototypes (id, railway_company_id, series_code, specification_type, is_custom) VALUES ('p1', 'rc1', 'BR 01', 'LOCOMOTIVE', 0)",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let prototypes = vec![
            PrototypeRecord {
                id: "p1".to_string(),
                railway_company_id: "rc1".to_string(),
                series_code: "BR 01".to_string(),
                specification_type: "LOCOMOTIVE".to_string(),
                ..Default::default()
            },
            PrototypeRecord {
                id: "p2".to_string(),
                railway_company_id: "rc1".to_string(),
                series_code: "BR 50".to_string(),
                specification_type: "LOCOMOTIVE".to_string(),
                ..Default::default()
            },
        ];

        let result = checker.check_prototypes(&prototypes).await.expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(result.duplicate_ids.contains(&"p1".to_string()));
        assert!(result.new_ids.contains(&"p2".to_string()));
    }

    #[tokio::test]
    async fn test_check_decoders_empty_input() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE decoders (id TEXT PRIMARY KEY, manufacturer_id TEXT NOT NULL, \
             product_code TEXT, decoder_type TEXT NOT NULL, protocol TEXT NOT NULL, \
             decoder_interface TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        let checker = DuplicateChecker::new(pool);
        let result = checker.check_decoders(&[]).await.expect("check");
        assert_eq!(result.total_count(), 0);
    }

    #[tokio::test]
    async fn test_check_decoders_detects_duplicate_id() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE decoders (id TEXT PRIMARY KEY, manufacturer_id TEXT NOT NULL, \
             product_code TEXT, decoder_type TEXT NOT NULL, protocol TEXT NOT NULL, \
             decoder_interface TEXT NOT NULL)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query(
            "INSERT INTO decoders (id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface) \
             VALUES ('trn:decoder:marklin:d100', 'marklin', 'D-100', 'PLAIN', 'DCC', 'NEM651')",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let decoders = vec![
            DecoderRecord {
                id: "trn:decoder:marklin:d100".to_string(),
                manufacturer_id: "marklin".to_string(),
                product_code: "D-100".to_string(),
                decoder_type: "PLAIN".to_string(),
                protocol: "DCC".to_string(),
                decoder_interface: "NEM651".to_string(),
            },
            DecoderRecord {
                id: "trn:decoder:marklin:d200".to_string(),
                manufacturer_id: "marklin".to_string(),
                product_code: "D-200".to_string(),
                decoder_type: "SOUND".to_string(),
                protocol: "DCC".to_string(),
                decoder_interface: "NEM651".to_string(),
            },
        ];

        let result = checker.check_decoders(&decoders).await.expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(
            result
                .duplicate_ids
                .contains(&"trn:decoder:marklin:d100".to_string())
        );
        assert!(
            result
                .new_ids
                .contains(&"trn:decoder:marklin:d200".to_string())
        );
    }

    #[tokio::test]
    async fn test_check_digital_roster_empty_input() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE digital_rolling_stocks (id TEXT PRIMARY KEY, \
             owned_rolling_stock_id TEXT NOT NULL, dcc_address INTEGER NOT NULL, \
             installed_decoder_id TEXT)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        let checker = DuplicateChecker::new(pool);
        let result = checker.check_digital_roster(&[]).await.expect("check");
        assert_eq!(result.total_count(), 0);
    }

    #[tokio::test]
    async fn test_check_digital_roster_detects_duplicate_by_owned_id() {
        let pool = test_pool().await;
        sqlx::query(
            "CREATE TABLE owned_rolling_stocks (id TEXT PRIMARY KEY, dcc_address INTEGER, installed_decoder_id TEXT)",
        )
        .execute(&pool)
        .await
        .expect("create table");
        sqlx::query("INSERT INTO owned_rolling_stocks (id, dcc_address) VALUES ('ors-abc', 3)")
            .execute(&pool)
            .await
            .expect("insert");

        let checker = DuplicateChecker::new(pool);
        let items = vec![
            DigitalRollingStockRecord {
                id: "drs-1".to_string(),
                owned_rolling_stock_id: "ors-abc".to_string(),
                dcc_address: 3,
                decoder_id: None,
            },
            DigitalRollingStockRecord {
                id: "drs-2".to_string(),
                owned_rolling_stock_id: "ors-xyz".to_string(),
                dcc_address: 7,
                decoder_id: None,
            },
        ];

        let result = checker.check_digital_roster(&items).await.expect("check");

        assert_eq!(result.duplicate_count(), 1);
        assert_eq!(result.new_count(), 1);
        assert!(result.duplicate_ids.contains(&"drs-1".to_string()));
        assert!(result.new_ids.contains(&"drs-2".to_string()));
    }
}
