use crate::database_backup::domain::errors::DatabaseBackupError;
use crate::database_backup::domain::validation::validate_export_destination;
use sqlx::SqlitePool;
use std::path::Path;

pub struct ExportResult {
    pub file_path: String,
    pub file_size_bytes: u64,
    pub duration_ms: u64,
}

/// Export the database to a file using VACUUM INTO
pub async fn export_database(
    pool: &SqlitePool,
    destination_path: &Path,
) -> Result<ExportResult, DatabaseBackupError> {
    let start = std::time::Instant::now();

    // Validate destination
    validate_export_destination(destination_path)?;

    let dest_str = destination_path.to_str().ok_or_else(|| {
        DatabaseBackupError::InvalidPath("Invalid destination path encoding".to_string())
    })?;

    // Use VACUUM INTO to create a clean copy of the database
    sqlx::query(&format!("VACUUM INTO '{}'", dest_str.replace('\'', "''")))
        .execute(pool)
        .await
        .map_err(|e| DatabaseBackupError::DatabaseError(e.to_string()))?;

    // Verify the exported file exists and get its size
    let metadata = tokio::fs::metadata(destination_path)
        .await
        .map_err(|e| DatabaseBackupError::FileSystemError(e.to_string()))?;

    let duration_ms = start.elapsed().as_millis() as u64;

    Ok(ExportResult {
        file_path: destination_path.to_string_lossy().to_string(),
        file_size_bytes: metadata.len(),
        duration_ms,
    })
}
