use crate::cloud_backup::application::operation_lock::{OperationType, try_acquire_lock};
use crate::cloud_backup::domain::{
    CloudBackupError, RestoreCompleteEvent, Result, dtos::RestoreBackupArgs,
};
use crate::cloud_backup::infrastructure::{DriveClient, is_online};
use chrono::Utc;
use flate2::read::GzDecoder;
use std::future::Future;
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
    let event = restore_backup_inner(args, db_path, drive_client, || async { is_online().await })
        .await?;

    // Emit restore-complete event so the frontend can reload
    emit_restore_complete(&app, event);

    Ok(())
}

async fn restore_backup_inner<F, Fut>(
    args: RestoreBackupArgs,
    db_path: &Path,
    drive_client: Arc<dyn DriveClient + Send + Sync>,
    online_check: F,
) -> Result<RestoreCompleteEvent>
where
    F: Fn() -> Fut,
    Fut: Future<Output = bool>,
{
    // T078: Acquire operation lock to prevent concurrent restores
    let _lock = try_acquire_lock(OperationType::Restore)?;

    validate_confirmation(&args.confirmation)?;
    ensure_online_with(online_check).await?;

    restore_backup_core(args, db_path, drive_client).await
}

fn validate_confirmation(confirmation: &str) -> Result<()> {
    if confirmation == "RESTORE" {
        Ok(())
    } else {
        Err(CloudBackupError::InvalidConfirmation)
    }
}

async fn ensure_online_with<F, Fut>(online_check: F) -> Result<()>
where
    F: Fn() -> Fut,
    Fut: Future<Output = bool>,
{
    if !online_check().await {
        return Err(CloudBackupError::OfflineError);
    }

    Ok(())
}

fn emit_restore_complete<R, E>(app: &E, event: RestoreCompleteEvent)
where
    R: tauri::Runtime,
    E: Emitter<R>,
{
    if let Err(e) = app.emit("cloud-backup://restore-complete", event) {
        tracing::warn!("Failed to emit restore-complete event: {e}");
    }
}

async fn restore_backup_core(
    args: RestoreBackupArgs,
    db_path: &Path,
    drive_client: Arc<dyn DriveClient + Send + Sync>,
) -> Result<RestoreCompleteEvent> {
    let backup_id = args.backup_id;

    // Download backup file
    let compressed_data = drive_client.download_file(&backup_id).await?;

    // Decompress the backup
    let decompressed_data = decompress_backup(&compressed_data)?;

    // Validate SQLite integrity (check magic bytes)
    validate_sqlite_integrity(&decompressed_data)?;

    // Create safety backup before replacing
    create_safety_backup(db_path).await?;

    // Replace database file
    replace_database(db_path, &decompressed_data).await?;

    Ok(RestoreCompleteEvent {
        backup_id,
        restored_at: Utc::now().to_rfc3339(),
    })
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
    use crate::cloud_backup::domain::dtos::RestoreBackupArgs;
    use crate::cloud_backup::infrastructure::mock::MockDriveClient;
    use serial_test::serial;
    use std::io::Write;
    use std::sync::Arc;
    use tempfile::tempdir;

    fn test_app_handle() -> AppHandle {
        tauri::Builder::default()
            .any_thread()
            .build(tauri::generate_context!())
            .expect("test app should build")
            .handle()
            .clone()
    }

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
        assert!(matches!(err, CloudBackupError::IntegrityCheckFailed(_)));
    }

    #[test]
    fn test_validate_sqlite_integrity_too_small() {
        let data = b"small";
        let err = validate_sqlite_integrity(data).unwrap_err();
        assert!(matches!(err, CloudBackupError::IntegrityCheckFailed(_)));
    }

    #[tokio::test]
    async fn restore_backup_core_replaces_db_with_downloaded_sqlite() {
        let tmp = tempdir().expect("tempdir should be created");
        let db_path = tmp.path().join("app.sqlite");
        std::fs::write(&db_path, b"old database bytes").expect("seed db should be writable");

        let mut sqlite_bytes = Vec::from(SQLITE_MAGIC);
        sqlite_bytes.extend_from_slice(&[1_u8; 256]);

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder
            .write_all(&sqlite_bytes)
            .expect("gzip encoder should write bytes");
        let compressed = encoder.finish().expect("gzip payload should finish");

        let mut drive = MockDriveClient::new();
        drive
            .expect_download_file()
            .withf(|backup_id| backup_id == "backup-123")
            .times(1)
            .return_once(move |_| Ok(compressed));

        let args = RestoreBackupArgs {
            backup_id: "backup-123".to_string(),
            confirmation: "RESTORE".to_string(),
        };

        let event = restore_backup_core(args, &db_path, Arc::new(drive))
            .await
            .expect("restore should succeed");

        assert_eq!(event.backup_id, "backup-123");

        let restored = std::fs::read(&db_path).expect("restored db should be readable");
        assert_eq!(restored, sqlite_bytes);

        let backups = std::fs::read_dir(tmp.path())
            .expect("tempdir should be enumerable")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().to_string())
            .filter(|name| name.starts_with("app.backup_") && name.ends_with(".sqlite"))
            .collect::<Vec<_>>();

        assert_eq!(backups.len(), 1, "a safety backup should be created");
    }

    #[tokio::test]
    #[serial]
    async fn restore_backup_inner_rejects_invalid_confirmation() {
        let tmp = tempdir().expect("tempdir should be created");
        let db_path = tmp.path().join("app.sqlite");
        std::fs::write(&db_path, b"old database bytes").expect("seed db should be writable");

        let mut drive = MockDriveClient::new();
        drive.expect_download_file().times(0);

        let args = RestoreBackupArgs {
            backup_id: "backup-123".to_string(),
            confirmation: "restore".to_string(),
        };

        let error = restore_backup_inner(args, &db_path, Arc::new(drive), || async { true })
            .await
            .expect_err("invalid confirmation should fail");

        assert!(matches!(error, CloudBackupError::InvalidConfirmation));
    }

    #[tokio::test]
    #[serial]
    async fn restore_backup_inner_rejects_offline_state() {
        let tmp = tempdir().expect("tempdir should be created");
        let db_path = tmp.path().join("app.sqlite");
        std::fs::write(&db_path, b"old database bytes").expect("seed db should be writable");

        let mut drive = MockDriveClient::new();
        drive.expect_download_file().times(0);

        let args = RestoreBackupArgs {
            backup_id: "backup-123".to_string(),
            confirmation: "RESTORE".to_string(),
        };

        let error = restore_backup_inner(args, &db_path, Arc::new(drive), || async { false })
            .await
            .expect_err("offline state should fail");

        assert!(matches!(error, CloudBackupError::OfflineError));
    }

    #[tokio::test]
    #[serial]
    async fn restore_backup_inner_replaces_database_when_online() {
        let tmp = tempdir().expect("tempdir should be created");
        let db_path = tmp.path().join("app.sqlite");
        std::fs::write(&db_path, b"old database bytes").expect("seed db should be writable");

        let mut sqlite_bytes = Vec::from(SQLITE_MAGIC);
        sqlite_bytes.extend_from_slice(&[7_u8; 256]);

        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder
            .write_all(&sqlite_bytes)
            .expect("gzip encoder should write bytes");
        let compressed = encoder.finish().expect("gzip payload should finish");

        let mut drive = MockDriveClient::new();
        drive
            .expect_download_file()
            .withf(|backup_id| backup_id == "backup-123")
            .times(1)
            .return_once(move |_| Ok(compressed));

        let args = RestoreBackupArgs {
            backup_id: "backup-123".to_string(),
            confirmation: "RESTORE".to_string(),
        };

        let event = restore_backup_inner(args, &db_path, Arc::new(drive), || async { true })
            .await
            .expect("restore should succeed");

        assert_eq!(event.backup_id, "backup-123");

        let restored = std::fs::read(&db_path).expect("restored db should be readable");
        assert_eq!(restored, sqlite_bytes);
    }

    #[tokio::test]
    #[serial]
    async fn restore_backup_emits_completion_event() {
        let app = test_app_handle();
        let event = RestoreCompleteEvent {
            backup_id: "backup-123".to_string(),
            restored_at: Utc::now().to_rfc3339(),
        };

        emit_restore_complete(&app, event);
    }
}
