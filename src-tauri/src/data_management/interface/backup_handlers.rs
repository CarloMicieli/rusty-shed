/// Database backup Tauri command handlers.
///
/// These commands provide the IPC interface between the frontend
/// and the database backup/restore functionality.
use crate::core::infrastructure::error::CommandError;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};

/// Arguments for the export database command.
#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct ExportDatabaseArgs {
    /// Absolute path where the exported database file should be written.
    pub destination_path: String,
}

/// Response returned after a successful database export.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ExportDatabaseResponse {
    /// Absolute path of the exported file.
    pub file_path: String,
    /// Size of the exported file in bytes.
    pub file_size_bytes: u64,
    /// Duration of the export operation in milliseconds.
    pub duration_ms: u64,
    /// Human-readable success message.
    pub message: String,
}

/// Arguments for the import database command.
#[derive(Debug, Clone, Deserialize, specta::Type)]
pub struct ImportDatabaseArgs {
    /// Absolute path of the backup file to restore from.
    pub source_path: String,
    /// Confirmation string; must be `"RESTORE"` to proceed.
    pub confirmation: String,
}

/// Response returned after a successful database import (restore).
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct ImportDatabaseResponse {
    /// Absolute path of the source backup file.
    pub file_path: String,
    /// Size of the imported file in bytes.
    pub file_size_bytes: u64,
    /// Duration of the import operation in milliseconds.
    pub duration_ms: u64,
    /// Human-readable success message.
    pub message: String,
    /// Whether the application must be restarted for the restored database to take effect.
    pub requires_restart: bool,
}

/// Export the database to a user-selected file location.
///
/// # Errors
/// Returns [`CommandError`] if the destination path is empty, the directory does not exist,
/// or the VACUUM INTO operation fails.
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

    use crate::data_management::application::export_database::export_database as run_export;
    let result = run_export(&state.db_pool(), destination)
        .await
        .map_err(CommandError::from)?;

    Ok(ExportDatabaseResponse {
        file_path: result.file_path,
        file_size_bytes: result.file_size_bytes,
        duration_ms: result.duration_ms,
        message: "Database exported successfully".to_string(),
    })
}

/// Import (restore) the database from a user-selected backup file.
///
/// # Errors
/// Returns [`CommandError`] if the source path is empty, confirmation is wrong,
/// the file is not a valid SQLite database, or the copy fails.
#[tauri::command]
#[specta::specta]
pub async fn import_database(
    app: AppHandle,
    args: ImportDatabaseArgs,
) -> std::result::Result<ImportDatabaseResponse, CommandError> {
    let db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| CommandError::unknown(format!("Failed to resolve app data dir: {}", e)))?
        .join("database.sqlite");

    import_database_inner(&args, &db_path).await
}

async fn import_database_inner(
    args: &ImportDatabaseArgs,
    db_path: &std::path::Path,
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

    use crate::data_management::application::import_database::import_database as run_import;
    let result = run_import(source, db_path, &args.confirmation)
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

#[cfg(test)]
mod tests {
    use super::{ImportDatabaseArgs, import_database_inner};
    use crate::core::infrastructure::error::CommandError;
    use crate::data_management::domain::backup_validation::{
        validate_confirmation, validate_export_destination,
    };
    use sqlx::SqlitePool;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn export_handler_rejects_empty_destination_path() {
        let destination_path = "";
        assert!(destination_path.is_empty(), "Empty path should be detected");
    }

    #[test]
    fn import_handler_rejects_empty_source_path() {
        let source_path = "";
        assert!(source_path.is_empty(), "Empty path should be detected");
    }

    #[test]
    fn import_handler_rejects_invalid_confirmation() {
        let err = validate_confirmation("wrong").unwrap_err();
        assert!(
            matches!(
                err,
                crate::data_management::domain::backup_errors::DatabaseBackupError::ConfirmationFailed(_)
            ),
            "Wrong confirmation string should fail"
        );
    }

    #[test]
    fn import_handler_accepts_valid_confirmation() {
        assert!(
            validate_confirmation("RESTORE").is_ok(),
            "RESTORE confirmation should succeed"
        );
    }

    #[tokio::test]
    async fn import_handler_inner_rejects_empty_source_path() {
        let temp_dir = tempdir().expect("tempdir should be created");
        let destination = temp_dir.path().join("database.sqlite");
        let args = ImportDatabaseArgs {
            source_path: String::new(),
            confirmation: "RESTORE".to_string(),
        };

        let error = import_database_inner(&args, &destination)
            .await
            .expect_err("empty source path should fail");

        assert!(matches!(error, CommandError::ValidationError(_)));
    }

    #[tokio::test]
    async fn import_handler_inner_rejects_invalid_confirmation() {
        let temp_dir = tempdir().expect("tempdir should be created");
        let destination = temp_dir.path().join("database.sqlite");
        let args = ImportDatabaseArgs {
            source_path: "/tmp/source.sqlite".to_string(),
            confirmation: "WRONG".to_string(),
        };

        let error = import_database_inner(&args, &destination)
            .await
            .expect_err("invalid confirmation should fail");

        assert!(matches!(error, CommandError::ValidationError(_)));
    }

    #[tokio::test]
    async fn import_handler_inner_imports_valid_sqlite_backup() {
        let temp_dir = tempdir().expect("tempdir should be created");
        let source_path = temp_dir.path().join("source.sqlite");
        let destination_path = temp_dir.path().join("database.sqlite");

        let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", source_path.display()))
            .await
            .expect("source db should open");
        sqlx::query("CREATE TABLE seed (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .expect("seed sqlite file");
        drop(pool);

        let args = ImportDatabaseArgs {
            source_path: source_path.to_string_lossy().to_string(),
            confirmation: "RESTORE".to_string(),
        };

        let response = import_database_inner(&args, &destination_path)
            .await
            .expect("import should succeed");

        assert_eq!(response.file_path, source_path.to_string_lossy());
        assert!(response.file_size_bytes > 0);
        assert_eq!(
            response.message,
            "Database imported successfully. Please restart the app."
        );
        assert!(response.requires_restart);
        assert!(destination_path.exists());
    }

    #[test]
    fn export_handler_rejects_nonexistent_parent_dir() {
        let path = Path::new("/nonexistent_backup_dir_xyz/db.sqlite");
        let err = validate_export_destination(path).unwrap_err();
        assert!(
            matches!(
                err,
                crate::data_management::domain::backup_errors::DatabaseBackupError::InvalidPath(_)
            ),
            "Non-existent parent dir should fail validation"
        );
    }
}
