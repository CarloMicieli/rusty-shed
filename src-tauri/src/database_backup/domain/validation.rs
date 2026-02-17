use super::errors::DatabaseBackupError;
use std::path::Path;

/// Validate that a destination path is writable for export
pub fn validate_export_destination(path: &Path) -> Result<(), DatabaseBackupError> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err(DatabaseBackupError::InvalidPath(format!(
                "Directory does not exist: {}",
                parent.display()
            )));
        }
    } else {
        return Err(DatabaseBackupError::InvalidPath(
            "Invalid destination path".to_string(),
        ));
    }
    Ok(())
}

/// Validate that a source file is a valid SQLite database
pub async fn validate_sqlite_file(path: &Path) -> Result<(), DatabaseBackupError> {
    if !path.exists() {
        return Err(DatabaseBackupError::InvalidPath(format!(
            "File does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(DatabaseBackupError::InvalidPath(format!(
            "Not a file: {}",
            path.display()
        )));
    }

    // Check SQLite magic bytes: first 16 bytes should be "SQLite format 3\000"
    let mut file = tokio::fs::File::open(path)
        .await
        .map_err(|e| DatabaseBackupError::FileSystemError(e.to_string()))?;

    let mut magic = [0u8; 16];
    use tokio::io::AsyncReadExt;
    let bytes_read = file
        .read(&mut magic)
        .await
        .map_err(|e| DatabaseBackupError::FileSystemError(e.to_string()))?;

    if bytes_read < 16 {
        return Err(DatabaseBackupError::InvalidDatabase(
            "File is too small to be a valid SQLite database".to_string(),
        ));
    }

    let sqlite_magic = b"SQLite format 3\x00";
    if &magic != sqlite_magic {
        return Err(DatabaseBackupError::InvalidDatabase(
            "File is not a valid SQLite database".to_string(),
        ));
    }

    Ok(())
}

/// Validate that the confirmation string matches "RESTORE"
pub fn validate_confirmation(confirmation: &str) -> Result<(), DatabaseBackupError> {
    if confirmation != "RESTORE" {
        return Err(DatabaseBackupError::ConfirmationFailed(
            "Must type 'RESTORE' to confirm".to_string(),
        ));
    }
    Ok(())
}
