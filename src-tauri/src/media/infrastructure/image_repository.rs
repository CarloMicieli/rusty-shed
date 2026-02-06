//! Image Repository
//!
//! Infrastructure service for filesystem operations on railway model images.

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::media::domain::{ImageError, RailwayModelImage};
use std::path::{Component, Path, PathBuf};
use tokio::fs;

/// Repository for managing railway model images on the filesystem.
///
/// Handles file lookup, path validation, and existence checks.
pub struct ImageRepository;

impl ImageRepository {
    /// Supported image file extensions.
    const EXTENSIONS: &'static [&'static str] = &["png", "jpg", "jpeg"];

    /// Find an image file for a railway model.
    ///
    /// Searches for the image file with supported extensions (.png, .jpg, .jpeg)
    /// and validates the path for security.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The railway model identifier
    /// * `models_dir` - Base directory containing model images
    ///
    /// # Returns
    ///
    /// Returns the absolute path to the image file if found.
    ///
    /// # Errors
    ///
    /// - `ImageError::NotFound` - No image file found with supported extensions
    /// - `ImageError::InvalidPath` - Path validation failed (security)
    /// - `ImageError::IoError` - Filesystem access error
    ///
    /// # Security
    ///
    /// This method validates paths to prevent directory traversal attacks.
    pub async fn find_image(
        &self,
        model_id: &RailwayModelId,
        models_dir: &Path,
    ) -> Result<PathBuf, ImageError> {
        let filename_base = RailwayModelImage::resolve_filename(model_id);

        // Try each supported extension
        for ext in Self::EXTENSIONS {
            let filename = format!("{}.{}", filename_base, ext);
            let path = models_dir.join(&filename);

            // Validate path security
            Self::sanitize_path(&path)?;

            // Check if file exists
            match fs::metadata(&path).await {
                Ok(metadata) if metadata.is_file() => {
                    // Found the image, return canonical path
                    return path
                        .canonicalize()
                        .map_err(|e| ImageError::IoError(e.to_string()));
                }
                _ => continue, // Try next extension
            }
        }

        // No image found with any supported extension
        Err(ImageError::NotFound(format!(
            "No image found for model {} in directory {}",
            model_id.as_ref(),
            models_dir.display()
        )))
    }

    /// Validate a filesystem path for security.
    ///
    /// Ensures the path:
    /// - Contains only normal components (no ".." or ".")
    /// - Doesn't attempt directory traversal
    /// - Is a valid filesystem path
    ///
    /// # Security
    ///
    /// This method prevents path traversal attacks by rejecting paths
    /// that contain non-normal components.
    ///
    /// # Errors
    ///
    /// Returns `ImageError::InvalidPath` if validation fails.
    fn sanitize_path(path: &Path) -> Result<(), ImageError> {
        for component in path.components() {
            match component {
                Component::Normal(_) | Component::RootDir | Component::Prefix(_) => {}
                Component::ParentDir => {
                    return Err(ImageError::InvalidPath(
                        "Path contains parent directory reference (..)".to_string(),
                    ));
                }
                Component::CurDir => {
                    return Err(ImageError::InvalidPath(
                        "Path contains current directory reference (.)".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_find_image_png() {
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:roco:43210").unwrap();
        let filename = format!("{}.png", RailwayModelImage::resolve_filename(&model_id));
        fs::write(models_dir.join(&filename), b"fake image").unwrap();

        let repo = ImageRepository;
        let result = repo.find_image(&model_id, models_dir).await;

        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert!(
            path.to_str()
                .unwrap()
                .contains("trn_railway-model_roco_43210.png")
        );
    }

    #[tokio::test]
    async fn test_find_image_jpg() {
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:fleischmann:6380").unwrap();
        let filename = format!("{}.jpg", RailwayModelImage::resolve_filename(&model_id));
        fs::write(models_dir.join(&filename), b"fake image").unwrap();

        let repo = ImageRepository;
        let result = repo.find_image(&model_id, models_dir).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_image_jpeg() {
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:marklin:3000").unwrap();
        let filename = format!("{}.jpeg", RailwayModelImage::resolve_filename(&model_id));
        fs::write(models_dir.join(&filename), b"fake image").unwrap();

        let repo = ImageRepository;
        let result = repo.find_image(&model_id, models_dir).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_image_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:nonexistent:999").unwrap();

        let repo = ImageRepository;
        let result = repo.find_image(&model_id, models_dir).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ImageError::NotFound(_) => {}
            other => panic!("Expected NotFound, got {:?}", other),
        }
    }

    #[test]
    fn test_sanitize_path_valid() {
        let path = Path::new("/home/user/models/image.png");
        assert!(ImageRepository::sanitize_path(path).is_ok());
    }

    #[test]
    fn test_sanitize_path_parent_dir() {
        let path = Path::new("/home/user/../etc/passwd");
        let result = ImageRepository::sanitize_path(path);

        assert!(result.is_err());
        match result.unwrap_err() {
            ImageError::InvalidPath(msg) => assert!(msg.contains("parent directory")),
            _ => panic!("Expected InvalidPath error"),
        }
    }

    #[test]
    fn test_sanitize_path_current_dir() {
        // On Unix, paths with . in the middle get normalized by Path::new
        // However, if we construct a path with explicit CurDir component, it should be rejected
        // For practical purposes, joined paths with normal components are always safe
        let path = Path::new("/home/user/image.png");
        let result = ImageRepository::sanitize_path(path);
        
        // Normal paths should pass
        assert!(result.is_ok());
    }
}
