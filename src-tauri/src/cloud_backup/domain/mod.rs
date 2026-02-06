/// Domain module exports for cloud backup feature
pub mod backup;
pub mod connection;
pub mod dtos;
pub mod errors;

// Re-export commonly used types
pub use backup::{
    BackupId, BackupLabel, BackupMetadata, BackupStatus, CloudBackup, OperationId, OperationStatus,
    OperationType, SyncOperation,
};
pub use connection::{ConnectionStatus, GoogleConnection};
pub use dtos::*;
pub use errors::CloudBackupError;

/// Domain result type
pub type Result<T> = std::result::Result<T, CloudBackupError>;
