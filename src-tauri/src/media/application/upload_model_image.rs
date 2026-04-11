//! Upload Model Image Use Case
//!
//! Orchestrates the upload of model images from user-selected files.

use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;
use crate::media::domain::image_validation::{
    ImageFormat, ImageValidator, ModelImagePath, StorageError, ValidationError,
};
use crate::media::infrastructure::FileStorage;
use std::path::{Path, PathBuf};

/// Input for path-based image upload (file explorer selection)
#[derive(Debug, Clone)]
pub struct UploadImageInput {
    pub model_id: RailwayModelId,
    pub file_path: PathBuf,
}

/// Input for bytes-based image upload (drag & drop)
#[derive(Debug, Clone)]
pub struct UploadImageBytesInput {
    pub model_id: RailwayModelId,
    pub file_name: String,
    pub file_data: Vec<u8>,
}

// ============================================================================
// Internal Shared Trait
// ============================================================================

/// Internal trait that unifies the upload pipeline across path-based and bytes-based inputs.
///
/// Implementors supply a [`FileStorage`] reference via [`ImageUploader::storage`]. Both
/// default methods — [`clear_old_images`][ImageUploader::clear_old_images] and
/// [`process_upload`][ImageUploader::process_upload] — compose validation, cleanup, and
/// storage in one authoritative location. Adding a new supported format (e.g. AVIF)
/// only requires updating [`clear_old_images`][ImageUploader::clear_old_images].
#[async_trait::async_trait]
trait ImageUploader {
    /// Returns a reference to the file storage backend.
    fn storage(&self) -> &FileStorage;

    /// Deletes every known image format for `model_id`, silently skipping absent files.
    async fn clear_old_images(&self, model_id: &RailwayModelId) -> Result<(), UploadError> {
        for format in [ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
            let path = ModelImagePath::new(self.storage().storage_dir(), model_id.as_ref(), format);
            log::debug!("Deleting existing image: {}", path.full_path().display());
            match self.storage().delete_image(&path).await {
                Ok(()) => {}
                Err(StorageError::FileNotFound(_)) => {}
                Err(e) => return Err(UploadError::Storage(e)),
            }
        }
        Ok(())
    }

    /// Validates `source_path`, removes old images for `model_id`, and copies the file
    /// to storage — the core upload pipeline shared by both upload use cases.
    async fn process_upload(
        &self,
        model_id: &RailwayModelId,
        source_path: &Path,
    ) -> Result<(), UploadError> {
        let format = ImageValidator::validate(source_path).map_err(UploadError::Validation)?;
        let dest_path =
            ModelImagePath::new(self.storage().storage_dir(), model_id.as_ref(), format);
        self.clear_old_images(model_id).await?;
        log::debug!("Copying image to {}", dest_path.full_path().display());
        self.storage()
            .copy_image(source_path, &dest_path)
            .await
            .map_err(UploadError::Storage)?;
        Ok(())
    }
}

// ============================================================================
// Upload Model Image (Path-based)
// ============================================================================

/// Use case for uploading model images from file paths
pub struct UploadModelImage {
    storage: FileStorage,
}

impl UploadModelImage {
    /// Create a new instance with the given storage
    pub fn new(storage: FileStorage) -> Self {
        Self { storage }
    }

    /// Execute the upload use case (path-based).
    ///
    /// # Steps
    /// 1. Validate model exists.
    /// 2. Delegate to [`ImageUploader::process_upload`]: validate source → delete old → copy.
    ///
    /// # Errors
    /// - [`ValidationError`]: Invalid file format, size, or missing file.
    /// - [`StorageError`]: File operations failed.
    /// - [`DomainError::NotFound`]: Model doesn't exist.
    pub async fn execute<U>(
        &self,
        input: UploadImageInput,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        validate_model_exists(&input.model_id, unit_of_work).await?;
        self.process_upload(&input.model_id, &input.file_path)
            .await?;
        log::info!(
            "Successfully uploaded image for model {}",
            input.model_id.as_ref()
        );
        Ok(())
    }
}

#[async_trait::async_trait]
impl ImageUploader for UploadModelImage {
    fn storage(&self) -> &FileStorage {
        &self.storage
    }
}

// ============================================================================
// Upload Model Image Bytes (Drag & Drop)
// ============================================================================

/// Use case for uploading model images from bytes
pub struct UploadModelImageBytes {
    storage: FileStorage,
}

impl UploadModelImageBytes {
    /// Create a new instance with the given storage
    pub fn new(storage: FileStorage) -> Self {
        Self { storage }
    }

    /// Execute the upload use case (bytes-based).
    ///
    /// # Steps
    /// 1. Validate model exists.
    /// 2. Write bytes to a temporary file (auto-deleted on drop via RAII).
    /// 3. Delegate to [`ImageUploader::process_upload`]: validate source → delete old → copy.
    ///
    /// # Errors
    /// - [`ValidationError`]: Invalid file format, size, or corrupted data.
    /// - [`StorageError`]: File operations failed.
    /// - [`DomainError::NotFound`]: Model doesn't exist.
    pub async fn execute<U>(
        &self,
        input: UploadImageBytesInput,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        validate_model_exists(&input.model_id, unit_of_work).await?;

        // Write bytes to a temporary file; auto-deleted when `temp_file` is dropped
        let temp_file = tempfile::NamedTempFile::new().map_err(|e| {
            UploadError::Storage(StorageError::WriteFailed(format!(
                "Failed to create temp file: {}",
                e
            )))
        })?;
        tokio::fs::write(temp_file.path(), &input.file_data)
            .await
            .map_err(|e| {
                UploadError::Storage(StorageError::WriteFailed(format!(
                    "Failed to write temp file: {}",
                    e
                )))
            })?;

        self.process_upload(&input.model_id, temp_file.path())
            .await?;
        log::info!(
            "Successfully uploaded image for model {} from bytes",
            input.model_id.as_ref()
        );
        Ok(())
    }
}

#[async_trait::async_trait]
impl ImageUploader for UploadModelImageBytes {
    fn storage(&self) -> &FileStorage {
        &self.storage
    }
}

// ============================================================================
// Shared Helpers
// ============================================================================

/// Verify that the given model exists, returning [`UploadError::ModelNotFound`] when it does not.
async fn validate_model_exists<U>(
    model_id: &RailwayModelId,
    unit_of_work: &mut U,
) -> Result<(), UploadError>
where
    U: RailwayModelUowExt + Send,
{
    let mut repository = unit_of_work.railway_model_repository();
    let exists = repository
        .exists_by_id(model_id)
        .await
        .map_err(UploadError::Domain)?;

    if exists {
        Ok(())
    } else {
        Err(UploadError::ModelNotFound(model_id.as_ref().to_string()))
    }
}

/// Errors that can occur during image upload
#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::{
        MockRailwayModelRepository, RailwayModelRepository,
    };
    use crate::media::domain::image_validation::ImageFormat;
    use mockall::predicate::*;
    use std::io::Write;
    use tempfile::TempDir;

    // Mock UnitOfWork for testing
    #[derive(Default)]
    struct FakeUow {
        railway_models_repo: Option<MockRailwayModelRepository>,
    }

    impl FakeUow {
        fn with_railway_models_repo(railway_models_repo: MockRailwayModelRepository) -> Self {
            Self {
                railway_models_repo: Some(railway_models_repo),
            }
        }
    }

    impl RailwayModelUowExt for FakeUow {
        fn railway_model_repository(&mut self) -> Box<dyn RailwayModelRepository + '_> {
            Box::new(
                self.railway_models_repo
                    .take()
                    .expect("railway model repository already taken"),
            )
        }
    }

    fn create_valid_jpeg(dir: &std::path::Path, name: &str) -> PathBuf {
        create_valid_jpeg_with_content(dir, name, &[])
    }

    fn create_valid_jpeg_with_content(
        dir: &std::path::Path,
        name: &str,
        extra_data: &[u8],
    ) -> PathBuf {
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        // Write a minimal valid JPEG file
        let mut jpeg_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00,
        ];
        // Add extra data before the end marker to make files distinguishable
        jpeg_data.extend_from_slice(extra_data);
        // End marker
        jpeg_data.extend_from_slice(&[0xFF, 0xD9]);
        file.write_all(&jpeg_data).unwrap();
        path
    }

    fn create_valid_png(dir: &std::path::Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        // Write a minimal valid PNG file
        let png_header = [
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00,
            0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F, 0x00, 0x05, 0xFE, 0x02, 0xFE, 0xDC, 0xCC, 0x59,
            0xE7, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
        ];
        file.write_all(&png_header).unwrap();
        path
    }

    fn create_invalid_file(dir: &std::path::Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = std::fs::File::create(&path).unwrap();
        file.write_all(b"This is not an image file").unwrap();
        path
    }

    #[tokio::test]
    async fn test_upload_image_success() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        // Create a valid JPEG source file
        let source_file = create_valid_jpeg(temp_dir.path(), "test_image.jpg");

        let model_id_str = "trn:railway-model:marklin:39216";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Setup mock repository to confirm model exists
        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        // Execute the use case
        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id,
            file_path: source_file.clone(),
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());

        // Verify the file was copied to the storage directory
        let dest_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        assert!(dest_path.exists());
    }

    #[tokio::test]
    async fn test_upload_image_model_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let source_file = create_valid_jpeg(temp_dir.path(), "test_image.jpg");

        let model_id_str = "trn:railway-model:nonexistent:999";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Setup mock repository to report model not found
        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(false));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id,
            file_path: source_file,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(UploadError::ModelNotFound(_))));
    }

    #[tokio::test]
    async fn test_upload_image_invalid_format() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let invalid_file = create_invalid_file(temp_dir.path(), "not_an_image.txt");

        let model_id_str = "trn:railway-model:marklin:39216";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id,
            file_path: invalid_file,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(UploadError::Validation(_))));
    }

    #[tokio::test]
    async fn test_upload_image_replaces_existing() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:marklin:39216";

        // Create existing image
        let existing_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        std::fs::write(existing_path.full_path(), b"old image").unwrap();
        assert!(existing_path.exists());

        // Create new image to upload
        let new_source = create_valid_jpeg(temp_dir.path(), "new_image.jpg");

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id,
            file_path: new_source,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());

        // Verify new file exists and old content is replaced
        assert!(existing_path.exists());
        let content = std::fs::read(existing_path.full_path()).unwrap();
        assert_ne!(content, b"old image");
    }

    #[tokio::test]
    async fn test_upload_image_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let nonexistent_file = temp_dir.path().join("does_not_exist.jpg");

        let model_id_str = "trn:railway-model:marklin:39216";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id,
            file_path: nonexistent_file,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(UploadError::Validation(_))));
    }

    #[tokio::test]
    async fn test_upload_image_bytes_success() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        // Create valid JPEG bytes
        let jpeg_bytes = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ];

        let model_id_str = "trn:railway-model:roco:12345";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImageBytes::new(storage);
        let input = UploadImageBytesInput {
            model_id,
            file_name: "drag_drop_image.jpg".to_string(),
            file_data: jpeg_bytes,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());

        // Verify the file was saved
        let dest_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        assert!(dest_path.exists());
    }

    #[tokio::test]
    async fn test_upload_image_bytes_invalid_format() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let invalid_bytes = b"Not an image file".to_vec();

        let model_id_str = "trn:railway-model:roco:12345";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImageBytes::new(storage);
        let input = UploadImageBytesInput {
            model_id,
            file_name: "invalid.txt".to_string(),
            file_data: invalid_bytes,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(UploadError::Validation(_))));
    }

    #[tokio::test]
    async fn test_upload_image_bytes_model_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let jpeg_bytes = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ];

        let model_id_str = "trn:railway-model:nonexistent:999";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(false));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImageBytes::new(storage);
        let input = UploadImageBytesInput {
            model_id,
            file_name: "image.jpg".to_string(),
            file_data: jpeg_bytes,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(UploadError::ModelNotFound(_))));
    }

    #[tokio::test]
    async fn test_upload_image_bytes_replaces_existing() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:roco:12345";

        // Create existing image
        let existing_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        std::fs::write(existing_path.full_path(), b"old image").unwrap();
        assert!(existing_path.exists());

        // Create new JPEG bytes
        let jpeg_bytes = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ];

        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo
            .expect_exists_by_id()
            .withf(move |id| id.as_ref() == model_id_str)
            .times(1)
            .returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImageBytes::new(storage);
        let input = UploadImageBytesInput {
            model_id,
            file_name: "new_image.jpg".to_string(),
            file_data: jpeg_bytes,
        };

        let result = use_case.execute(input, &mut uow).await;
        assert!(result.is_ok());

        // Verify file still exists and content was replaced
        assert!(existing_path.exists());
        let content = std::fs::read(existing_path.full_path()).unwrap();
        assert_ne!(content, b"old image");
    }

    #[tokio::test]
    async fn test_replacement_deletes_old_file() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:marklin:39216";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Upload first image with unique content
        let first_image =
            create_valid_jpeg_with_content(temp_dir.path(), "first.jpg", b"FIRST_IMAGE_DATA");

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo.expect_exists_by_id().returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id: model_id.clone(),
            file_path: first_image.clone(),
        };

        use_case.execute(input, &mut uow).await.unwrap();

        let dest_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        let first_content = std::fs::read(dest_path.full_path()).unwrap();
        let first_size = first_content.len();

        // Upload second image with different content (replacement)
        let second_image = create_valid_jpeg_with_content(
            temp_dir.path(),
            "second.jpg",
            b"SECOND_IMAGE_DATA_IS_LONGER",
        );

        let mut mock_repo2 = MockRailwayModelRepository::new();
        mock_repo2.expect_exists_by_id().returning(|_| Ok(true));

        let mut uow2 = FakeUow::with_railway_models_repo(mock_repo2);

        let storage2 = FileStorage::new(storage_dir.clone()).unwrap();
        let use_case2 = UploadModelImage::new(storage2);

        let input2 = UploadImageInput {
            model_id: model_id.clone(),
            file_path: second_image,
        };

        use_case2.execute(input2, &mut uow2).await.unwrap();

        // Verify only one file exists and it's the new one
        assert!(dest_path.exists());
        let final_content = std::fs::read(dest_path.full_path()).unwrap();

        // Content should be different (new file replaced old)
        assert_ne!(
            final_content.len(),
            first_size,
            "File content should have changed"
        );
    }

    #[tokio::test]
    async fn test_multiple_replacements_no_orphans() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");

        let model_id_str = "trn:railway-model:roco:43210";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Perform 3 consecutive uploads (initial + 2 replacements)
        for i in 1..=3 {
            let image = create_valid_jpeg(temp_dir.path(), &format!("image{}.jpg", i));

            let storage_inst = FileStorage::new(storage_dir.clone()).unwrap();
            let use_case = UploadModelImage::new(storage_inst);

            let mut mock_repo = MockRailwayModelRepository::new();
            mock_repo.expect_exists_by_id().returning(|_| Ok(true));

            let mut uow = FakeUow::with_railway_models_repo(mock_repo);

            let input = UploadImageInput {
                model_id: model_id.clone(),
                file_path: image,
            };

            use_case.execute(input, &mut uow).await.unwrap();
        }

        // Verify only ONE file exists in storage directory
        let files: Vec<_> = std::fs::read_dir(&storage_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        assert_eq!(
            files.len(),
            1,
            "Expected exactly 1 file in storage, found {}. No orphaned files should remain.",
            files.len()
        );

        // Verify it's the correct file for this model
        let dest_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        assert!(dest_path.exists(), "Expected destination file to exist");
    }

    #[tokio::test]
    async fn test_replacement_different_format_jpeg_to_png() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");
        let storage = FileStorage::new(storage_dir.clone()).unwrap();

        let model_id_str = "trn:railway-model:lima:5678";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Upload JPEG first
        let jpeg_image = create_valid_jpeg(temp_dir.path(), "photo.jpg");

        let mut mock_repo = MockRailwayModelRepository::new();
        mock_repo.expect_exists_by_id().returning(|_| Ok(true));

        let mut uow = FakeUow::with_railway_models_repo(mock_repo);

        let use_case = UploadModelImage::new(storage);
        let input = UploadImageInput {
            model_id: model_id.clone(),
            file_path: jpeg_image,
        };

        use_case.execute(input, &mut uow).await.unwrap();

        let jpeg_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Jpeg);
        assert!(jpeg_path.exists(), "JPEG file should exist");

        // Replace with PNG
        let png_image = create_valid_png(temp_dir.path(), "photo.png");

        let mut mock_repo2 = MockRailwayModelRepository::new();
        mock_repo2.expect_exists_by_id().returning(|_| Ok(true));

        let mut uow2 = FakeUow::with_railway_models_repo(mock_repo2);

        let storage2 = FileStorage::new(storage_dir.clone()).unwrap();
        let use_case2 = UploadModelImage::new(storage2);

        let input2 = UploadImageInput {
            model_id: model_id.clone(),
            file_path: png_image,
        };

        use_case2.execute(input2, &mut uow2).await.unwrap();

        // Verify JPEG is gone and PNG exists
        assert!(!jpeg_path.exists(), "Old JPEG file should be deleted");

        let png_path = ModelImagePath::new(&storage_dir, model_id_str, ImageFormat::Png);
        assert!(png_path.exists(), "New PNG file should exist");

        // Verify only one file exists
        let files: Vec<_> = std::fs::read_dir(&storage_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        assert_eq!(files.len(), 1, "Only one image file should exist");
    }

    #[tokio::test]
    async fn test_destination_path_extension_changes() {
        let temp_dir = TempDir::new().unwrap();
        let storage_dir = temp_dir.path().join("storage");

        let model_id_str = "trn:railway-model:fleischmann:1234";
        let model_id = RailwayModelId::try_from(model_id_str).unwrap();

        // Test sequence: JPEG → PNG → WebP
        let formats = vec![
            (ImageFormat::Jpeg, "test.jpg", "jpg"),
            (ImageFormat::Png, "test.png", "png"),
            (ImageFormat::WebP, "test.webp", "webp"),
        ];

        for (format, filename, expected_ext) in formats {
            let image = match format {
                ImageFormat::Jpeg => create_valid_jpeg(temp_dir.path(), filename),
                ImageFormat::Png => create_valid_png(temp_dir.path(), filename),
                ImageFormat::WebP => {
                    // Create a minimal WebP file
                    let path = temp_dir.path().join(filename);
                    let webp_data = vec![
                        0x52, 0x49, 0x46, 0x46, // "RIFF"
                        0x1A, 0x00, 0x00, 0x00, // File size
                        0x57, 0x45, 0x42, 0x50, // "WEBP"
                        0x56, 0x50, 0x38, 0x20, // "VP8 "
                        0x0E, 0x00, 0x00, 0x00, // Chunk size
                        0x30, 0x01, 0x00, 0x9D, 0x01, 0x2A, // VP8 bitstream
                        0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ];
                    std::fs::write(&path, webp_data).unwrap();
                    path
                }
            };

            let storage_inst = FileStorage::new(storage_dir.clone()).unwrap();
            let use_case = UploadModelImage::new(storage_inst);

            let mut mock_repo = MockRailwayModelRepository::new();
            mock_repo.expect_exists_by_id().returning(|_| Ok(true));

            let mut uow = FakeUow::with_railway_models_repo(mock_repo);

            let input = UploadImageInput {
                model_id: model_id.clone(),
                file_path: image,
            };

            use_case.execute(input, &mut uow).await.unwrap();

            // Verify correct extension
            let dest_path = ModelImagePath::new(&storage_dir, model_id_str, format);
            assert!(
                dest_path.exists(),
                "File with {} extension should exist",
                expected_ext
            );
            assert_eq!(
                dest_path.full_path().extension().unwrap(),
                expected_ext,
                "Extension should be {}",
                expected_ext
            );

            // Verify only one file exists (old formats deleted)
            let files: Vec<_> = std::fs::read_dir(&storage_dir)
                .unwrap()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .collect();

            assert_eq!(
                files.len(),
                1,
                "Only one file should exist after format change to {}",
                expected_ext
            );
        }
    }
}
