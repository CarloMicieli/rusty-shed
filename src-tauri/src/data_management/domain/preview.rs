use crate::data_management::domain::{ImportWarning, RecordCounts, ValidationError};
use serde::{Deserialize, Serialize};
use specta::Type;

/// Details of duplicate records found during preview
#[derive(Debug, Clone, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "camelCase")]
pub struct DuplicateDetails {
    /// Duplicate manufacturer names
    pub manufacturers: Vec<String>,
    /// Duplicate railway model IDs
    pub railway_models: Vec<String>,
    /// Duplicate collection item IDs
    pub collection_items: Vec<String>,
    /// Duplicate seller names
    pub sellers: Vec<String>,
    /// Duplicate track product TRN identifiers
    pub track_products: Vec<String>,
    /// Duplicate track inventory IDs
    pub track_inventories: Vec<String>,
    /// Duplicate train formation names
    pub train_formations: Vec<String>,
}

/// Preview of import before confirmation.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreview {
    /// Total records found in manifest
    pub total_records: RecordCounts,
    /// Records that will be imported (new)
    pub new_records: RecordCounts,
    /// Records that will be skipped (duplicates)
    pub duplicate_records: RecordCounts,
    /// Specific duplicate record identifiers
    pub duplicate_details: DuplicateDetails,
    /// Validation errors (blocking)
    pub errors: Vec<ValidationError>,
    /// Warnings (non-blocking)
    pub warnings: Vec<ImportWarning>,
}

impl ImportPreview {
    /// Create a new import preview.
    pub fn new() -> Self {
        Self {
            total_records: RecordCounts::new(),
            new_records: RecordCounts::new(),
            duplicate_records: RecordCounts::new(),
            duplicate_details: DuplicateDetails::default(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Check if import can proceed (no blocking errors).
    pub fn can_import(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the count of blocking errors.
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Get the count of warnings.
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
}

impl Default for ImportPreview {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_preview_can_import_no_errors() {
        let preview = ImportPreview::new();
        assert!(preview.can_import());
    }

    #[test]
    fn test_import_preview_cannot_import_with_errors() {
        let mut preview = ImportPreview::new();
        preview
            .errors
            .push(ValidationError::new("test", "code", "msg"));
        assert!(!preview.can_import());
    }

    #[test]
    fn test_import_preview_warning_count() {
        let mut preview = ImportPreview::new();
        preview
            .warnings
            .push(ImportWarning::missing_image("test.jpg"));
        preview
            .warnings
            .push(ImportWarning::missing_image("test2.jpg"));
        assert_eq!(preview.warning_count(), 2);
    }
}
