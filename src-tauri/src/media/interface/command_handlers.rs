//! Tauri Command Handlers for Media Module
//!
//! Exposes media functionality to the frontend via Tauri IPC.

use crate::catalog::domain::railway_model::RailwayModelId;
use crate::core::infrastructure::error::CommandError;
use crate::media::application::{GetImagePlaceholder, GetRailwayModelImage};
use crate::media::domain::ImageError;
use crate::media::interface::RailwayModelImageResponse;
use crate::state::AppState;
use log::{debug, warn};

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
