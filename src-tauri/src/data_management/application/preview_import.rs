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
            owned_rolling_stocks: manifest.data.owned_rolling_stocks.len() as u32,
            sellers: manifest.data.sellers.len() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: manifest.data.track_products.len() as u32,
            track_inventories: manifest.data.track_inventories.len() as u32,
            prototypes: manifest.data.prototypes.len() as u32,
            formation_categories: manifest.data.formation_categories.len() as u32,
            train_formations: manifest.data.train_formations.len() as u32,
            wishlists: manifest.data.wishlists.len() as u32,
            decoders: manifest.data.decoders.len() as u32,
            digital_rolling_stocks: manifest.data.digital_rolling_stocks.len() as u32,
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
            owned_rolling_stocks: 0, // Owned rolling stocks deduplicated via INSERT OR IGNORE
            sellers: duplicates.seller_dupes.duplicate_count() as u32,
            maintenance_cards: 0, // Maintenance cards are linked to collection items
            track_products: duplicates.track_product_dupes.duplicate_count() as u32,
            track_inventories: duplicates.track_inventory_dupes.duplicate_count() as u32,
            prototypes: duplicates.prototype_dupes.duplicate_count() as u32,
            formation_categories: duplicates.formation_category_dupes.duplicate_count() as u32,
            train_formations: duplicates.train_formation_dupes.duplicate_count() as u32,
            wishlists: duplicates.wishlist_dupes.duplicate_count() as u32,
            decoders: duplicates.decoder_dupes.duplicate_count() as u32,
            digital_rolling_stocks: duplicates.digital_roster_dupes.duplicate_count() as u32,
        };

        // Calculate new records
        let new_records = RecordCounts {
            manufacturers: duplicates.manufacturer_dupes.new_count() as u32,
            railway_companies: manifest.data.railway_companies.len() as u32,
            railway_models: duplicates.railway_model_dupes.new_count() as u32,
            collection_items: duplicates.collection_item_dupes.new_count() as u32,
            owned_rolling_stocks: manifest.data.owned_rolling_stocks.len() as u32,
            sellers: duplicates.seller_dupes.new_count() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: duplicates.track_product_dupes.new_count() as u32,
            track_inventories: duplicates.track_inventory_dupes.new_count() as u32,
            prototypes: duplicates.prototype_dupes.new_count() as u32,
            formation_categories: duplicates.formation_category_dupes.new_count() as u32,
            train_formations: duplicates.train_formation_dupes.new_count() as u32,
            wishlists: duplicates.wishlist_dupes.new_count() as u32,
            decoders: duplicates.decoder_dupes.new_count() as u32,
            digital_rolling_stocks: duplicates.digital_roster_dupes.new_count() as u32,
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
        preview.duplicate_details.wishlists = duplicates.wishlist_dupes.duplicate_ids;
        preview.duplicate_details.decoders = duplicates.decoder_dupes.duplicate_ids;
        preview.duplicate_details.digital_rolling_stocks =
            duplicates.digital_roster_dupes.duplicate_ids;

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

        let referenced_images = collect_referenced_images(manifest);

        // Build a set of archive file paths for O(1) lookup
        let archive_file_set: HashSet<String> = archive_files.into_iter().collect();

        Ok(build_image_warnings(&archive_file_set, referenced_images))
    }
}

fn collect_referenced_images(
    manifest: &crate::data_management::domain::ManifestDto,
) -> HashSet<String> {
    manifest
        .data
        .railway_models
        .iter()
        .filter_map(|model| model.image.as_deref())
        .chain(
            manifest
                .data
                .collection_items
                .iter()
                .filter_map(|item| item.image.as_deref()),
        )
        .filter(|image| !image.is_empty())
        .map(ToString::to_string)
        .collect()
}

fn build_image_warnings(
    archive_file_set: &HashSet<String>,
    referenced_images: HashSet<String>,
) -> Vec<ImportWarning> {
    referenced_images
        .into_iter()
        .filter_map(|image_filename| {
            let expected_path = format!("images/{image_filename}");
            let found = archive_file_set.contains(&expected_path)
                || archive_file_set.contains(&image_filename);

            if !found {
                Some(ImportWarning::missing_image(&image_filename))
            } else if !ArchiveExtractor::is_valid_image_extension(&image_filename) {
                Some(ImportWarning {
                    code: "invalid_image_extension".to_string(),
                    message: format!(
                        "Image has invalid extension (only .png, .jpg, .jpeg allowed): {image_filename}"
                    ),
                    context: Some(image_filename),
                })
            } else {
                None
            }
        })
        .collect()
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
    use crate::data_management::domain::{
        CollectionItemRecord, DataContainerDto, LocalizedTextRecord, ManifestDto,
        RailwayModelRecord,
    };
    use std::io::Write;
    use tempfile::{Builder, NamedTempFile};
    use zip::write::SimpleFileOptions;

    fn make_manifest(
        railway_model_images: Vec<Option<&str>>,
        collection_item_images: Vec<Option<&str>>,
    ) -> ManifestDto {
        let railway_models = railway_model_images
            .into_iter()
            .enumerate()
            .map(|(idx, image)| RailwayModelRecord {
                id: format!("model-{idx}"),
                manufacturer_id: "man-1".to_string(),
                product_code: format!("P{idx}"),
                description: LocalizedTextRecord {
                    en: Some("desc".to_string()),
                    it: None,
                },
                scale: "HO".to_string(),
                epoch: "IV".to_string(),
                category: "LOCOMOTIVES".to_string(),
                power_method: "DC".to_string(),
                image: image.map(ToString::to_string),
                ..RailwayModelRecord::default()
            })
            .collect::<Vec<_>>();

        let collection_items = collection_item_images
            .into_iter()
            .enumerate()
            .map(|(idx, image)| CollectionItemRecord {
                id: format!("item-{idx}"),
                railway_model_id: "model-0".to_string(),
                added_date: "2026-01-01".to_string(),
                removed_date: None,
                purchase_condition: None,
                model_condition: None,
                box_condition: None,
                notes: None,
                image: image.map(ToString::to_string),
                purchase: None,
            })
            .collect::<Vec<_>>();

        ManifestDto {
            schema: None,
            version: "1.0.0".to_string(),
            exported_at: None,
            source: None,
            data: DataContainerDto {
                railway_models,
                collection_items,
                ..DataContainerDto::default()
            },
        }
    }

    fn create_zip_archive(files: &[(&str, &[u8])]) -> NamedTempFile {
        let mut temp_file = Builder::new()
            .suffix(".zip")
            .tempfile()
            .expect("temp zip file should be created");
        {
            let writer = temp_file.as_file_mut();
            let mut zip = zip::ZipWriter::new(writer);
            let options = SimpleFileOptions::default();

            for (name, contents) in files {
                zip.start_file(name, options)
                    .expect("zip file entry should be created");
                zip.write_all(contents)
                    .expect("zip entry should be writable");
            }

            zip.finish().expect("zip archive should be finalized");
        }

        temp_file
    }

    #[test]
    fn test_preview_error_display() {
        let err = PreviewImportError::SchemaLoad("test error".to_string());
        assert_eq!(err.to_string(), "Failed to load schema: test error");

        let err = PreviewImportError::Deserialization("bad json".to_string());
        assert_eq!(err.to_string(), "Manifest deserialization failed: bad json");

        let err = PreviewImportError::Database("connection failed".to_string());
        assert_eq!(err.to_string(), "Database error: connection failed");
    }

    #[test]
    fn test_collect_referenced_images_filters_empty_and_deduplicates() {
        let manifest = make_manifest(
            vec![Some("same.png"), Some(""), None, Some("same.png")],
            vec![Some("other.jpg"), Some("same.png"), None],
        );

        let images = collect_referenced_images(&manifest);
        assert_eq!(images.len(), 2);
        assert!(images.contains("same.png"));
        assert!(images.contains("other.jpg"));
    }

    #[tokio::test]
    async fn test_check_missing_images_warns_for_missing_file() {
        let archive = create_zip_archive(&[("manifest.json", b"{}")]);
        let manifest = make_manifest(vec![Some("missing.png")], vec![]);

        let warnings = PreviewImportUseCase::check_missing_images(archive.path(), &manifest)
            .await
            .expect("missing image check should complete");

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "missing_image");
        assert_eq!(warnings[0].context.as_deref(), Some("missing.png"));
    }

    #[tokio::test]
    async fn test_check_missing_images_accepts_images_folder_or_root_path() {
        let archive = create_zip_archive(&[
            ("images/found-in-folder.jpg", b"image"),
            ("found-at-root.png", b"image"),
        ]);
        let manifest = make_manifest(
            vec![Some("found-in-folder.jpg")],
            vec![Some("found-at-root.png")],
        );

        let warnings = PreviewImportUseCase::check_missing_images(archive.path(), &manifest)
            .await
            .expect("image check should succeed");

        assert!(warnings.is_empty());
    }

    #[tokio::test]
    async fn test_check_missing_images_warns_for_invalid_extension() {
        let archive = create_zip_archive(&[("images/photo.gif", b"gif")]);
        let manifest = make_manifest(vec![Some("photo.gif")], vec![]);

        let warnings = PreviewImportUseCase::check_missing_images(archive.path(), &manifest)
            .await
            .expect("image check should succeed");

        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].code, "invalid_image_extension");
        assert_eq!(warnings[0].context.as_deref(), Some("photo.gif"));
    }

    #[tokio::test]
    async fn test_check_missing_images_maps_archive_listing_error() {
        let manifest = make_manifest(vec![Some("photo.png")], vec![]);
        let invalid_path = std::path::PathBuf::from("/tmp/not-an-archive.txt");

        let error = PreviewImportUseCase::check_missing_images(&invalid_path, &manifest)
            .await
            .expect_err("invalid archive should error");

        match error {
            PreviewImportError::Archive(msg) => assert!(msg.contains("Failed to list archive")),
            other => panic!("expected Archive error, got: {other:?}"),
        }
    }
}
