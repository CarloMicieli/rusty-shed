use thiserror::Error;

/// Enumeration of all possible errors that can occur in cloud backup operations
///
/// This is the primary error type returned from cloud backup use cases.
/// It provides detailed context for each failure scenario to enable
/// meaningful user feedback and proper error handling.
#[derive(Error, Debug, Clone)]
pub enum CloudBackupError {
    /// User is not connected to a Google Drive account
    #[error("Not connected to Google Drive")]
    NotConnected,

    /// OAuth authentication failed for an unspecified reason
    #[error("OAuth authentication failed: {0}")]
    OAuthFailed(String),

    /// User cancelled the OAuth flow
    #[error("OAuth authentication cancelled by user")]
    OAuthCancelled,

    /// OAuth flow timed out waiting for user response
    #[error("OAuth authentication timed out")]
    OAuthTimeout,

    /// Failed to exchange authorization code for access tokens
    #[error("Failed to exchange authorization code for tokens: {0}")]
    TokenExchangeFailed(String),

    /// OAuth access token has expired and cannot be refreshed
    /// Requires re-authentication via OAuth flow
    #[error("OAuth token has expired")]
    TokenExpired,

    /// Failed to securely store credentials
    #[error("Failed to store credentials securely: {0}")]
    StorageError(String),

    /// Failed to read credentials from secure storage
    #[error("Failed to read from secure storage: {0}")]
    StorageReadError(String),

    /// Google Drive API returned an error
    #[error("Google Drive API error: {0}")]
    DriveError(String),

    /// Network request failed (timeout, connection error, etc.)
    #[error("Network request failed: {0}")]
    NetworkError(String),

    /// Device is offline and cannot perform cloud operations
    #[error("No internet connection")]
    OfflineError,

    /// Cannot sync backup while data import is in progress
    /// Prevents concurrent modification of the database
    #[error("Cannot sync while data import is in progress")]
    ImportInProgress,

    /// Failed to compress database file for upload
    #[error("Failed to compress database: {0}")]
    CompressionError(String),

    /// Failed to decompress backup file during restore
    #[error("Failed to decompress backup: {0}")]
    DecompressionError(String),

    /// Failed to restore database from backup
    #[error("Failed to restore database: {0}")]
    RestoreError(String),

    /// Database is locked by another process
    /// Retry the operation after ensuring no other app is using the database
    #[error("Could not acquire database lock")]
    DatabaseLocked,

    /// Database integrity check failed during restore
    /// The backup file may be corrupted or invalid
    #[error("Database integrity check failed: {0}")]
    IntegrityCheckFailed(String),

    /// Invalid or missing restore confirmation string
    /// User must type 'RESTORE' to confirm the operation
    #[error("Invalid restore confirmation. Type 'RESTORE' to confirm.")]
    InvalidConfirmation,

    /// Requested backup was not found
    #[error("Backup not found: {0}")]
    BackupNotFound(String),

    /// Maximum backup limit (5) has been reached
    /// Oldest backup must be deleted before creating a new one
    #[error("Maximum backup limit reached (5 backups)")]
    BackupLimitExceeded,

    /// The backup ID is invalid or malformed
    #[error("Invalid backup ID: {0}")]
    InvalidBackupId(String),

    /// Backup file appears to be corrupted
    #[error("Backup file is corrupted: {0}")]
    CorruptedBackup(String),

    /// Database schema version is not supported
    /// Backup was created with a newer version of the application
    #[error("Unsupported database schema version: {0}")]
    UnsupportedSchemaVersion(i32),

    /// An unexpected error occurred
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    /// The requested operation is not yet implemented on this platform
    #[error("Operation not implemented on this platform")]
    NotImplemented,
}

impl CloudBackupError {
    /// Returns true if this error should trigger a re-authentication flow
    pub fn requires_reauth(&self) -> bool {
        matches!(self, CloudBackupError::TokenExpired)
    }

    /// Returns true if this error is recoverable by retrying
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CloudBackupError::NetworkError(_) | CloudBackupError::DriveError(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_expired_requires_reauth() {
        let error = CloudBackupError::TokenExpired;
        assert!(error.requires_reauth());
    }

    #[test]
    fn test_network_error_is_retryable() {
        let error = CloudBackupError::NetworkError("timeout".to_string());
        assert!(error.is_retryable());
    }

    #[test]
    fn test_oauth_cancelled_not_retryable() {
        let error = CloudBackupError::OAuthCancelled;
        assert!(!error.is_retryable());
    }
}
