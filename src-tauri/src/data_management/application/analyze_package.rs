use crate::data_management::domain::{ArchiveFormat, ManifestDto, RecordCounts, ValidationError};
use crate::data_management::infrastructure::{ArchiveExtractor, Normalizer, SchemaValidator};
use serde_json::Value;
use std::path::Path;

/// Validates an import package for schema compliance and basic integrity.
///
/// This use case:
/// 1. Detects archive format (ZIP or tar.gz)
/// 2. Extracts and parses the manifest
/// 3. Validates against the JSON schema
/// 4. Counts records by entity type
pub struct ValidatePackageUseCase;

impl ValidatePackageUseCase {
    /// Validate a package archive.
    ///
    /// # Arguments
    /// * `package_path` - Absolute path to the .zip or .tar.gz file
    ///
    /// # Returns
    /// A tuple of (archive_format, manifest, record_counts, validation_errors)
    ///
    /// # Errors
    /// Returns `Err(ValidationError)` if archive cannot be read or manifest is missing.
    pub async fn execute(
        package_path: &Path,
    ) -> Result<(ArchiveFormat, ManifestDto, RecordCounts), ValidationError> {
        // Extract manifest JSON from archive
        let manifest_bytes = ArchiveExtractor::extract_manifest(package_path).map_err(|e| {
            ValidationError::new(
                "archive",
                "archive_error",
                format!("Failed to extract manifest: {}", e),
            )
        })?;

        // Parse manifest JSON
        let manifest_str = String::from_utf8(manifest_bytes).map_err(|e| {
            ValidationError::new(
                "manifest",
                "invalid_encoding",
                format!("Manifest is not valid UTF-8: {}", e),
            )
        })?;

        let mut manifest_value: Value = serde_json::from_str(&manifest_str).map_err(|e| {
            ValidationError::new(
                "manifest",
                "invalid_json",
                format!("Manifest is not valid JSON: {}", e),
            )
        })?;

        // Normalize old-format enum values (SCREAMING_SNAKE_CASE → schema canonical)
        // so archives exported before the normalization fix can still be imported.
        Normalizer::normalize_manifest(&mut manifest_value);

        // Validate against schema
        let validator = SchemaValidator::new().map_err(|e| {
            ValidationError::new(
                "schema",
                "schema_load_error",
                format!("Schema validation setup failed: {}", e),
            )
        })?;

        validator.validate(&manifest_value).map_err(|e| {
            ValidationError::new(
                "manifest",
                "schema_validation_error",
                format!("Manifest does not conform to schema: {}", e),
            )
        })?;

        // Deserialize manifest to domain type
        let manifest: ManifestDto = serde_json::from_value(manifest_value).map_err(|e| {
            ValidationError::new(
                "manifest",
                "deserialization_error",
                format!("Could not deserialize manifest: {}", e),
            )
        })?;

        // Detect archive format from file extension
        let format = Self::detect_archive_format(package_path)?;

        // Count records
        let counts = Self::count_records(&manifest);

        Ok((format, manifest, counts))
    }

    /// Detect archive format from file extension.
    fn detect_archive_format(path: &Path) -> Result<ArchiveFormat, ValidationError> {
        match path.extension() {
            Some(ext) => {
                let ext_str = ext.to_string_lossy().to_lowercase();
                match ext_str.as_str() {
                    "zip" => Ok(ArchiveFormat::Zip),
                    "gz" => Ok(ArchiveFormat::TarGz),
                    _ => Err(ValidationError::new(
                        "archive",
                        "invalid_format",
                        "Archive must be .zip or .tar.gz".to_string(),
                    )),
                }
            }
            None => Err(ValidationError::new(
                "archive",
                "no_extension",
                "Archive file has no extension".to_string(),
            )),
        }
    }

    /// Count records by entity type in the manifest.
    fn count_records(manifest: &ManifestDto) -> RecordCounts {
        RecordCounts {
            manufacturers: manifest.data.manufacturers.len() as u32,
            railway_companies: manifest.data.railway_companies.len() as u32,
            railway_models: manifest.data.railway_models.len() as u32,
            collection_items: manifest.data.collection_items.len() as u32,
            sellers: manifest.data.sellers.len() as u32,
            maintenance_cards: manifest.data.maintenance_cards.len() as u32,
            track_products: manifest.data.track_products.len() as u32,
            track_inventories: manifest.data.track_inventories.len() as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_records() {
        let manifest = ManifestDto {
            schema: None,
            version: "1.0".to_string(),
            exported_at: None,
            source: None,
            data: crate::data_management::domain::DataContainerDto {
                manufacturers: vec![Default::default()],
                railway_companies: vec![],
                railway_models: vec![Default::default(), Default::default()],
                collection_items: vec![],
                sellers: vec![],
                maintenance_cards: vec![],
                track_products: vec![],
                track_inventories: vec![],
            },
        };

        let counts = ValidatePackageUseCase::count_records(&manifest);
        assert_eq!(counts.manufacturers, 1);
        assert_eq!(counts.railway_models, 2);
        assert_eq!(counts.total(), 3);
    }
}
