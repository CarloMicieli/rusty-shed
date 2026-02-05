use crate::import::domain::{ImportWarning, RecordCounts};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::time::Duration;

/// Result of a completed import operation.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    /// Session that was executed
    pub session_id: String,
    /// Records successfully added
    pub added: RecordCounts,
    /// Records skipped (duplicates)
    pub skipped: RecordCounts,
    /// Images successfully imported
    pub images_imported: u32,
    /// Images that failed (with reasons)
    pub images_failed: Vec<ImageFailure>,
    /// Total execution duration in milliseconds
    pub duration_ms: u64,
    /// Any warnings during import
    pub warnings: Vec<ImportWarning>,
}

/// Details about a failed image import.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageFailure {
    pub filename: String,
    pub reason: String,
}

impl ImageFailure {
    /// Create a new image failure.
    pub fn new(filename: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            reason: reason.into(),
        }
    }
}

impl ImportResult {
    /// Create a new import result.
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            added: RecordCounts::new(),
            skipped: RecordCounts::new(),
            images_imported: 0,
            images_failed: Vec::new(),
            duration_ms: 0,
            warnings: Vec::new(),
        }
    }

    /// Set the duration from a `Duration`.
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration_ms = duration.as_millis() as u64;
        self
    }

    /// Add an image failure.
    pub fn add_image_failure(&mut self, filename: impl Into<String>, reason: impl Into<String>) {
        self.images_failed.push(ImageFailure::new(filename, reason));
    }

    /// Add a warning.
    pub fn add_warning(&mut self, warning: ImportWarning) {
        self.warnings.push(warning);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_failure_creation() {
        let failure = ImageFailure::new("test.jpg", "Not found");
        assert_eq!(failure.filename, "test.jpg");
        assert_eq!(failure.reason, "Not found");
    }

    #[test]
    fn test_import_result_creation() {
        let result = ImportResult::new("session-1");
        assert_eq!(result.session_id, "session-1");
        assert_eq!(result.added.total(), 0);
    }

    #[test]
    fn test_import_result_with_duration() {
        let duration = Duration::from_secs(5);
        let result = ImportResult::new("session-1").with_duration(duration);
        assert_eq!(result.duration_ms, 5000);
    }

    #[test]
    fn test_import_result_add_failures() {
        let mut result = ImportResult::new("session-1");
        result.add_image_failure("test.jpg", "Not found");
        result.add_image_failure("test2.jpg", "Corrupted");
        assert_eq!(result.images_failed.len(), 2);
    }
}
