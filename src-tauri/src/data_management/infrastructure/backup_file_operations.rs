use crate::data_management::domain::backup_errors::DatabaseBackupError;
use std::path::Path;

/// Copy a file from `source` to `destination`.
///
/// # Errors
/// Returns [`DatabaseBackupError::InvalidPath`] if the source does not exist.
/// Returns [`DatabaseBackupError::PermissionDenied`] if the OS denies access.
/// Returns [`DatabaseBackupError::FileSystemError`] for other I/O errors.
pub async fn copy_file(source: &Path, destination: &Path) -> Result<u64, DatabaseBackupError> {
    // Ensure the source exists
    if !source.exists() {
        return Err(DatabaseBackupError::InvalidPath(format!(
            "Source file does not exist: {}",
            source.display()
        )));
    }

    // Copy the file
    tokio::fs::copy(source, destination).await.map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            DatabaseBackupError::PermissionDenied(format!(
                "Permission denied copying to {}: {}",
                destination.display(),
                e
            ))
        } else {
            DatabaseBackupError::FileSystemError(format!("Failed to copy file: {}", e))
        }
    })
}

/// Get the size of a file in bytes.
///
/// # Errors
/// Returns [`DatabaseBackupError::FileSystemError`] if the metadata cannot be retrieved.
pub async fn get_file_size(path: &Path) -> Result<u64, DatabaseBackupError> {
    let metadata = tokio::fs::metadata(path).await.map_err(|e| {
        DatabaseBackupError::FileSystemError(format!("Failed to get file metadata: {}", e))
    })?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[tokio::test]
    async fn get_file_size_with_nonexistent_file_returns_error() {
        let path = Path::new("/nonexistent_file_xyz987.sqlite");
        let err = get_file_size(path).await.unwrap_err();
        assert!(matches!(err, DatabaseBackupError::FileSystemError(_)));
    }

    #[tokio::test]
    async fn copy_file_with_nonexistent_source_returns_error() {
        let source = Path::new("/nonexistent_source_xyz987.sqlite");
        let destination = Path::new("/tmp/destination.sqlite");
        let err = copy_file(source, destination).await.unwrap_err();
        assert!(matches!(err, DatabaseBackupError::InvalidPath(_)));
    }
}
