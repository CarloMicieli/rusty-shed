/// Get connection status use case
use crate::cloud_backup::domain::{CloudBackupError, ConnectionStatusResponse};
use crate::cloud_backup::infrastructure::SecureStorage;
use std::sync::Arc;

type Result<T> = std::result::Result<T, CloudBackupError>;

/// Get current Google connection status
pub async fn get_connection_status(
    storage: Arc<dyn SecureStorage>,
    user_email: Option<String>,
) -> Result<ConnectionStatusResponse> {
    if let Some(email) = user_email {
        // Check if tokens exist
        let has_tokens = storage.has_tokens(&email).await?;

        if has_tokens {
            // TODO: Get last sync timestamp from settings/database
            return Ok(ConnectionStatusResponse {
                is_connected: true,
                email: Some(email),
                connected_at: None, // TODO: Retrieve from storage
                last_sync_at: None, // TODO: Retrieve from settings
            });
        }
    }

    Ok(ConnectionStatusResponse {
        is_connected: false,
        email: None,
        connected_at: None,
        last_sync_at: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_not_connected_status() {
        use crate::cloud_backup::infrastructure::KeyringStorage;

        let storage = Arc::new(KeyringStorage::new("test-service".to_string()));
        let status = get_connection_status(storage, None).await.unwrap();

        assert!(!status.is_connected);
        assert!(status.email.is_none());
    }
}
