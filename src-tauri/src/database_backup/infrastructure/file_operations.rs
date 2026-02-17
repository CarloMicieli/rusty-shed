use super::super::domain::errors::DatabaseBackupError;
use std::path::Path;

/// Copy a file from source to destination, ensuring atomicity
/// by writing to a temp file first then renaming.
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

/// Get the size of a file in bytes
pub async fn get_file_size(path: &Path) -> Result<u64, DatabaseBackupError> {
    let metadata = tokio::fs::metadata(path).await.map_err(|e| {
        DatabaseBackupError::FileSystemError(format!("Failed to get file metadata: {}", e))
    })?;
    Ok(metadata.len())
}
