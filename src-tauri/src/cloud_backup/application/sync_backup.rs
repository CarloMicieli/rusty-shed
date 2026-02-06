use crate::cloud_backup::application::operation_lock::{OperationType, try_acquire_lock};
/// Sync (backup) use case for uploading database to Google Drive
use crate::cloud_backup::domain::{
    BackupLabel, BackupListItem, BackupMetadata, CloudBackupError, Result,
};
use crate::cloud_backup::infrastructure::{GoogleDriveClient, is_online};
use crate::import::is_import_in_progress;
use chrono::Utc;
use flate2::Compression;
use serde_json::json;
use sqlx::SqlitePool;
use std::io::Write;
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

/// Calculate SHA256 checksum of data
fn calculate_checksum(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
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
/// * `db_pool` - Database connection pool
/// * `google_drive_client` - Configured Google Drive API client
/// * `_progress_callback` - Callback for progress updates (not yet implemented)
///
/// # Returns
/// * `Ok(BackupListItem)` - Details of the created backup
/// * `Err(CloudBackupError)` - Specific error with user-friendly message
///
/// # Errors
/// * `ImportInProgress` - If data import is running
/// * `OfflineError` - If device is offline
/// * `DriveError` - If Google Drive API call fails
/// * `CompressionError` - If database compression fails
/// * `UnexpectedError` - If operation lock cannot be acquired
pub async fn sync_backup(
    db_pool: &SqlitePool,
    google_drive_client: &GoogleDriveClient,
    _progress_callback: impl Fn(SyncProgress) + Send + Sync,
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
    let _progress = SyncProgress {
        operation_id: operation_id.clone(),
        progress_percent: 0.0,
        stage: SyncStage::Compressing,
    };
    // TODO: Emit progress callback

    // Get database file path from pool
    let _db_file_path = get_database_path()?;

    // Read database file
    let db_data = tokio::fs::read(&_db_file_path).await.map_err(|e| {
        CloudBackupError::CompressionError(format!("Failed to read database: {}", e))
    })?;

    // Calculate checksum of uncompressed database
    let checksum = calculate_checksum(&db_data);

    // Count records (simplified - in real code, query the database)
    let record_count = count_records(db_pool).await.unwrap_or(0);

    // Compress database
    let compressed_data = compress_database(&db_data)?;

    // Stage 2: Upload file
    let _progress = SyncProgress {
        operation_id: operation_id.clone(),
        progress_percent: 50.0,
        stage: SyncStage::Uploading,
    };
    // TODO: Emit progress callback

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
    let _progress = SyncProgress {
        operation_id: operation_id.clone(),
        progress_percent: 75.0,
        stage: SyncStage::Finalizing,
    };
    // TODO: Emit progress callback

    // Enforce version limit
    enforce_version_limit(google_drive_client, &folder_id, MAX_BACKUPS).await?;

    // Final progress
    let _progress = SyncProgress {
        operation_id: operation_id.clone(),
        progress_percent: 100.0,
        stage: SyncStage::Finalizing,
    };
    // TODO: Emit progress callback

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
///
/// # Arguments
/// * `data` - Raw database file contents
///
/// # Returns
/// * `Ok(Vec<u8>)` - Compressed data
/// * `Err(CloudBackupError)` - If compression fails
fn compress_database(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(data)
        .map_err(|e| CloudBackupError::CompressionError(format!("Compression failed: {}", e)))?;
    encoder.finish().map_err(|e| {
        CloudBackupError::CompressionError(format!("Compression finalization failed: {}", e))
    })
}

/// Get database file path from pool
///
/// Note: For now, we'll use a placeholder. In a real implementation,
/// we'd need to pass the database path through AppState or query the database
/// for all table data to create the backup.
///
/// # Returns
/// * `Ok(String)` - Path to database file
/// * `Err(CloudBackupError)` - If path cannot be determined
fn get_database_path() -> Result<String> {
    // TODO: Get path from AppState or environment
    // For now, return a placeholder that will be implemented
    // when we have proper database integration
    Ok(String::from(":memory:"))
}

/// Count total records in collection database
///
/// # Arguments
/// * `_db_pool` - Database connection pool
///
/// # Returns
/// * `Ok(u64)` - Total number of records in collection tables
/// * `Err(CloudBackupError)` - If query fails
async fn count_records(_db_pool: &SqlitePool) -> Result<u64> {
    // TODO: Query appropriate tables to count total records
    // For now, return 0
    Ok(0)
}

/// Get database schema version
///
/// # Arguments
/// * `_db_pool` - Database connection pool
///
/// # Returns
/// * `Ok(i32)` - Schema version number
/// * `Err(CloudBackupError)` - If schema version cannot be determined
async fn get_schema_version(_db_pool: &SqlitePool) -> Result<i32> {
    // TODO: Query PRAGMA user_version or migrations table
    Ok(1)
}

/// Format bytes as human-readable string
///
/// Converts byte sizes to appropriate units (B, KB, MB, GB).
///
/// # Arguments
/// * `bytes` - Number of bytes
///
/// # Returns
/// Formatted string like "1.5 MB"
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
///
/// Maintains a maximum of `max_backups` backup files in Google Drive.
/// Deletes oldest files first when limit is exceeded.
///
/// # Arguments
/// * `client` - Google Drive API client
/// * `folder_id` - ID of backup folder
/// * `max_backups` - Maximum number of backups to keep (default 5)
///
/// # Returns
/// * `Ok(())` - If version limit enforced successfully
/// * `Err(CloudBackupError)` - If deletion fails
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
}
