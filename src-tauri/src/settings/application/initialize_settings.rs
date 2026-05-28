use crate::core::domain::Language;
use crate::settings::domain::user_settings::UserSettings;
use crate::settings::infrastructure::os_language::detect_os_language;
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use tauri::AppHandle;

/// Initialize settings on first run with OS language detection
pub fn initialize_settings(app: &AppHandle) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();

    // Load existing settings (or get defaults if none exist)
    let settings = repository.load(app)?;

    let (settings, should_save) = bootstrap_settings_language(settings, detect_os_language);

    if should_save {
        repository.save(app, &settings)?;
    }

    Ok(settings)
}

fn bootstrap_settings_language(
    mut settings: UserSettings,
    detect_language: impl FnOnce() -> Language,
) -> (UserSettings, bool) {
    if !settings.has_completed_onboarding && settings.language == Language::English {
        settings.language = detect_language();
        return (settings, true);
    }

    (settings, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_settings_language_updates_first_run_english() {
        let settings = UserSettings::default();
        assert!(!settings.has_completed_onboarding);

        let (bootstrapped, should_save) =
            bootstrap_settings_language(settings, || Language::Italian);

        assert!(should_save);
        assert_eq!(bootstrapped.language, Language::Italian);
        assert!(!bootstrapped.has_completed_onboarding);
    }

    #[test]
    fn bootstrap_settings_language_leaves_completed_settings_unchanged() {
        let settings = UserSettings {
            has_completed_onboarding: true,
            ..UserSettings::default()
        };

        let (bootstrapped, should_save) = bootstrap_settings_language(settings.clone(), || {
            panic!("should not detect language for completed onboarding")
        });

        assert!(!should_save);
        assert_eq!(bootstrapped.language, settings.language);
        assert_eq!(
            bootstrapped.has_completed_onboarding,
            settings.has_completed_onboarding
        );
    }
}
