/// Execute export use case
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};

use crate::export::domain::entity_selection::ExportEntitySelection;
use crate::export::domain::error::ExportError;
use crate::export::domain::export_config::ExportConfig;
use crate::export::domain::export_result::ExportResult;
use crate::export::infrastructure::{
    archive_writer, disk_space_checker, manifest_builder, media_collector,
};

/// Execute the export operation
pub async fn export_to_archive(
    pool: &SqlitePool,
    config: &ExportConfig,
    selection: &ExportEntitySelection,
) -> Result<ExportResult, ExportError> {
    // Step 1: Validate destination path exists
    let dest_path = Path::new(&config.destination_path);
    if !dest_path.exists() {
        return Err(ExportError::InvalidPath(dest_path.display().to_string()));
    }

    // Step 2: Validate entity selection
    if !selection.is_valid() {
        return Err(ExportError::NoDataToExport);
    }

    // Step 3: Check available disk space (100 MB estimated for now)
    const ESTIMATED_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
    disk_space_checker::validate_disk_space(dest_path, ESTIMATED_SIZE)?;

    // Step 4: Build manifest from database
    let manifest = manifest_builder::build_manifest(pool, selection).await?;

    // Step 5: Collect media files
    let media_files =
        media_collector::collect_media_files(pool, selection, &PathBuf::from("/tmp")).await?;

    // Step 6: Create archive
    let filename = format!(
        "rusty-shed-export-{}.zip",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    );

    let archive_path =
        archive_writer::create_archive(dest_path, &manifest, media_files, &filename).await?;

    let file_size = std::fs::metadata(&archive_path)?.len();

    Ok(ExportResult::new(
        archive_path.display().to_string(),
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
