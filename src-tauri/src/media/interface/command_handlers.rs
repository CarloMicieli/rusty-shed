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
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use tracing::{debug, warn};

pub async fn get_railway_model_image_inner(
    state: &AppState,
    railway_model_id: RailwayModelId,
) -> Result<RailwayModelImageResponse, CommandError> {
    debug!(
        "Fetching image for railway model: {}",
        railway_model_id.as_ref()
    );

    let models_dir = state.models_dir();
    let use_case = GetRailwayModelImage;

    match use_case.execute(&railway_model_id, models_dir).await {
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
                    CommandError::unknown("Failed to convert path to string".to_string())
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

/// Inner implementation for [`get_image_path`].
pub async fn get_image_path_inner(
    state: &AppState,
    id: String,
    category: String,
) -> Result<String, CommandError> {
    match category.as_str() {
        "static" => Ok(id),
        "railway_model" => {
            let id_path = Path::new(&id);
            let valid = id_path
                .components()
                .all(|c| matches!(c, Component::Normal(_)));

            if !valid {
                return Err(CommandError::validation_field(
                    "id",
                    "Invalid image id; must be a file name",
                ));
            }

            let mut full_path = state.models_dir().to_path_buf();
            full_path.push(id_path);

            match tokio::fs::metadata(&full_path).await {
                Ok(meta) if meta.is_file() => Ok(full_path
                    .to_str()
                    .map(|s| s.to_string())
                    .ok_or_else(|| CommandError::unknown("Non-Unicode path"))?),
                _ => Err(CommandError::NotFound(format!(
                    "No image found for railway model {id}"
                ))),
            }
        }
        other => Err(CommandError::validation_field(
            "category",
            format!("Unsupported category '{other}'"),
        )),
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_image_path(
    state: tauri::State<'_, AppState>,
    id: String,
    category: String,
) -> Result<String, CommandError> {
    get_image_path_inner(&state, id, category).await
}

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
    get_railway_model_image_inner(&state, railway_model_id).await
}

pub async fn upload_model_image_inner(
    state: &AppState,
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
    let storage = FileStorage::new(models_dir.to_path_buf()).map_err(map_storage_error)?;

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

    unit_of_work.commit().await?;

    debug!(
        "Image uploaded successfully for model: {}",
        model_id.as_ref()
    );
    Ok(())
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
    upload_model_image_inner(&state, args).await
}

pub async fn upload_model_image_bytes_inner(
    state: &AppState,
    args: UploadModelImageBytesArgs,
) -> Result<(), CommandError> {
    debug!("Uploading image bytes for model: {}", args.model_id);

    // Validate arguments
    args.validate().map_err(CommandError::from)?;

    // Parse model ID
    let model_id = RailwayModelId::try_from(args.model_id.as_str())
        .map_err(|e| CommandError::BusinessRule(format!("Invalid model ID: {}", e)))?;

    // Get storage directory
    let models_dir = state.models_dir();
    let storage = FileStorage::new(models_dir.to_path_buf()).map_err(map_storage_error)?;

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

    unit_of_work.commit().await?;

    debug!("Image bytes uploaded successfully");
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
    upload_model_image_bytes_inner(&state, args).await
}

pub async fn delete_model_image_inner(
    state: &AppState,
    args: DeleteModelImageArgs,
) -> Result<(), CommandError> {
    debug!("Deleting image for model: {}", args.model_id);

    // Validate arguments
    args.validate().map_err(CommandError::from)?;

    // Parse model ID
    let model_id = RailwayModelId::try_from(args.model_id.as_str())
        .map_err(|e| CommandError::BusinessRule(format!("Invalid model ID: {}", e)))?;

    // Get storage directory
    let models_dir = state.models_dir();
    let storage = FileStorage::new(models_dir.to_path_buf()).map_err(map_storage_error)?;

    // Execute use case
    let use_case = DeleteModelImage::new(storage);
    let input = DeleteImageInput { model_id };

    let mut unit_of_work = state.unit_of_work().await?;
    use_case
        .execute(input, &mut unit_of_work)
        .await
        .map_err(map_delete_error)?;

    unit_of_work.commit().await?;

    debug!("Image deleted successfully");
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
    delete_model_image_inner(&state, args).await
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
    if let ValidationError::IoError(msg) = err {
        return CommandError::unknown(msg);
    }

    CommandError::BusinessRule(map_validation_error_message(&err))
}

fn map_validation_error_message(err: &ValidationError) -> String {
    match err {
        ValidationError::FileNotFound => "File not found".to_string(),
        ValidationError::FileTooLarge { size_mb, max_mb } => format!(
            "File size ({} MB) exceeds maximum allowed size ({} MB)",
            size_mb, max_mb
        ),
        ValidationError::UnsupportedFormat { format } => format!(
            "Unsupported format: {}. Supported formats: JPEG, PNG, WebP",
            format
        ),
        ValidationError::CorruptedImage => "Image file is corrupted or invalid".to_string(),
        ValidationError::IoError(_) => {
            unreachable!("io errors are handled in map_validation_error")
        }
    }
}

/// Map StorageError to CommandError
fn map_storage_error(err: StorageError) -> CommandError {
    match err {
        StorageError::DirectoryCreation(msg) => {
            CommandError::unknown(format!("Failed to create storage directory: {}", msg))
        }
        StorageError::CopyFailed(msg) => {
            CommandError::unknown(format!("Failed to copy file: {}", msg))
        }
        StorageError::WriteFailed(msg) => {
            CommandError::unknown(format!("Failed to write file: {}", msg))
        }
        StorageError::DeleteFailed(msg) => {
            CommandError::unknown(format!("Failed to delete file: {}", msg))
        }
        StorageError::FileNotFound(msg) => CommandError::NotFound(msg),
        StorageError::IoError(msg) => CommandError::unknown(msg),
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
    use sqlx::SqlitePool;
    use tempfile::tempdir;

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

    #[test]
    fn test_map_storage_error_directory_creation() {
        let err = StorageError::DirectoryCreation("mkdir failed".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::Unknown { message, .. } => {
                assert!(message.contains("Failed to create storage directory"));
                assert!(message.contains("mkdir failed"));
            }
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_map_storage_error_copy_failed() {
        let err = StorageError::CopyFailed("copy failed".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::Unknown { message, .. } => {
                assert!(message.contains("Failed to copy file"));
                assert!(message.contains("copy failed"));
            }
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_map_storage_error_write_failed() {
        let err = StorageError::WriteFailed("write failed".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::Unknown { message, .. } => {
                assert!(message.contains("Failed to write file"));
                assert!(message.contains("write failed"));
            }
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_map_storage_error_delete_failed() {
        let err = StorageError::DeleteFailed("delete failed".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::Unknown { message, .. } => {
                assert!(message.contains("Failed to delete file"));
                assert!(message.contains("delete failed"));
            }
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_map_storage_error_file_not_found() {
        let err = StorageError::FileNotFound("missing.png".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::NotFound(message) => assert_eq!(message, "missing.png"),
            _ => panic!("Expected NotFound variant"),
        }
    }

    #[test]
    fn test_map_storage_error_io_error() {
        let err = StorageError::IoError("io failed".to_string());
        let cmd_err = map_storage_error(err);

        match cmd_err {
            CommandError::Unknown { message, .. } => assert_eq!(message, "io failed"),
            _ => panic!("Expected Unknown variant"),
        }
    }

    #[test]
    fn test_map_validation_error_file_not_found() {
        let cmd_err = map_validation_error(ValidationError::FileNotFound);

        match cmd_err {
            CommandError::BusinessRule(msg) => assert_eq!(msg, "File not found"),
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[test]
    fn test_map_validation_error_file_too_large() {
        let cmd_err = map_validation_error(ValidationError::FileTooLarge {
            size_mb: 11,
            max_mb: 10,
        });

        match cmd_err {
            CommandError::BusinessRule(msg) => {
                assert!(msg.contains("11"));
                assert!(msg.contains("10"));
            }
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[test]
    fn test_map_validation_error_unsupported_format() {
        let cmd_err = map_validation_error(ValidationError::UnsupportedFormat {
            format: "gif".to_string(),
        });

        match cmd_err {
            CommandError::BusinessRule(msg) => {
                assert!(msg.contains("Unsupported format"));
                assert!(msg.contains("gif"));
            }
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[test]
    fn test_map_validation_error_corrupted_image() {
        let cmd_err = map_validation_error(ValidationError::CorruptedImage);

        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("corrupted")),
            _ => panic!("Expected BusinessRule variant"),
        }
    }

    #[test]
    fn test_map_validation_error_io_error() {
        let cmd_err = map_validation_error(ValidationError::IoError("io".to_string()));

        match cmd_err {
            CommandError::Unknown { message, .. } => assert_eq!(message, "io"),
            _ => panic!("Expected Unknown variant"),
        }
    }

    async fn test_state(models_dir: std::path::PathBuf) -> AppState {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("in-memory pool");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        AppState::new(pool, models_dir, std::path::PathBuf::new())
    }

    #[tokio::test]
    async fn test_get_image_path_static_returns_id() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;

        let result =
            get_image_path_inner(&state, "logo.svg".to_string(), "static".to_string()).await;
        assert_eq!(result.expect("should return static id"), "logo.svg");
    }

    #[tokio::test]
    async fn test_get_image_path_railway_model_returns_absolute_path_when_file_exists() {
        let temp_dir = tempdir().expect("tempdir");
        let file_name = "abc123.png";
        let file_path = temp_dir.path().join(file_name);
        tokio::fs::write(&file_path, b"image")
            .await
            .expect("write image");
        let state = test_state(temp_dir.path().to_path_buf()).await;

        let result =
            get_image_path_inner(&state, file_name.to_string(), "railway_model".to_string()).await;

        assert_eq!(
            result.expect("path expected"),
            file_path.to_str().expect("unicode path")
        );
    }

    #[tokio::test]
    async fn test_get_image_path_rejects_path_traversal() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;

        let result = get_image_path_inner(
            &state,
            "../secret.png".to_string(),
            "railway_model".to_string(),
        )
        .await;

        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_get_image_path_rejects_unsupported_category() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;

        let result = get_image_path_inner(&state, "x.png".to_string(), "other".to_string()).await;

        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[tokio::test]
    async fn test_get_image_path_returns_not_found_for_missing_file() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;

        let result = get_image_path_inner(
            &state,
            "missing.png".to_string(),
            "railway_model".to_string(),
        )
        .await;

        assert!(matches!(result, Err(CommandError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_upload_model_image_invalid_model_id_returns_business_rule() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;
        let args = UploadModelImageArgs {
            model_id: "invalid-model-id".to_string(),
            file_path: "/tmp/model.png".to_string(),
        };

        let result = upload_model_image_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn test_upload_model_image_bytes_invalid_model_id_returns_business_rule() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;
        let args = UploadModelImageBytesArgs {
            model_id: "invalid-model-id".to_string(),
            file_name: "model.png".to_string(),
            file_data: vec![1, 2, 3],
        };

        let result = upload_model_image_bytes_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::BusinessRule(_))));
    }

    #[tokio::test]
    async fn test_delete_model_image_invalid_model_id_returns_business_rule() {
        let temp_dir = tempdir().expect("tempdir");
        let state = test_state(temp_dir.path().to_path_buf()).await;
        let args = DeleteModelImageArgs {
            model_id: "invalid-model-id".to_string(),
        };

        let result = delete_model_image_inner(&state, args).await;
        assert!(matches!(result, Err(CommandError::BusinessRule(_))));
    }
}
