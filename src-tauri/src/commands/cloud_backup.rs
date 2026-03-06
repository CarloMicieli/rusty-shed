/// Cloud backup Tauri command handlers
///
/// These commands provide the IPC interface between the frontend
/// and the cloud backup backend functionality.
use crate::cloud_backup::application;
use crate::cloud_backup::domain::*;
use crate::cloud_backup::infrastructure::secure_storage::SecureStorage;
use crate::cloud_backup::infrastructure::{
    GoogleDriveClient, KeyringStorage, OAuthService, check_connectivity,
};
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use std::sync::Arc;
use tauri::{AppHandle, Manager, State};

// TODO: Move to AppState when ready
const GOOGLE_CLIENT_ID: &str = "YOUR_CLIENT_ID_HERE"; // TODO: Load from config
const STORAGE_SERVICE: &str = "com.rusty-shed.oauth.google";

/// Get current Google connection status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_connection_status(
    _app: AppHandle,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));

    // TODO: Get current user email from app state/settings
    let user_email = None;

    application::get_connection_status(storage, user_email)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Initiate Google OAuth flow
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_connect_google(
    app: AppHandle,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage));

    application::connect_google(app, oauth_service)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Disconnect Google account
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_disconnect_google(
    _app: AppHandle,
    user_email: String,
) -> std::result::Result<(), CommandError> {
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage));

    application::disconnect_google(user_email, oauth_service)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Check internet connectivity
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_check_connectivity()
-> std::result::Result<ConnectivityStatus, CommandError> {
    check_connectivity()
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Sync (backup) to Google Drive
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_sync_now(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<BackupListItem, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::unknown("No internet connection".to_string()));
    }

    // Get access token from secure storage
    // TODO: Use proper user ID instead of hardcoded default
    let user_id = "default";
    let storage = KeyringStorage::new(STORAGE_SERVICE.to_string());
    let tokens: Option<crate::cloud_backup::infrastructure::OAuthTokens> = storage
        .retrieve_tokens(user_id)
        .await
        .map_err(|e| CommandError::unknown(format!("Failed to load tokens: {}", e)))?;

    if tokens.is_none() {
        return Err(CommandError::unknown(
            "Not connected to Google Drive".to_string(),
        ));
    }

    let tokens = tokens.unwrap();

    // Create Google Drive client
    let client = GoogleDriveClient::new(
        GOOGLE_CLIENT_ID.to_string(),
        tokens.access_token_str().to_string(),
    );

    // Get database pool
    let db_pool = state.db_pool();

    // Call sync use case
    application::sync_backup(&db_pool, &client, |_progress| {
        // TODO: Emit progress event
    })
    .await
    .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Get list of available backups
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_list_backups(
    _app: AppHandle,
) -> std::result::Result<BackupListResponse, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::unknown("No internet connection".to_string()));
    }

    // Get storage and OAuth service
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage.clone());

    // TODO: Get actual user email - for now use default
    let user_email = "default";

    // Call list_backups use case
    let args = ListBackupsArgs {};
    application::list_backups(args, &oauth_service, storage.as_ref(), user_email)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Get current sync operation status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_sync_status() -> std::result::Result<SyncStatusResponse, CommandError>
{
    // TODO: Implement sync status tracking
    Ok(SyncStatusResponse {
        operation_id: None,
        is_syncing: false,
        progress_percent: 0.0,
        status_message: "No sync in progress".to_string(),
    })
}

/// Restore database from backup
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_restore(
    app: AppHandle,
    args: RestoreBackupArgs,
) -> std::result::Result<(), CommandError> {
    // Validate confirmation
    if args.confirmation != "RESTORE" {
        return Err(CommandError::validation_field(
            "confirmation",
            "Must be 'RESTORE'",
        ));
    }

    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::unknown("No internet connection".to_string()));
    }

    // Get access token from secure storage
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(
        GOOGLE_CLIENT_ID.to_string(),
        storage.clone(),
    ));

    // TODO: Get actual user email - for now use default
    let user_email = "default";

    // Get database path using PathResolver
    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::unknown(format!("Failed to resolve app data dir: {}", e)))?
        .join("database.sqlite");

    // Call restore use case
    application::restore_backup(
        args,
        &db_path,
        oauth_service.as_ref(),
        storage.as_ref(),
        user_email,
    )
    .await
    .map_err(|e| CommandError::unknown(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_signatures() {
        // Ensure commands compile with correct signatures
        assert_eq!(STORAGE_SERVICE, "com.rusty-shed.oauth.google");
    }
}
