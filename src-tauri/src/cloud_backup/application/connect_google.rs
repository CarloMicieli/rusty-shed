/// Connect Google account use case
use crate::cloud_backup::domain::{CloudBackupError, ConnectionStatusResponse};
use crate::cloud_backup::infrastructure::OAuthService;
use oauth2::{CsrfToken, PkceCodeVerifier};
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::oneshot;

type Result<T> = std::result::Result<T, CloudBackupError>;

/// State for OAuth callback
pub struct OAuthState {
    pub verifier: PkceCodeVerifier,
    pub csrf_token: CsrfToken,
    pub tx: oneshot::Sender<Result<String>>,
}

/// Connect Google account via OAuth PKCE flow
pub async fn connect_google(
    _app: AppHandle,
    oauth_service: Arc<OAuthService>,
) -> Result<ConnectionStatusResponse> {
    // Start OAuth flow
    let (auth_url, pkce_verifier, _csrf_token) = oauth_service.start_oauth_flow()?;

    // Create channel for OAuth callback
    let (_tx, rx) = oneshot::channel();

    // Store OAuth state in app state (for callback handler)
    // TODO: Store pkce_verifier and csrf_token for callback verification

    // Open browser with authorization URL
    let _ = tauri_plugin_opener::open_url(&auth_url, None::<&str>);

    // Start local server to receive OAuth callback
    #[cfg(not(target_os = "android"))]
    let auth_code_result = {
        // Use tauri-plugin-oauth to start callback server
        // For now, wait for callback with timeout
        tokio::select! {
            result = rx => {
                result.map_err(|_| crate::cloud_backup::domain::CloudBackupError::OAuthCancelled)?
            }
            _ = tokio::time::sleep(Duration::from_secs(300)) => {
                Err(crate::cloud_backup::domain::CloudBackupError::OAuthTimeout)
            }
        }
    };

    #[cfg(target_os = "android")]
    let auth_code_result = {
        // Use deep-link plugin for Android
        // Wait for callback via custom URI scheme
        tokio::select! {
            result = rx => {
                result.map_err(|_| crate::cloud_backup::domain::CloudBackupError::OAuthCancelled)?
            }
            _ = tokio::time::sleep(Duration::from_secs(300)) => {
                Err(crate::cloud_backup::domain::CloudBackupError::OAuthTimeout)
            }
        }
    };

    let auth_code = auth_code_result?;

    // Complete OAuth flow
    let connection = oauth_service
        .complete_oauth_flow(auth_code, pkce_verifier)
        .await?;

    // Return connection status
    Ok(ConnectionStatusResponse {
        is_connected: true,
        email: Some(connection.email.clone()),
        connected_at: Some(connection.connected_at.to_rfc3339()),
        last_sync_at: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_state_creation() {
        let (tx, _rx) = oneshot::channel();
        let (_, verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let csrf = CsrfToken::new_random();

        let _state = OAuthState {
            verifier,
            csrf_token: csrf,
            tx,
        };
    }
}
