//! Update settings use case

use crate::core::domain::Language;
use crate::settings::domain::user_settings::{AppTheme, MeasureUnit, PowerSystem, UserSettings};
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
    pub power_system: Option<PowerSystem>,
    pub theme: Option<AppTheme>,
}

/// Update user settings (partial update supported)
pub fn update_settings(
    app: &AppHandle,
    input: UpdateSettingsInput,
) -> Result<UserSettings, String> {
    let repository = StoreSettingsRepository::new();

    // Load current settings
    let mut current = repository.load(app)?;

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
    if let Some(power_system) = input.power_system {
        current.power_system = power_system;
    }
    if let Some(theme) = input.theme {
        current.theme = theme;
    }

    // Validate merged settings
    current.validate()?;

    // Save updated settings
    repository.save(app, &current)?;

    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_settings_input_default() {
        let input = UpdateSettingsInput::default();
        assert!(input.currency.is_none());
        assert!(input.language.is_none());
    }
}
