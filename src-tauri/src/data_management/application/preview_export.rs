use crate::data_management::domain::{ExportEntitySelection, ExportError};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Export preview information
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ExportPreview {
    pub railway_model_count: u32,
    pub collection_item_count: u32,
    pub seller_count: u32,
    pub maintenance_log_count: u32,
    pub dcc_roster_count: u32,
    pub train_formation_count: u32,
    pub image_count: u32,
    pub orphaned_image_count: u32,
    pub estimated_size_bytes: u64,
    pub warnings: Vec<String>,
}

impl ExportPreview {
    /// Create a new export preview
    pub fn new() -> Self {
        ExportPreview {
            railway_model_count: 0,
            collection_item_count: 0,
            seller_count: 0,
            maintenance_log_count: 0,
            dcc_roster_count: 0,
            train_formation_count: 0,
            image_count: 0,
            orphaned_image_count: 0,
            estimated_size_bytes: 0,
            warnings: Vec::new(),
        }
    }
}

impl Default for ExportPreview {
    fn default() -> Self {
        Self::new()
    }
}

/// Get export preview for given entity selection
pub async fn get_export_preview(
    pool: &SqlitePool,
    selection: &ExportEntitySelection,
) -> Result<ExportPreview, ExportError> {
    let mut preview = ExportPreview::new();

    // Count railway models if selected
    if selection.include_railway_models {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM railway_models")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.railway_model_count = count.0 as u32;
    }

    // Count collection items if selected
    if selection.include_collection_items {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM collection_items")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.collection_item_count = count.0 as u32;
    }

    // Count sellers if selected
    if selection.include_sellers {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sellers")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.seller_count = count.0 as u32;
    }

    // Count maintenance logs if selected
    if selection.include_maintenance_logs {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM maintenance_events")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.maintenance_log_count = count.0 as u32;
    }

    // Count DCC roster entries if selected
    if selection.include_dcc_roster {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM owned_rolling_stocks WHERE dcc_address IS NOT NULL OR installed_decoder_id IS NOT NULL",
        )
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.dcc_roster_count = count.0 as u32;
    }

    // Count train formations if selected
    if selection.include_train_formations {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM train_formations")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.train_formation_count = count.0 as u32;
    }

    // Estimate total data size (rough calculation: 1KB per record + images)
    let total_records = preview.railway_model_count
        + preview.collection_item_count
        + preview.seller_count
        + preview.maintenance_log_count
        + preview.dcc_roster_count
        + preview.train_formation_count;

    preview.estimated_size_bytes =
        (total_records as u64 * 1024) + (preview.image_count as u64 * 500 * 1024);

    // Check if there's data to export
    if total_records == 0 {
        preview
            .warnings
            .push("No data selected for export".to_string());
    }

    Ok(preview)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    fn all_selection() -> ExportEntitySelection {
        ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: true,
            include_sellers: true,
            include_maintenance_logs: true,
            include_dcc_roster: true,
            include_orphaned_images: false,
            include_track_inventory: false,
            include_train_formations: false,
            include_wishlists: false,
        }
    }

    async fn setup_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");

        sqlx::query(
            "CREATE TABLE railway_models (
                id TEXT PRIMARY KEY,
                manufacturer_id TEXT NOT NULL,
                product_code TEXT NOT NULL,
                power_method TEXT NOT NULL,
                scale TEXT NOT NULL,
                epoch TEXT NOT NULL,
                category TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("create railway_models");

        sqlx::query(
            "CREATE TABLE collection_items (
                id TEXT PRIMARY KEY,
                railway_model_id TEXT NOT NULL,
                added_date TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("create collection_items");

        sqlx::query(
            "CREATE TABLE sellers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                type TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .expect("create sellers");

        sqlx::query(
            "CREATE TABLE maintenance_events (
                id TEXT PRIMARY KEY,
                maintenance_card_id TEXT NOT NULL,
                date_performed TEXT NOT NULL,
                maintenance_type TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("create maintenance_events");

        sqlx::query(
            "CREATE TABLE owned_rolling_stocks (
                id TEXT PRIMARY KEY,
                dcc_address INTEGER,
                installed_decoder_id TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("create owned_rolling_stocks");

        pool
    }

    #[tokio::test]
    async fn test_empty_db_all_counts_zero_with_warning() {
        let pool = setup_db().await;
        let selection = all_selection();

        let preview = get_export_preview(&pool, &selection)
            .await
            .expect("preview");

        assert_eq!(preview.railway_model_count, 0);
        assert_eq!(preview.collection_item_count, 0);
        assert_eq!(preview.seller_count, 0);
        assert_eq!(preview.maintenance_log_count, 0);
        assert_eq!(preview.dcc_roster_count, 0);
        assert!(
            !preview.warnings.is_empty(),
            "should warn when no data selected"
        );
    }

    #[tokio::test]
    async fn test_counts_railway_models_correctly() {
        let pool = setup_db().await;

        sqlx::query(
            "INSERT INTO railway_models (id, manufacturer_id, product_code, power_method, scale, epoch, category) \
             VALUES ('rm-1', 'mfr-1', 'CODE-1', 'DC', 'H0', 'IV', 'LOCOMOTIVES')",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let selection = ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: false,
            include_sellers: false,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
            include_train_formations: false,
            include_wishlists: false,
        };

        let preview = get_export_preview(&pool, &selection)
            .await
            .expect("preview");

        assert_eq!(preview.railway_model_count, 1);
    }

    #[tokio::test]
    async fn test_estimated_size_nonzero_when_records_exist() {
        let pool = setup_db().await;

        sqlx::query(
            "INSERT INTO railway_models (id, manufacturer_id, product_code, power_method, scale, epoch, category) \
             VALUES ('rm-1', 'mfr-1', 'P1', 'DC', 'H0', 'IV', 'LOCOMOTIVES')",
        )
        .execute(&pool)
        .await
        .expect("insert");

        let selection = ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: false,
            include_sellers: false,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
            include_train_formations: false,
            include_wishlists: false,
        };

        let preview = get_export_preview(&pool, &selection)
            .await
            .expect("preview");

        assert!(
            preview.estimated_size_bytes > 0,
            "estimated size must be positive when records exist"
        );
    }

    #[tokio::test]
    async fn test_counts_train_formations_correctly() {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        sqlx::query(
            "CREATE TABLE train_formations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE
            )",
        )
        .execute(&pool)
        .await
        .expect("create train_formations");

        sqlx::query("INSERT INTO train_formations (id, name) VALUES ('tf-1', 'Test Formation')")
            .execute(&pool)
            .await
            .expect("insert formation");

        let selection = ExportEntitySelection {
            include_railway_models: false,
            include_collection_items: false,
            include_sellers: false,
            include_maintenance_logs: false,
            include_dcc_roster: false,
            include_orphaned_images: false,
            include_track_inventory: false,
            include_train_formations: true,
            include_wishlists: false,
        };

        let preview = get_export_preview(&pool, &selection)
            .await
            .expect("preview");

        assert_eq!(preview.train_formation_count, 1);
    }
}
