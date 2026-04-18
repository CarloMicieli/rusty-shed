use async_trait::async_trait;
use serde_json::Value;
use std::path::{Path, PathBuf};

use crate::data_management::domain::{ExportEntitySelection, ExportError};

/// Repository contract for export read-model data access.
///
/// The application export use case depends on this abstraction instead of a
/// concrete database implementation.
#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ExportRepository: Send + Sync {
    /// Build the full export manifest for the selected entities.
    async fn build_manifest(
        &self,
        selection: &ExportEntitySelection,
        media_dir: &Path,
    ) -> Result<Value, ExportError>;

    /// Collect media files that must be included in the exported archive.
    async fn collect_media_files(
        &self,
        selection: &ExportEntitySelection,
        media_dir: &Path,
    ) -> Result<Vec<PathBuf>, ExportError>;
}
