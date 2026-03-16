//! User settings domain types and validation logic

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::catalog::domain::railway_model::PowerMethod;
use crate::core::domain::Language;

/// Measurement system for dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
pub enum MeasureUnit {
    #[default]
    Metric,
    Imperial,
}

/// Application theme preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type, Default)]
#[serde(rename_all = "kebab-case")]
pub enum AppTheme {
    #[default]
    SteampunkDark,
    SteampunkLight,
    System,
}

/// User-configurable application preferences
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase", default)]
pub struct UserSettings {
    /// User's preferred currency for displaying prices (e.g., "EUR", "USD")
    pub currency: String,

    /// Application display language
    pub language: Language,

    /// Application theme preference
    pub theme: AppTheme,

    /// Measurement system for dimensions
    pub measure_unit: MeasureUnit,

    /// User's preferred model railway scale (e.g., "HO", "N", "OO")
    pub favourite_scale: String,

    /// Preferred power method for model railways
    #[serde(alias = "powerSystem")]
    pub power_method: PowerMethod,

    /// Flag indicating if this is the user's first app launch
    pub first_run: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        UserSettings {
            currency: "EUR".to_string(),
            language: Language::English,
            theme: AppTheme::default(),
            measure_unit: MeasureUnit::Metric,
            favourite_scale: String::new(),
            power_method: PowerMethod::DC,
            first_run: true,
        }
    }
}

impl UserSettings {
    /// Validate currency field
    pub fn validate_currency(currency: &str) -> Result<(), String> {
        if currency.is_empty() {
            return Err("Currency must not be empty".to_string());
        }
        if currency.len() > 10 {
            return Err("Currency must be at most 10 characters".to_string());
        }
        Ok(())
    }

    /// Validate favourite_scale field
    pub fn validate_favourite_scale(scale: &str) -> Result<(), String> {
        if scale.len() > 20 {
            return Err("Favourite scale must be at most 20 characters".to_string());
        }
        Ok(())
    }

    /// Validate all fields
    pub fn validate(&self) -> Result<(), String> {
        Self::validate_currency(&self.currency)?;
        Self::validate_favourite_scale(&self.favourite_scale)?;
        Ok(())
    }

    /// Set currency with validation
    pub fn set_currency(&mut self, currency: String) -> Result<(), String> {
        Self::validate_currency(&currency)?;
        self.currency = currency;
        Ok(())
    }

    /// Set favourite scale with validation
    pub fn set_favourite_scale(&mut self, scale: String) -> Result<(), String> {
        Self::validate_favourite_scale(&scale)?;
        self.favourite_scale = scale;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let settings = UserSettings::default();
        assert_eq!(settings.currency, "EUR");
        assert_eq!(settings.language, Language::English);
        assert_eq!(settings.theme, AppTheme::SteampunkDark);
        assert_eq!(settings.measure_unit, MeasureUnit::Metric);
        assert_eq!(settings.favourite_scale, "");
        assert_eq!(settings.power_method, PowerMethod::DC);
        assert!(settings.first_run);
    }

    #[test]
    fn test_currency_validation_empty() {
        let result = UserSettings::validate_currency("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must not be empty"));
    }

    #[test]
    fn test_currency_validation_too_long() {
        let result = UserSettings::validate_currency("12345678901"); // 11 chars
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at most 10 characters"));
    }

    #[test]
    fn test_currency_validation_valid() {
        assert!(UserSettings::validate_currency("EUR").is_ok());
        assert!(UserSettings::validate_currency("USD").is_ok());
        assert!(UserSettings::validate_currency("1234567890").is_ok()); // Exactly 10
    }

    #[test]
    fn test_favourite_scale_validation_valid() {
        assert!(UserSettings::validate_favourite_scale("").is_ok()); // Empty allowed
        assert!(UserSettings::validate_favourite_scale("HO").is_ok());
        assert!(UserSettings::validate_favourite_scale("12345678901234567890").is_ok()); // 20 chars
    }

    #[test]
    fn test_favourite_scale_validation_too_long() {
        let result = UserSettings::validate_favourite_scale("123456789012345678901"); // 21 chars
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("at most 20 characters"));
    }

    #[test]
    fn test_validate_all_fields() {
        let mut settings = UserSettings::default();
        assert!(settings.validate().is_ok());

        settings.currency = "".to_string();
        assert!(settings.validate().is_err());

        settings.currency = "EUR".to_string();
        settings.favourite_scale = "x".repeat(21);
        assert!(settings.validate().is_err());
    }
}
