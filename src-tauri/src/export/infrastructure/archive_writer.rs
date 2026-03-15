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
    media_files: Vec<PathBuf>,
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
    for source_path in &media_files {
        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| ExportError::ArchiveError("Invalid media filename".to_string()))?;

        let archive_entry = format!("images/{}", file_name);
        let options = zip::write::FileOptions::default();
        zip.start_file(&archive_entry, options)?;

        let data = std::fs::read(source_path)?;
        zip.write_all(&data)?;
    }

    // Finish writing
    zip.finish()?;

    Ok(archive_path)
}
