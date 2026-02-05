use serde::{Deserialize, Serialize};
use specta::Type;

/// An import warning (non-blocking issue).
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ImportWarning {
    /// Warning code for i18n lookup
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Related entity or file (optional context)
    pub context: Option<String>,
}

impl ImportWarning {
    /// Create a new warning for a missing image.
    pub fn missing_image(filename: &str) -> Self {
        Self {
            code: "missing_image".to_string(),
            message: format!("Image not found: {}", filename),
            context: Some(filename.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_image_warning() {
        let warning = ImportWarning::missing_image("test.jpg");
        assert_eq!(warning.code, "missing_image");
        assert_eq!(warning.message, "Image not found: test.jpg");
        assert_eq!(warning.context, Some("test.jpg".to_string()));
    }
}
