use crate::cloud_backup::application;
use crate::cloud_backup::domain::*;
use crate::cloud_backup::infrastructure::{
    DriveClient, GoogleDriveClient, OAuthService, check_connectivity, create_platform_storage,
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
            CloudBackupError::NotImplemented => CommandError::BusinessRule(
                "NOT_IMPLEMENTED: This operation is not supported on this platform".into(),
            ),
            other => CommandError::unknown(other.to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Inner implementation for `cloud_backup_get_connection_status`
pub async fn cloud_backup_get_connection_status_inner(
    state: &AppState,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
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

/// Get current Google connection status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_connection_status(
    state: State<'_, AppState>,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    cloud_backup_get_connection_status_inner(&state).await
}

/// Initiate Google OAuth flow
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_connect_google(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<ConnectionStatusResponse, CommandError> {
    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
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

/// Inner implementation for `cloud_backup_disconnect_google`
pub async fn cloud_backup_disconnect_google_inner(
    state: &AppState,
) -> std::result::Result<(), CommandError> {
    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::BusinessRule("NOT_CONNECTED: Not connected".into()))?;

    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
    let oauth_service = Arc::new(OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage));

    application::disconnect_google(user_email, oauth_service)
        .await
        .map_err(CommandError::from)?;

    // Clear connected email from AppState
    state.set_connected_email(None);
    state.set_last_sync_at(None);

    Ok(())
}

/// Disconnect Google account
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_disconnect_google(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<(), CommandError> {
    cloud_backup_disconnect_google_inner(&state).await
}

/// Inner implementation for `cloud_backup_check_connectivity`
pub async fn cloud_backup_check_connectivity_inner()
-> std::result::Result<ConnectivityStatus, CommandError> {
    check_connectivity().await.map_err(CommandError::from)
}

/// Check internet connectivity
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_check_connectivity()
-> std::result::Result<ConnectivityStatus, CommandError> {
    cloud_backup_check_connectivity_inner().await
}

/// Inner implementation for `cloud_backup_sync_now`
pub async fn cloud_backup_sync_now_inner(
    app: &tauri::AppHandle,
    state: &AppState,
) -> std::result::Result<BackupListItem, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
    let tokens = storage
        .retrieve_tokens(&user_email)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    // Create Google Drive client wrapped in Arc<dyn DriveClient>
    let client: Arc<dyn DriveClient + Send + Sync> = Arc::new(GoogleDriveClient::new(
        GOOGLE_CLIENT_ID.to_string(),
        tokens.access_token_str().to_string(),
    ));

    let db_pool = state.db_pool();
    let db_path = state.db_path();

    // Mark operation as in-progress
    state.set_sync_state(None, true, 0.0, "Preparing backup…");

    // Wire progress via app event — sync_backup emits events directly
    let result =
        application::sync_backup(app, &db_pool, db_path, client, &state.import_session_store).await;

    match &result {
        Ok(item) => {
            state.set_sync_state(None, false, 100.0, "Backup complete");
            state.set_last_sync_at(Some(item.created_at.clone()));
        }
        Err(_) => {
            state.set_sync_state(None, false, 0.0, "Backup failed");
        }
    }

    result.map_err(CommandError::from)
}

/// Sync (backup) to Google Drive
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_sync_now(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<BackupListItem, CommandError> {
    cloud_backup_sync_now_inner(&app, &state).await
}

/// Inner implementation for `cloud_backup_list_backups`
pub async fn cloud_backup_list_backups_inner(
    state: &AppState,
) -> std::result::Result<BackupListResponse, CommandError> {
    // Check if online
    if !crate::cloud_backup::infrastructure::is_online().await {
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    let user_email = state
        .connected_email()
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
    let oauth_service = OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage.clone());

    let args = ListBackupsArgs {};
    application::list_backups(args, &oauth_service, storage.as_ref(), &user_email)
        .await
        .map_err(CommandError::from)
}

/// Get list of available backups
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_list_backups(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<BackupListResponse, CommandError> {
    cloud_backup_list_backups_inner(&state).await
}

/// Inner implementation for `cloud_backup_get_sync_status`
pub async fn cloud_backup_get_sync_status_inner(
    state: &AppState,
) -> std::result::Result<SyncStatusResponse, CommandError> {
    let (operation_id, is_syncing, progress_percent, status_message) = state.sync_state();
    Ok(SyncStatusResponse {
        operation_id,
        is_syncing,
        progress_percent,
        status_message,
    })
}

/// Get current sync operation status
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_get_sync_status(
    state: State<'_, AppState>,
) -> std::result::Result<SyncStatusResponse, CommandError> {
    cloud_backup_get_sync_status_inner(&state).await
}

fn validate_restore_confirmation(
    args: &RestoreBackupArgs,
) -> std::result::Result<(), CommandError> {
    if args.confirmation != "RESTORE" {
        return Err(CommandError::validation_field(
            "confirmation",
            "Must be 'RESTORE'",
        ));
    }

    Ok(())
}

fn ensure_online_for_restore(is_online: bool) -> std::result::Result<(), CommandError> {
    if !is_online {
        return Err(CommandError::from(CloudBackupError::OfflineError));
    }

    Ok(())
}

fn resolve_restore_user_email(
    connected_email: Option<String>,
) -> std::result::Result<String, CommandError> {
    connected_email.ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))
}

/// Inner implementation for `cloud_backup_restore`
pub async fn cloud_backup_restore_inner(
    app: tauri::AppHandle,
    state: &AppState,
    args: RestoreBackupArgs,
    db_path: std::path::PathBuf,
) -> std::result::Result<(), CommandError> {
    validate_restore_confirmation(&args)?;

    ensure_online_for_restore(crate::cloud_backup::infrastructure::is_online().await)?;

    let user_email = resolve_restore_user_email(state.connected_email())?;

    let storage = create_platform_storage(STORAGE_SERVICE.to_string());
    let oauth_service = OAuthService::new(GOOGLE_CLIENT_ID.to_string(), storage.clone());

    // Retrieve tokens, refreshing if expired, then build the Drive client
    let tokens = storage
        .retrieve_tokens(&user_email)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::from(CloudBackupError::NotConnected))?;

    let access_token = if tokens.is_expired() {
        let refreshed = oauth_service
            .refresh_token(&user_email)
            .await
            .map_err(CommandError::from)?;
        storage
            .store_tokens(&user_email, &refreshed)
            .await
            .map_err(CommandError::from)?;
        refreshed.access_token_str().to_string()
    } else {
        tokens.access_token_str().to_string()
    };

    let drive_client: Arc<dyn DriveClient + Send + Sync> = Arc::new(GoogleDriveClient::new(
        GOOGLE_CLIENT_ID.to_string(),
        access_token,
    ));

    // Close the pool before replacing the database file to prevent corruption.
    // The frontend will reload the app via the restore-complete event.
    state.db_pool().close().await;

    application::restore_backup(args, &db_path, drive_client, app)
        .await
        .map_err(CommandError::from)
}

/// Restore database from backup
#[tauri::command]
#[specta::specta]
pub async fn cloud_backup_restore(
    app: AppHandle,
    state: State<'_, AppState>,
    args: RestoreBackupArgs,
) -> std::result::Result<(), CommandError> {
    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::unknown(format!("Failed to resolve app data dir: {}", e)))?
        .join("database.sqlite");

    cloud_backup_restore_inner(app, &state, args, db_path).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn restore_args_with_confirmation(confirmation: &str) -> RestoreBackupArgs {
        RestoreBackupArgs {
            backup_id: "backup-1".to_string(),
            confirmation: confirmation.to_string(),
        }
    }

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

    #[test]
    fn test_error_mapping_offline_error() {
        let err = CloudBackupError::OfflineError;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("OFFLINE_ERROR")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_oauth_cancelled() {
        let err = CloudBackupError::OAuthCancelled;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("OAUTH_CANCELLED")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_oauth_timeout() {
        let err = CloudBackupError::OAuthTimeout;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("OAUTH_TIMEOUT")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_token_expired() {
        let err = CloudBackupError::TokenExpired;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("TOKEN_EXPIRED")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_import_in_progress() {
        let err = CloudBackupError::ImportInProgress;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("IMPORT_IN_PROGRESS")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_backup_not_found() {
        let err = CloudBackupError::BackupNotFound("bkp-123".to_string());
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::NotFound(msg) => assert_eq!(msg, "Backup not found: bkp-123"),
            _ => panic!("Expected NotFound"),
        }
    }

    #[test]
    fn test_error_mapping_integrity_check_failed() {
        let err = CloudBackupError::IntegrityCheckFailed("hash mismatch".to_string());
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => {
                assert!(msg.contains("INTEGRITY_ERROR"));
                assert!(msg.contains("hash mismatch"));
            }
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_not_implemented() {
        let err = CloudBackupError::NotImplemented;
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::BusinessRule(msg) => assert!(msg.contains("NOT_IMPLEMENTED")),
            _ => panic!("Expected BusinessRule"),
        }
    }

    #[test]
    fn test_error_mapping_fallback_to_unknown() {
        let err = CloudBackupError::UnexpectedError("boom".to_string());
        let cmd_err = CommandError::from(err);
        match cmd_err {
            CommandError::Unknown { message, .. } => assert!(message.contains("boom")),
            _ => panic!("Expected Unknown"),
        }
    }

    #[test]
    fn restore_validation_rejects_wrong_confirmation() {
        let args = restore_args_with_confirmation("WRONG");
        let result = validate_restore_confirmation(&args);
        assert!(matches!(result, Err(CommandError::ValidationError(_))));
    }

    #[test]
    fn restore_validation_rejects_offline_mode() {
        let result = ensure_online_for_restore(false);
        match result {
            Err(CommandError::BusinessRule(msg)) => {
                assert!(msg.contains("OFFLINE_ERROR"), "unexpected message: {msg}");
            }
            other => panic!("Expected offline business-rule error, got: {other:?}"),
        }
    }

    #[test]
    fn restore_validation_rejects_when_not_connected() {
        let result = resolve_restore_user_email(None);
        match result {
            Err(CommandError::BusinessRule(msg)) => {
                assert!(msg.contains("NOT_CONNECTED"), "unexpected message: {msg}");
            }
            other => panic!("Expected not-connected business-rule error, got: {other:?}"),
        }
    }
}
