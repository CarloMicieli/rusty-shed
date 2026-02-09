/// Export result value object
use serde::{Deserialize, Serialize};

/// Represents the result of an export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    /// Path to the created archive
    pub archive_path: String,
    /// File size in bytes
    pub file_size_bytes: u64,
    /// Number of records exported
    pub records_exported: u32,
    /// Any warnings during export
    pub warnings: Vec<String>,
}

impl ExportResult {
    /// Create a new export result
    pub fn new(archive_path: String, file_size_bytes: u64, records_exported: u32) -> Self {
        ExportResult {
            archive_path,
            file_size_bytes,
            records_exported,
            warnings: Vec::new(),
        }
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}
