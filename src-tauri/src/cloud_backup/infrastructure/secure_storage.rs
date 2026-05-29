use crate::cloud_backup::domain::{CloudBackupError, Result};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// OAuth tokens stored securely
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
#[zeroize(drop)]
pub struct OAuthTokens {
    #[serde(serialize_with = "serialize_secret")]
    #[serde(deserialize_with = "deserialize_secret")]
    pub access_token: SecretString,
    #[serde(serialize_with = "serialize_secret_option")]
    #[serde(deserialize_with = "deserialize_secret_option")]
    pub refresh_token: Option<SecretString>,
    pub expires_at: i64, // Unix timestamp
    pub token_type: String,
}

fn serialize_secret<S>(secret: &SecretString, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

fn deserialize_secret<'de, D>(deserializer: D) -> std::result::Result<SecretString, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(SecretString::new(s.into_boxed_str()))
}

fn serialize_secret_option<S>(
    secret: &Option<SecretString>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match secret {
        Some(s) => serializer.serialize_some(s.expose_secret()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_secret_option<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<SecretString>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| SecretString::new(s.into_boxed_str())))
}

impl OAuthTokens {
    /// Create new tokens
    pub fn new(
        access_token: String,
        refresh_token: Option<String>,
        expires_at: i64,
        token_type: String,
    ) -> Self {
        Self {
            access_token: SecretString::new(access_token.into_boxed_str()),
            refresh_token: refresh_token.map(|t| SecretString::new(t.into_boxed_str())),
            expires_at,
            token_type,
        }
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        self.expires_at <= now
    }

    /// Get access token as string
    pub fn access_token_str(&self) -> &str {
        self.access_token.expose_secret()
    }

    /// Get refresh token as string
    pub fn refresh_token_str(&self) -> Option<&str> {
        self.refresh_token
            .as_ref()
            .map(|t: &SecretString| t.expose_secret())
    }
}

/// Trait for secure token storage
#[async_trait]
pub trait SecureStorage: Send + Sync {
    /// Store OAuth tokens
    async fn store_tokens(&self, user_id: &str, tokens: &OAuthTokens) -> Result<()>;

    /// Retrieve OAuth tokens
    async fn retrieve_tokens(&self, user_id: &str) -> Result<Option<OAuthTokens>>;

    /// Delete OAuth tokens
    async fn delete_tokens(&self, user_id: &str) -> Result<()>;

    /// Check if tokens exist for user
    async fn has_tokens(&self, user_id: &str) -> Result<bool>;
}

/// Keyring-based storage for desktop platforms (Windows, Linux)
#[cfg(not(target_os = "android"))]
pub struct KeyringStorage {
    service: String,
}

#[cfg(not(target_os = "android"))]
trait KeyringEntryOps {
    fn get_password(&self) -> std::result::Result<String, keyring::Error>;
    fn delete_credential(&self) -> std::result::Result<(), keyring::Error>;
}

#[cfg(not(target_os = "android"))]
impl KeyringEntryOps for keyring::Entry {
    fn get_password(&self) -> std::result::Result<String, keyring::Error> {
        keyring::Entry::get_password(self)
    }

    fn delete_credential(&self) -> std::result::Result<(), keyring::Error> {
        keyring::Entry::delete_credential(self)
    }
}

#[cfg(not(target_os = "android"))]
impl KeyringStorage {
    /// Create new keyring storage
    pub fn new(service: String) -> Self {
        Self { service }
    }

    fn key_name(&self, user_id: &str) -> String {
        format!("{}.{}", self.service, user_id)
    }

    fn entry_for_user(&self, user_id: &str) -> Result<keyring::Entry> {
        let key = self.key_name(user_id);
        keyring::Entry::new(&self.service, &key)
            .map_err(|e| CloudBackupError::StorageReadError(format!("Keyring entry failed: {}", e)))
    }

    fn decode_stored_tokens(json: &str) -> Result<OAuthTokens> {
        serde_json::from_str(json).map_err(|e| {
            CloudBackupError::StorageReadError(format!("Deserialization failed: {}", e))
        })
    }

    fn read_tokens_from_entry<E: KeyringEntryOps>(entry: &E) -> Result<Option<OAuthTokens>> {
        match entry.get_password() {
            Ok(json) => Ok(Some(Self::decode_stored_tokens(&json)?)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CloudBackupError::StorageReadError(format!(
                "Failed to retrieve: {}",
                e
            ))),
        }
    }

    fn delete_tokens_from_entry<E: KeyringEntryOps>(entry: &E) -> Result<()> {
        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CloudBackupError::StorageError(format!(
                "Failed to delete: {}",
                e
            ))),
        }
    }
}

#[cfg(not(target_os = "android"))]
#[async_trait]
impl SecureStorage for KeyringStorage {
    async fn store_tokens(&self, user_id: &str, tokens: &OAuthTokens) -> Result<()> {
        let key = self.key_name(user_id);
        let json = serde_json::to_string(tokens)
            .map_err(|e| CloudBackupError::StorageError(format!("Serialization failed: {}", e)))?;

        let entry = keyring::Entry::new(&self.service, &key)
            .map_err(|e| CloudBackupError::StorageError(format!("Keyring entry failed: {}", e)))?;

        entry
            .set_password(&json)
            .map_err(|e| CloudBackupError::StorageError(format!("Failed to store: {}", e)))?;

        Ok(())
    }

    async fn retrieve_tokens(&self, user_id: &str) -> Result<Option<OAuthTokens>> {
        let entry = self.entry_for_user(user_id)?;
        Self::read_tokens_from_entry(&entry)
    }

    async fn delete_tokens(&self, user_id: &str) -> Result<()> {
        let key = self.key_name(user_id);
        let entry = keyring::Entry::new(&self.service, &key)
            .map_err(|e| CloudBackupError::StorageError(format!("Keyring entry failed: {}", e)))?;

        Self::delete_tokens_from_entry(&entry)
    }

    async fn has_tokens(&self, user_id: &str) -> Result<bool> {
        Ok(self.retrieve_tokens(user_id).await?.is_some())
    }
}

/// Stronghold-based storage for Android
#[cfg(target_os = "android")]
pub struct StrongholdStorage {
    // TODO: Implement using tauri-plugin-stronghold
    // This will be implemented when Android build is needed
}

#[cfg(target_os = "android")]
impl StrongholdStorage {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(target_os = "android")]
#[async_trait]
impl SecureStorage for StrongholdStorage {
    async fn store_tokens(&self, _user_id: &str, _tokens: &OAuthTokens) -> Result<()> {
        // TODO: Implement stronghold storage
        Err(CloudBackupError::StorageError(
            "Stronghold storage not yet implemented".to_string(),
        ))
    }

    async fn retrieve_tokens(&self, _user_id: &str) -> Result<Option<OAuthTokens>> {
        // TODO: Implement stronghold retrieval
        Err(CloudBackupError::StorageReadError(
            "Stronghold storage not yet implemented".to_string(),
        ))
    }

    async fn delete_tokens(&self, _user_id: &str) -> Result<()> {
        // TODO: Implement stronghold deletion
        Err(CloudBackupError::NotImplemented)
    }

    async fn has_tokens(&self, _user_id: &str) -> Result<bool> {
        Err(CloudBackupError::NotImplemented)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(target_os = "android"))]
    use std::cell::RefCell;

    #[cfg(not(target_os = "android"))]
    struct FakeEntry {
        get_password_result: RefCell<Option<std::result::Result<String, keyring::Error>>>,
        delete_result: RefCell<Option<std::result::Result<(), keyring::Error>>>,
    }

    #[cfg(not(target_os = "android"))]
    impl KeyringEntryOps for FakeEntry {
        fn get_password(&self) -> std::result::Result<String, keyring::Error> {
            self.get_password_result
                .borrow_mut()
                .take()
                .expect("password result available")
        }

        fn delete_credential(&self) -> std::result::Result<(), keyring::Error> {
            self.delete_result
                .borrow_mut()
                .take()
                .expect("delete result available")
        }
    }

    #[test]
    fn test_token_expiry() {
        let future_time = chrono::Utc::now().timestamp() + 3600;
        let tokens = OAuthTokens::new(
            "access".to_string(),
            Some("refresh".to_string()),
            future_time,
            "Bearer".to_string(),
        );
        assert!(!tokens.is_expired());

        let past_time = chrono::Utc::now().timestamp() - 3600;
        let expired_tokens =
            OAuthTokens::new("access".to_string(), None, past_time, "Bearer".to_string());
        assert!(expired_tokens.is_expired());
    }

    #[test]
    fn test_token_access() {
        let tokens = OAuthTokens::new(
            "my_access_token".to_string(),
            Some("my_refresh_token".to_string()),
            chrono::Utc::now().timestamp() + 3600,
            "Bearer".to_string(),
        );

        assert_eq!(tokens.access_token_str(), "my_access_token");
        assert_eq!(tokens.refresh_token_str(), Some("my_refresh_token"));
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn test_decode_stored_tokens_success() {
        let json = r#"{"access_token":"abc","refresh_token":"ref","expires_at":123,"token_type":"Bearer"}"#;
        let parsed = KeyringStorage::decode_stored_tokens(json).expect("decode should succeed");

        assert_eq!(parsed.access_token_str(), "abc");
        assert_eq!(parsed.refresh_token_str(), Some("ref"));
        assert_eq!(parsed.expires_at, 123);
        assert_eq!(parsed.token_type, "Bearer");
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn test_decode_stored_tokens_invalid_json() {
        let err = KeyringStorage::decode_stored_tokens("not-json").expect_err("should fail");

        match err {
            CloudBackupError::StorageReadError(msg) => {
                assert!(msg.contains("Deserialization failed"));
            }
            other => panic!("unexpected error variant: {other:?}"),
        }
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn test_read_tokens_from_entry_success() {
        let tokens_json = serde_json::to_string(&OAuthTokens::new(
            "abc".to_string(),
            Some("ref".to_string()),
            123,
            "Bearer".to_string(),
        ))
        .expect("serialize tokens");

        let entry = FakeEntry {
            get_password_result: RefCell::new(Some(Ok(tokens_json))),
            delete_result: RefCell::new(Some(Ok(()))),
        };

        let tokens = KeyringStorage::read_tokens_from_entry(&entry)
            .expect("read should succeed")
            .expect("tokens should exist");

        assert_eq!(tokens.access_token_str(), "abc");
        assert_eq!(tokens.refresh_token_str(), Some("ref"));
    }

    #[cfg(not(target_os = "android"))]
    #[test]
    fn test_delete_tokens_from_entry_ignores_missing_entry() {
        let entry = FakeEntry {
            get_password_result: RefCell::new(Some(Err(keyring::Error::NoEntry))),
            delete_result: RefCell::new(Some(Err(keyring::Error::NoEntry))),
        };

        let result = KeyringStorage::delete_tokens_from_entry(&entry);
        assert!(result.is_ok());
    }
}
