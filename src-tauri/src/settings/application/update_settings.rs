use crate::catalog::domain::railway_model::PowerMethod;
use crate::core::domain::Language;
use crate::settings::domain::user_settings::{AppTheme, MeasureUnit, UserSettings};
use crate::settings::infrastructure::{SettingsRepository, StoreSettingsRepository};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;

/// Input for partial settings updates
#[derive(Debug, Clone, Default, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase", default)]
pub struct UpdateSettingsInput {
    pub currency: Option<String>,
    pub language: Option<Language>,
    pub measure_unit: Option<MeasureUnit>,
    pub favourite_scale: Option<String>,
    pub power_method: Option<PowerMethod>,
    pub theme: Option<AppTheme>,
    pub has_completed_onboarding: Option<bool>,
}

/// Update user settings (partial update supported)
pub fn update_settings(
    app: &AppHandle,
    input: UpdateSettingsInput,
) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();

    update_settings_with_io(
        input,
        || repository.load(app),
        |settings| repository.save(app, settings),
    )
}

fn update_settings_with_io<Load, Save>(
    input: UpdateSettingsInput,
    load: Load,
    save: Save,
) -> Result<UserSettings, String>
where
    Load: FnOnce() -> Result<UserSettings, String>,
    Save: FnOnce(&UserSettings) -> Result<(), String>,
{
    let current = load()?;
    let updated = apply_settings_update(current, input)?;
    save(&updated)?;
    Ok(updated)
}

fn apply_settings_update(
    mut current: UserSettings,
    input: UpdateSettingsInput,
) -> Result<UserSettings, String> {
    apply_optional_updates(&mut current, input)?;
    current.validate()?;
    Ok(current)
}

fn apply_optional_updates(
    current: &mut UserSettings,
    input: UpdateSettingsInput,
) -> Result<(), String> {
    // Apply updates (only update provided fields)
    if let Some(currency) = input.currency {
        current.set_currency(currency)?;
    }
    if let Some(language) = input.language {
        current.language = language;
    }
    if let Some(measure_unit) = input.measure_unit {
        current.measure_unit = measure_unit;
    }
    if let Some(favourite_scale) = input.favourite_scale {
        current.set_favourite_scale(favourite_scale)?;
    }
    if let Some(power_method) = input.power_method {
        current.power_method = power_method;
    }
    if let Some(theme) = input.theme {
        current.theme = theme;
    }
    if let Some(has_completed_onboarding) = input.has_completed_onboarding {
        current.has_completed_onboarding = has_completed_onboarding;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::domain::railway_model::PowerMethod;
    use crate::core::domain::Language;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_update_settings_input_default() {
        let input = UpdateSettingsInput::default();
        assert!(input.currency.is_none());
        assert!(input.language.is_none());
    }

    #[test]
    fn test_apply_settings_update_updates_multiple_fields() {
        let current = UserSettings::default();
        let input = UpdateSettingsInput {
            currency: Some("USD".to_string()),
            language: Some(Language::Italian),
            measure_unit: Some(MeasureUnit::Imperial),
            favourite_scale: Some("N".to_string()),
            power_method: Some(PowerMethod::AC),
            theme: Some(AppTheme::System),
            has_completed_onboarding: Some(true),
        };

        let updated = apply_settings_update(current, input).expect("update should succeed");
        assert_eq!(updated.currency, "USD");
        assert_eq!(updated.language, Language::Italian);
        assert_eq!(updated.measure_unit, MeasureUnit::Imperial);
        assert_eq!(updated.favourite_scale, "N");
        assert_eq!(updated.power_method, PowerMethod::AC);
        assert_eq!(updated.theme, AppTheme::System);
        assert!(updated.has_completed_onboarding);
    }

    #[test]
    fn test_apply_settings_update_rejects_invalid_currency_before_save() {
        let current = UserSettings::default();
        let input = UpdateSettingsInput {
            currency: Some(String::new()),
            ..UpdateSettingsInput::default()
        };

        let error = apply_settings_update(current, input).expect_err("invalid currency expected");
        assert!(error.contains("Currency must not be empty"));
    }

    #[test]
    fn test_apply_settings_update_rejects_invalid_favourite_scale() {
        let current = UserSettings::default();
        let input = UpdateSettingsInput {
            favourite_scale: Some("x".repeat(21)),
            ..UpdateSettingsInput::default()
        };

        let error = apply_settings_update(current, input).expect_err("invalid scale expected");
        assert!(error.contains("Favourite scale must be at most 20 characters"));
    }

    #[test]
    fn test_update_settings_with_io_propagates_load_failure() {
        let error = update_settings_with_io(
            UpdateSettingsInput::default(),
            || Err("load failed".to_string()),
            |_| Ok(()),
        )
        .expect_err("load failure expected");

        assert_eq!(error, "load failed");
    }

    #[test]
    fn test_update_settings_with_io_propagates_save_failure() {
        let error = update_settings_with_io(
            UpdateSettingsInput {
                currency: Some("USD".to_string()),
                ..UpdateSettingsInput::default()
            },
            || Ok(UserSettings::default()),
            |_| Err("save failed".to_string()),
        )
        .expect_err("save failure expected");

        assert_eq!(error, "save failed");
    }

    #[test]
    fn test_update_settings_with_io_saves_updated_state() {
        let saved = Arc::new(Mutex::new(None::<UserSettings>));
        let saved_ref = saved.clone();

        let updated = update_settings_with_io(
            UpdateSettingsInput {
                currency: Some("USD".to_string()),
                ..UpdateSettingsInput::default()
            },
            || Ok(UserSettings::default()),
            move |settings| {
                *saved_ref.lock().expect("saved lock") = Some(settings.clone());
                Ok(())
            },
        )
        .expect("update should succeed");

        assert_eq!(updated.currency, "USD");
        let saved_currency = saved
            .lock()
            .expect("saved lock")
            .as_ref()
            .map(|s| s.currency.clone())
            .unwrap_or_default();
        assert_eq!(saved_currency, "USD");
    }

    #[test]
    fn test_update_settings_with_io_no_input_preserves_existing_values() {
        let mut existing = UserSettings::default();
        existing.currency = "CHF".to_string();
        existing.language = Language::Italian;

        let updated = update_settings_with_io(
            UpdateSettingsInput::default(),
            || Ok(existing.clone()),
            |_| Ok(()),
        )
        .expect("update should succeed");

        assert_eq!(updated.currency, "CHF");
        assert_eq!(updated.language, Language::Italian);
    }
}
