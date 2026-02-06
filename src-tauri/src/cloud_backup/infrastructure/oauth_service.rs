/// OAuth 2.0 service with PKCE flow for Google Drive
use crate::cloud_backup::domain::{CloudBackupError, GoogleConnection, Result};
use crate::cloud_backup::infrastructure::{OAuthTokens, SecureStorage};
use oauth2::{AuthUrl, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl};
use std::sync::Arc;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
const REDIRECT_URI: &str = "http://127.0.0.1:0"; // Port assigned dynamically
const DRIVE_APPDATA_SCOPE: &str = "https://www.googleapis.com/auth/drive.appdata";

#[allow(dead_code)] // Reserved for future timeout implementation
const OAUTH_TIMEOUT_SECS: u64 = 300; // 5 minutes

/// OAuth service for Google authentication
pub struct OAuthService {
    client_id: String,
    storage: Arc<dyn SecureStorage>,
}

impl OAuthService {
    /// Create new OAuth service
    pub fn new(client_id: String, storage: Arc<dyn SecureStorage>) -> Self {
        Self { client_id, storage }
    }

    /// Start OAuth flow and return authorization URL
    pub fn start_oauth_flow(&self) -> Result<(String, PkceCodeVerifier, CsrfToken)> {
        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate CSRF token
        let csrf_token = CsrfToken::new_random();

        // Build authorization URL manually
        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string())
            .map_err(|e| CloudBackupError::OAuthFailed(format!("Invalid auth URL: {}", e)))?;

        let redirect_url = RedirectUrl::new(REDIRECT_URI.to_string())
            .map_err(|e| CloudBackupError::OAuthFailed(format!("Invalid redirect URL: {}", e)))?;

        // Manually construct authorization URL
        let auth_url_string = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            auth_url.as_str(),
            urlencoding::encode(&self.client_id),
            urlencoding::encode(redirect_url.as_str()),
            urlencoding::encode(&format!("{} email", DRIVE_APPDATA_SCOPE)),
            csrf_token.secret(),
            pkce_challenge.as_str()
        );

        Ok((auth_url_string, pkce_verifier, csrf_token))
    }

    /// Complete OAuth flow by exchanging authorization code for tokens
    pub async fn complete_oauth_flow(
        &self,
        auth_code: String,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<GoogleConnection> {
        // Exchange authorization code for access token using manual HTTP request
        // oauth2 v5 has breaking API changes, so we'll implement token exchange manually
        let token_url = GOOGLE_TOKEN_URL;
        let http_client = reqwest::Client::new();

        let params = [
            ("code", auth_code.as_str()),
            ("client_id", self.client_id.as_str()),
            ("code_verifier", pkce_verifier.secret()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", REDIRECT_URI),
        ];

        let response = http_client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(CloudBackupError::TokenExchangeFailed(format!(
                "Token exchange failed: {}",
                error_text
            )));
        }

        let token_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudBackupError::TokenExchangeFailed(e.to_string()))?;

        // Extract token fields
        let access_token = token_data["access_token"]
            .as_str()
            .ok_or_else(|| CloudBackupError::TokenExchangeFailed("No access token".to_string()))?
            .to_string();

        let refresh_token = token_data["refresh_token"].as_str().map(|s| s.to_string());

        let expires_in = token_data["expires_in"].as_i64().unwrap_or(3600);
        let expires_at = chrono::Utc::now().timestamp() + expires_in;

        // Create OAuth tokens
        let tokens = OAuthTokens::new(
            access_token.clone(),
            refresh_token,
            expires_at,
            "Bearer".to_string(),
        );

        // Fetch user email
        let email = self.fetch_user_email(&access_token).await?;

        // Store tokens securely
        self.storage.store_tokens(&email, &tokens).await?;

        Ok(GoogleConnection::new(email))
    }

    /// Refresh expired access token
    pub async fn refresh_token(&self, user_email: &str) -> Result<OAuthTokens> {
        let tokens = self
            .storage
            .retrieve_tokens(user_email)
            .await?
            .ok_or(CloudBackupError::NotConnected)?;

        let refresh_token = tokens
            .refresh_token_str()
            .ok_or(CloudBackupError::TokenExpired)?;

        let http_client = reqwest::Client::new();

        let params = [
            ("client_id", self.client_id.as_str()),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = http_client
            .post(GOOGLE_TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(CloudBackupError::TokenExchangeFailed(
                "Token refresh failed".to_string(),
            ));
        }

        let token_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudBackupError::TokenExchangeFailed(e.to_string()))?;

        let access_token = token_data["access_token"]
            .as_str()
            .ok_or_else(|| CloudBackupError::TokenExchangeFailed("No access token".to_string()))?
            .to_string();

        let new_refresh_token = token_data["refresh_token"]
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| Some(refresh_token.to_string())); // Keep old if not provided

        let expires_in = token_data["expires_in"].as_i64().unwrap_or(3600);
        let expires_at = chrono::Utc::now().timestamp() + expires_in;

        let new_tokens = OAuthTokens::new(
            access_token,
            new_refresh_token,
            expires_at,
            "Bearer".to_string(),
        );

        self.storage.store_tokens(user_email, &new_tokens).await?;

        Ok(new_tokens)
    }

    /// Revoke OAuth tokens
    pub async fn revoke_token(&self, user_email: &str) -> Result<()> {
        // Best effort - don't fail if tokens don't exist
        if let Ok(Some(tokens)) = self.storage.retrieve_tokens(user_email).await {
            // Call Google's revoke endpoint
            let client = reqwest::Client::new();
            let _ = client
                .post("https://oauth2.googleapis.com/revoke")
                .form(&[("token", tokens.access_token_str())])
                .send()
                .await;
        }

        self.storage.delete_tokens(user_email).await?;
        Ok(())
    }

    /// Fetch user email from Google API
    async fn fetch_user_email(&self, access_token: &str) -> Result<String> {
        let client = reqwest::Client::new();
        let response = client
            .get(GOOGLE_USERINFO_URL)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(CloudBackupError::OAuthFailed(format!(
                "Failed to fetch user info: {}",
                response.status()
            )));
        }

        let user_info: serde_json::Value = response
            .json()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        user_info
            .get("email")
            .and_then(|e| e.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| CloudBackupError::OAuthFailed("No email in user info".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth2::TokenUrl;

    #[test]
    fn test_oauth_urls() {
        assert!(AuthUrl::new(GOOGLE_AUTH_URL.to_string()).is_ok());
        assert!(TokenUrl::new(GOOGLE_TOKEN_URL.to_string()).is_ok());
        assert!(RedirectUrl::new(REDIRECT_URI.to_string()).is_ok());
    }

    #[test]
    fn test_pkce_generation() {
        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
        assert!(!verifier.secret().is_empty());
        assert!(!challenge.as_str().is_empty());
    }
}
