use crate::catalog::domain::railway_model::RailwayModelId;
use crate::media::domain::{ImageError, RailwayModelImage};
use crate::media::infrastructure::ImageRepository;
use std::path::Path;

/// Use case for retrieving railway model images.
///
/// This use case coordinates domain logic and infrastructure services
/// to locate and return railway model images from the filesystem.
pub struct GetRailwayModelImage;

impl GetRailwayModelImage {
    /// Execute the use case to retrieve a railway model image.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The railway model identifier
    /// * `models_dir` - Base directory containing model images
    ///
    /// # Returns
    ///
    /// Returns a `RailwayModelImage` if found, or an `ImageError` if:
    /// - The image file doesn't exist
    /// - Path validation fails
    /// - I/O errors occur
    ///
    /// # Errors
    ///
    /// - `ImageError::NotFound` - No image file found for the model
    /// - `ImageError::InvalidPath` - Path validation failed
    /// - `ImageError::IoError` - Filesystem access error
    pub async fn execute(
        &self,
        model_id: &RailwayModelId,
        models_dir: &Path,
    ) -> Result<RailwayModelImage, ImageError> {
        // Create base image entity
        let image = RailwayModelImage::from_model_id(model_id, models_dir)?;

        // Use infrastructure to find the actual file
        let repository = ImageRepository;
        let path = repository.find_image(model_id, models_dir).await?;

        // Update entity with found path
        Ok(image.with_path_and_exists(path, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_execute_image_found() {
        // Create temporary directory with test image
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:roco:43210").unwrap();
        let filename = format!("{}.png", RailwayModelImage::resolve_filename(&model_id));
        let image_path = models_dir.join(&filename);

        fs::write(&image_path, b"fake image data").unwrap();

        // Execute use case
        let use_case = GetRailwayModelImage;
        let result = use_case.execute(&model_id, models_dir).await;

        assert!(result.is_ok());
        let image = result.unwrap();
        assert!(image.exists());
        assert_eq!(image.path(), &image_path);
    }

    #[tokio::test]
    async fn test_execute_image_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let models_dir = temp_dir.path();

        let model_id = RailwayModelId::try_from("trn:railway-model:nonexistent:999").unwrap();

        let use_case = GetRailwayModelImage;
        let result = use_case.execute(&model_id, models_dir).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            ImageError::NotFound(_) => {} // Expected
            other => panic!("Expected NotFound, got {:?}", other),
        }
    }
}
