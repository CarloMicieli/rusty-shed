/// ZIP archive writer for export
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::ZipWriter;

use crate::data_management::domain::ExportError;

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

    // Serialize the manifest once before entering spawn_blocking
    let manifest_json = serde_json::to_string_pretty(manifest)
        .map_err(|e| ExportError::ArchiveError(e.to_string()))?;

    // Wrap all synchronous ZIP/file I/O in spawn_blocking to avoid blocking the async runtime.
    let archive_path_clone = archive_path.clone();
    tokio::task::spawn_blocking(move || -> Result<(), ExportError> {
        let file = File::create(&archive_path_clone)?;
        let mut zip = ZipWriter::new(file);

        // Add manifest.json
        let options = zip::write::FileOptions::default();
        zip.start_file("manifest.json", options)?;
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

        zip.finish()?;
        Ok(())
    })
    .await
    .map_err(|e| ExportError::ArchiveError(format!("spawn_blocking error: {}", e)))??;

    Ok(archive_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Read;
    use zip::ZipArchive;

    fn make_selection() -> serde_json::Value {
        json!({ "data": {} })
    }

    #[tokio::test]
    async fn test_create_archive_manifest_only() {
        let dir = tempfile::tempdir().expect("tempdir");
        let manifest = make_selection();

        let path = create_archive(dir.path(), &manifest, vec![], "test.zip")
            .await
            .expect("create_archive");

        assert!(path.exists());
        assert_eq!(path.file_name().and_then(|n| n.to_str()), Some("test.zip"));
    }

    #[tokio::test]
    async fn test_created_zip_contains_manifest_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        let manifest = json!({ "version": "1.0" });

        let path = create_archive(dir.path(), &manifest, vec![], "out.zip")
            .await
            .expect("create_archive");

        let file = std::fs::File::open(&path).expect("open zip");
        let mut archive = ZipArchive::new(file).expect("ZipArchive");
        let entry = archive.by_name("manifest.json");
        assert!(entry.is_ok(), "manifest.json must be present in ZIP");
    }

    #[tokio::test]
    async fn test_manifest_json_content_is_correct() {
        let dir = tempfile::tempdir().expect("tempdir");
        let manifest = json!({ "version": "1.0", "source": "rusty-shed" });

        let path = create_archive(dir.path(), &manifest, vec![], "out.zip")
            .await
            .expect("create_archive");

        let file = std::fs::File::open(&path).expect("open zip");
        let mut archive = ZipArchive::new(file).expect("ZipArchive");
        let mut entry = archive.by_name("manifest.json").expect("manifest.json");
        let mut content = String::new();
        entry.read_to_string(&mut content).expect("read");
        let parsed: serde_json::Value =
            serde_json::from_str(&content).expect("valid JSON in manifest.json");
        assert_eq!(parsed["version"], json!("1.0"));
    }

    #[tokio::test]
    async fn test_create_archive_with_media_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let manifest = json!({ "data": {} });

        // Create a fake image file
        let img_path = dir.path().join("model.png");
        std::fs::write(&img_path, b"PNG_DATA").expect("write image");

        let path = create_archive(dir.path(), &manifest, vec![img_path], "export.zip")
            .await
            .expect("create_archive");

        let file = std::fs::File::open(&path).expect("open zip");
        let mut archive = ZipArchive::new(file).expect("ZipArchive");
        let entry = archive.by_name("images/model.png");
        assert!(entry.is_ok(), "images/model.png must be present in ZIP");
    }

    #[tokio::test]
    async fn test_created_archive_path_matches_requested_filename() {
        let dir = tempfile::tempdir().expect("tempdir");
        let manifest = json!({});
        let filename = "rusty-shed-2026-03-15.zip";

        let path = create_archive(dir.path(), &manifest, vec![], filename)
            .await
            .expect("create_archive");

        assert_eq!(
            path,
            dir.path().join(filename),
            "returned path must match destination_path + filename"
        );
    }
}
