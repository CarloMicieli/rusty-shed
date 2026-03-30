use crate::data_management::application::ports::ImportRepository;
use crate::data_management::domain::{ImportPreview, ImportWarning, RecordCounts};
use crate::data_management::infrastructure::{ArchiveExtractor, SchemaValidator};
use serde_json::Value;
use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

/// Use case for generating an import preview without writing to the database.
///
/// This shows users what will be imported, identifies duplicates, and reports validation issues.
pub struct PreviewImportUseCase {
    repo: Arc<dyn ImportRepository>,
    /// Schema validator constructed once to avoid re-parsing the embedded JSON schema per request.
    schema_validator: SchemaValidator,
}

impl PreviewImportUseCase {
    /// Create a new preview import use case.
    pub fn new(repo: Arc<dyn ImportRepository>) -> Result<Self, PreviewImportError> {
        let schema_validator =
            SchemaValidator::new().map_err(|e| PreviewImportError::SchemaLoad(e.to_string()))?;
        Ok(Self {
            repo,
            schema_validator,
        })
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
        let validation_errors = match self.schema_validator.validate_detailed(&manifest_json) {
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
                .map_err(|e| PreviewImportError::Deserialization(e.to_string()))?;

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
            prototypes: manifest.data.prototypes.len() as u32,
            formation_categories: manifest.data.formation_categories.len() as u32,
            train_formations: manifest.data.train_formations.len() as u32,
        };

        // Check for duplicates via repository port
        let duplicates = self
            .repo
            .check_duplicates(&manifest.data)
            .await
            .map_err(|e| PreviewImportError::Database(e.to_string()))?;

        // Calculate duplicate counts
        let duplicate_counts = RecordCounts {
            manufacturers: duplicates.manufacturer_dupes.duplicate_count() as u32,
            railway_companies: 0, // Railway companies checked by name via unique index
            railway_models: duplicates.railway_model_dupes.duplicate_count() as u32,
            collection_items: duplicates.collection_item_dupes.duplicate_count() as u32,
            sellers: duplicates.seller_dupes.duplicate_count() as u32,
            maintenance_cards: 0, // Maintenance cards are linked to collection items
            track_products: duplicates.track_product_dupes.duplicate_count() as u32,
            track_inventories: duplicates.track_inventory_dupes.duplicate_count() as u32,
            prototypes: duplicates.prototype_dupes.duplicate_count() as u32,
            formation_categories: duplicates.formation_category_dupes.duplicate_count() as u32,
            train_formations: duplicates.train_formation_dupes.duplicate_count() as u32,
        };

        // Calculate new records
        let new_records = RecordCounts {
            manufacturers: duplicates.manufacturer_dupes.new_count() as u32,
            railway_companies: manifest.data.railway_companies.len() as u32,
            railway_models: duplicates.railway_model_dupes.new_count() as u32,
            collection_items: duplicates.collection_item_dupes.new_count() as u32,
            sellers: duplicates.seller_dupes.new_count() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: duplicates.track_product_dupes.new_count() as u32,
            track_inventories: duplicates.track_inventory_dupes.new_count() as u32,
            prototypes: duplicates.prototype_dupes.new_count() as u32,
            formation_categories: duplicates.formation_category_dupes.new_count() as u32,
            train_formations: duplicates.train_formation_dupes.new_count() as u32,
        };

        // Create preview
        let mut preview = ImportPreview::new();
        preview.total_records = total_counts;
        preview.new_records = new_records;
        preview.duplicate_records = duplicate_counts;
        preview.duplicate_details.manufacturers = duplicates.manufacturer_dupes.duplicate_ids;
        preview.duplicate_details.railway_models = duplicates.railway_model_dupes.duplicate_ids;
        preview.duplicate_details.collection_items = duplicates.collection_item_dupes.duplicate_ids;
        preview.duplicate_details.sellers = duplicates.seller_dupes.duplicate_ids;
        preview.duplicate_details.track_products = duplicates.track_product_dupes.duplicate_ids;
        preview.duplicate_details.track_inventories =
            duplicates.track_inventory_dupes.duplicate_ids;
        preview.duplicate_details.train_formations = duplicates.train_formation_dupes.duplicate_ids;

        // Check for missing images if archive path is provided
        if let Some(path) = archive_path {
            let image_warnings = Self::check_missing_images(path, &manifest).await?;
            preview.warnings.extend(image_warnings);
        }

        Ok(preview)
    }

    /// Check for missing images referenced in the manifest.
    async fn check_missing_images(
        archive_path: &Path,
        manifest: &crate::data_management::domain::ManifestDto,
    ) -> Result<Vec<ImportWarning>, PreviewImportError> {
        // List all files in the archive via spawn_blocking to avoid blocking the async runtime
        let archive_files = ArchiveExtractor::list_files_async(archive_path.to_path_buf())
            .await
            .map_err(|e| {
                PreviewImportError::Archive(format!("Failed to list archive files: {}", e))
            })?;

        // Collect all image filenames referenced in the manifest (deduplicated)
        let mut referenced_images: HashSet<String> = HashSet::new();

        for model in &manifest.data.railway_models {
            if let Some(ref image) = model.image
                && !image.is_empty()
            {
                referenced_images.insert(image.clone());
            }
        }

        for item in &manifest.data.collection_items {
            if let Some(ref image) = item.image
                && !image.is_empty()
            {
                referenced_images.insert(image.clone());
            }
        }

        // Build a set of archive file paths for O(1) lookup
        let archive_file_set: HashSet<String> = archive_files.into_iter().collect();

        let mut warnings = Vec::new();
        for image_filename in referenced_images {
            let expected_path = format!("images/{}", image_filename);
            let found = archive_file_set.contains(&expected_path)
                || archive_file_set.contains(&image_filename);

            if !found {
                warnings.push(ImportWarning::missing_image(&image_filename));
            } else if !ArchiveExtractor::is_valid_image_extension(&image_filename) {
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

        Ok(warnings)
    }
}

/// Errors that can occur during preview generation
#[derive(Debug, thiserror::Error)]
pub enum PreviewImportError {
    #[error("Failed to load schema: {0}")]
    SchemaLoad(String),

    #[error("Manifest deserialization failed: {0}")]
    Deserialization(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Archive error: {0}")]
    Archive(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_error_display() {
        let err = PreviewImportError::SchemaLoad("test error".to_string());
        assert_eq!(err.to_string(), "Failed to load schema: test error");

        let err = PreviewImportError::Deserialization("bad json".to_string());
        assert_eq!(err.to_string(), "Manifest deserialization failed: bad json");

        let err = PreviewImportError::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }
}
