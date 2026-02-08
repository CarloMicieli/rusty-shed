//! Upload Model Image Use Case
//!
//! Orchestrates the upload of model images from user-selected files.

use crate::catalog::domain::railway_model::{RailwayModelId, RailwayModelUowExt};
use crate::core::domain::domain_error::DomainError;
use crate::media::domain::image_validation::{
    ImageValidator, ModelImagePath, StorageError, ValidationError,
};
use crate::media::infrastructure::FileStorage;
use std::path::PathBuf;

// ============================================================================
// Use Case Inputs
// ============================================================================

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

    /// Execute the upload use case (path-based)
    ///
    /// # Steps
    /// 1. Validate model exists
    /// 2. Validate source file (format, size)
    /// 3. Determine destination path
    /// 4. Delete existing image if present
    /// 5. Copy file to destination
    ///
    /// # Errors
    /// - ValidationError: Invalid file format, size, or missing file
    /// - StorageError: File operations failed
    /// - DomainError::NotFound: Model doesn't exist
    pub async fn execute<U>(
        &self,
        input: UploadImageInput,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        // Step 1: Validate model exists
        self.validate_model_exists(&input.model_id, unit_of_work)
            .await?;

        // Step 2: Validate source file
        let format = ImageValidator::validate(&input.file_path).map_err(UploadError::Validation)?;

        // Step 3: Determine destination path
        let dest_path =
            ModelImagePath::new(self.storage.storage_dir(), input.model_id.as_ref(), format);

        // Step 4: Delete existing image if present (replacement)
        if dest_path.exists() {
            log::debug!(
                "Deleting existing image: {}",
                dest_path.full_path().display()
            );
            self.storage
                .delete_image(&dest_path)
                .await
                .map_err(UploadError::Storage)?;
        }

        // Step 5: Copy file to destination
        log::debug!(
            "Copying image from {} to {}",
            input.file_path.display(),
            dest_path.full_path().display()
        );
        self.storage
            .copy_image(&input.file_path, &dest_path)
            .await
            .map_err(UploadError::Storage)?;

        log::info!(
            "Successfully uploaded image for model {}",
            input.model_id.as_ref()
        );
        Ok(())
    }

    /// Validate that the model exists in the database
    async fn validate_model_exists<U>(
        &self,
        model_id: &RailwayModelId,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        let model = repository
            .find_by_id(model_id)
            .await
            .map_err(UploadError::Domain)?;

        match model {
            Some(_) => Ok(()),
            None => Err(UploadError::ModelNotFound(model_id.as_ref().to_string())),
        }
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

    /// Execute the upload use case (bytes-based)
    ///
    /// # Steps
    /// 1. Validate model exists
    /// 2. Write bytes to temporary file
    /// 3. Validate temporary file (format, size)
    /// 4. Determine destination path
    /// 5. Delete existing image if present
    /// 6. Move temporary file to destination
    /// 7. Clean up temporary file (on error)
    ///
    /// # Errors
    /// - ValidationError: Invalid file format, size, or corrupted data
    /// - StorageError: File operations failed
    /// - DomainError::NotFound: Model doesn't exist
    pub async fn execute<U>(
        &self,
        input: UploadImageBytesInput,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        // Step 1: Validate model exists
        self.validate_model_exists(&input.model_id, unit_of_work)
            .await?;

        // Step 2: Create temporary file
        let temp_dir = std::env::temp_dir();
        let temp_filename = format!("rusty_shed_upload_{}", uuid::Uuid::new_v4());
        let temp_path = temp_dir.join(temp_filename);

        // Write bytes to temp file
        tokio::fs::write(&temp_path, &input.file_data)
            .await
            .map_err(|e| {
                UploadError::Storage(StorageError::WriteFailed(format!(
                    "Failed to write temp file: {}",
                    e
                )))
            })?;

        // Ensure temp file cleanup on any error
        let cleanup_result = async {
            // Step 3: Validate temporary file
            let format = ImageValidator::validate(&temp_path).map_err(UploadError::Validation)?;

            // Step 4: Determine destination path
            let dest_path =
                ModelImagePath::new(self.storage.storage_dir(), input.model_id.as_ref(), format);

            // Step 5: Delete existing image if present
            if dest_path.exists() {
                log::debug!(
                    "Deleting existing image: {}",
                    dest_path.full_path().display()
                );
                self.storage
                    .delete_image(&dest_path)
                    .await
                    .map_err(UploadError::Storage)?;
            }

            // Step 6: Copy temp file to destination
            log::debug!(
                "Moving image from temp to {}",
                dest_path.full_path().display()
            );
            self.storage
                .copy_image(&temp_path, &dest_path)
                .await
                .map_err(UploadError::Storage)?;

            log::info!(
                "Successfully uploaded image for model {} from bytes",
                input.model_id.as_ref()
            );
            Ok::<(), UploadError>(())
        }
        .await;

        // Step 7: Clean up temp file
        tokio::fs::remove_file(&temp_path).await.ok();

        cleanup_result
    }

    /// Validate that the model exists in the database
    async fn validate_model_exists<U>(
        &self,
        model_id: &RailwayModelId,
        unit_of_work: &mut U,
    ) -> Result<(), UploadError>
    where
        U: RailwayModelUowExt + Send,
    {
        let mut repository = unit_of_work.railway_model_repository();
        let model = repository
            .find_by_id(model_id)
            .await
            .map_err(UploadError::Domain)?;

        match model {
            Some(_) => Ok(()),
            None => Err(UploadError::ModelNotFound(model_id.as_ref().to_string())),
        }
    }
}

// ============================================================================
// Errors
// ============================================================================

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

// ============================================================================
// Tests (TODO: Re-enable after fixing mock setup)
// ============================================================================

// Tests temporarily disabled due to complex mock requirements
// Will be implemented in integration tests instead
