use crate::database_backup::domain::errors::DatabaseBackupError;
use crate::database_backup::domain::validation::{validate_confirmation, validate_sqlite_file};
use crate::database_backup::infrastructure::file_operations::{copy_file, get_file_size};
use std::path::Path;

pub struct ImportResult {
    pub file_path: String,
    pub file_size_bytes: u64,
    pub duration_ms: u64,
    pub requires_restart: bool,
}

/// Import (restore) a database from a backup file by copying it over the current database
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
