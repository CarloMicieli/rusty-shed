//! Railway Model Image Response DTO
//!
//! Data Transfer Object for image responses sent to the frontend.

use serde::Serialize;

/// Response DTO for railway model image requests.
///
/// This DTO contains either the image path (if found) or placeholder HTML.
/// The frontend can check `has_image` to determine which field to use.
///
/// # Example
///
/// ```ignore
/// // Image found
/// RailwayModelImageResponse {
///     image_path: Some("/app/data/models/trn_railway-model_roco_43210.png".to_string()),
///     placeholder_html: None,
///     has_image: true,
/// }
///
/// // No image (placeholder)
/// RailwayModelImageResponse {
///     image_path: None,
///     placeholder_html: Some("<div>...</div>".to_string()),
///     has_image: false,
/// }
/// ```
#[derive(Debug, Clone, Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct RailwayModelImageResponse {
    /// Absolute path to the image file (if found)
    pub image_path: Option<String>,

    /// HTML/CSS placeholder markup (if no image found)
    pub placeholder_html: Option<String>,

    /// Quick flag indicating if image exists
    pub has_image: bool,
}

impl RailwayModelImageResponse {
    /// Create a response with an image path.
    pub fn with_image(path: String) -> Self {
        RailwayModelImageResponse {
            image_path: Some(path),
            placeholder_html: None,
            has_image: true,
        }
    }

    /// Create a response with a placeholder.
    pub fn with_placeholder(html: String) -> Self {
        RailwayModelImageResponse {
            image_path: None,
            placeholder_html: Some(html),
            has_image: false,
        }
    }
}
