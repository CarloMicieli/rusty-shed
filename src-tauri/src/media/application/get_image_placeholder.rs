use crate::media::domain::ImagePlaceholder;

/// Use case for generating image placeholders.
///
/// This use case creates a ready-to-use HTML/CSS placeholder
/// that can be displayed when a railway model has no associated image.
pub struct GetImagePlaceholder;

impl GetImagePlaceholder {
    /// Execute the use case to generate an image placeholder.
    ///
    /// Returns an `ImagePlaceholder` containing HTML/CSS markup
    /// ready for rendering in the frontend.
    ///
    /// This operation never fails and always returns a valid placeholder.
    pub fn execute(&self) -> ImagePlaceholder {
        ImagePlaceholder::generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        let use_case = GetImagePlaceholder;
        let placeholder = use_case.execute();

        assert_eq!(placeholder.text(), "No picture yet");
        assert!(!placeholder.html_content().is_empty());
    }

    #[test]
    fn test_execute_multiple_calls() {
        let use_case = GetImagePlaceholder;

        let placeholder1 = use_case.execute();
        let placeholder2 = use_case.execute();

        // Each call should return equivalent placeholders
        assert_eq!(placeholder1, placeholder2);
    }

    #[test]
    fn test_placeholder_html_structure() {
        let use_case = GetImagePlaceholder;
        let placeholder = use_case.execute();
        let html = placeholder.html_content();

        // Verify HTML contains expected structure
        assert!(html.contains("<div"));
        assert!(html.contains("class="));
        assert!(html.contains("No picture yet"));
    }
}
