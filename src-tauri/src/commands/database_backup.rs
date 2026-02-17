/// Database backup Tauri command handlers
///
/// These commands provide the IPC interface between the frontend
/// and the database backup/restore functionality.
use crate::core::infrastructure::error::CommandError;
use crate::database_backup::application;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct ExportDatabaseArgs {
    pub destination_path: String,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ExportDatabaseResponse {
    pub file_path: String,
    pub file_size_bytes: u64,
    pub duration_ms: u64,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct ImportDatabaseArgs {
    pub source_path: String,
    pub confirmation: String,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ImportDatabaseResponse {
    pub file_path: String,
    pub file_size_bytes: u64,
    pub duration_ms: u64,
    pub message: String,
    pub requires_restart: bool,
}

/// Export the database to a user-selected file location
#[tauri::command]
#[specta::specta]
pub async fn export_database(
    args: ExportDatabaseArgs,
    state: State<'_, AppState>,
) -> std::result::Result<ExportDatabaseResponse, CommandError> {
    if args.destination_path.is_empty() {
        return Err(CommandError::validation_field(
            "destination_path",
            "Destination path is required",
        ));
    }

    let destination = std::path::Path::new(&args.destination_path);

    let result = application::export_database::export_database(&state.db_pool(), destination)
        .await
        .map_err(CommandError::from)?;

    Ok(ExportDatabaseResponse {
        file_path: result.file_path,
        file_size_bytes: result.file_size_bytes,
        duration_ms: result.duration_ms,
        message: "Database exported successfully".to_string(),
    })
}

/// Import (restore) the database from a user-selected backup file
#[tauri::command]
#[specta::specta]
pub async fn import_database(
    app: AppHandle,
    args: ImportDatabaseArgs,
) -> std::result::Result<ImportDatabaseResponse, CommandError> {
    if args.source_path.is_empty() {
        return Err(CommandError::validation_field(
            "source_path",
            "Source path is required",
        ));
    }

    if args.confirmation != "RESTORE" {
        return Err(CommandError::validation_field(
            "confirmation",
            "Must type 'RESTORE' to confirm",
        ));
    }

    let source = std::path::Path::new(&args.source_path);

    // Resolve the current database path
    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::Unknown(format!("Failed to resolve app data dir: {}", e)))?
        .join("database.sqlite");

    let result =
        application::import_database::import_database(source, &db_path, &args.confirmation)
            .await
            .map_err(CommandError::from)?;

    Ok(ImportDatabaseResponse {
        file_path: result.file_path,
        file_size_bytes: result.file_size_bytes,
        duration_ms: result.duration_ms,
        message: "Database imported successfully. Please restart the app.".to_string(),
        requires_restart: result.requires_restart,
    })
}
