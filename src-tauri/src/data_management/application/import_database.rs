use crate::data_management::domain::backup_errors::DatabaseBackupError;
use crate::data_management::domain::backup_validation::{
    validate_confirmation, validate_sqlite_file,
};
use crate::data_management::infrastructure::backup_file_operations::{copy_file, get_file_size};
use std::path::Path;

/// Result returned by a successful database import (restore) operation.
pub struct ImportResult {
    /// Absolute path of the source backup file that was imported.
    pub file_path: String,
    /// Size of the imported file in bytes.
    pub file_size_bytes: u64,
    /// Time taken to complete the import in milliseconds.
    pub duration_ms: u64,
    /// Whether the application must be restarted for changes to take effect.
    pub requires_restart: bool,
}

/// Import (restore) a database from a backup file by copying it over the current database.
///
/// # Errors
/// Returns [`DatabaseBackupError`] if confirmation fails, the source is not a valid SQLite db,
/// or the file copy operation fails.
pub async fn import_database(
    source_path: &Path,
    destination_path: &Path,
    confirmation: &str,
) -> Result<ImportResult, DatabaseBackupError> {
    let start = std::time::Instant::now();

    // Validate confirmation first
    validate_confirmation(confirmation)?;

    // Validate source file is a valid SQLite database
    validate_sqlite_file(source_path).await?;

    let file_size_bytes = get_file_size(source_path).await?;

    // Copy backup file over the current database
    copy_file(source_path, destination_path).await?;

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(ImportResult {
        file_path: source_path.to_string_lossy().to_string(),
        file_size_bytes,
        duration_ms,
        requires_restart: true,
    })
}
