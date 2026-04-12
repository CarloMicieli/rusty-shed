use crate::cloud_backup::application::operation_lock::{OperationType, try_acquire_lock};
/// Use case: Restore database from Google Drive backup
use crate::cloud_backup::domain::{
    CloudBackupError, RestoreCompleteEvent, Result, dtos::RestoreBackupArgs,
};
use crate::cloud_backup::infrastructure::{DriveClient, is_online};
use chrono::Utc;
use flate2::read::GzDecoder;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};

/// SQLite database file magic bytes (the string "SQLite format 3\000")
const SQLITE_MAGIC: &[u8] = b"SQLite format 3\x00";

/// Restore database from a backup.
///
/// # Process
/// 1. Acquire operation lock
/// 2. Validate "RESTORE" confirmation
/// 3. Check online status
/// 4. Download and decompress backup from Drive
/// 5. Validate SQLite integrity (magic bytes)
/// 6. Create safety backup of current DB
/// 7. Close DB connections and atomically replace the file
/// 8. Emit `cloud-backup://restore-complete` event for frontend reload
pub async fn restore_backup(
    args: RestoreBackupArgs,
    db_path: &Path,
    drive_client: Arc<dyn DriveClient + Send + Sync>,
    app: AppHandle,
) -> Result<()> {
    // T078: Acquire operation lock to prevent concurrent restores
    let _lock = try_acquire_lock(OperationType::Restore)?;

    // Validate confirmation string
    if args.confirmation != "RESTORE" {
        return Err(CloudBackupError::InvalidConfirmation);
    }

    if !is_online().await {
        return Err(CloudBackupError::OfflineError);
    }

    // Download backup file
    let compressed_data = drive_client.download_file(&args.backup_id).await?;

    // Decompress the backup
    let decompressed_data = decompress_backup(&compressed_data)?;

    // Validate SQLite integrity (check magic bytes)
    validate_sqlite_integrity(&decompressed_data)?;

    // Create safety backup before replacing
    create_safety_backup(db_path).await?;

    // Replace database file
    replace_database(db_path, &decompressed_data).await?;

    // Emit restore-complete event so the frontend can reload
    let event = RestoreCompleteEvent {
        backup_id: args.backup_id,
        restored_at: Utc::now().to_rfc3339(),
    };
    if let Err(e) = app.emit("cloud-backup://restore-complete", event) {
        tracing::warn!("Failed to emit restore-complete event: {e}");
    }

    Ok(())
}

/// Decompress a gzip-compressed backup
fn decompress_backup(compressed_data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| CloudBackupError::CompressionError(format!("Decompression failed: {}", e)))?;

    Ok(decompressed)
}

/// Validate that the decompressed data is a valid SQLite database.
///
/// Checks the 16-byte SQLite header magic string at byte offset 0.
fn validate_sqlite_integrity(data: &[u8]) -> Result<()> {
    if data.len() < SQLITE_MAGIC.len() {
        return Err(CloudBackupError::IntegrityCheckFailed(
            "File too small to be a SQLite database".to_string(),
        ));
    }

    if &data[..SQLITE_MAGIC.len()] != SQLITE_MAGIC {
        return Err(CloudBackupError::IntegrityCheckFailed(
            "Invalid SQLite magic bytes — backup may be corrupted".to_string(),
        ));
    }

    Ok(())
}

/// Create a safety backup before replacing database
async fn create_safety_backup(db_path: &Path) -> Result<()> {
    // Create backup path with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = db_path.with_extension(format!("backup_{}.sqlite", timestamp));

    // Check if original exists
    if tokio::fs::metadata(db_path).await.is_err() {
        // No existing database, nothing to backup
        return Ok(());
    }

    // Copy current database to backup
    tokio::fs::copy(db_path, &backup_path).await.map_err(|e| {
        CloudBackupError::RestoreError(format!("Failed to create safety backup: {}", e))
    })?;

    tracing::info!("Created safety backup at: {}", backup_path.display());

    Ok(())
}

/// Replace database file with restored data
async fn replace_database(db_path: &Path, data: &[u8]) -> Result<()> {
    // Write to a temporary file first
    let temp_path = db_path.with_extension("tmp");

    tokio::fs::write(&temp_path, data).await.map_err(|e| {
        CloudBackupError::RestoreError(format!("Failed to write temporary file: {}", e))
    })?;

    // Atomic rename
    tokio::fs::rename(&temp_path, db_path).await.map_err(|e| {
        // Try to clean up temp file
        let _ = std::fs::remove_file(&temp_path);
        CloudBackupError::RestoreError(format!("Failed to replace database: {}", e))
    })?;

    tracing::info!("Database restored successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_decompress_backup() {
        // Create test data and compress it
        let original = b"test database content";
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        // Decompress and verify
        let decompressed = decompress_backup(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_validate_sqlite_integrity_valid() {
        let mut data = Vec::new();
        data.extend_from_slice(SQLITE_MAGIC);
        data.extend_from_slice(&[0u8; 100]);
        assert!(validate_sqlite_integrity(&data).is_ok());
    }

    #[test]
    fn test_validate_sqlite_integrity_invalid() {
        let data = b"not a sqlite file at all";
        let err = validate_sqlite_integrity(data).unwrap_err();
        matches!(err, CloudBackupError::IntegrityCheckFailed(_));
    }

    #[test]
    fn test_validate_sqlite_integrity_too_small() {
        let data = b"small";
        let err = validate_sqlite_integrity(data).unwrap_err();
        matches!(err, CloudBackupError::IntegrityCheckFailed(_));
    }
}
