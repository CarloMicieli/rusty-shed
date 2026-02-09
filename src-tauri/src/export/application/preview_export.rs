/// Preview export use case
use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

/// Export preview information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreview {
    pub railway_model_count: u32,
    pub collection_item_count: u32,
    pub seller_count: u32,
    pub maintenance_log_count: u32,
    pub dcc_roster_count: u32,
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
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM maintenance_cards")
            .fetch_one(pool)
            .await
            .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.maintenance_log_count = count.0 as u32;
    }

    // Count DCC roster if selected
    if selection.include_dcc_roster {
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM rolling_stocks WHERE digital_setup IS NOT NULL")
                .fetch_one(pool)
                .await
                .map_err(|e| ExportError::DatabaseError(e.to_string()))?;
        preview.dcc_roster_count = count.0 as u32;
    }

    // Estimate total data size (rough calculation: 1KB per record + images)
    let total_records = preview.railway_model_count
        + preview.collection_item_count
        + preview.seller_count
        + preview.maintenance_log_count
        + preview.dcc_roster_count;

    preview.estimated_size_bytes =
        (total_records as u64 * 1024) + (preview.image_count as u64 * 500 * 1024); // Assume 500KB per image

    // Check if there's data to export
    if total_records == 0 {
        preview
            .warnings
            .push("No data selected for export".to_string());
    }

    Ok(preview)
}
