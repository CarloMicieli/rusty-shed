use crate::media::domain::ImagePlaceholder;

/// Service for generating image placeholders.
///
/// Creates HTML/CSS markup for displaying when no image is available.
pub struct PlaceholderGenerator;

impl PlaceholderGenerator {
    /// Generate an HTML/CSS placeholder.
    ///
    /// Creates a responsive, accessible placeholder with Tailwind CSS styling.
    /// The placeholder displays "No picture yet" with an icon.
    ///
    /// # Returns
    ///
    /// Returns an `ImagePlaceholder` containing complete HTML/CSS markup.
    ///
    /// # Design
    ///
    /// - Responsive: Works on mobile (320px), tablet (768px), desktop (1920px+)
    /// - Accessible: Semantic HTML with proper ARIA attributes
    /// - Dark mode: Supports light and dark themes
    /// - Tailwind: Uses utility classes for consistent styling
    pub fn generate() -> ImagePlaceholder {
        ImagePlaceholder::generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_placeholder() {
        let placeholder = PlaceholderGenerator::generate();

        assert_eq!(placeholder.text(), "No picture yet");
        assert!(!placeholder.html_content().is_empty());
    }

    #[test]
    fn test_generated_html_structure() {
        let placeholder = PlaceholderGenerator::generate();
        let html = placeholder.html_content();

        // Check for responsive design classes
        assert!(html.contains("w-full"));
        assert!(html.contains("aspect-video"));

        // Check for layout classes
        assert!(html.contains("flex"));
        assert!(html.contains("items-center"));
        assert!(html.contains("justify-center"));

        // Check for background colors
        assert!(html.contains("bg-gray-"));

        // Check for dark mode support
        assert!(html.contains("dark:"));

        // Check for border styling
        assert!(html.contains("border-"));
        assert!(html.contains("rounded-"));

        // Check for icon SVG
        assert!(html.contains("<svg"));
        assert!(html.contains("</svg>"));

        // Check for text content
        assert!(html.contains("No picture yet"));
    }

    #[test]
    fn test_multiple_generations_are_consistent() {
        let placeholder1 = PlaceholderGenerator::generate();
        let placeholder2 = PlaceholderGenerator::generate();

        assert_eq!(placeholder1, placeholder2);
    }
}
