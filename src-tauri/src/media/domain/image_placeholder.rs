//! Image Placeholder Value Object
//!
//! Represents an HTML/CSS placeholder for missing railway model images.

/// A placeholder image rendered as HTML/CSS.
///
/// Used when a railway model has no associated image file.
/// The placeholder displays a centered message with appropriate styling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImagePlaceholder {
    /// Display text shown in the placeholder
    text: &'static str,
    /// Complete HTML/CSS markup
    html: String,
}

impl ImagePlaceholder {
    /// Generate a new placeholder with default styling.
    ///
    /// Creates a responsive, accessible HTML/CSS placeholder that displays
    /// "No picture yet" in a visually consistent manner.
    pub fn generate() -> Self {
        let text = "No picture yet";
        let html = Self::generate_html(text);

        ImagePlaceholder { text, html }
    }

    /// Get the placeholder text.
    pub fn text(&self) -> &str {
        self.text
    }

    /// Get the complete HTML content.
    ///
    /// Returns HTML/CSS that can be directly rendered in the frontend.
    pub fn html_content(&self) -> &str {
        &self.html
    }

    /// Generate HTML/CSS markup for the placeholder.
    ///
    /// Creates a responsive design that works on mobile, tablet, and desktop.
    /// Uses Tailwind CSS classes for consistent styling.
    ///
    /// # Design Specifications
    ///
    /// - Centered content (horizontally and vertically)
    /// - Light gray background
    /// - Responsive aspect ratio (16:9 or similar)
    /// - Accessible semantic HTML
    /// - Icon optional (can be added via CSS or SVG)
    fn generate_html(text: &str) -> String {
        format!(
            r#"<div class="w-full aspect-video bg-gray-100 dark:bg-gray-800 rounded-lg flex items-center justify-center border-2 border-dashed border-gray-300 dark:border-gray-600">
  <div class="text-center">
    <svg class="mx-auto h-12 w-12 text-gray-400 dark:text-gray-500 mb-2" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
    </svg>
    <p class="text-sm text-gray-500 dark:text-gray-400 font-medium">{}</p>
  </div>
</div>"#,
            text
        )
    }
}

impl Default for ImagePlaceholder {
    fn default() -> Self {
        Self::generate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_generation() {
        let placeholder = ImagePlaceholder::generate();

        assert_eq!(placeholder.text(), "No picture yet");
        assert!(!placeholder.html_content().is_empty());
        assert!(placeholder.html_content().contains("No picture yet"));
    }

    #[test]
    fn test_html_content_structure() {
        let placeholder = ImagePlaceholder::generate();
        let html = placeholder.html_content();

        // Check for Tailwind classes
        assert!(html.contains("flex"));
        assert!(html.contains("items-center"));
        assert!(html.contains("justify-center"));
        assert!(html.contains("bg-gray-"));

        // Check for responsive design
        assert!(html.contains("aspect-video"));

        // Check for dark mode support
        assert!(html.contains("dark:"));

        // Check for accessibility
        assert!(html.contains("aria-hidden"));
    }

    #[test]
    fn test_default_implementation() {
        let placeholder1 = ImagePlaceholder::generate();
        let placeholder2 = ImagePlaceholder::default();

        assert_eq!(placeholder1, placeholder2);
    }
}
