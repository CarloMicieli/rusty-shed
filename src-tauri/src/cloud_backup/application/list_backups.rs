use crate::cloud_backup::domain::{
    CloudBackupError, Result,
    dtos::{BackupListItem, BackupListResponse, ListBackupsArgs},
};
use crate::cloud_backup::infrastructure::{
    google_drive::GoogleDriveClient, oauth_service::OAuthService, secure_storage::SecureStorage,
};
use chrono::DateTime;

const GOOGLE_CLIENT_ID: &str = "YOUR_CLIENT_ID_HERE"; // TODO: Load from config

/// List all backups available in Google Drive
pub async fn list_backups(
    _args: ListBackupsArgs,
    oauth_service: &OAuthService,
    storage: &dyn SecureStorage,
    user_email: &str,
) -> Result<BackupListResponse> {
    // Get access token
    let tokens = storage
        .retrieve_tokens(user_email)
        .await?
        .ok_or(CloudBackupError::NotConnected)?;

    // Refresh if expired
    let access_token = if tokens.is_expired() {
        let refreshed = oauth_service.refresh_token(user_email).await?;
        storage.store_tokens(user_email, &refreshed).await?;
        refreshed.access_token_str().to_string()
    } else {
        tokens.access_token_str().to_string()
    };

    // Create Drive client
    let drive_client = GoogleDriveClient::new(GOOGLE_CLIENT_ID.to_string(), access_token);

    // Get backup folder
    let folder_id = drive_client.get_or_create_backup_folder().await?;

    // List all files in folder
    let files = drive_client.list_files(&folder_id).await?;

    // Convert to backup list items
    let mut backups: Vec<BackupListItem> = Vec::new();
    for file in files {
        // Parse app properties
        let app_props = file.app_properties.as_ref();

        // Extract label (fallback to filename; UI can localize)
        let label = file.name.clone();

        // Extract record count
        let record_count = app_props
            .and_then(|p| p.get("recordCount"))
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);

        // Determine if initial
        let is_initial = app_props
            .and_then(|p| p.get("isInitial"))
            .and_then(|v| v.as_str())
            .map(|value| value == "true")
            .unwrap_or(false);

        // Format created_at from modified_time
        let created_at = app_props
            .and_then(|p| p.get("backupTimestamp"))
            .and_then(|v| v.as_str())
            .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
            .map(|dt| dt.to_rfc3339())
            .or_else(|| {
                file.modified_time
                    .as_ref()
                    .and_then(|t| DateTime::parse_from_rfc3339(t).ok())
                    .map(|dt| dt.to_rfc3339())
            })
            .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

        backups.push(BackupListItem {
            id: file.id,
            label,
            created_at,
            size_bytes: file.size,
            size_formatted: format_bytes(file.size),
            record_count,
            is_initial,
        });
    }

    Ok(BackupListResponse {
        total_count: backups.len(),
        backups,
    })
}

/// Format bytes to human-readable string
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 B".to_string();
    }

    let bytes_f = bytes as f64;
    let index = (bytes_f.log10() / 3.0).floor() as usize;
    let index = index.min(UNITS.len() - 1);
    let value = bytes_f / 1024_f64.powi(index as i32);

    format!("{:.1} {}", value, UNITS[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_backup::domain::CloudBackupError;
    use crate::cloud_backup::infrastructure::{OAuthTokens, SecureStorage};
    use async_trait::async_trait;
    use std::sync::Arc;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(500), "500.0 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1_048_576), "1.0 MB");
        assert_eq!(format_bytes(1_073_741_824), "1.0 GB");
    }

    struct TestStorage {
        tokens: Option<OAuthTokens>,
    }

    impl TestStorage {
        fn new(tokens: Option<OAuthTokens>) -> Self {
            Self { tokens }
        }
    }

    #[async_trait]
    impl SecureStorage for TestStorage {
        async fn store_tokens(&self, _user_id: &str, _tokens: &OAuthTokens) -> Result<()> {
            Ok(())
        }

        async fn retrieve_tokens(&self, _user_id: &str) -> Result<Option<OAuthTokens>> {
            Ok(self.tokens.clone())
        }

        async fn delete_tokens(&self, _user_id: &str) -> Result<()> {
            Ok(())
        }

        async fn has_tokens(&self, _user_id: &str) -> Result<bool> {
            Ok(self.tokens.is_some())
        }
    }

    #[tokio::test]
    async fn list_backups_returns_not_connected_when_no_tokens() {
        let storage = Arc::new(TestStorage::new(None));
        let oauth_service = OAuthService::new("test-client".to_string(), storage.clone());

        let err = list_backups(ListBackupsArgs {}, &oauth_service, storage.as_ref(), "user@test")
            .await
            .expect_err("missing tokens should fail");

        assert!(matches!(err, CloudBackupError::NotConnected));
    }

    #[tokio::test]
    async fn list_backups_returns_token_expired_when_refresh_token_missing() {
        let expired_tokens = OAuthTokens::new(
            "expired-access".to_string(),
            None,
            chrono::Utc::now().timestamp() - 60,
            "Bearer".to_string(),
        );

        let storage = Arc::new(TestStorage::new(Some(expired_tokens)));
        let oauth_service = OAuthService::new("test-client".to_string(), storage.clone());

        let err = list_backups(ListBackupsArgs {}, &oauth_service, storage.as_ref(), "user@test")
            .await
            .expect_err("expired token without refresh token should fail");

        assert!(matches!(err, CloudBackupError::TokenExpired));
    }
}
