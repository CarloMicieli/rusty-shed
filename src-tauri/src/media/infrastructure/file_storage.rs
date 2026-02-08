//! File storage infrastructure for model images
//!
//! This module provides filesystem operations for storing, retrieving,
//! and deleting model images in the application's data directory.

use crate::media::domain::image_validation::{ModelImagePath, StorageError};
use std::path::{Path, PathBuf};
use tokio::fs;

/// Infrastructure service for file storage operations
pub struct FileStorage {
    storage_dir: PathBuf,
}

impl FileStorage {
    /// Create a new FileStorage instance
    ///
    /// Initializes the storage directory and ensures it's writable
    pub fn new(storage_dir: PathBuf) -> Result<Self, StorageError> {
        // Ensure directory exists
        std::fs::create_dir_all(&storage_dir).map_err(|e| {
            StorageError::DirectoryCreation(format!("{}: {}", storage_dir.display(), e))
        })?;

        // Check writability by attempting to create a temporary file
        let test_file = storage_dir.join(".write_test");
        std::fs::write(&test_file, b"test").map_err(|e| {
            StorageError::DirectoryCreation(format!("Directory not writable: {}", e))
        })?;
        std::fs::remove_file(&test_file).ok(); // Clean up test file

        Ok(Self { storage_dir })
    }

    /// Get the storage directory path
    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }

    /// Copy an image from source path to destination in storage
    pub async fn copy_image(
        &self,
        source: &Path,
        dest: &ModelImagePath,
    ) -> Result<(), StorageError> {
        fs::copy(source, dest.full_path()).await.map_err(|e| {
            StorageError::CopyFailed(format!(
                "Failed to copy from {} to {}: {}",
                source.display(),
                dest.full_path().display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Write image bytes to storage
    pub async fn write_image(
        &self,
        data: &[u8],
        dest: &ModelImagePath,
    ) -> Result<(), StorageError> {
        fs::write(dest.full_path(), data).await.map_err(|e| {
            StorageError::WriteFailed(format!(
                "Failed to write to {}: {}",
                dest.full_path().display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Delete an image from storage
    pub async fn delete_image(&self, path: &ModelImagePath) -> Result<(), StorageError> {
        if !path.exists() {
            return Err(StorageError::FileNotFound(
                path.full_path().display().to_string(),
            ));
        }

        fs::remove_file(path.full_path()).await.map_err(|e| {
            StorageError::DeleteFailed(format!(
                "Failed to delete {}: {}",
                path.full_path().display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Check if an image exists at the given path
    pub fn exists(&self, path: &ModelImagePath) -> bool {
        path.exists()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::media::domain::image_validation::ImageFormat;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(content).unwrap();
        path
    }

    #[test]
    fn test_file_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");

        let storage = FileStorage::new(storage_dir.clone());
        assert!(storage.is_ok());
        assert!(storage_dir.exists());
    }

    #[test]
    fn test_file_storage_already_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        std::fs::create_dir_all(&storage_dir).unwrap();

        let storage = FileStorage::new(storage_dir.clone());
        assert!(storage.is_ok());
    }

    #[test]
    fn test_storage_dir_getter() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");

        let storage = FileStorage::new(storage_dir.clone()).unwrap();
        assert_eq!(storage.storage_dir(), storage_dir.as_path());
    }

    #[tokio::test]
    async fn test_copy_image() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        // Create source file
        let source = create_test_file(temp_dir.path(), "source.jpg", b"test image data");

        // Create destination path
        let dest = ModelImagePath::new(&storage_dir, "marklin:39216", ImageFormat::Jpeg);

        // Copy file
        let result = storage.copy_image(&source, &dest).await;
        assert!(result.is_ok());
        assert!(dest.exists());

        // Verify content
        let content = std::fs::read(dest.full_path()).unwrap();
        assert_eq!(content, b"test image data");
    }

    #[tokio::test]
    async fn test_copy_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let source = temp_dir.path().join("nonexistent.jpg");
        let dest = ModelImagePath::new(&storage_dir, "marklin:39216", ImageFormat::Jpeg);

        let result = storage.copy_image(&source, &dest).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StorageError::CopyFailed(_))));
    }

    #[tokio::test]
    async fn test_write_image() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let dest = ModelImagePath::new(&storage_dir, "roco:12345", ImageFormat::Png);
        let data = b"image bytes";

        let result = storage.write_image(data, &dest).await;
        assert!(result.is_ok());
        assert!(dest.exists());

        let content = std::fs::read(dest.full_path()).unwrap();
        assert_eq!(content, b"image bytes");
    }

    #[tokio::test]
    async fn test_delete_image() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        // Create a file first
        let dest = ModelImagePath::new(&storage_dir, "fleischmann:4321", ImageFormat::WebP);
        std::fs::write(dest.full_path(), b"test").unwrap();
        assert!(dest.exists());

        // Delete it
        let result = storage.delete_image(&dest).await;
        assert!(result.is_ok());
        assert!(!dest.exists());
    }

    #[tokio::test]
    async fn test_delete_nonexistent_image() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let dest = ModelImagePath::new(&storage_dir, "nonexistent:999", ImageFormat::Jpeg);

        let result = storage.delete_image(&dest).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StorageError::FileNotFound(_))));
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let path = ModelImagePath::new(&storage_dir, "test:123", ImageFormat::Jpeg);

        // File doesn't exist yet
        assert!(!storage.exists(&path));

        // Create file
        std::fs::write(path.full_path(), b"test").unwrap();

        // Now it exists
        assert!(storage.exists(&path));
    }
}
