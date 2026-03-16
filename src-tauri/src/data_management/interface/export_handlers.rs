/// Tauri command handlers for the export feature.
use crate::core::infrastructure::error::CommandError;
use crate::data_management::application::execute_export;
use crate::data_management::application::preview_export::{self, ExportPreview};
use crate::data_management::domain::{ExportEntitySelection, ExportResult};
use crate::data_management::infrastructure::file_picker;
use crate::state::AppState;
use log::info;
use tauri::{Manager, State};

/// Get a preview of what will be included in the export.
///
/// Returns counts for each entity type and estimated archive size.
#[tauri::command]
#[specta::specta]
pub async fn get_export_preview(state: State<'_, AppState>) -> Result<ExportPreview, CommandError> {
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
    };

    preview_export::get_export_preview(&state.db_pool(), &selection)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}

/// Open the native file save dialog to select where to save the export.
///
/// Returns the selected path as a string, or null if the user cancelled.
#[tauri::command]
#[specta::specta]
pub async fn open_export_file_dialog(
    app: tauri::AppHandle,
) -> Result<Option<String>, CommandError> {
    info!("open_export_file_dialog");

    let default_filename = format!(
        "rusty-shed-export-{}.zip",
        chrono::Local::now().format("%Y-%m-%d")
    );

    let default_dir = app.path().home_dir().ok();

    let path = file_picker::open_save_dialog(&app, &default_filename, default_dir)
        .map_err(|e| CommandError::unknown(e.to_string()))?;

    Ok(path.map(|p| p.display().to_string()))
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
    };

    execute_export::export_to_archive(&state.db_pool(), archive_path, &media_dir, &selection)
        .await
        .map_err(|e| CommandError::unknown(e.to_string()))
}
