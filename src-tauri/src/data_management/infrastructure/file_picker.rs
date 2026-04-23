use std::path::PathBuf;

use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;

use crate::data_management::domain::ExportError;

/// Open a file save dialog to select export destination
///
/// # Arguments
/// * `app` - The Tauri app handle
/// * `default_filename` - Default filename for the export
/// * `default_dir` - Optional directory to open the dialog in
///
/// # Returns
/// The selected path, or None if cancelled
pub fn open_save_dialog(
    app: &AppHandle,
    default_filename: &str,
    default_dir: Option<PathBuf>,
) -> Result<Option<PathBuf>, ExportError> {
    let mut builder = app
        .dialog()
        .file()
        .set_title("Export Rusty Shed Data")
        .set_file_name(default_filename)
        .add_filter("ZIP Archive", &["zip"])
        .add_filter("All Files", &["*"]);

    if let Some(dir) = default_dir {
        builder = builder.set_directory(dir);
    }

    let result = builder.blocking_save_file();

    // FilePath can be converted to path through its Display impl
    Ok(result.map(|fp| PathBuf::from(fp.to_string())))
}
