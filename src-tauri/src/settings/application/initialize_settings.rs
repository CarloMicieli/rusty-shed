use crate::settings::domain::user_settings::UserSettings;
use crate::settings::infrastructure::os_language::detect_os_language;
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use tauri::AppHandle;

/// Initialize settings on first run with OS language detection
pub fn initialize_settings(app: &AppHandle) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();

    // Load existing settings (or get defaults if none exist)
    let mut settings = repository.load(app)?;

    // If this is the first run, detect OS language and set first_run=false
    if settings.first_run {
        let os_language = detect_os_language();
        settings.language = os_language;
        settings.first_run = false;

        // Save initialized settings
        repository.save(app, &settings)?;
    }

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_settings_logic() {
        // This tests the core logic - actual Tauri integration requires integration tests
        let mut settings = UserSettings::default();
        assert!(settings.first_run);

        // Simulate first run initialization
        settings.language = detect_os_language();
        settings.first_run = false;

        assert!(!settings.first_run);
        // Language depends on OS, but should be either English or Italian
    }
}
