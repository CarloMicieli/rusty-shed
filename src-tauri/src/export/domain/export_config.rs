/// Export configuration value object
use serde::{Deserialize, Serialize};

/// Configuration for an export operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Destination path for the archive
    pub destination_path: String,
    /// Custom filename (optional)
    pub custom_filename: Option<String>,
    /// Include orphaned images in export
    pub include_orphaned_images: bool,
}

impl ExportConfig {
    /// Create a new export configuration
    pub fn new(destination_path: String) -> Self {
        ExportConfig {
            destination_path,
            custom_filename: None,
            include_orphaned_images: false,
        }
    }

    /// Set custom filename
    pub fn with_filename(mut self, filename: String) -> Self {
        self.custom_filename = Some(filename);
        self
    }

    /// Set whether to include orphaned images
    pub fn with_orphaned_images(mut self, include: bool) -> Self {
        self.include_orphaned_images = include;
        self
    }
}
