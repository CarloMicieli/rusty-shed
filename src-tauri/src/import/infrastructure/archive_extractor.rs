use crate::import::domain::ArchiveFormat;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use tar::Archive;
use zip::ZipArchive;

/// Error type for archive extraction operations.
#[derive(Debug, Clone)]
pub enum ArchiveError {
    /// Error extracting or opening archive
    ExtractError(String),
    /// File not found in archive
    NotFound(String),
    /// Invalid archive format
    InvalidFormat(String),
}

impl std::fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExtractError(msg) => write!(f, "Archive extraction error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found in archive: {}", msg),
            Self::InvalidFormat(msg) => write!(f, "Invalid archive format: {}", msg),
        }
    }
}

impl std::error::Error for ArchiveError {}

/// Extracts and lists contents of .zip and .tar.gz archives.
#[derive(Debug)]
pub struct ArchiveExtractor;

impl ArchiveExtractor {
    /// Create a new archive extractor.
    pub fn new() -> Self {
        Self
    }

    /// Validate that a file path has an allowed image extension.
    ///
    /// Allowed extensions: .png, .jpg, .jpeg
    ///
    /// # Returns
    /// `true` if the file has a valid image extension, `false` otherwise.
    pub fn is_valid_image_extension(file_path: &str) -> bool {
        let path = Path::new(file_path);
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            matches!(ext_str.as_str(), "png" | "jpg" | "jpeg")
        } else {
            false
        }
    }

    /// Extract manifest.json from an archive.
    ///
    /// Detects format (ZIP or tar.gz) automatically based on file extension.
    ///
    /// # Errors
    /// Returns `ArchiveError` if extraction fails or manifest is missing.
    pub fn extract_manifest(archive_path: &Path) -> Result<Vec<u8>, ArchiveError> {
        let format = Self::detect_format(archive_path)?;
        match format {
            ArchiveFormat::Zip => Self::extract_manifest_from_zip(archive_path),
            ArchiveFormat::TarGz => Self::extract_manifest_from_targz(archive_path),
        }
    }

    /// List all files in an archive.
    ///
    /// # Errors
    /// Returns `ArchiveError` if archive cannot be read.
    pub fn list_files(archive_path: &Path) -> Result<Vec<String>, ArchiveError> {
        let format = Self::detect_format(archive_path)?;
        match format {
            ArchiveFormat::Zip => Self::list_files_from_zip(archive_path),
            ArchiveFormat::TarGz => Self::list_files_from_targz(archive_path),
        }
    }

    /// Extract a specific file from an archive.
    ///
    /// # Errors
    /// Returns `ArchiveError` if extraction fails.
    pub fn extract_file(archive_path: &Path, file_path: &str) -> Result<Vec<u8>, ArchiveError> {
        let format = Self::detect_format(archive_path)?;
        match format {
            ArchiveFormat::Zip => Self::extract_file_from_zip(archive_path, file_path),
            ArchiveFormat::TarGz => Self::extract_file_from_targz(archive_path, file_path),
        }
    }

    /// Detect archive format from file extension.
    fn detect_format(archive_path: &Path) -> Result<ArchiveFormat, ArchiveError> {
        match archive_path.extension() {
            Some(ext) => {
                let ext_str = ext.to_string_lossy().to_lowercase();
                match ext_str.as_str() {
                    "zip" => Ok(ArchiveFormat::Zip),
                    "gz" => Ok(ArchiveFormat::TarGz),
                    _ => Err(ArchiveError::InvalidFormat(
                        "Unsupported archive format. Use .zip or .tar.gz".to_string(),
                    )),
                }
            }
            None => Err(ArchiveError::InvalidFormat(
                "Archive file has no extension".to_string(),
            )),
        }
    }

    /// Extract manifest.json from a ZIP archive.
    fn extract_manifest_from_zip(archive_path: &Path) -> Result<Vec<u8>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open ZIP archive: {}", e))
        })?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| ArchiveError::ExtractError(format!("Failed to read ZIP: {}", e)))?;

        let mut manifest_file = archive.by_name("manifest.json").map_err(|_| {
            ArchiveError::NotFound("manifest.json not found in archive".to_string())
        })?;

        let mut content = Vec::new();
        manifest_file.read_to_end(&mut content).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to read manifest.json: {}", e))
        })?;

        Ok(content)
    }

    /// Extract manifest.json from a tar.gz archive.
    fn extract_manifest_from_targz(archive_path: &Path) -> Result<Vec<u8>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open tar.gz archive: {}", e))
        })?;
        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        for entry_result in archive.entries().map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to read tar.gz entries: {}", e))
        })? {
            let mut entry = entry_result.map_err(|e| {
                ArchiveError::ExtractError(format!("Failed to read tar entry: {}", e))
            })?;

            let is_manifest = entry
                .path()
                .map(|p| p.to_string_lossy() == "manifest.json")
                .unwrap_or(false);

            if is_manifest {
                let mut content = Vec::new();
                entry.read_to_end(&mut content).map_err(|e| {
                    ArchiveError::ExtractError(format!("Failed to read manifest.json: {}", e))
                })?;
                return Ok(content);
            }
        }

        Err(ArchiveError::NotFound(
            "manifest.json not found in archive".to_string(),
        ))
    }

    /// List all files in a ZIP archive.
    fn list_files_from_zip(archive_path: &Path) -> Result<Vec<String>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open ZIP archive: {}", e))
        })?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| ArchiveError::ExtractError(format!("Failed to read ZIP: {}", e)))?;

        let mut files = Vec::new();
        for i in 0..archive.len() {
            let file = archive.by_index(i).map_err(|e| {
                ArchiveError::ExtractError(format!("Failed to read ZIP entry: {}", e))
            })?;
            if !file.is_dir() {
                files.push(file.name().to_string());
            }
        }

        Ok(files)
    }

    /// List all files in a tar.gz archive.
    fn list_files_from_targz(archive_path: &Path) -> Result<Vec<String>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open tar.gz archive: {}", e))
        })?;
        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        let mut files = Vec::new();
        for entry_result in archive.entries().map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to read tar.gz entries: {}", e))
        })? {
            let entry = entry_result.map_err(|e| {
                ArchiveError::ExtractError(format!("Failed to read tar entry: {}", e))
            })?;

            if let Ok(path) = entry.path()
                && !entry.header().entry_type().is_dir()
            {
                files.push(path.to_string_lossy().into_owned());
            }
        }

        Ok(files)
    }

    /// Extract a specific file from a ZIP archive.
    fn extract_file_from_zip(
        archive_path: &Path,
        file_path: &str,
    ) -> Result<Vec<u8>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open ZIP archive: {}", e))
        })?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| ArchiveError::ExtractError(format!("Failed to read ZIP: {}", e)))?;

        let mut extracted_file = archive.by_name(file_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to find '{}': {}", file_path, e))
        })?;

        let mut content = Vec::new();
        extracted_file.read_to_end(&mut content).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to read '{}': {}", file_path, e))
        })?;

        Ok(content)
    }

    /// Extract a specific file from a tar.gz archive.
    fn extract_file_from_targz(
        archive_path: &Path,
        file_path: &str,
    ) -> Result<Vec<u8>, ArchiveError> {
        let file = std::fs::File::open(archive_path).map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to open tar.gz archive: {}", e))
        })?;
        let gz = GzDecoder::new(file);
        let mut archive = Archive::new(gz);

        for entry_result in archive.entries().map_err(|e| {
            ArchiveError::ExtractError(format!("Failed to read tar.gz entries: {}", e))
        })? {
            let mut entry = entry_result.map_err(|e| {
                ArchiveError::ExtractError(format!("Failed to read tar entry: {}", e))
            })?;

            let is_target = entry
                .path()
                .map(|p| p.to_string_lossy() == file_path)
                .unwrap_or(false);

            if is_target {
                let mut content = Vec::new();
                entry.read_to_end(&mut content).map_err(|e| {
                    ArchiveError::ExtractError(format!("Failed to read '{}': {}", file_path, e))
                })?;
                return Ok(content);
            }
        }

        Err(ArchiveError::NotFound(format!(
            "File '{}' not found in archive",
            file_path
        )))
    }
}

impl Default for ArchiveExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_image_extension_png() {
        assert!(ArchiveExtractor::is_valid_image_extension("image.png"));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "path/to/image.PNG"
        ));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "images/model.png"
        ));
    }

    #[test]
    fn test_is_valid_image_extension_jpg() {
        assert!(ArchiveExtractor::is_valid_image_extension("image.jpg"));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "path/to/image.JPG"
        ));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "images/model.jpg"
        ));
    }

    #[test]
    fn test_is_valid_image_extension_jpeg() {
        assert!(ArchiveExtractor::is_valid_image_extension("image.jpeg"));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "path/to/image.JPEG"
        ));
        assert!(ArchiveExtractor::is_valid_image_extension(
            "images/model.jpeg"
        ));
    }

    #[test]
    fn test_is_valid_image_extension_invalid() {
        assert!(!ArchiveExtractor::is_valid_image_extension("image.gif"));
        assert!(!ArchiveExtractor::is_valid_image_extension("image.bmp"));
        assert!(!ArchiveExtractor::is_valid_image_extension("image.webp"));
        assert!(!ArchiveExtractor::is_valid_image_extension("image.svg"));
        assert!(!ArchiveExtractor::is_valid_image_extension("manifest.json"));
        assert!(!ArchiveExtractor::is_valid_image_extension("no_extension"));
        assert!(!ArchiveExtractor::is_valid_image_extension(""));
    }

    #[test]
    fn test_detect_format_zip() {
        let path = Path::new("test.zip");
        let format = ArchiveExtractor::detect_format(path).unwrap();
        assert!(matches!(format, ArchiveFormat::Zip));
    }

    #[test]
    fn test_detect_format_targz() {
        let path = Path::new("test.tar.gz");
        let format = ArchiveExtractor::detect_format(path).unwrap();
        assert!(matches!(format, ArchiveFormat::TarGz));
    }

    #[test]
    fn test_detect_format_gz() {
        let path = Path::new("test.gz");
        let format = ArchiveExtractor::detect_format(path).unwrap();
        assert!(matches!(format, ArchiveFormat::TarGz));
    }

    #[test]
    fn test_detect_format_invalid() {
        let path = Path::new("test.rar");
        let result = ArchiveExtractor::detect_format(path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ArchiveError::InvalidFormat(_)
        ));
    }

    #[test]
    fn test_detect_format_no_extension() {
        let path = Path::new("test");
        let result = ArchiveExtractor::detect_format(path);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ArchiveError::InvalidFormat(_)
        ));
    }
}
