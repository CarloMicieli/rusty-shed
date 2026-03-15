use crate::cloud_backup::application::operation_lock::{OperationType, try_acquire_lock};
/// Sync (backup) use case for uploading database to Google Drive
use crate::cloud_backup::domain::{
    BackupLabel, BackupListItem, BackupMetadata, CloudBackupError, Result, SyncProgressEvent,
    SyncStage as DomainSyncStage,
};
use crate::cloud_backup::infrastructure::{GoogleDriveClient, is_online};
use crate::data_management::interface::is_import_in_progress;
use chrono::Utc;
use flate2::Compression;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::io::Write;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

const MAX_BACKUPS: usize = 5;

/// Progress tracker for sync operations
///
/// Used to report progress updates to the UI during backup upload.
/// Includes the current operation stage and percentage complete.
#[derive(Debug, Clone)]
pub struct SyncProgress {
    /// Unique operation identifier for tracking
    pub operation_id: String,
    /// Progress percentage (0.0 - 100.0)
    pub progress_percent: f32,
    /// Current stage of the sync operation
    pub stage: SyncStage,
}

/// Stages of sync operation
///
/// Describes the different phases of a backup sync process:
/// 1. Compressing: Database file is being compressed
/// 2. Uploading: Compressed file is being uploaded to Google Drive
/// 3. Finalizing: Version limits being enforced, metadata being updated
#[derive(Debug, Clone, Copy)]
pub enum SyncStage {
    /// Database is being compressed for upload
    Compressing,
    /// Compressed backup is being uploaded to Google Drive
    Uploading,
    /// Upload complete, enforcing version limits and finalizing
    Finalizing,
}

impl std::fmt::Display for SyncStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncStage::Compressing => write!(f, "compressing"),
            SyncStage::Uploading => write!(f, "uploading"),
            SyncStage::Finalizing => write!(f, "finalizing"),
        }
    }
}

impl From<&SyncStage> for DomainSyncStage {
    fn from(stage: &SyncStage) -> Self {
        match stage {
            SyncStage::Compressing => DomainSyncStage::Compressing,
            SyncStage::Uploading => DomainSyncStage::Uploading,
            SyncStage::Finalizing => DomainSyncStage::Finalizing,
        }
    }
}

/// Get app version (should be read from config)
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get platform string
fn get_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else if cfg!(target_os = "android") {
        "android".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        "unknown".to_string()
    }
}

/// Calculate SHA-256 checksum of data, returning a lowercase hex string.
fn calculate_checksum(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Emit a progress event to the frontend.
fn emit_progress(app: &AppHandle, operation_id: &str, percent: f32, stage: &SyncStage) {
    let event = SyncProgressEvent {
        operation_id: operation_id.to_string(),
        progress_percent: percent,
        stage: DomainSyncStage::from(stage),
    };
    if let Err(e) = app.emit("cloud-backup://sync-progress", event) {
        log::warn!("Failed to emit sync progress event: {e}");
    }
}

/// Sync (backup) database to Google Drive
///
/// Creates a compressed backup of the collection database and uploads it to
/// the user's Google Drive in the app's private data folder.
///
/// # Process
/// 1. Checks that import is not in progress (BR-03)
/// 2. Verifies internet connectivity
/// 3. Acquires operation lock to prevent concurrent syncs
/// 4. Compresses the database file
/// 5. Uploads to Google Drive (creates backup folder if needed)
/// 6. Enforces version limit (max 5 backups)
/// 7. Returns backup metadata
///
/// # Arguments
/// * `app` - Tauri app handle for emitting progress events
/// * `db_pool` - Database connection pool (for metadata queries)
/// * `db_path` - Filesystem path to the SQLite database file
/// * `google_drive_client` - Configured Google Drive API client
///
/// # Returns
/// * `Ok(BackupListItem)` - Details of the created backup
/// * `Err(CloudBackupError)` - Specific error with user-friendly message
pub async fn sync_backup(
    app: &AppHandle,
    db_pool: &SqlitePool,
    db_path: &Path,
    google_drive_client: &GoogleDriveClient,
) -> Result<BackupListItem> {
    // T078: Acquire operation lock to prevent concurrent syncs
    let _lock = try_acquire_lock(OperationType::Backup)?;

    // BR-03: Prevent backup during data import
    if is_import_in_progress() {
        return Err(CloudBackupError::ImportInProgress);
    }

    if !is_online().await {
        return Err(CloudBackupError::OfflineError);
    }

    let operation_id = Uuid::new_v4().to_string();

    // Stage 1: Compress database
    emit_progress(app, &operation_id, 0.0, &SyncStage::Compressing);

    // Read database file
    let db_data = tokio::fs::read(db_path).await.map_err(|e| {
        CloudBackupError::CompressionError(format!("Failed to read database: {}", e))
    })?;

    // Calculate SHA-256 checksum of uncompressed database
    let checksum = calculate_checksum(&db_data);

    // Count records and get schema version
    let record_count = count_records(db_pool).await.unwrap_or(0);

    // Compress database
    let compressed_data = compress_database(&db_data)?;

    // Stage 2: Upload file
    emit_progress(app, &operation_id, 50.0, &SyncStage::Uploading);

    // Get or create backup folder
    let folder_id = google_drive_client.get_or_create_backup_folder().await?;

    // Determine if this is the initial backup
    let existing_files = google_drive_client.list_files(&folder_id).await?;
    let is_initial = existing_files.is_empty();

    let timestamp = Utc::now();
    let label = if is_initial {
        BackupLabel::initial()
    } else {
        BackupLabel::manual(timestamp)
    };

    // Create file name with schema version
    let schema_version = get_schema_version(db_pool).await?;
    let file_name = format!(
        "rusty_shed_backup_{}_v{}.db.gz",
        timestamp.format("%Y%m%dT%H%M%SZ"),
        schema_version
    );

    // Create metadata
    let metadata = BackupMetadata::new(
        get_app_version(),
        record_count,
        get_platform(),
        checksum.clone(),
    );

    // Prepare app properties for metadata
    let app_properties = json!({
        "appVersion": metadata.app_version,
        "dbSchemaVersion": schema_version.to_string(),
        "recordCount": record_count.to_string(),
        "backupTimestamp": timestamp.to_rfc3339(),
        "platform": metadata.platform,
        "checksum": checksum,
        "isInitial": is_initial.to_string(),
    });

    // Upload file
    let file_metadata = json!({
        "appProperties": app_properties
    });

    let uploaded = google_drive_client
        .upload_file(
            &folder_id,
            &file_name,
            compressed_data.clone(),
            file_metadata,
        )
        .await?;

    // Stage 3: Finalize (enforce version limit)
    emit_progress(app, &operation_id, 75.0, &SyncStage::Finalizing);

    // Enforce version limit
    enforce_version_limit(google_drive_client, &folder_id, MAX_BACKUPS).await?;

    // Final progress
    emit_progress(app, &operation_id, 100.0, &SyncStage::Finalizing);

    Ok(BackupListItem {
        id: uploaded.id,
        label: label.display(),
        created_at: timestamp.to_rfc3339(),
        size_bytes: uploaded.size,
        size_formatted: format_bytes(uploaded.size),
        record_count,
        is_initial,
    })
}

/// Compress database file using gzip
fn compress_database(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|e| CloudBackupError::CompressionError(format!("Compression failed: {}", e)))?;
    encoder.finish().map_err(|e| {
        CloudBackupError::CompressionError(format!("Compression finalization failed: {}", e))
    })
}

/// Count total records in collection database
async fn count_records(db_pool: &SqlitePool) -> Result<u64> {
    let row: (i64,) = sqlx::query_as(
        r#"
        SELECT SUM(cnt) FROM (
            SELECT COUNT(*) AS cnt FROM railway_models
            UNION ALL
            SELECT COUNT(*) AS cnt FROM rolling_stocks
        )
        "#,
    )
    .fetch_one(db_pool)
    .await
    .map_err(|e| CloudBackupError::UnexpectedError(format!("Failed to count records: {e}")))?;

    Ok(row.0.max(0) as u64)
}

/// Get database schema version from PRAGMA user_version
async fn get_schema_version(db_pool: &SqlitePool) -> Result<i32> {
    let row: (i32,) = sqlx::query_as("PRAGMA user_version")
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            CloudBackupError::UnexpectedError(format!("Failed to get schema version: {e}"))
        })?;

    Ok(row.0)
}

/// Format bytes as human-readable string
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_idx])
}

/// Enforce version limit by deleting oldest backups
async fn enforce_version_limit(
    client: &GoogleDriveClient,
    folder_id: &str,
    max_backups: usize,
) -> Result<()> {
    let files = client.list_files(folder_id).await?;

    if files.len() > max_backups {
        // Delete oldest files (list is sorted by modified time descending)
        for file in &files[max_backups..] {
            client.delete_file(&file.id).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512.0 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(5 * 1024 * 1024), "5.0 MB");
    }

    #[test]
    fn test_get_platform() {
        let platform = get_platform();
        assert!(!platform.is_empty());
    }

    #[test]
    fn test_compress_database() {
        let data = b"test data for compression";
        let compressed = compress_database(data).unwrap();
        assert!(!compressed.is_empty());
    }

    #[test]
    fn test_calculate_checksum_sha256() {
        let data = b"hello world";
        let checksum = calculate_checksum(data);
        assert_eq!(
            checksum.len(),
            64,
            "SHA-256 hex output must be 64 characters"
        );
        assert!(checksum.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
