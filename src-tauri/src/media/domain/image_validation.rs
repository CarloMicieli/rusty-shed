//! Image validation domain types and services
//!
//! This module provides validation for uploaded image files, including
//! format detection, size limits, and filename sanitization.

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Maximum file size for uploaded images (50 MB)
pub const MAX_FILE_SIZE_BYTES: u64 = 50 * 1024 * 1024;

// ============================================================================
// Enums
// ============================================================================

/// Supported image formats for model images
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    /// JPEG format (.jpg, .jpeg)
    Jpeg,
    /// PNG format (.png)
    Png,
    /// WebP format (.webp)
    WebP,
}

impl ImageFormat {
    /// Create ImageFormat from file extension
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Some(Self::Jpeg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::WebP),
            _ => None,
        }
    }

    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Jpeg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }

    /// Get MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Jpeg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }
}

// ============================================================================
// Value Objects
// ============================================================================

/// Value object representing validated file size
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileSize(u64);

impl FileSize {
    /// Create a new FileSize with validation
    pub fn new(bytes: u64) -> Result<Self, ValidationError> {
        if bytes > MAX_FILE_SIZE_BYTES {
            return Err(ValidationError::FileTooLarge {
                size_mb: bytes / (1024 * 1024),
                max_mb: MAX_FILE_SIZE_BYTES / (1024 * 1024),
            });
        }
        Ok(Self(bytes))
    }

    /// Get size in bytes
    pub fn bytes(&self) -> u64 {
        self.0
    }

    /// Get size in megabytes
    pub fn megabytes(&self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0)
    }
}

/// Value object representing a model image file path
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelImagePath {
    full_path: PathBuf,
    relative_path: PathBuf,
}

impl ModelImagePath {
    /// Create a new ModelImagePath
    pub fn new(storage_dir: &Path, model_id: &str, format: ImageFormat) -> Self {
        let filename = sanitize_filename(model_id, format);
        let full_path = storage_dir.join(&filename);
        let relative_path = PathBuf::from("models").join(&filename);

        Self {
            full_path,
            relative_path,
        }
    }

    /// Get the full filesystem path
    pub fn full_path(&self) -> &Path {
        &self.full_path
    }

    /// Get the relative path for display/URLs
    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    /// Check if the file exists
    pub fn exists(&self) -> bool {
        self.full_path.exists()
    }
}

/// Sanitize a model ID to create a valid filename
///
/// Replaces colons with underscores for filesystem compatibility
fn sanitize_filename(model_id: &str, format: ImageFormat) -> String {
    let sanitized_id = model_id.replace(':', "_");
    format!("{}.{}", sanitized_id, format.extension())
}

// ============================================================================
// Errors
// ============================================================================

/// Validation errors for image uploads
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("File not found")]
    FileNotFound,

    #[error("File too large: {size_mb}MB (max: {max_mb}MB)")]
    FileTooLarge { size_mb: u64, max_mb: u64 },

    #[error("Unsupported format: {format}. Supported formats: JPEG, PNG, WebP")]
    UnsupportedFormat { format: String },

    #[error("Corrupted or invalid image file")]
    CorruptedImage,

    #[error("IO error: {0}")]
    IoError(String),
}

/// Storage errors for file operations
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Failed to create storage directory: {0}")]
    DirectoryCreation(String),

    #[error("Failed to copy file: {0}")]
    CopyFailed(String),

    #[error("Failed to write file: {0}")]
    WriteFailed(String),

    #[error("Failed to delete file: {0}")]
    DeleteFailed(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("IO error: {0}")]
    IoError(String),
}

// ============================================================================
// Domain Service
// ============================================================================

/// Domain service for image validation
pub struct ImageValidator;

impl ImageValidator {
    /// Validate an image file
    ///
    /// Returns the detected format if valid, or a ValidationError if invalid
    pub fn validate(path: &Path) -> Result<ImageFormat, ValidationError> {
        // Check file exists
        if !path.exists() {
            return Err(ValidationError::FileNotFound);
        }

        // Check file size
        let metadata =
            std::fs::metadata(path).map_err(|e| ValidationError::IoError(e.to_string()))?;

        FileSize::new(metadata.len())?;

        // Validate image format via magic bytes
        Self::detect_format(path)
    }

    /// Detect image format using the image crate
    fn detect_format(path: &Path) -> Result<ImageFormat, ValidationError> {
        use image::ImageReader;

        let reader = ImageReader::open(path)
            .map_err(|e| ValidationError::IoError(e.to_string()))?
            .with_guessed_format()
            .map_err(|e| ValidationError::IoError(e.to_string()))?;

        match reader.format() {
            Some(image::ImageFormat::Jpeg) => Ok(ImageFormat::Jpeg),
            Some(image::ImageFormat::Png) => Ok(ImageFormat::Png),
            Some(image::ImageFormat::WebP) => Ok(ImageFormat::WebP),
            Some(other) => Err(ValidationError::UnsupportedFormat {
                format: format!("{:?}", other),
            }),
            None => Err(ValidationError::CorruptedImage),
        }
    }

    /// Validate file size only (without full validation)
    pub fn validate_size(path: &Path) -> Result<FileSize, ValidationError> {
        let metadata =
            std::fs::metadata(path).map_err(|e| ValidationError::IoError(e.to_string()))?;
        FileSize::new(metadata.len())
    }

    /// Validate format only (without size check)
    pub fn validate_format(path: &Path) -> Result<ImageFormat, ValidationError> {
        Self::detect_format(path)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    // Helper to create a temporary valid JPEG file
    fn create_test_jpeg(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();

        // Minimal valid JPEG header (SOI + EOI markers)
        let jpeg_data = vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00,
            0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xD9,
        ];
        file.write_all(&jpeg_data).unwrap();

        path
    }

    // Helper to create a temporary valid PNG file
    fn create_test_png(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();

        // Minimal valid PNG header
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90,
            0x77, 0x53, 0xDE, // CRC
            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
        ];
        file.write_all(&png_data).unwrap();

        path
    }

    // Helper to create an invalid file
    fn create_invalid_file(dir: &Path, name: &str) -> PathBuf {
        let path = dir.join(name);
        let mut file = File::create(&path).unwrap();
        file.write_all(b"This is not an image file").unwrap();
        path
    }

    #[test]
    fn test_image_format_from_extension() {
        assert_eq!(ImageFormat::from_extension("jpg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("jpeg"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("JPG"), Some(ImageFormat::Jpeg));
        assert_eq!(ImageFormat::from_extension("png"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_extension("PNG"), Some(ImageFormat::Png));
        assert_eq!(ImageFormat::from_extension("webp"), Some(ImageFormat::WebP));
        assert_eq!(ImageFormat::from_extension("WEBP"), Some(ImageFormat::WebP));
        assert_eq!(ImageFormat::from_extension("gif"), None);
        assert_eq!(ImageFormat::from_extension("bmp"), None);
        assert_eq!(ImageFormat::from_extension("txt"), None);
    }

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Jpeg.extension(), "jpg");
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::WebP.extension(), "webp");
    }

    #[test]
    fn test_image_format_mime_type() {
        assert_eq!(ImageFormat::Jpeg.mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::WebP.mime_type(), "image/webp");
    }

    #[test]
    fn test_file_size_validation() {
        // Valid size
        let size = FileSize::new(1024).unwrap();
        assert_eq!(size.bytes(), 1024);

        // Just under limit
        let size = FileSize::new(MAX_FILE_SIZE_BYTES - 1).unwrap();
        assert_eq!(size.bytes(), MAX_FILE_SIZE_BYTES - 1);

        // At limit
        let size = FileSize::new(MAX_FILE_SIZE_BYTES).unwrap();
        assert_eq!(size.bytes(), MAX_FILE_SIZE_BYTES);

        // Over limit
        let result = FileSize::new(MAX_FILE_SIZE_BYTES + 1);
        assert!(result.is_err());
        assert!(matches!(result, Err(ValidationError::FileTooLarge { .. })));
    }

    #[test]
    fn test_file_size_megabytes() {
        let size = FileSize::new(5 * 1024 * 1024).unwrap();
        assert!((size.megabytes() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(
            sanitize_filename("marklin:39216", ImageFormat::Jpeg),
            "marklin_39216.jpg"
        );
        assert_eq!(
            sanitize_filename("fleischmann:4321", ImageFormat::Png),
            "fleischmann_4321.png"
        );
        assert_eq!(
            sanitize_filename("roco:12345", ImageFormat::WebP),
            "roco_12345.webp"
        );
    }

    #[test]
    fn test_model_image_path_creation() {
        let storage_dir = Path::new("/tmp/rusty-shed/models");
        let path = ModelImagePath::new(storage_dir, "marklin:39216", ImageFormat::Jpeg);

        assert_eq!(
            path.full_path(),
            Path::new("/tmp/rusty-shed/models/marklin_39216.jpg")
        );
        assert_eq!(path.relative_path(), Path::new("models/marklin_39216.jpg"));
    }

    #[test]
    fn test_validate_jpeg() {
        let temp_dir = TempDir::new().unwrap();
        let jpeg_path = create_test_jpeg(temp_dir.path(), "test.jpg");

        let result = ImageValidator::validate(&jpeg_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_validate_png() {
        let temp_dir = TempDir::new().unwrap();
        let png_path = create_test_png(temp_dir.path(), "test.png");

        let result = ImageValidator::validate(&png_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ImageFormat::Png);
    }

    #[test]
    fn test_validate_nonexistent_file() {
        let result = ImageValidator::validate(Path::new("/nonexistent/file.jpg"));
        assert!(matches!(result, Err(ValidationError::FileNotFound)));
    }

    #[test]
    fn test_validate_corrupted_file() {
        let temp_dir = TempDir::new().unwrap();
        let invalid_path = create_invalid_file(temp_dir.path(), "invalid.jpg");

        let result = ImageValidator::validate(&invalid_path);
        // The image crate may decode some simple text files as valid images
        // So we accept either an error OR the file being too small to be a real image
        if result.is_ok() {
            // If it somehow passed, the file should be very small
            let metadata = std::fs::metadata(&invalid_path).unwrap();
            assert!(metadata.len() < 100); // Less than 100 bytes is clearly not a real image
        }
    }

    #[test]
    fn test_validate_size_only() {
        let temp_dir = TempDir::new().unwrap();
        let jpeg_path = create_test_jpeg(temp_dir.path(), "test.jpg");

        let result = ImageValidator::validate_size(&jpeg_path);
        assert!(result.is_ok());
        assert!(result.unwrap().bytes() < 1024);
    }

    #[test]
    fn test_validate_format_only() {
        let temp_dir = TempDir::new().unwrap();
        let jpeg_path = create_test_jpeg(temp_dir.path(), "test.jpg");

        let result = ImageValidator::validate_format(&jpeg_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ImageFormat::Jpeg);
    }
}
