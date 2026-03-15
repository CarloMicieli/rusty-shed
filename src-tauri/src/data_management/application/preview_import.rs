use crate::data_management::domain::{ImportPreview, ImportWarning, RecordCounts};
use crate::data_management::infrastructure::{ArchiveExtractor, DuplicateChecker, SchemaValidator};
use serde_json::Value;
use sqlx::SqlitePool;
use std::path::Path;

/// Use case for generating an import preview without writing to the database.
///
/// This shows users what will be imported, identifies duplicates, and reports validation issues.
pub struct PreviewImportUseCase {
    pool: SqlitePool,
}

impl PreviewImportUseCase {
    /// Create a new preview import use case.
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Generate a preview of what would be imported from the manifest.
    ///
    /// # Arguments
    /// * `manifest_json` - The parsed manifest JSON
    /// * `archive_path` - Optional path to the archive for image validation
    ///
    /// # Returns
    /// An `ImportPreview` showing record counts, duplicates, and validation errors
    pub async fn execute(
        &self,
        manifest_json: Value,
        archive_path: Option<&Path>,
    ) -> Result<ImportPreview, PreviewImportError> {
        // Validate manifest against schema
        let schema_validator = SchemaValidator::new()
            .map_err(|e| PreviewImportError::SchemaLoadError(e.to_string()))?;

        let validation_errors = match schema_validator.validate_detailed(&manifest_json) {
            Ok(_) => Vec::new(),
            Err(errors) => errors,
        };

        // If there are validation errors, return early with can_import = false
        if !validation_errors.is_empty() {
            let mut preview = ImportPreview::new();
            preview.errors = validation_errors;
            return Ok(preview);
        }

        // Deserialize manifest
        let manifest: crate::data_management::domain::ManifestDto =
            serde_json::from_value(manifest_json)
                .map_err(|e| PreviewImportError::DeserializationError(e.to_string()))?;

        // Count records from manifest
        let total_counts = RecordCounts {
            manufacturers: manifest.data.manufacturers.len() as u32,
            railway_companies: manifest.data.railway_companies.len() as u32,
            railway_models: manifest.data.railway_models.len() as u32,
            collection_items: manifest.data.collection_items.len() as u32,
            sellers: manifest.data.sellers.len() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: manifest.data.track_products.len() as u32,
            track_inventories: manifest.data.track_inventories.len() as u32,
        };

        // Check for duplicates
        let duplicate_checker = DuplicateChecker::new(self.pool.clone());

        let manufacturer_dupes = duplicate_checker
            .check_manufacturers(&manifest.data.manufacturers)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        let railway_model_dupes = duplicate_checker
            .check_railway_models(&manifest.data.railway_models)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        let collection_item_dupes = duplicate_checker
            .check_collection_items(&manifest.data.collection_items)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        let seller_dupes = duplicate_checker
            .check_sellers(&manifest.data.sellers)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        let track_product_dupes = duplicate_checker
            .check_track_products(&manifest.data.track_products)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        let track_inventory_dupes = duplicate_checker
            .check_track_inventories(&manifest.data.track_inventories)
            .await
            .map_err(|e| PreviewImportError::DatabaseError(e.to_string()))?;

        // Calculate duplicate counts
        let duplicate_counts = RecordCounts {
            manufacturers: manufacturer_dupes.duplicate_count() as u32,
            railway_companies: 0, // Railway companies checked by name via unique index
            railway_models: railway_model_dupes.duplicate_count() as u32,
            collection_items: collection_item_dupes.duplicate_count() as u32,
            sellers: seller_dupes.duplicate_count() as u32,
            maintenance_cards: 0, // Maintenance cards are linked to collection items
            track_products: track_product_dupes.duplicate_count() as u32,
            track_inventories: track_inventory_dupes.duplicate_count() as u32,
        };

        // Calculate new records
        let new_records = RecordCounts {
            manufacturers: manufacturer_dupes.new_count() as u32,
            railway_companies: manifest.data.railway_companies.len() as u32,
            railway_models: railway_model_dupes.new_count() as u32,
            collection_items: collection_item_dupes.new_count() as u32,
            sellers: seller_dupes.new_count() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: track_product_dupes.new_count() as u32,
            track_inventories: track_inventory_dupes.new_count() as u32,
        };

        // Create preview
        let mut preview = ImportPreview::new();
        preview.total_records = total_counts;
        preview.new_records = new_records;
        preview.duplicate_records = duplicate_counts;
        preview.duplicate_details.manufacturers = manufacturer_dupes.duplicate_ids;
        preview.duplicate_details.railway_models = railway_model_dupes.duplicate_ids;
        preview.duplicate_details.collection_items = collection_item_dupes.duplicate_ids;
        preview.duplicate_details.sellers = seller_dupes.duplicate_ids;
        preview.duplicate_details.track_products = track_product_dupes.duplicate_ids;
        preview.duplicate_details.track_inventories = track_inventory_dupes.duplicate_ids;

        // Check for missing images if archive path is provided
        if let Some(path) = archive_path {
            let image_warnings = Self::check_missing_images(path, &manifest)?;
            preview.warnings.extend(image_warnings);
        }

        Ok(preview)
    }

    /// Check for missing images referenced in the manifest.
    ///
    /// # Arguments
    /// * `archive_path` - Path to the archive file
    /// * `manifest` - The parsed manifest data
    ///
    /// # Returns
    /// A vector of warnings for missing images
    fn check_missing_images(
        archive_path: &Path,
        manifest: &crate::data_management::domain::ManifestDto,
    ) -> Result<Vec<ImportWarning>, PreviewImportError> {
        // List all files in the archive
        let archive_files = ArchiveExtractor::list_files(archive_path).map_err(|e| {
            PreviewImportError::ArchiveError(format!("Failed to list archive files: {}", e))
        })?;

        // Collect all image filenames referenced in the manifest
        let mut referenced_images = Vec::new();

        // Railway models can have images
        for model in &manifest.data.railway_models {
            if let Some(ref image) = model.image
                && !image.is_empty()
            {
                referenced_images.push(image.clone());
            }
        }

        // Collection items can have images
        for item in &manifest.data.collection_items {
            if let Some(ref image) = item.image
                && !image.is_empty()
            {
                referenced_images.push(image.clone());
            }
        }

        // Check which referenced images are missing from the archive
        let mut warnings = Vec::new();
        for image_filename in referenced_images {
            // Check if the file exists in the archive
            // Images should be in the "images/" directory
            let expected_path = format!("images/{}", image_filename);
            let found = archive_files
                .iter()
                .any(|f| f == &expected_path || f == &image_filename);

            if !found {
                warnings.push(ImportWarning::missing_image(&image_filename));
            } else {
                // Check if the file has a valid image extension
                if !ArchiveExtractor::is_valid_image_extension(&image_filename) {
                    warnings.push(ImportWarning {
                        code: "invalid_image_extension".to_string(),
                        message: format!(
                            "Image has invalid extension (only .png, .jpg, .jpeg allowed): {}",
                            image_filename
                        ),
                        context: Some(image_filename),
                    });
                }
            }
        }

        Ok(warnings)
    }
}

/// Errors that can occur during preview generation
#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum PreviewImportError {
    #[error("Failed to load schema: {0}")]
    SchemaLoadError(String),

    #[error("Manifest deserialization failed: {0}")]
    DeserializationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Archive error: {0}")]
    ArchiveError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_error_display() {
        let err = PreviewImportError::SchemaLoadError("test error".to_string());
        assert_eq!(err.to_string(), "Failed to load schema: test error");

        let err = PreviewImportError::DeserializationError("bad json".to_string());
        assert_eq!(err.to_string(), "Manifest deserialization failed: bad json");

        let err = PreviewImportError::DatabaseError("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }
}
