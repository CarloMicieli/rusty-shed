use crate::data_management::domain::ArchiveFormat;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::{Component, Path, PathBuf};
use tar::Archive;
use zip::ZipArchive;

/// Error type for archive extraction operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ArchiveError {
    /// Error extracting or opening archive
    #[error("Archive extraction error: {0}")]
    ExtractError(String),
    /// File not found in archive
    #[error("Not found in archive: {0}")]
    NotFound(String),
    /// Invalid archive format
    #[error("Invalid archive format: {0}")]
    InvalidFormat(String),
}

/// Extracts and lists contents of .zip and .tar.gz archives.
#[derive(Debug)]
pub struct ArchiveExtractor;

impl ArchiveExtractor {
    /// Create a new archive extractor.
    pub fn new() -> Self {
        Self
    }

    /// Validate that an archive member path contains no traversal components.
    ///
    /// Rejects paths containing `..`, absolute roots, or platform-specific prefixes.
    /// Only `Component::Normal` segments are permitted — the same check used in
    /// `get_image_path` to prevent path traversal in Tauri commands.
    ///
    /// # Returns
    /// `true` if every component is a normal filename segment, `false` otherwise.
    pub fn is_safe_archive_path(member_path: &str) -> bool {
        Path::new(member_path)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
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

    /// Extract manifest.json from an archive — async wrapper using `spawn_blocking`.
    pub async fn extract_manifest_async(archive_path: PathBuf) -> Result<Vec<u8>, ArchiveError> {
        tokio::task::spawn_blocking(move || Self::extract_manifest(&archive_path))
            .await
            .map_err(|e| ArchiveError::ExtractError(format!("spawn_blocking error: {}", e)))?
    }

    /// List all files in an archive — async wrapper using `spawn_blocking`.
    pub async fn list_files_async(archive_path: PathBuf) -> Result<Vec<String>, ArchiveError> {
        tokio::task::spawn_blocking(move || Self::list_files(&archive_path))
            .await
            .map_err(|e| ArchiveError::ExtractError(format!("spawn_blocking error: {}", e)))?
    }

    /// Extract all named files from an archive in a single blocking task.
    ///
    /// Returns a `Vec` of `(filename, Result<bytes>)` pairs preserving input order.
    pub async fn extract_files_batch_async(
        archive_path: PathBuf,
        file_paths: Vec<String>,
    ) -> Result<Vec<(String, Result<Vec<u8>, ArchiveError>)>, ArchiveError> {
        tokio::task::spawn_blocking(move || {
            file_paths
                .into_iter()
                .map(|fp| {
                    let result = Self::extract_file(&archive_path, &fp);
                    (fp, result)
                })
                .collect()
        })
        .await
        .map_err(|e| ArchiveError::ExtractError(format!("spawn_blocking error: {}", e)))
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
                let name = file.name().to_string();
                if !Self::is_safe_archive_path(&name) {
                    return Err(ArchiveError::InvalidFormat(format!(
                        "Archive contains unsafe path: {}",
                        name
                    )));
                }
                files.push(name);
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
                let name = path.to_string_lossy().into_owned();
                if !Self::is_safe_archive_path(&name) {
                    return Err(ArchiveError::InvalidFormat(format!(
                        "Archive contains unsafe path: {}",
                        name
                    )));
                }
                files.push(name);
            }
        }

        Ok(files)
    }

    /// Extract a specific file from a ZIP archive.
    fn extract_file_from_zip(
        archive_path: &Path,
        file_path: &str,
    ) -> Result<Vec<u8>, ArchiveError> {
        if !Self::is_safe_archive_path(file_path) {
            return Err(ArchiveError::InvalidFormat(format!(
                "Unsafe archive member path requested: {}",
                file_path
            )));
        }
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
        if !Self::is_safe_archive_path(file_path) {
            return Err(ArchiveError::InvalidFormat(format!(
                "Unsafe archive member path requested: {}",
                file_path
            )));
        }
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
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::fs;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tar::{Builder, Header};

    fn create_targz(entries: &[(&str, &[u8])]) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let archive_path = std::env::temp_dir().join(format!("rusty-shed-{nanos}.tar.gz"));

        if archive_path.exists() {
            let _ = fs::remove_file(&archive_path);
        }

        let file = std::fs::File::create(&archive_path).expect("create archive file");
        let encoder = GzEncoder::new(file, Compression::default());
        let mut builder = Builder::new(encoder);

        for (path, content) in entries {
            let mut header = Header::new_gnu();
            header.set_mode(0o644);
            header.set_size(content.len() as u64);
            header.set_cksum();
            builder
                .append_data(&mut header, *path, &mut std::io::Cursor::new(*content))
                .expect("append tar entry");
        }

        builder.finish().expect("finish tar builder");
        let mut encoder = builder.into_inner().expect("extract encoder");
        encoder.flush().expect("flush encoder");
        encoder.finish().expect("finish gzip encoder");

        archive_path
    }

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

    #[test]
    fn test_is_safe_archive_path_normal() {
        assert!(ArchiveExtractor::is_safe_archive_path("manifest.json"));
        assert!(ArchiveExtractor::is_safe_archive_path("images/model.png"));
        assert!(ArchiveExtractor::is_safe_archive_path(
            "images/sub/photo.jpg"
        ));
    }

    #[test]
    fn test_is_safe_archive_path_traversal() {
        assert!(!ArchiveExtractor::is_safe_archive_path("../evil.txt"));
        assert!(!ArchiveExtractor::is_safe_archive_path(
            "images/../../etc/passwd"
        ));
        assert!(!ArchiveExtractor::is_safe_archive_path(
            "../../root/.ssh/id_rsa"
        ));
    }

    #[test]
    fn test_is_safe_archive_path_absolute() {
        assert!(!ArchiveExtractor::is_safe_archive_path("/etc/passwd"));
        assert!(!ArchiveExtractor::is_safe_archive_path(
            "/absolute/path.txt"
        ));
    }

    #[test]
    fn test_extract_manifest_from_targz_success() {
        let archive_path = create_targz(&[
            ("manifest.json", br#"{"version":1}"#),
            ("images/model.png", b"PNG"),
        ]);

        let manifest = ArchiveExtractor::extract_manifest(&archive_path).expect("extract manifest");
        assert_eq!(manifest, br#"{"version":1}"#);
    }

    #[test]
    fn test_extract_manifest_from_targz_not_found() {
        let archive_path = create_targz(&[("images/model.png", b"PNG")]);

        let err = ArchiveExtractor::extract_manifest(&archive_path)
            .expect_err("missing manifest should fail");
        assert!(matches!(err, ArchiveError::NotFound(_)));
    }

    #[test]
    fn test_list_files_from_targz_success() {
        let archive_path = create_targz(&[
            ("manifest.json", b"{}"),
            ("images/model.png", b"PNG"),
            ("images/sub/model.jpg", b"JPG"),
        ]);

        let mut files = ArchiveExtractor::list_files(&archive_path).expect("list files");
        files.sort();

        assert_eq!(
            files,
            vec![
                "images/model.png".to_string(),
                "images/sub/model.jpg".to_string(),
                "manifest.json".to_string(),
            ]
        );
    }

    #[test]
    fn test_extract_file_from_targz_success() {
        let archive_path = create_targz(&[("images/model.png", b"PNGDATA")]);

        let content = ArchiveExtractor::extract_file(&archive_path, "images/model.png")
            .expect("extract target file");
        assert_eq!(content, b"PNGDATA");
    }

    #[test]
    fn test_extract_file_from_targz_not_found() {
        let archive_path = create_targz(&[("images/model.png", b"PNGDATA")]);

        let err = ArchiveExtractor::extract_file(&archive_path, "images/absent.png")
            .expect_err("missing file should fail");
        assert!(matches!(err, ArchiveError::NotFound(_)));
    }

    #[test]
    fn test_extract_file_from_targz_rejects_unsafe_requested_path() {
        let archive_path = create_targz(&[("images/model.png", b"PNGDATA")]);

        let err = ArchiveExtractor::extract_file(&archive_path, "../evil.txt")
            .expect_err("unsafe requested path should fail");
        assert!(matches!(err, ArchiveError::InvalidFormat(_)));
    }
}
