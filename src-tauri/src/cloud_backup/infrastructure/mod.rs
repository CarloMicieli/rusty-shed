/// Infrastructure module exports
pub mod connectivity;
pub mod google_drive;
pub mod oauth_service;
pub mod secure_storage;

// Re-export commonly used types
pub use connectivity::{check_connectivity, is_online, start_connectivity_monitor};
pub use google_drive::{DriveFile, GoogleDriveClient, UploadedFile};
pub use oauth_service::OAuthService;
pub use secure_storage::{OAuthTokens, SecureStorage};

#[cfg(not(target_os = "android"))]
pub use secure_storage::KeyringStorage;

#[cfg(target_os = "android")]
pub use secure_storage::StrongholdStorage;
