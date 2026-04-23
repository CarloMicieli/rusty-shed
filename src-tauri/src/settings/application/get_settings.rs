use crate::settings::domain::user_settings::UserSettings;
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use tauri::AppHandle;

/// Get current user settings
pub fn get_settings(app: &AppHandle) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();
    repository.load(app)
}
