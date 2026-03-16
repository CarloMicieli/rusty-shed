use crate::data_management::domain::{
    CollectionItemRecord, ManufacturerRecord, RailwayModelRecord, SellerRecord,
    TrackInventoryRecord, TrackProductRecord,
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

    /// Check for duplicate manufacturers by name (case-insensitive).
    ///
    /// Returns which manufacturer names already exist in the database.
    pub async fn check_manufacturers(
        &self,
        manufacturers: &[ManufacturerRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if manufacturers.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Extract names for lookup
        let names: Vec<String> = manufacturers.iter().map(|m| m.name.clone()).collect();

        // Query existing manufacturers by name
        let query = format!(
            "SELECT name FROM manufacturers WHERE name IN ({})",
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

        // Partition into duplicates and new records
        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for manufacturer in manufacturers {
            if existing_names.contains(&manufacturer.name) {
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
    /// Note: This uses added_date (the date item was added to collection) as the uniqueness
    /// criterion since the manifest uses this field analogous to purchase_date.
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

        // Build composite keys for items that have a purchase_date.
        // Separator '|' must not appear in valid railway_model_id or date values.
        let composite_keys: Vec<String> = items
            .iter()
            .filter_map(|item| {
                item.purchase
                    .as_ref()
                    .and_then(|p| p.purchase_date.as_ref())
                    .map(|date| format!("{}|{}", item.railway_model_id, date))
            })
            .collect();

        let existing_keys: HashSet<String> = if composite_keys.is_empty() {
            HashSet::new()
        } else {
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

            query_builder
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .collect()
        };

        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for item in items {
            if let Some(ref purchase) = item.purchase
                && let Some(ref purchase_date) = purchase.purchase_date
            {
                let key = format!("{}|{}", item.railway_model_id, purchase_date);
                if existing_keys.contains(&key) {
                    duplicate_ids.push(item.id.clone());
                } else {
                    new_ids.push(item.id.clone());
                }
                continue;
            }
            // Items without purchase record or purchase_date are considered new
            new_ids.push(item.id.clone());
        }

        Ok(DuplicateCheckResult {
            duplicate_ids,
            new_ids,
        })
    }

    /// Check for duplicate sellers by name (case-insensitive).
    ///
    /// Returns which seller names already exist in the database.
    pub async fn check_sellers(
        &self,
        sellers: &[SellerRecord],
    ) -> Result<DuplicateCheckResult, sqlx::Error> {
        if sellers.is_empty() {
            return Ok(DuplicateCheckResult::default());
        }

        // Extract names for lookup
        let names: Vec<String> = sellers.iter().map(|s| s.name.clone()).collect();

        // Query existing sellers by name
        let query = format!(
            "SELECT name FROM sellers WHERE name IN ({})",
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

        // Partition into duplicates and new records
        let mut duplicate_ids = Vec::new();
        let mut new_ids = Vec::new();

        for seller in sellers {
            if existing_names.contains(&seller.name) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
