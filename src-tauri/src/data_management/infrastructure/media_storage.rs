use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Result of a single image import operation.
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImageImportResult {
    /// Original filename from the archive
    pub original_name: String,
    /// Stored filename with UUID prefix
    pub stored_name: String,
    /// Whether the import was successful
    pub success: bool,
    /// Error message if import failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Error type for media storage operations.
#[derive(Debug, Clone)]
pub enum MediaStorageError {
    /// I/O error during file operations
    IoError(String),
    /// Invalid file extension
    InvalidExtension(String),
    /// File not found in archive
    NotFound(String),
    /// Directory creation failed
    DirectoryError(String),
}

impl std::fmt::Display for MediaStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "I/O error: {}", msg),
            Self::InvalidExtension(msg) => write!(f, "Invalid file extension: {}", msg),
            Self::NotFound(msg) => write!(f, "File not found: {}", msg),
            Self::DirectoryError(msg) => write!(f, "Directory error: {}", msg),
        }
    }
}

impl std::error::Error for MediaStorageError {}

/// Manages image storage for imported data with collision avoidance.
#[derive(Debug)]
pub struct MediaStorage {
    /// Root directory where images are stored
    storage_dir: PathBuf,
}

impl MediaStorage {
    /// Create a new media storage instance.
    ///
    /// The `storage_dir` should be the application's data directory
    /// (typically `state.models_dir()`).
    pub fn new(storage_dir: PathBuf) -> Self {
        Self { storage_dir }
    }

    /// Copy an image file from source to the media storage directory.
    ///
    /// Applies UUID-prefix collision avoidance to the filename.
    /// Valid extensions: .png, .jpg, .jpeg
    ///
    /// # Arguments
    /// * `source_bytes` - The raw file data from the archive
    /// * `original_filename` - The original filename from the archive
    ///
    /// # Returns
    /// The stored filename (with UUID prefix) if successful.
    ///
    /// # Errors
    /// Returns `MediaStorageError` if file validation fails or I/O fails.
    pub async fn import_image(
        &self,
        source_bytes: &[u8],
        original_filename: &str,
    ) -> Result<String, MediaStorageError> {
        // Validate file extension
        Self::validate_extension(original_filename)?;

        // Create storage directory if needed
        tokio::fs::create_dir_all(&self.storage_dir)
            .await
            .map_err(|e| MediaStorageError::DirectoryError(e.to_string()))?;

        // Generate UUID-prefixed filename to avoid collisions
        let stored_filename = Self::generate_safe_filename(original_filename);

        // Write file to storage
        let file_path = self.storage_dir.join(&stored_filename);
        tokio::fs::write(&file_path, source_bytes)
            .await
            .map_err(|e| MediaStorageError::IoError(e.to_string()))?;

        Ok(stored_filename)
    }

    /// Import multiple images and collect results.
    ///
    /// Continues on individual failures and returns all results.
    pub async fn import_images(&self, images: Vec<(String, Vec<u8>)>) -> Vec<ImageImportResult> {
        let mut results = Vec::new();

        for (filename, data) in images {
            let result = match self.import_image(&data, &filename).await {
                Ok(stored_name) => ImageImportResult {
                    original_name: filename.clone(),
                    stored_name,
                    success: true,
                    error: None,
                },
                Err(e) => ImageImportResult {
                    original_name: filename.clone(),
                    stored_name: String::new(),
                    success: false,
                    error: Some(e.to_string()),
                },
            };
            results.push(result);
        }

        results
    }

    /// Check if a file exists in the storage directory.
    pub async fn file_exists(&self, filename: &str) -> bool {
        let path = self.storage_dir.join(filename);
        path.exists() && path.is_file()
    }

    /// Delete an image file from storage.
    pub async fn delete_image(&self, filename: &str) -> Result<(), MediaStorageError> {
        let path = self.storage_dir.join(filename);
        if !path.exists() {
            return Err(MediaStorageError::NotFound(filename.to_string()));
        }
        tokio::fs::remove_file(&path)
            .await
            .map_err(|e| MediaStorageError::IoError(e.to_string()))
    }

    /// Validate file extension.
    fn validate_extension(filename: &str) -> Result<(), MediaStorageError> {
        let path = Path::new(filename);
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        match ext.as_deref() {
            Some("png") | Some("jpg") | Some("jpeg") => Ok(()),
            Some(ext) => Err(MediaStorageError::InvalidExtension(format!(
                "'.{}' is not allowed. Use .png, .jpg, or .jpeg",
                ext
            ))),
            None => Err(MediaStorageError::InvalidExtension(
                "File has no extension".to_string(),
            )),
        }
    }

    /// Generate a UUID-prefixed filename to avoid collisions.
    fn generate_safe_filename(original: &str) -> String {
        let uuid = Uuid::new_v4().to_string();
        // Take first 8 chars of UUID
        let prefix = &uuid[..8];

        // Get the extension from the original filename
        let path = Path::new(original);
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("png");

        format!("{}_{}.{}", prefix, file_stem, extension)
    }

    /// Get the full path for a stored image.
    pub fn get_path(&self, filename: &str) -> PathBuf {
        self.storage_dir.join(filename)
    }

    /// Get the storage directory path.
    pub fn storage_dir(&self) -> &Path {
        &self.storage_dir
    }
}

impl Default for MediaStorage {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_extension_valid() {
        assert!(MediaStorage::validate_extension("image.png").is_ok());
        assert!(MediaStorage::validate_extension("image.jpg").is_ok());
        assert!(MediaStorage::validate_extension("image.jpeg").is_ok());
        assert!(MediaStorage::validate_extension("PHOTO.PNG").is_ok());
    }

    #[test]
    fn test_validate_extension_invalid() {
        assert!(MediaStorage::validate_extension("image.gif").is_err());
        assert!(MediaStorage::validate_extension("image.bmp").is_err());
        assert!(MediaStorage::validate_extension("imagefile").is_err());
    }

    #[test]
    fn test_generate_safe_filename() {
        let filename = MediaStorage::generate_safe_filename("photo.jpg");
        // Should have format: {uuid}_{name}.{ext}
        assert!(filename.contains("_photo.jpg"));
        // Should have 8-char UUID prefix
        let parts: Vec<&str> = filename.split('_').collect();
        assert_eq!(parts[0].len(), 8);
    }

    #[test]
    fn test_generate_safe_filename_collision_avoidance() {
        let filename1 = MediaStorage::generate_safe_filename("photo.jpg");
        let filename2 = MediaStorage::generate_safe_filename("photo.jpg");
        // Filenames should be different (UUID prefix differs)
        assert_ne!(filename1, filename2);
        // But both should have the same base name
        assert!(filename1.ends_with("_photo.jpg"));
        assert!(filename2.ends_with("_photo.jpg"));
    }
}
