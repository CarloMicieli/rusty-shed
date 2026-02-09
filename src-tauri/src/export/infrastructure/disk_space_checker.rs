/// Disk space checker utility
use std::path::Path;

use crate::export::domain::error::ExportError;

/// Check available disk space at a given path
///
/// # Arguments
/// * `path` - The path to check available space for
///
/// # Returns
/// The available space in bytes
pub fn check_available_space(path: &Path) -> Result<u64, ExportError> {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        let metadata = std::fs::metadata(path).map_err(|e| ExportError::IoError(e))?;
        Ok(metadata.file_size())
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = std::fs::metadata(path).map_err(ExportError::IoError)?;
        Ok(metadata.blocks() * 512)
    }
}

/// Validate that there's enough disk space for export
///
/// # Arguments
/// * `path` - The destination path
/// * `estimated_size` - Estimated archive size in bytes
/// * `safety_buffer` - Safety buffer in bytes (default 100MB)
///
/// # Returns
/// Ok if there's sufficient space, or an error
pub fn validate_disk_space(path: &Path, estimated_size: u64) -> Result<(), ExportError> {
    const SAFETY_BUFFER: u64 = 100 * 1024 * 1024; // 100 MB

    let available = check_available_space(path)?;
    let required = estimated_size + SAFETY_BUFFER;

    if available < required {
        Err(ExportError::DiskSpaceError)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_disk_space_sufficient() {
        // This test will pass on any system with more than 200MB free space
        if let Ok(available) = check_available_space(Path::new("/tmp"))
            && available > 200 * 1024 * 1024
        {
            assert!(validate_disk_space(Path::new("/tmp"), 50 * 1024 * 1024).is_ok());
        }
    }
}
