//! Image Error Types
//!
//! Domain-level errors for image operations.

use std::fmt;

/// Errors that can occur during image operations.
///
/// These errors represent domain-level failures when working with railway model images.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageError {
    /// Image file was not found in the filesystem.
    ///
    /// Contains the path that was checked.
    NotFound(String),

    /// Path validation failed (potential security issue).
    ///
    /// This error occurs when:
    /// - Path contains traversal attempts (..)
    /// - Path has non-normal components
    /// - Path is outside allowed directory
    InvalidPath(String),

    /// I/O operation failed.
    ///
    /// Contains the error message from the underlying I/O operation.
    IoError(String),

    /// Model ID is invalid or malformed.
    ///
    /// This error occurs when the model ID doesn't match expected format.
    InvalidModelId(String),
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageError::NotFound(path) => write!(f, "Image not found: {}", path),
            ImageError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            ImageError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ImageError::InvalidModelId(msg) => write!(f, "Invalid model ID: {}", msg),
        }
    }
}

impl std::error::Error for ImageError {}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        ImageError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_error_display() {
        assert_eq!(
            ImageError::NotFound("/path/to/image.png".to_string()).to_string(),
            "Image not found: /path/to/image.png"
        );

        assert_eq!(
            ImageError::InvalidPath("path traversal detected".to_string()).to_string(),
            "Invalid path: path traversal detected"
        );

        assert_eq!(
            ImageError::IoError("permission denied".to_string()).to_string(),
            "I/O error: permission denied"
        );

        assert_eq!(
            ImageError::InvalidModelId("missing namespace".to_string()).to_string(),
            "Invalid model ID: missing namespace"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let img_err: ImageError = io_err.into();

        match img_err {
            ImageError::IoError(msg) => assert!(msg.contains("file not found")),
            _ => panic!("Expected IoError variant"),
        }
    }
}
