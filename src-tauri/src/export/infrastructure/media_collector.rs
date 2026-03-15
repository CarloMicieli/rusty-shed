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

const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg"];

/// Collect media files for export.
///
/// Scans the `media_dir` for image files referenced by the selected entities.
/// For MVP: returns all image files in the media directory.
///
/// # Arguments
/// * `_pool` - Database connection pool (reserved for future FK-based filtering)
/// * `_selection` - Selected entity types (reserved for future selective collection)
/// * `media_dir` - The app's media/models directory where images are stored
pub async fn collect_media_files(
    _pool: &SqlitePool,
    _selection: &ExportEntitySelection,
    media_dir: &std::path::Path,
) -> Result<Vec<PathBuf>, ExportError> {
    if !media_dir.exists() {
        return Ok(Vec::new());
    }

    let mut image_files = Vec::new();

    let mut read_dir = tokio::fs::read_dir(media_dir)
        .await
        .map_err(ExportError::IoError)?;

    while let Some(entry) = read_dir.next_entry().await.map_err(ExportError::IoError)? {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if let Some(ext) = extension
            && IMAGE_EXTENSIONS.contains(&ext.as_str())
        {
            image_files.push(path);
        }
    }

    Ok(image_files)
}

/// Detect orphaned images (not referenced by any record).
/// MVP stub — returns empty list.
pub async fn detect_orphaned_images() -> Result<Vec<MediaFile>, ExportError> {
    Ok(Vec::new())
}
