use crate::settings::domain::user_settings::UserSettings;
use crate::settings::infrastructure::os_language::detect_os_language;
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use tauri::AppHandle;

/// Initialize settings on first run with OS language detection
pub fn initialize_settings(app: &AppHandle) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();

    // Load existing settings (or get defaults if none exist)
    let mut settings = repository.load(app)?;

    // If onboarding is not complete, opportunistically bootstrap OS language once
    // while preserving the onboarding-required status.
    if !settings.has_completed_onboarding
        && settings.language == crate::core::domain::Language::English
    {
        let os_language = detect_os_language();
        settings.language = os_language;

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
        assert!(!settings.has_completed_onboarding);

        // Simulate startup initialization
        settings.language = detect_os_language();

        assert!(!settings.has_completed_onboarding);
        // Language depends on OS, but should be either English or Italian
    }
}
