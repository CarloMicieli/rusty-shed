/// CloudBackup entity and related types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a backup
pub type BackupId = Uuid;

/// Unique identifier for a sync operation
pub type OperationId = Uuid;

/// Represents a single backup instance stored in Google Drive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudBackup {
    /// Unique identifier
    pub id: BackupId,

    /// Google Drive file ID
    pub file_id: String,

    /// Human-readable label
    pub label: BackupLabel,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Compressed file size in bytes
    pub size_bytes: u64,

    /// Database schema version
    pub schema_version: i32,

    /// Current backup status
    pub status: BackupStatus,

    /// Additional metadata
    pub metadata: BackupMetadata,
}

impl CloudBackup {
    /// Create a new backup instance
    pub fn new(
        file_id: String,
        label: BackupLabel,
        size_bytes: u64,
        schema_version: i32,
        metadata: BackupMetadata,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            file_id,
            label,
            created_at: Utc::now(),
            size_bytes,
            schema_version,
            status: BackupStatus::Available,
            metadata,
        }
    }

    /// Check if this is the initial backup
    pub fn is_initial(&self) -> bool {
        matches!(self.label, BackupLabel::Initial)
    }

    /// Format size as human-readable string
    pub fn size_formatted(&self) -> String {
        format_bytes(self.size_bytes)
    }
}

/// Backup label types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackupLabel {
    /// First backup ever
    Initial,

    /// User-initiated backup with timestamp
    Manual {
        #[serde(with = "chrono::serde::ts_seconds")]
        timestamp: DateTime<Utc>,
    },
}

impl BackupLabel {
    /// Create a label for the initial backup
    pub fn initial() -> Self {
        BackupLabel::Initial
    }

    /// Create a label for a manual backup
    pub fn manual(timestamp: DateTime<Utc>) -> Self {
        BackupLabel::Manual { timestamp }
    }

    /// Get display string for the label
    pub fn display(&self) -> String {
        match self {
            BackupLabel::Initial => "Initial Backup".to_string(),
            BackupLabel::Manual { timestamp } => {
                timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()
            }
        }
    }
}

/// Backup status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStatus {
    /// Ready for restore
    Available,

    /// Currently being uploaded
    Uploading,

    /// Integrity check failed
    Corrupted,
}

/// Additional backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// App version at backup time
    pub app_version: String,

    /// Number of collection items
    pub record_count: u64,

    /// OS platform
    pub platform: String,

    /// SHA-256 checksum of uncompressed DB
    pub checksum: String,
}

impl BackupMetadata {
    /// Create new metadata
    pub fn new(app_version: String, record_count: u64, platform: String, checksum: String) -> Self {
        Self {
            app_version,
            record_count,
            platform,
            checksum,
        }
    }
}

/// Represents an in-progress or completed sync operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncOperation {
    /// Unique identifier
    pub id: OperationId,

    /// Type of operation
    pub operation_type: OperationType,

    /// Operation start time
    pub started_at: DateTime<Utc>,

    /// Completion time (if finished)
    pub completed_at: Option<DateTime<Utc>>,

    /// Current status
    pub status: OperationStatus,

    /// Progress (0.0 to 1.0)
    pub progress_percent: f32,

    /// Error message if failed
    pub error_message: Option<String>,
}

impl SyncOperation {
    /// Create a new operation
    pub fn new(operation_type: OperationType) -> Self {
        Self {
            id: Uuid::new_v4(),
            operation_type,
            started_at: Utc::now(),
            completed_at: None,
            status: OperationStatus::InProgress,
            progress_percent: 0.0,
            error_message: None,
        }
    }

    /// Update progress
    pub fn update_progress(&mut self, progress: f32) {
        self.progress_percent = progress.clamp(0.0, 1.0);
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.status = OperationStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.progress_percent = 1.0;
    }

    /// Mark as failed
    pub fn fail(&mut self, error: String) {
        self.status = OperationStatus::Failed;
        self.completed_at = Some(Utc::now());
        self.error_message = Some(error);
    }

    /// Check if operation is complete
    pub fn is_finished(&self) -> bool {
        matches!(
            self.status,
            OperationStatus::Completed | OperationStatus::Failed | OperationStatus::Cancelled
        )
    }
}

/// Operation type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationType {
    Backup,
    Restore,
}

/// Operation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Format bytes as human-readable string
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, UNITS[unit_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_creation() {
        let metadata = BackupMetadata::new(
            "1.0.0".to_string(),
            100,
            "linux".to_string(),
            "abc123".to_string(),
        );
        let backup = CloudBackup::new(
            "drive_file_123".to_string(),
            BackupLabel::initial(),
            1024 * 1024,
            5,
            metadata,
        );

        assert!(backup.is_initial());
        assert_eq!(backup.file_id, "drive_file_123");
        assert_eq!(backup.schema_version, 5);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500.0 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1536 * 1024), "1.5 MB");
    }

    #[test]
    fn test_sync_operation_progress() {
        let mut op = SyncOperation::new(OperationType::Backup);
        assert_eq!(op.progress_percent, 0.0);

        op.update_progress(0.5);
        assert_eq!(op.progress_percent, 0.5);

        op.complete();
        assert_eq!(op.progress_percent, 1.0);
        assert!(op.is_finished());
    }

    #[test]
    fn test_backup_label_display() {
        let initial = BackupLabel::initial();
        assert_eq!(initial.display(), "Initial Backup");

        let manual = BackupLabel::manual(Utc::now());
        assert!(manual.display().contains("UTC"));
    }
}
