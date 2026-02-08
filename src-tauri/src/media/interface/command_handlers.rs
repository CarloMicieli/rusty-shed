//! Tauri Command Handlers for Media Module
//!
//! Exposes media functionality to the frontend via Tauri IPC.

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::infrastructure::error::CommandError;
use crate::media::application::{
    DeleteError, DeleteImageInput, DeleteModelImage, GetImagePlaceholder, GetRailwayModelImage,
    UploadError, UploadImageBytesInput, UploadImageInput, UploadModelImage, UploadModelImageBytes,
};
use crate::media::domain::ImageError;
use crate::media::domain::image_validation::{StorageError, ValidationError};
use crate::media::infrastructure::FileStorage;
use crate::media::interface::RailwayModelImageResponse;
use crate::state::AppState;
use garde::Validate;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Get the image for a railway model.
///
/// Returns either the path to the image file (if found) or an HTML/CSS
/// placeholder (if no image exists).
///
/// # Arguments
///
/// * `state` - Application state containing models directory path
/// * `railway_model_id` - The model ID to retrieve an image for
///
/// # Returns
///
/// Returns `RailwayModelImageResponse` with either:
/// - `image_path` set if image found
/// - `placeholder_html` set if no image found
///
/// # Errors
///
/// Returns `CommandError` if:
/// - Path validation fails (security)
/// - I/O errors occur
/// - Model ID is invalid
#[tauri::command]
#[specta::specta]
pub async fn get_railway_model_image(
    state: tauri::State<'_, AppState>,
    railway_model_id: RailwayModelId,
) -> Result<RailwayModelImageResponse, CommandError> {
    debug!(
        "Fetching image for railway model: {}",
        railway_model_id.as_ref()
    );

    let models_dir = state.models_dir();
    let use_case = GetRailwayModelImage;

    match use_case.execute(&railway_model_id, &models_dir).await {
        Ok(image) => {
            debug!(
                "Found image for model {}: {:?}",
                railway_model_id.as_ref(),
                image.path()
            );

            let path_str = image
                .path()
                .to_str()
                .ok_or_else(|| {
                    CommandError::Unknown("Failed to convert path to string".to_string())
                })?
                .to_string();

            Ok(RailwayModelImageResponse::with_image(path_str))
        }
        Err(ImageError::NotFound(_)) => {
            warn!(
                "No image found for model {}, returning placeholder",
                railway_model_id.as_ref()
            );

            let placeholder_use_case = GetImagePlaceholder;
            let placeholder = placeholder_use_case.execute();

            Ok(RailwayModelImageResponse::with_placeholder(
                placeholder.html_content().to_string(),
            ))
        }
        Err(err) => Err(map_image_error(err)),
    }
}

/// Upload a model image from a file path (file explorer selection).
///
/// # Arguments
///
/// * `state` - Application state containing models directory path
/// * `args` - Upload arguments (model ID and file path)
///
/// # Returns
///
/// Returns `Ok(())` on success
///
/// # Errors
///
/// Returns `CommandError` if:
/// - Model doesn't exist
/// - File validation fails (format, size)
/// - Storage operations fail
#[tauri::command]
#[specta::specta]
pub async fn upload_model_image(
    state: tauri::State<'_, AppState>,
    args: UploadModelImageArgs,
) -> Result<(), CommandError> {
    debug!(
        "Uploading image for model: {} from path: {}",
        args.model_id, args.file_path
    );

    // Validate arguments
    args.validate().map_err(|e| {
        let err_msg = format!("Invalid upload arguments: {}", e);
        warn!("{}", err_msg);
        CommandError::BusinessRule(err_msg)
    })?;

    // Parse model ID
    let model_id = RailwayModelId::try_from(args.model_id.as_str()).map_err(|e| {
        let err_msg = format!("Invalid model ID: {}", e);
        warn!("{}", err_msg);
        CommandError::BusinessRule(err_msg)
    })?;

    // Get storage directory
    let models_dir = state.models_dir();
    debug!("Using models directory: {:?}", models_dir);
    let storage = FileStorage::new(models_dir.clone()).map_err(map_storage_error)?;

    // Execute use case
    let use_case = UploadModelImage::new(storage);
    let input = UploadImageInput {
        model_id: model_id.clone(),
        file_path: PathBuf::from(&args.file_path),
    };

    debug!("Executing upload use case...");
    let mut unit_of_work = state.unit_of_work().await?;
    use_case
        .execute(input, &mut unit_of_work)
        .await
        .map_err(|e| {
            warn!("Upload failed: {:?}", e);
            map_upload_error(e)
        })?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    debug!(
        "Image uploaded successfully for model: {}",
        model_id.as_ref()
    );
    Ok(())
}

/// Upload a model image from bytes (drag & drop).
///
/// # Arguments
///
/// * `state` - Application state containing models directory path
/// * `args` - Upload arguments (model ID, filename, file data)
///
/// # Returns
///
/// Returns `Ok(())` on success
///
/// # Errors
///
/// Returns `CommandError` if:
/// - Model doesn't exist
/// - File validation fails (format, size)
/// - Storage operations fail
#[tauri::command]
#[specta::specta]
pub async fn upload_model_image_bytes(
    state: tauri::State<'_, AppState>,
    args: UploadModelImageBytesArgs,
) -> Result<(), CommandError> {
    debug!("Uploading image bytes for model: {}", args.model_id);

    // Validate arguments
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Invalid upload arguments: {}", e)))?;

    // Parse model ID
    let model_id = RailwayModelId::try_from(args.model_id.as_str())
        .map_err(|e| CommandError::BusinessRule(format!("Invalid model ID: {}", e)))?;

    // Get storage directory
    let models_dir = state.models_dir();
    let storage = FileStorage::new(models_dir.clone()).map_err(map_storage_error)?;

    // Execute use case
    let use_case = UploadModelImageBytes::new(storage);
    let input = UploadImageBytesInput {
        model_id,
        file_name: args.file_name,
        file_data: args.file_data,
    };

    let mut unit_of_work = state.unit_of_work().await?;
    use_case
        .execute(input, &mut unit_of_work)
        .await
        .map_err(map_upload_error)?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    debug!("Image bytes uploaded successfully");
    Ok(())
}

/// Delete a model image.
///
/// # Arguments
///
/// * `state` - Application state containing models directory path
/// * `args` - Delete arguments (model ID)
///
/// # Returns
///
/// Returns `Ok(())` on success (idempotent - no error if image doesn't exist)
///
/// # Errors
///
/// Returns `CommandError` if:
/// - Model doesn't exist
/// - Storage operations fail
#[tauri::command]
#[specta::specta]
pub async fn delete_model_image(
    state: tauri::State<'_, AppState>,
    args: DeleteModelImageArgs,
) -> Result<(), CommandError> {
    debug!("Deleting image for model: {}", args.model_id);

    // Validate arguments
    args.validate()
        .map_err(|e| CommandError::BusinessRule(format!("Invalid delete arguments: {}", e)))?;

    // Parse model ID
    let model_id = RailwayModelId::try_from(args.model_id.as_str())
        .map_err(|e| CommandError::BusinessRule(format!("Invalid model ID: {}", e)))?;

    // Get storage directory
    let models_dir = state.models_dir();
    let storage = FileStorage::new(models_dir.clone()).map_err(map_storage_error)?;

    // Execute use case
    let use_case = DeleteModelImage::new(storage);
    let input = DeleteImageInput { model_id };

    let mut unit_of_work = state.unit_of_work().await?;
    use_case
        .execute(input, &mut unit_of_work)
        .await
        .map_err(map_delete_error)?;

    unit_of_work.commit().await.map_err(CommandError::from)?;

    debug!("Image deleted successfully");
    Ok(())
}

// ============================================================================
// DTOs
// ============================================================================

/// Arguments for deleting a model image
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DeleteModelImageArgs {
    #[garde(length(min = 1))]
    pub model_id: String,
}

/// Arguments for uploading a model image from file path
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageArgs {
    #[garde(length(min = 1))]
    pub model_id: String,

    #[garde(length(min = 1))]
    pub file_path: String,
}

/// Arguments for uploading a model image from bytes
#[derive(Debug, Clone, Deserialize, Serialize, specta::Type, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UploadModelImageBytesArgs {
    #[garde(length(min = 1))]
    pub model_id: String,

    #[garde(length(min = 1))]
    pub file_name: String,

    #[garde(length(min = 1))]
    pub file_data: Vec<u8>,
}

// ============================================================================
// Error Mapping
// ============================================================================

/// Map UploadError to CommandError
fn map_upload_error(err: UploadError) -> CommandError {
    match err {
        UploadError::ModelNotFound(msg) => CommandError::NotFound(msg),
        UploadError::Validation(e) => map_validation_error(e),
        UploadError::Storage(e) => map_storage_error(e),
        UploadError::Domain(e) => CommandError::from(e),
    }
}

/// Map DeleteError to CommandError
fn map_delete_error(err: DeleteError) -> CommandError {
    match err {
        DeleteError::ModelNotFound(msg) => CommandError::NotFound(msg),
        DeleteError::Storage(e) => map_storage_error(e),
        DeleteError::Domain(e) => CommandError::from(e),
    }
}

/// Map ValidationError to CommandError
fn map_validation_error(err: ValidationError) -> CommandError {
    match err {
        ValidationError::FileNotFound => CommandError::BusinessRule("File not found".to_string()),
        ValidationError::FileTooLarge { size_mb, max_mb } => CommandError::BusinessRule(format!(
            "File size ({} MB) exceeds maximum allowed size ({} MB)",
            size_mb, max_mb
        )),
        ValidationError::UnsupportedFormat { format } => CommandError::BusinessRule(format!(
            "Unsupported format: {}. Supported formats: JPEG, PNG, WebP",
            format
        )),
        ValidationError::CorruptedImage => {
            CommandError::BusinessRule("Image file is corrupted or invalid".to_string())
        }
        ValidationError::IoError(msg) => CommandError::Unknown(msg),
    }
}

/// Map StorageError to CommandError
fn map_storage_error(err: StorageError) -> CommandError {
    match err {
        StorageError::DirectoryCreation(msg) => {
            CommandError::Unknown(format!("Failed to create storage directory: {}", msg))
        }
        StorageError::CopyFailed(msg) => {
            CommandError::Unknown(format!("Failed to copy file: {}", msg))
        }
        StorageError::WriteFailed(msg) => {
            CommandError::Unknown(format!("Failed to write file: {}", msg))
        }
        StorageError::DeleteFailed(msg) => {
            CommandError::Unknown(format!("Failed to delete file: {}", msg))
        }
        StorageError::FileNotFound(msg) => CommandError::NotFound(msg),
        StorageError::IoError(msg) => CommandError::Unknown(msg),
    }
}

/// Map ImageError to CommandError.
///
/// Converts domain-level image errors to application-level command errors
/// that the frontend can understand and display to users.
fn map_image_error(err: ImageError) -> CommandError {
    match err {
        ImageError::NotFound(msg) => CommandError::NotFound(msg),
        ImageError::InvalidPath(msg) => {
            CommandError::BusinessRule(format!("Invalid image path: {}", msg))
        }
        ImageError::IoError(msg) => CommandError::DatabaseError(format!("I/O error: {}", msg)),
        ImageError::InvalidModelId(msg) => {
            CommandError::BusinessRule(format!("Invalid model ID: {}", msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_image_error_not_found() {
        let err = ImageError::NotFound("test.png".to_string());
        let cmd_err = map_image_error(err);

        match cmd_err {
            CommandError::NotFound(msg) => assert_eq!(msg, "test.png"),
            _ => panic!("Expected NotFound variant"),
        }
    }

    #[test]
    fn test_map_image_error_invalid_path() {
        let err = ImageError::InvalidPath("traversal attempt".to_string());
        let cmd_err = map_image_error(err);

        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("Invalid image path")),
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[test]
    fn test_map_image_error_io() {
        let err = ImageError::IoError("permission denied".to_string());
        let cmd_err = map_image_error(err);

        match cmd_err {
            CommandError::DatabaseError(msg) => assert!(msg.contains("I/O error")),
            _ => panic!("Expected DatabaseError variant"),
        }
    }
}
