use crate::cloud_backup::domain::{CloudBackupError, GoogleConnection, Result};
use crate::cloud_backup::infrastructure::{OAuthTokens, SecureStorage};
use async_trait::async_trait;
use oauth2::{AuthUrl, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use std::sync::Arc;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";
#[cfg(test)]
const REDIRECT_URI: &str = "http://127.0.0.1:0"; // Port assigned dynamically — used in tests
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
    ///
    /// # Arguments
    /// * `redirect_uri` - The redirect URI where Google will send the auth code (must match registered URI)
    pub fn start_oauth_flow(
        &self,
        redirect_uri: &str,
    ) -> Result<(String, PkceCodeVerifier, CsrfToken)> {
        // Generate PKCE challenge
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // Generate CSRF token
        let csrf_token = CsrfToken::new_random();

        // Build authorization URL manually
        let auth_url = AuthUrl::new(GOOGLE_AUTH_URL.to_string())
            .map_err(|e| CloudBackupError::OAuthFailed(format!("Invalid auth URL: {}", e)))?;

        // Manually construct authorization URL
        let auth_url_string = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
            auth_url.as_str(),
            urlencoding::encode(&self.client_id),
            urlencoding::encode(redirect_uri),
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
        redirect_uri: &str,
    ) -> Result<GoogleConnection> {
        let token_data = self
            .exchange_authorization_code(&auth_code, &pkce_verifier, redirect_uri)
            .await?;
        let tokens = Self::parse_token_payload(&token_data)?;

        // Fetch user email
        let email = self.fetch_user_email(tokens.access_token_str()).await?;

        // Store tokens securely
        self.storage.store_tokens(&email, &tokens).await?;

        Ok(GoogleConnection::new(email))
    }

    async fn exchange_authorization_code(
        &self,
        auth_code: &str,
        pkce_verifier: &PkceCodeVerifier,
        redirect_uri: &str,
    ) -> Result<serde_json::Value> {
        let http_client = reqwest::Client::new();
        let params = [
            ("code", auth_code),
            ("client_id", self.client_id.as_str()),
            ("code_verifier", pkce_verifier.secret()),
            ("grant_type", "authorization_code"),
            ("redirect_uri", redirect_uri),
        ];

        let response = http_client
            .post(GOOGLE_TOKEN_URL)
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

        response
            .json()
            .await
            .map_err(|e| CloudBackupError::TokenExchangeFailed(e.to_string()))
    }

    fn parse_token_payload(token_data: &serde_json::Value) -> Result<OAuthTokens> {
        let access_token = token_data["access_token"]
            .as_str()
            .ok_or_else(|| CloudBackupError::TokenExchangeFailed("No access token".to_string()))?
            .to_string();
        let refresh_token = token_data["refresh_token"].as_str().map(|s| s.to_string());
        let expires_in = token_data["expires_in"].as_i64().unwrap_or(3600);
        let expires_at = chrono::Utc::now().timestamp() + expires_in;

        Ok(OAuthTokens::new(
            access_token,
            refresh_token,
            expires_at,
            "Bearer".to_string(),
        ))
    }

    /// Refresh expired access token
    pub async fn refresh_token(&self, user_email: &str) -> Result<OAuthTokens> {
        let transport = ReqwestRefreshTransport {
            client_id: &self.client_id,
        };

        self.refresh_token_with_transport(user_email, &transport)
            .await
    }

    async fn refresh_token_with_transport<T: RefreshTokenTransport + Sync>(
        &self,
        user_email: &str,
        transport: &T,
    ) -> Result<OAuthTokens> {
        let tokens = self
            .storage
            .retrieve_tokens(user_email)
            .await?
            .ok_or(CloudBackupError::NotConnected)?;

        let refresh_token = tokens
            .refresh_token_str()
            .ok_or(CloudBackupError::TokenExpired)?;

        let response = transport.refresh(refresh_token).await?;
        if !(200..300).contains(&response.status) {
            return Err(CloudBackupError::TokenExchangeFailed(
                "Token refresh failed".to_string(),
            ));
        }

        let new_tokens = parse_refreshed_tokens(&response.body, refresh_token)?;

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

#[derive(Debug, Clone)]
struct RefreshTokenResponse {
    status: u16,
    body: String,
}

#[async_trait]
trait RefreshTokenTransport {
    async fn refresh(&self, refresh_token: &str) -> Result<RefreshTokenResponse>;
}

struct ReqwestRefreshTransport<'a> {
    client_id: &'a str,
}

#[async_trait]
impl RefreshTokenTransport for ReqwestRefreshTransport<'_> {
    async fn refresh(&self, refresh_token: &str) -> Result<RefreshTokenResponse> {
        let http_client = reqwest::Client::new();
        let params = [
            ("client_id", self.client_id),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token"),
        ];

        let response = http_client
            .post(GOOGLE_TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        let status = response.status().as_u16();
        let body = response
            .text()
            .await
            .map_err(|e| CloudBackupError::NetworkError(e.to_string()))?;

        Ok(RefreshTokenResponse { status, body })
    }
}

fn parse_refreshed_tokens(body: &str, previous_refresh_token: &str) -> Result<OAuthTokens> {
    let token_data: serde_json::Value = serde_json::from_str(body)
        .map_err(|e| CloudBackupError::TokenExchangeFailed(e.to_string()))?;

    let access_token = token_data["access_token"]
        .as_str()
        .ok_or_else(|| CloudBackupError::TokenExchangeFailed("No access token".to_string()))?
        .to_string();

    let refresh_token = token_data["refresh_token"]
        .as_str()
        .map(|value| value.to_string())
        .or_else(|| Some(previous_refresh_token.to_string()));

    let expires_in = token_data["expires_in"].as_i64().unwrap_or(3600);
    let expires_at = chrono::Utc::now().timestamp() + expires_in;

    Ok(OAuthTokens::new(
        access_token,
        refresh_token,
        expires_at,
        "Bearer".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth2::{RedirectUrl, TokenUrl};
    use std::collections::HashMap;
    use std::sync::Mutex;

    struct TestStorage {
        data: Mutex<HashMap<String, OAuthTokens>>,
    }

    impl TestStorage {
        fn new() -> Self {
            Self {
                data: Mutex::new(HashMap::new()),
            }
        }
    }

    #[async_trait]
    impl SecureStorage for TestStorage {
        async fn store_tokens(&self, user_id: &str, tokens: &OAuthTokens) -> Result<()> {
            self.data
                .lock()
                .expect("storage lock")
                .insert(user_id.to_string(), tokens.clone());
            Ok(())
        }

        async fn retrieve_tokens(&self, user_id: &str) -> Result<Option<OAuthTokens>> {
            Ok(self
                .data
                .lock()
                .expect("storage lock")
                .get(user_id)
                .cloned())
        }

        async fn delete_tokens(&self, user_id: &str) -> Result<()> {
            self.data.lock().expect("storage lock").remove(user_id);
            Ok(())
        }

        async fn has_tokens(&self, user_id: &str) -> Result<bool> {
            Ok(self
                .data
                .lock()
                .expect("storage lock")
                .contains_key(user_id))
        }
    }

    #[derive(Clone)]
    struct FakeRefreshTransport {
        response: Result<RefreshTokenResponse>,
    }

    #[async_trait]
    impl RefreshTokenTransport for FakeRefreshTransport {
        async fn refresh(&self, _refresh_token: &str) -> Result<RefreshTokenResponse> {
            self.response.clone()
        }
    }

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

    #[test]
    fn test_parse_refreshed_tokens_keeps_previous_refresh_token_when_missing() {
        let tokens = parse_refreshed_tokens(r#"{"access_token":"new-access"}"#, "old-refresh")
            .expect("expected refreshed token parsing to succeed");

        assert_eq!(tokens.access_token_str(), "new-access");
        assert_eq!(tokens.refresh_token_str(), Some("old-refresh"));
    }

    #[tokio::test]
    async fn test_refresh_token_with_transport_success_stores_tokens() {
        let storage = Arc::new(TestStorage::new());
        let user_email = "alice@example.com";
        storage
            .store_tokens(
                user_email,
                &OAuthTokens::new(
                    "old-access".to_string(),
                    Some("old-refresh".to_string()),
                    chrono::Utc::now().timestamp() - 10,
                    "Bearer".to_string(),
                ),
            )
            .await
            .expect("seed tokens should store");

        let service = OAuthService::new("client-id".to_string(), storage.clone());
        let transport = FakeRefreshTransport {
            response: Ok(RefreshTokenResponse {
                status: 200,
                body: r#"{"access_token":"new-access","expires_in":120}"#.to_string(),
            }),
        };

        let refreshed = service
            .refresh_token_with_transport(user_email, &transport)
            .await
            .expect("refresh should succeed");

        assert_eq!(refreshed.access_token_str(), "new-access");
        assert_eq!(refreshed.refresh_token_str(), Some("old-refresh"));

        let stored = storage
            .retrieve_tokens(user_email)
            .await
            .expect("retrieve should succeed")
            .expect("tokens should exist");
        assert_eq!(stored.access_token_str(), "new-access");
    }

    #[tokio::test]
    async fn test_refresh_token_with_transport_requires_existing_connection() {
        let storage = Arc::new(TestStorage::new());
        let service = OAuthService::new("client-id".to_string(), storage);
        let transport = FakeRefreshTransport {
            response: Ok(RefreshTokenResponse {
                status: 200,
                body: r#"{"access_token":"new-access"}"#.to_string(),
            }),
        };

        let error = service
            .refresh_token_with_transport("missing@example.com", &transport)
            .await
            .expect_err("refresh should fail when no stored tokens exist");

        assert!(matches!(error, CloudBackupError::NotConnected));
    }

    #[tokio::test]
    async fn test_refresh_token_with_transport_requires_refresh_token() {
        let storage = Arc::new(TestStorage::new());
        let user_email = "alice@example.com";
        storage
            .store_tokens(
                user_email,
                &OAuthTokens::new(
                    "old-access".to_string(),
                    None,
                    chrono::Utc::now().timestamp() - 10,
                    "Bearer".to_string(),
                ),
            )
            .await
            .expect("seed tokens should store");

        let service = OAuthService::new("client-id".to_string(), storage);
        let transport = FakeRefreshTransport {
            response: Ok(RefreshTokenResponse {
                status: 200,
                body: r#"{"access_token":"new-access"}"#.to_string(),
            }),
        };

        let error = service
            .refresh_token_with_transport(user_email, &transport)
            .await
            .expect_err("refresh should fail when no refresh token exists");

        assert!(matches!(error, CloudBackupError::TokenExpired));
    }

    #[tokio::test]
    async fn test_refresh_token_with_transport_rejects_non_success_response() {
        let storage = Arc::new(TestStorage::new());
        let user_email = "alice@example.com";
        storage
            .store_tokens(
                user_email,
                &OAuthTokens::new(
                    "old-access".to_string(),
                    Some("old-refresh".to_string()),
                    chrono::Utc::now().timestamp() - 10,
                    "Bearer".to_string(),
                ),
            )
            .await
            .expect("seed tokens should store");

        let service = OAuthService::new("client-id".to_string(), storage);
        let transport = FakeRefreshTransport {
            response: Ok(RefreshTokenResponse {
                status: 401,
                body: "expired".to_string(),
            }),
        };

        let error = service
            .refresh_token_with_transport(user_email, &transport)
            .await
            .expect_err("refresh should fail for non-success status");

        assert!(matches!(error, CloudBackupError::TokenExchangeFailed(_)));
    }
}
