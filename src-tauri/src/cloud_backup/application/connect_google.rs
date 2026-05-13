use crate::cloud_backup::domain::{CloudBackupError, ConnectionStatusResponse};
use crate::cloud_backup::infrastructure::OAuthService;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::oneshot;

type Result<T> = std::result::Result<T, CloudBackupError>;

/// Extract the `code` query parameter from an OAuth callback URL.
#[cfg(not(target_os = "android"))]
fn extract_auth_code(url: &str) -> Option<String> {
    url::Url::parse(url).ok().and_then(|parsed| {
        parsed
            .query_pairs()
            .find(|(k, _)| k == "code")
            .map(|(_, v)| v.to_string())
    })
}

/// Connect Google account via OAuth PKCE flow.
///
/// On desktop this starts a localhost callback server via `tauri-plugin-oauth`
/// to receive the authorization code after the user grants consent in the browser.
pub async fn connect_google(
    app: AppHandle,
    oauth_service: Arc<OAuthService>,
) -> Result<ConnectionStatusResponse> {
    // Create oneshot channel — the OAuth callback will send the auth code here.
    let (tx, rx) = oneshot::channel::<Result<String>>();
    let tx_cell = Arc::new(Mutex::new(Some(tx)));

    #[cfg(not(target_os = "android"))]
    let redirect_uri = {
        let tx_for_closure = tx_cell.clone();

        let port = tauri_plugin_oauth::start(move |url| {
            if let Some(tx) = tx_for_closure
                .lock()
                .expect("oauth tx lock poisoned")
                .take()
            {
                let send_result = match extract_auth_code(&url) {
                    Some(code) => tx.send(Ok(code)),
                    None => tx.send(Err(CloudBackupError::OAuthFailed(
                        "No authorization code in callback URL".to_string(),
                    ))),
                };
                if send_result.is_err() {
                    tracing::warn!("OAuth callback: receiver already dropped");
                }
            }
        })
        .map_err(|e| CloudBackupError::OAuthFailed(format!("Failed to start OAuth server: {e}")))?;

        format!("http://127.0.0.1:{port}")
    };

    #[cfg(target_os = "android")]
    let redirect_uri = {
        // Android uses a deep-link custom URI scheme. The deep-link plugin must
        // register a handler that sends the URL into `tx_cell`. This is a
        // placeholder — full Android support requires wiring the deep-link event.
        let _ = tx_cell; // suppress unused warning
        "com.rusty-shed://oauth".to_string()
    };

    // Build the authorization URL with the dynamic redirect URI.
    let (auth_url, pkce_verifier, _csrf_token) = oauth_service.start_oauth_flow(&redirect_uri)?;

    // Open the user's browser.
    let _ = tauri_plugin_opener::open_url(&auth_url, None::<&str>);
    let _ = &app; // ensure app is used (needed for Android path)

    // Wait for the callback with a 5-minute timeout.
    let auth_code = tokio::select! {
        result = rx => {
            result.map_err(|_| CloudBackupError::OAuthCancelled)??
        }
        _ = tokio::time::sleep(Duration::from_secs(300)) => {
            return Err(CloudBackupError::OAuthTimeout);
        }
    };

    // Exchange the authorization code for tokens and fetch the user email.
    let connection = oauth_service
        .complete_oauth_flow(auth_code, pkce_verifier, &redirect_uri)
        .await?;

    Ok(ConnectionStatusResponse {
        is_connected: true,
        email: Some(connection.email.clone()),
        connected_at: Some(connection.connected_at.to_rfc3339()),
        last_sync_at: None,
    })
}

#[cfg(all(test, not(target_os = "android")))]
mod tests {
    use super::*;

    #[test]
    fn test_extract_auth_code_present() {
        let url = "http://127.0.0.1:12345/?code=abc123&state=xyz";
        assert_eq!(extract_auth_code(url), Some("abc123".to_string()));
    }

    #[test]
    fn test_extract_auth_code_missing() {
        let url = "http://127.0.0.1:12345/?error=access_denied";
        assert_eq!(extract_auth_code(url), None);
    }
}
