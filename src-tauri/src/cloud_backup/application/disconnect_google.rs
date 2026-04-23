use crate::cloud_backup::domain::CloudBackupError;
use crate::cloud_backup::infrastructure::OAuthService;
use std::sync::Arc;

type Result<T> = std::result::Result<T, CloudBackupError>;

/// Disconnect Google account and revoke tokens
pub async fn disconnect_google(user_email: String, oauth_service: Arc<OAuthService>) -> Result<()> {
    // Revoke OAuth tokens
    oauth_service.revoke_token(&user_email).await?;

    tracing::info!("Google account disconnected");

    Ok(())
}

#[cfg(test)]
mod tests {
    // Tests would require mocking OAuth service
    #[test]
    fn test_disconnect_placeholder() {
        // Placeholder for future integration tests
    }
}
