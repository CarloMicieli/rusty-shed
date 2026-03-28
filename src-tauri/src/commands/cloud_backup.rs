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

/// Google OAuth Client ID — loaded from the `GOOGLE_CLIENT_ID` build-time
/// environment variable. Set this in your `.env` file or CI/CD pipeline.
/// Falls back to an empty string which will cause Google to reject requests.
const GOOGLE_CLIENT_ID: &str = match option_env!("GOOGLE_CLIENT_ID") {
    Some(id) => id,
    None => "",
};

const STORAGE_SERVICE: &str = "com.rusty-shed.oauth.google";

/// Map a `CloudBackupError` to a typed `CommandError`.
impl From<CloudBackupError> for CommandError {
    fn from(err: CloudBackupError) -> Self {
        match err {
            CloudBackupError::NotConnected => {
                CommandError::BusinessRule("NOT_CONNECTED: Not connected to Google Drive".into())
            }
            CloudBackupError::OfflineError => {
                CommandError::BusinessRule("OFFLINE_ERROR: No internet connection".into())
            }
            CloudBackupError::OAuthCancelled => {
                CommandError::BusinessRule("OAUTH_CANCELLED: OAuth flow was cancelled".into())
            }
            CloudBackupError::OAuthTimeout => {
                CommandError::BusinessRule("OAUTH_TIMEOUT: OAuth flow timed out".into())
            }
            CloudBackupError::TokenExpired => CommandError::BusinessRule(
                "TOKEN_EXPIRED: Please reconnect your Google account".into(),
            ),
            CloudBackupError::ImportInProgress => CommandError::BusinessRule(
                "IMPORT_IN_PROGRESS: Cannot backup while import is in progress".into(),
            ),
            CloudBackupError::InvalidConfirmation => {
                CommandError::validation_field("confirmation", "Must type 'RESTORE' to confirm")
            }
            CloudBackupError::BackupNotFound(id) => {
                CommandError::NotFound(format!("Backup not found: {id}"))
            }
            CloudBackupError::IntegrityCheckFailed(msg) => {
                CommandError::BusinessRule(format!("INTEGRITY_ERROR: {msg}"))
            }
            other => CommandError::unknown(other.to_string()),
        }
    }
}

/// Get current Google connection status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_connection_status(
    state: State<'_, AppState>,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let user_email = state.connected_email();

    let mut response = application::get_connection_status(storage, user_email)
        .await
        .map_err(CommandError::from)?;

    // Enrich with in-memory last_sync_at (persists for the session)
    if response.last_sync_at.is_none() {
        response.last_sync_at = state.last_sync_at();
    }

    Ok(response)
}

/// Initiate Google OAuth flow
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_connect_google(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage));

    let response = application::connect_google(app, oauth_service)
        .await
        .map_err(CommandError::from)?;

    // Store the connected email in AppState for use by other commands
    if let Some(email) = &response.email {
        state.set_connected_email(Some(email.clone()));
    }

    Ok(response)
}

/// Disconnect Google account
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_disconnect_google(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<(), CommandError> {
    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::BusinessRule("NOT_CONNECTED: Not connected".into()))?;

    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage));

    application::disconnect_google(user_email, oauth_service)
        .await
        .map_err(CommandError::from)?;

    // Clear connected email from AppState
    state.set_connected_email(None);
    state.set_last_sync_at(None);

    Ok(())
}

/// Check internet connectivity
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_check_connectivity()
-> std::result::Result<ConnectivityStatus, CommandError> {
    check_connectivity().await.map_err(CommandError::from)
}

/// Sync (backup) to Google Drive
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_sync_now(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<BackupListItem, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let storage = KeyringStorage::new(STORAGE_SERVICE.to_string());
    let tokens = storage
        .retrieve_tokens(&user_email)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    // Create Google Drive client
    let client = GoogleDriveClient::new(
        GOOGLE_CLIENT_ID.to_string(),
        tokens.access_token_str().to_string(),
    );

    let db_pool = state.db_pool();
    let db_path = state.db_path();

    // Mark operation as in-progress
    state.set_sync_state(None, true, 0.0, "Preparing backup…");

    // Listen for progress events to update AppState sync state
    let state_ref = state.inner();

    // Wire progress via app event — sync_backup emits events directly
    let result = application::sync_backup(
        &app,
        &db_pool,
        db_path,
        &client,
        &state.import_session_store,
    )
    .await;

    match &result {
        Ok(item) => {
            state_ref.set_sync_state(None, false, 100.0, "Backup complete");
            state_ref.set_last_sync_at(Some(item.created_at.clone()));
        }
        Err(_) => {
            state_ref.set_sync_state(None, false, 0.0, "Backup failed");
        }
    }

    result.map_err(CommandError::from)
}

/// Get list of available backups
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_list_backups(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<BackupListResponse, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage.clone());

    let args = ListBackupsArgs {};
    application::list_backups(args, &oauth_service, storage.as_ref(), &user_email)
        .await
        .map_err(CommandError::from)
}

/// Get current sync operation status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_sync_status(
    state: State<'_, AppState>,
) -> std::result::Result<SyncStatusResponse, CommandError> {
    let (operation_id, is_syncing, progress_percent, status_message) = state.sync_state();
    Ok(SyncStatusResponse {
        operation_id,
        is_syncing,
        progress_percent,
        status_message,
    })
}

/// Restore database from backup
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_restore(
    app: AppHandle,
    state: State<'_, AppState>,
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
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let storage = Arc::new(KeyringStorage::new(STORAGE_SERVICE.to_string()));
    let oauth_service = Arc::new(OAuthService::new(
        GOOGLE_CLIENT_ID.to_string(),
        storage.clone(),
    ));

    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::unknown(format!("Failed to resolve app data dir: {}", e)))?
        .join("database.sqlite");

    // Close the pool before replacing the database file to prevent corruption.
    // The frontend will reload the app via the restore-complete event.
    state.db_pool().close().await;

    application::restore_backup(
        args,
        &db_path,
        GOOGLE_CLIENT_ID,
        oauth_service.as_ref(),
        storage.as_ref(),
        &user_email,
        app,
    )
    .await
    .map_err(CommandError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_signatures() {
        assert_eq!(STORAGE_SERVICE, "com.rusty-shed.oauth.google");
    }

    #[test]
    fn test_error_mapping_not_connected() {
        let err = CloudBackupError::NotConnected;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => {
                assert!(msg.contains("NOT_CONNECTED"));
            }
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_invalid_confirmation() {
        let err = CloudBackupError::InvalidConfirmation;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::ValidationError(_) => {}
            _ => panic!("Expected ValidationError"),
        }
    }
}
