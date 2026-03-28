use super::backup_errors::DatabaseBackupError;
use std::path::Path;

/// Validate that a destination path is writable for export.
///
/// # Errors
/// Returns [`DatabaseBackupError::InvalidPath`] if the parent directory does not exist
/// or if the path has no parent component.
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

/// Validate that a source file is a valid SQLite database by checking its magic bytes.
///
/// # Errors
/// Returns [`DatabaseBackupError::InvalidPath`] if the path does not exist or is not a file.
/// Returns [`DatabaseBackupError::InvalidDatabase`] if the file is too small or not a SQLite db.
/// Returns [`DatabaseBackupError::FileSystemError`] on I/O errors.
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

/// Validate that the confirmation string matches `"RESTORE"`.
///
/// # Errors
/// Returns [`DatabaseBackupError::ConfirmationFailed`] if `confirmation` is not `"RESTORE"`.
pub fn validate_confirmation(confirmation: &str) -> Result<(), DatabaseBackupError> {
    if confirmation != "RESTORE" {
        return Err(DatabaseBackupError::ConfirmationFailed(
            "Must type 'RESTORE' to confirm".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn validate_confirmation_with_restore_succeeds() {
        assert!(validate_confirmation("RESTORE").is_ok());
    }

    #[test]
    fn validate_confirmation_with_wrong_string_fails() {
        let err = validate_confirmation("restore").unwrap_err();
        assert!(matches!(err, DatabaseBackupError::ConfirmationFailed(_)));
    }

    #[test]
    fn validate_export_destination_with_nonexistent_parent_fails() {
        let path = Path::new("/nonexistent_directory_abc123/output.sqlite");
        let err = validate_export_destination(path).unwrap_err();
        assert!(matches!(err, DatabaseBackupError::InvalidPath(_)));
    }
}
