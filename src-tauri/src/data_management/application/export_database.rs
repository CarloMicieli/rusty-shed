use crate::data_management::domain::backup_errors::DatabaseBackupError;
use crate::data_management::domain::backup_validation::validate_export_destination;
use sqlx::SqlitePool;
use std::path::Path;

/// Result returned by a successful database export operation.
pub struct ExportResult {
    /// Absolute path of the exported database file.
    pub file_path: String,
    /// Size of the exported file in bytes.
    pub file_size_bytes: u64,
    /// Time taken to complete the export in milliseconds.
    pub duration_ms: u64,
}

/// Export the database to a file using `VACUUM INTO`.
///
/// # Errors
/// Returns [`DatabaseBackupError`] if the destination is invalid, the VACUUM fails,
/// or the resulting file cannot be stat-ed.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn returns_invalid_path_when_destination_parent_does_not_exist() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite should connect");
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .expect("enable foreign keys");
        let destination = std::path::PathBuf::from("/definitely-not-existing-rusty-shed/export.db");

        let result = export_database(&pool, &destination).await;

        assert!(matches!(result, Err(DatabaseBackupError::InvalidPath(_))));
    }
}
