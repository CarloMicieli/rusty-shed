/// ZIP archive writer for export
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::ZipWriter;

use crate::export::domain::error::ExportError;

/// Write manifest and media to ZIP archive
///
/// # Arguments
/// * `destination_path` - Directory to create archive in
/// * `manifest` - Manifest JSON to include
/// * `media_files` - List of image files to include
/// * `filename` - Archive filename
///
/// # Returns
/// Path to created archive
pub async fn create_archive(
    destination_path: &Path,
    manifest: &Value,
    _media_files: Vec<PathBuf>,
    filename: &str,
) -> Result<PathBuf, ExportError> {
    let archive_path = destination_path.join(filename);

    // Create ZIP file
    let file = File::create(&archive_path)?;

    let mut zip = ZipWriter::new(file);

    // Add manifest.json
    let options = zip::write::FileOptions::default();
    zip.start_file("manifest.json", options)?;

    let manifest_json = serde_json::to_string_pretty(manifest)
        .map_err(|e| ExportError::ArchiveError(e.to_string()))?;
    zip.write_all(manifest_json.as_bytes())?;

    // Add media files to /images/ folder in archive
    // Note: In full implementation, iterate through media_files and add each with progress tracking
    // For now, we create the /images/ folder placeholder
    let options = zip::write::FileOptions::default();
    zip.start_file("images/.gitkeep", options)?;
    zip.write_all(b"")?;

    // Finish writing
    zip.finish()?;

    Ok(archive_path)
}
