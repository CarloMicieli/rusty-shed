/// Media file collector for export
use sqlx::SqlitePool;
use std::path::PathBuf;

use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;

/// Represents a media file to be included in export
#[derive(Debug, Clone)]
pub struct MediaFile {
    pub source_path: PathBuf,
    pub relative_path: String,
    pub size_bytes: u64,
}

/// Collect media files for export based on selected entities
pub async fn collect_media_files(
    _pool: &SqlitePool,
    _selection: &ExportEntitySelection,
    _working_directory: &std::path::Path,
) -> Result<Vec<PathBuf>, ExportError> {
    // This will be implemented in Phase 2 to collect actual image files
    // For now, return empty vec for compilation
    Ok(Vec::new())
}

/// Detect orphaned images (not referenced by any record)
pub async fn detect_orphaned_images() -> Result<Vec<MediaFile>, ExportError> {
    // This will be implemented in Phase 2
    Ok(Vec::new())
}
