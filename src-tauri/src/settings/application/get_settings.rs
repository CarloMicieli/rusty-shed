use crate::settings::domain::user_settings::UserSettings;
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use tauri::AppHandle;

/// Get current user settings
#[tracing::instrument(level = "debug", skip_all, fields(error.type = tracing::field::Empty, error.message = tracing::field::Empty))]
pub fn get_settings(app: &AppHandle) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();
    repository.load(app)
}
