use sqlx::SqlitePool;
use std::path::PathBuf;

use crate::data_management::domain::{ExportEntitySelection, ExportError};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_management::domain::ExportEntitySelection;
    use sqlx::SqlitePool;

    fn all_selection() -> ExportEntitySelection {
        ExportEntitySelection {
            include_railway_models: true,
            include_collection_items: true,
            include_sellers: true,
            include_maintenance_logs: true,
            include_dcc_roster: true,
            include_orphaned_images: false,
            include_track_inventory: true,
            include_train_formations: false,
            include_wishlists: false,
        }
    }

    async fn make_pool() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        pool
    }

    #[tokio::test]
    async fn test_empty_media_dir_returns_empty_vec() {
        let pool = make_pool().await;
        let dir = tempfile::tempdir().expect("tempdir");
        let selection = all_selection();

        let files = collect_media_files(&pool, &selection, dir.path())
            .await
            .expect("collect");

        assert!(files.is_empty());
    }

    #[tokio::test]
    async fn test_collects_png_jpg_jpeg_files() {
        let pool = make_pool().await;
        let dir = tempfile::tempdir().expect("tempdir");
        let selection = all_selection();

        std::fs::write(dir.path().join("a.png"), b"PNG").expect("write");
        std::fs::write(dir.path().join("b.jpg"), b"JPG").expect("write");
        std::fs::write(dir.path().join("c.jpeg"), b"JPEG").expect("write");

        let mut files = collect_media_files(&pool, &selection, dir.path())
            .await
            .expect("collect");
        files.sort();

        let names: Vec<&str> = files
            .iter()
            .filter_map(|p| p.file_name()?.to_str())
            .collect();
        assert!(names.contains(&"a.png"));
        assert!(names.contains(&"b.jpg"));
        assert!(names.contains(&"c.jpeg"));
        assert_eq!(files.len(), 3);
    }

    #[tokio::test]
    async fn test_ignores_non_image_files() {
        let pool = make_pool().await;
        let dir = tempfile::tempdir().expect("tempdir");
        let selection = all_selection();

        std::fs::write(dir.path().join("readme.txt"), b"text").expect("write");
        std::fs::write(dir.path().join("data.json"), b"{}").expect("write");
        std::fs::write(dir.path().join("image.png"), b"PNG").expect("write");

        let files = collect_media_files(&pool, &selection, dir.path())
            .await
            .expect("collect");

        assert_eq!(files.len(), 1);
        assert_eq!(
            files[0].file_name().and_then(|n| n.to_str()),
            Some("image.png")
        );
    }

    #[tokio::test]
    async fn test_nonexistent_media_dir_returns_empty() {
        let pool = make_pool().await;
        let dir = tempfile::tempdir().expect("tempdir");
        let selection = all_selection();
        let nonexistent = dir.path().join("does_not_exist");

        let files = collect_media_files(&pool, &selection, &nonexistent)
            .await
            .expect("collect");

        assert!(files.is_empty());
    }
}
