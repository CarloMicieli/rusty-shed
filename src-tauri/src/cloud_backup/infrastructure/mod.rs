/// Infrastructure module exports
pub mod connectivity;
pub mod google_drive;
pub mod oauth_service;
pub mod secure_storage;

use async_trait::async_trait;

// Re-export commonly used types
pub use connectivity::{check_connectivity, is_online, start_connectivity_monitor};
pub use google_drive::{DriveFile, GoogleDriveClient, UploadedFile};
pub use oauth_service::OAuthService;
pub use secure_storage::{OAuthTokens, SecureStorage};

#[cfg(not(target_os = "android"))]
pub use secure_storage::KeyringStorage;

#[cfg(target_os = "android")]
pub use secure_storage::StrongholdStorage;

/// Trait defining core Google Drive operations for cloud backup management.
///
/// Abstracting over the concrete `GoogleDriveClient` enables dependency injection
/// and in-process unit testing via a `MockDriveClient` without hitting the network.
#[async_trait]
pub trait DriveClient: Send + Sync {
    /// Get or create the dedicated backup folder in appDataFolder.
    ///
    /// Returns the Drive file ID of the folder.
    async fn get_or_create_backup_folder(&self) -> crate::cloud_backup::domain::Result<String>;

    /// Upload a file into the given folder.
    ///
    /// Implementations should select between simple and resumable upload
    /// automatically based on file size.
    async fn upload_file(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> crate::cloud_backup::domain::Result<UploadedFile>;

    /// Download raw file contents by Drive file ID.
    async fn download_file(&self, file_id: &str) -> crate::cloud_backup::domain::Result<Vec<u8>>;

    /// List all non-trashed files in a folder, sorted newest first.
    async fn list_files(
        &self,
        folder_id: &str,
    ) -> crate::cloud_backup::domain::Result<Vec<DriveFile>>;

    /// Permanently delete a file (404 is treated as success).
    async fn delete_file(&self, file_id: &str) -> crate::cloud_backup::domain::Result<()>;

    /// Rename a file in Drive — used for the stage-and-commit upload pattern.
    async fn rename_file(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> crate::cloud_backup::domain::Result<()>;
}

#[async_trait]
impl DriveClient for GoogleDriveClient {
    async fn get_or_create_backup_folder(&self) -> crate::cloud_backup::domain::Result<String> {
        self.get_or_create_backup_folder().await
    }

    async fn upload_file(
        &self,
        folder_id: &str,
        file_name: &str,
        file_data: Vec<u8>,
        metadata: serde_json::Value,
    ) -> crate::cloud_backup::domain::Result<UploadedFile> {
        self.upload_file(folder_id, file_name, file_data, metadata)
            .await
    }

    async fn download_file(&self, file_id: &str) -> crate::cloud_backup::domain::Result<Vec<u8>> {
        self.download_file(file_id).await
    }

    async fn list_files(
        &self,
        folder_id: &str,
    ) -> crate::cloud_backup::domain::Result<Vec<DriveFile>> {
        self.list_files(folder_id).await
    }

    async fn delete_file(&self, file_id: &str) -> crate::cloud_backup::domain::Result<()> {
        self.delete_file(file_id).await
    }

    async fn rename_file(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> crate::cloud_backup::domain::Result<()> {
        self.rename_file(file_id, new_name).await
    }
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use crate::cloud_backup::domain::Result;
    use mockall::mock;

    mock! {
        /// Mock implementation of DriveClient for use in unit tests.
        pub DriveClient {}

        #[async_trait]
        impl DriveClient for DriveClient {
            async fn get_or_create_backup_folder(&self) -> Result<String>;
            async fn upload_file(
                &self,
                folder_id: &str,
                file_name: &str,
                file_data: Vec<u8>,
                metadata: serde_json::Value,
            ) -> Result<UploadedFile>;
            async fn download_file(&self, file_id: &str) -> Result<Vec<u8>>;
            async fn list_files(&self, folder_id: &str) -> Result<Vec<DriveFile>>;
            async fn delete_file(&self, file_id: &str) -> Result<()>;
            async fn rename_file(&self, file_id: &str, new_name: &str) -> Result<()>;
        }
    }
}
