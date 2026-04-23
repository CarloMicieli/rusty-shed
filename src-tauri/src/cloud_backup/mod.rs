pub mod application;
pub mod commands;
pub mod domain;
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{
    BackupListItem, BackupListResponse, CloudBackupError, ConnectGoogleArgs,
    ConnectionStatusResponse, ConnectivityStatus, DisconnectGoogleArgs, ListBackupsArgs,
    RestoreBackupArgs, Result, SyncBackupArgs, SyncStatusResponse,
};
pub use infrastructure::SecureStorage;

#[cfg(not(target_os = "android"))]
pub use infrastructure::KeyringStorage;

#[cfg(target_os = "android")]
pub use infrastructure::StrongholdStorage;
