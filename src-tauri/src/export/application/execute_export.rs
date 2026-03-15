/// Execute export use case
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};

use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;
use crate::export::domain::export_result::ExportResult;
use crate::export::infrastructure::{
    archive_writer, disk_space_checker, manifest_builder, media_collector,
};

/// Execute the export operation.
///
/// # Arguments
/// * `pool` - Database connection pool
/// * `archive_path` - Full path of the output ZIP file (e.g. `/home/user/Desktop/rusty-shed-export-2026-03-15.zip`)
/// * `media_dir` - App media/models directory where images are stored
/// * `selection` - Entity types to include in the export
pub async fn export_to_archive(
    pool: &SqlitePool,
    archive_path: &Path,
    media_dir: &Path,
    selection: &ExportEntitySelection,
) -> Result<ExportResult, ExportError> {
    // Validate entity selection
    if !selection.is_valid() {
        return Err(ExportError::NoDataToExport);
    }

    // Validate destination directory exists
    let dest_dir = archive_path
        .parent()
        .ok_or_else(|| ExportError::InvalidPath(archive_path.display().to_string()))?;

    if !dest_dir.exists() {
        return Err(ExportError::InvalidPath(dest_dir.display().to_string()));
    }

    // Check available disk space (100 MB estimated)
    const ESTIMATED_SIZE: u64 = 100 * 1024 * 1024;
    disk_space_checker::validate_disk_space(dest_dir, ESTIMATED_SIZE)?;

    // Build manifest from database
    let manifest = manifest_builder::build_manifest(pool, selection, media_dir).await?;

    // Collect media files
    let media_files = media_collector::collect_media_files(pool, selection, media_dir).await?;

    // Extract directory and filename from archive_path
    let filename = archive_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("rusty-shed-export.zip");

    // Create archive
    let created_path =
        archive_writer::create_archive(dest_dir, &manifest, media_files, filename).await?;

    let file_size = std::fs::metadata(&created_path)?.len();

    Ok(ExportResult::new(
        created_path.display().to_string(),
        file_size,
        selection.get_entity_count(),
    ))
}

/// Get temporary file path for export
pub fn get_temp_export_path() -> Result<PathBuf, ExportError> {
    let temp_dir = std::env::temp_dir();
    let filename = format!("rusty-shed-export-{}.zip", uuid::Uuid::new_v4());
    Ok(temp_dir.join(filename))
}
