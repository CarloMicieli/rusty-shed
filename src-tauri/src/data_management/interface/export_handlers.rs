/// Tauri command handlers for the export feature.
use crate::core::infrastructure::error::CommandError;
use crate::data_management::application::execute_export;
use crate::data_management::application::preview_export::{self, ExportPreview};
use crate::data_management::domain::{ExportEntitySelection, ExportResult};
use crate::data_management::infrastructure::file_picker;
use crate::state::AppState;
use tauri::{Manager, State};
use tracing::info;

// ---------------------------------------------------------------------------
// Inner (testable) implementations – take &AppState directly
// ---------------------------------------------------------------------------

/// Inner implementation for [`get_export_preview`].
pub async fn get_export_preview_inner(state: &AppState) -> Result<ExportPreview, CommandError> {
    info!("get_export_preview");

    // Select all entity types for the default full-collection preview
    let selection = ExportEntitySelection {
        include_railway_models: true,
        include_collection_items: true,
        include_sellers: true,
        include_maintenance_logs: true,
        include_dcc_roster: true,
        include_orphaned_images: false,
        include_track_inventory: true,
        include_train_formations: true,
        include_wishlists: true,
    };

    preview_export::get_export_preview(&state.db_pool(), &selection)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Get a preview of what will be included in the export.
///
/// Returns counts for each entity type and estimated archive size.
#[tauri::command]
#[specta::specta]
pub async fn get_export_preview(state: State<'_, AppState>) -> Result<ExportPreview, CommandError> {
    get_export_preview_inner(&state).await
}

/// Inner implementation for [`open_export_file_dialog`].
///
/// `dialog_runner` is a closure that opens the OS dialog and returns the selected path.
/// This makes the function testable without a live Tauri context.
pub fn open_export_file_dialog_inner<F, E>(
    default_filename: String,
    default_dir: Option<std::path::PathBuf>,
    dialog_runner: F,
) -> Result<Option<String>, CommandError>
where
    F: FnOnce(String, Option<std::path::PathBuf>) -> Result<Option<std::path::PathBuf>, E>,
    E: std::fmt::Display,
{
    info!("open_export_file_dialog");
    let path = dialog_runner(default_filename, default_dir)
        .map_err(|e| CommandError::unknown(e.to_string()))?;
    Ok(path.map(|p| p.display().to_string()))
}

/// Open the native file save dialog to select where to save the export.
///
/// Returns the selected path as a string, or null if the user cancelled.
#[tauri::command]
#[specta::specta]
pub async fn open_export_file_dialog(
    app: tauri::AppHandle,
) -> Result<Option<String>, CommandError> {
    let default_filename = format!(
        "rusty-shed-export-{}.zip",
        chrono::Local::now().format("%Y-%m-%d")
    );
    let default_dir = app.path().home_dir().ok();
    open_export_file_dialog_inner(default_filename, default_dir, |filename, dir| {
        file_picker::open_save_dialog(&app, &filename, dir)
    })
}

/// Inner implementation for [`execute_export`].
pub async fn execute_export_inner(
    state: &AppState,
    destination_path: String,
) -> Result<ExportResult, CommandError> {
    info!("execute_export: destination={}", destination_path);

    let archive_path = std::path::Path::new(&destination_path);
    let media_dir = state.models_dir();

    // Export all entity types (DCC excluded in MVP — uses different schema)
    let selection = ExportEntitySelection {
        include_railway_models: true,
        include_collection_items: true,
        include_sellers: true,
        include_maintenance_logs: true,
        include_dcc_roster: false,
        include_orphaned_images: false,
        include_track_inventory: true,
        include_train_formations: true,
        include_wishlists: true,
    };

    let mut unit_of_work = state.unit_of_work().await?;

    execute_export::export_to_archive(&mut unit_of_work, archive_path, media_dir, &selection)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Execute the export operation.
///
/// Exports all collection data to a ZIP archive at the specified path.
///
/// # Arguments
/// * `destination_path` - Full path for the output archive (e.g. `/home/user/backup.zip`)
#[tauri::command]
#[specta::specta]
pub async fn execute_export(
    destination_path: String,
    state: State<'_, AppState>,
) -> Result<ExportResult, CommandError> {
    execute_export_inner(&state, destination_path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    fn app_state(pool: SqlitePool) -> AppState {
        AppState::for_test(pool)
    }

    #[tokio::test]
    async fn get_export_preview_inner_maps_sql_errors_to_unknown() {
        let pool = SqlitePool::connect("sqlite::memory:")
            .await
            .expect("in-memory sqlite should connect");
        let state = app_state(pool);

        let result = get_export_preview_inner(&state).await;

        assert!(
            matches!(result, Err(CommandError::Unknown { .. })),
            "{result:?}"
        );
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("../../../fixtures/test_tracks_inventory.sql")
    )]
    async fn get_export_preview_inner_returns_counts_with_default_selection(pool: SqlitePool) {
        let state = app_state(pool);

        let preview = get_export_preview_inner(&state)
            .await
            .expect("preview should succeed");

        assert_eq!(preview.seller_count, 1);
        assert_eq!(preview.train_formation_count, 0);
        assert_eq!(preview.collection_item_count, 0);
    }

    #[test]
    fn open_export_file_dialog_inner_returns_selected_path() {
        let result = open_export_file_dialog_inner("backup.zip".to_string(), None, |_, _| {
            Ok::<Option<std::path::PathBuf>, std::io::Error>(Some(std::path::PathBuf::from(
                "/tmp/backup.zip",
            )))
        })
        .expect("dialog result should succeed");

        assert_eq!(result.as_deref(), Some("/tmp/backup.zip"));
    }

    #[test]
    fn open_export_file_dialog_inner_maps_dialog_error_to_unknown() {
        let result = open_export_file_dialog_inner("backup.zip".to_string(), None, |_, _| {
            Err::<Option<std::path::PathBuf>, std::io::Error>(std::io::Error::other(
                "dialog backend failed",
            ))
        });

        assert!(
            matches!(result, Err(CommandError::Unknown { ref message, .. }) if message.contains("dialog backend failed")),
            "{result:?}"
        );
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn execute_export_inner_maps_invalid_destination_to_unknown(pool: SqlitePool) {
        let state = app_state(pool);
        let invalid_destination =
            "/definitely-not-existing-rusty-shed/export/backup.zip".to_string();

        let result = execute_export_inner(&state, invalid_destination).await;

        assert!(
            matches!(result, Err(CommandError::Unknown { .. })),
            "{result:?}"
        );
    }
}
