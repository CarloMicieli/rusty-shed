/// Data Transfer Objects for cloud backup commands
use garde::Validate;
use serde::{Deserialize, Serialize};

/// Arguments for initiating OAuth flow
#[derive(Debug, Clone, Validate, specta::Type, Deserialize)]
pub struct ConnectGoogleArgs {
    // No arguments needed - OAuth flow is stateless
}

/// Arguments for disconnecting Google account
#[derive(Debug, Clone, Validate, specta::Type, Deserialize)]
pub struct DisconnectGoogleArgs {
    // No arguments needed
}

/// Arguments for initiating a backup
#[derive(Debug, Clone, Validate, specta::Type, Deserialize)]
pub struct SyncBackupArgs {
    // No arguments needed - backs up current state
}

/// Arguments for restoring from backup
#[derive(Debug, Clone, Validate, specta::Type, Deserialize)]
pub struct RestoreBackupArgs {
    /// The backup ID to restore from
    #[garde(length(min = 1))]
    pub backup_id: String,

    /// User confirmation (must be "RESTORE")
    #[garde(custom(validate_restore_confirmation))]
    pub confirmation: String,
}

/// Custom validator for restore confirmation
fn validate_restore_confirmation(value: &str, _context: &()) -> garde::Result {
    if value == "RESTORE" {
        Ok(())
    } else {
        Err(garde::Error::new("Must be 'RESTORE'"))
    }
}

/// Arguments for listing backups
#[derive(Debug, Clone, Validate, specta::Type, Deserialize)]
pub struct ListBackupsArgs {
    // No arguments needed - lists all available
}

/// Connection status response
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusResponse {
    pub is_connected: bool,
    pub email: Option<String>,
    pub connected_at: Option<String>, // ISO 8601
    pub last_sync_at: Option<String>, // ISO 8601
}

/// Single backup item in list
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupListItem {
    pub id: String,
    pub label: String,
    pub created_at: String, // ISO 8601
    pub size_bytes: u64,
    pub size_formatted: String, // "2.4 MB"
    pub record_count: u64,
    pub is_initial: bool,
}

/// Backup list response
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupListResponse {
    pub backups: Vec<BackupListItem>,
    pub total_count: usize,
}

/// Sync operation status (for progress tracking)
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatusResponse {
    pub operation_id: Option<String>,
    pub is_syncing: bool,
    pub progress_percent: f32,
    pub status_message: String,
}

/// Network connectivity status
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityStatus {
    pub is_online: bool,
    pub checked_at: String, // ISO 8601
}

/// Sync progress event payload
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncProgressEvent {
    pub operation_id: String,
    pub progress_percent: f32,
    pub stage: SyncStage,
}

/// Stages of sync operation
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncStage {
    Compressing,
    Uploading,
    Finalizing,
    Downloading,
    Decompressing,
    Validating,
    Replacing,
}

/// Restore complete event payload
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreCompleteEvent {
    pub backup_id: String,
    pub restored_at: String, // ISO 8601
}

/// Connectivity changed event payload
#[derive(Debug, Clone, specta::Type, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectivityChangedEvent {
    pub is_online: bool,
    pub checked_at: String, // ISO 8601
}

#[cfg(test)]
mod tests {
    use super::*;
    use garde::Validate;

    #[test]
    fn test_restore_confirmation_validation() {
        let valid = RestoreBackupArgs {
            backup_id: "test-id".to_string(),
            confirmation: "RESTORE".to_string(),
        };
        assert!(valid.validate().is_ok());

        let invalid = RestoreBackupArgs {
            backup_id: "test-id".to_string(),
            confirmation: "restore".to_string(),
        };
        assert!(invalid.validate().is_err());

        let empty = RestoreBackupArgs {
            backup_id: "".to_string(),
            confirmation: "RESTORE".to_string(),
        };
        assert!(empty.validate().is_err());
    }
}
