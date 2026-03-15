use crate::data_management::domain::{ArchiveFormat, ImportWarning, RecordCounts, ValidationError};
use serde::{Deserialize, Serialize};
use specta::Type;

/// Arguments for analyze_import_package command
#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeImportPackageArgs {
    /// Absolute path to the import package file (.zip or .tar.gz)
    pub file_path: String,
}

/// Response for analyze_import_package command
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AnalyzeImportPackageResponse {
    /// Unique session ID for subsequent operations
    pub session_id: String,
    /// Detected archive format
    pub format: ArchiveFormat,
    /// Whether the manifest was found and parseable
    pub manifest_found: bool,
    /// Initial validation status
    pub validation_status: ValidationStatus,
    /// Quick summary of found records
    pub record_counts: RecordCounts,
    /// List of images found in the archive
    pub images_found: Vec<String>,
}

/// Validation status
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ValidationStatus {
    /// Schema validation passed
    Valid,
    /// Schema validation failed with errors
    Invalid { error_count: u32 },
    /// Manifest could not be parsed
    ParseError { message: String },
}

/// Arguments for get_import_preview command
#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GetImportPreviewArgs {
    /// Session ID from analyze_import_package
    pub session_id: String,
}

/// Response for get_import_preview command
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportPreviewResponse {
    /// Session ID
    pub session_id: String,
    /// Total records in manifest
    pub total_records: RecordCounts,
    /// Records that will be imported (new)
    pub new_records: RecordCounts,
    /// Records that will be skipped (duplicates)
    pub duplicate_records: RecordCounts,
    /// Specific duplicate record identifiers
    pub duplicate_details: crate::data_management::domain::DuplicateDetails,
    /// Validation errors (if any, import cannot proceed)
    pub errors: Vec<ValidationError>,
    /// Warnings (non-blocking, e.g., missing images)
    pub warnings: Vec<ImportWarning>,
    /// Whether import can proceed
    pub can_import: bool,
}

/// Arguments for execute_import command
#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteImportArgs {
    /// Session ID from analyze_import_package
    pub session_id: String,
}

/// Response for execute_import command
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportResultResponse {
    /// Session ID
    pub session_id: String,
    /// Import outcome
    pub status: ImportOutcome,
    /// Records successfully added
    pub added: RecordCounts,
    /// Records skipped (duplicates)
    pub skipped: RecordCounts,
    /// Images successfully imported
    pub images_imported: u32,
    /// Images that failed to import
    pub images_failed: Vec<ImageFailureDto>,
    /// Duration in milliseconds
    pub duration_ms: u64,
    /// Any warnings during import
    pub warnings: Vec<ImportWarning>,
}

/// Import outcome
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ImportOutcome {
    /// All operations succeeded
    Success,
    /// Import completed with some warnings
    SuccessWithWarnings,
    /// Import failed and was rolled back
    Failed { reason: String },
}

/// Details about a failed image import
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageFailureDto {
    pub filename: String,
    pub reason: String,
}

/// Arguments for cancel_import_session command
#[derive(Debug, Clone, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CancelImportSessionArgs {
    pub session_id: String,
}

/// Response for cancel_import_session command
#[derive(Debug, Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CancelImportSessionResponse {
    pub session_id: String,
    pub cancelled: bool,
}
