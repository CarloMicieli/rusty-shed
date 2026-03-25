//! IPC command handlers for settings management

use crate::settings::application;
use crate::settings::application::update_settings::UpdateSettingsInput;
use crate::settings::domain::user_settings::UserSettings;
use tauri::{AppHandle, command};

/// Get current user settings
#[command]
#[specta::specta]
pub async fn get_settings(app: AppHandle) -> Result<UserSettings, String> {
    application::get_settings::get_settings(&app)
}

/// Update user settings (partial update supported)
#[command]
#[specta::specta]
pub async fn update_settings(
    app: AppHandle,
    input: UpdateSettingsInput,
) -> Result<UserSettings, String> {
    application::update_settings::update_settings(&app, input)
}

/// Initialize settings on first run
#[command]
#[specta::specta]
pub async fn initialize_settings(app: AppHandle) -> Result<UserSettings, String> {
    application::initialize_settings::initialize_settings(&app)
}

/// Return the OS system locale as a BCP 47 tag (e.g. "en-US", "it-IT").
#[command]
#[specta::specta]
pub fn get_locale() -> Option<String> {
    tauri_plugin_os::locale()
}
