use std::str::FromStr;

use crate::catalog::domain::railway_model::PowerMethod;
use crate::catalog::domain::scale::Scale;
use crate::core::domain::currency::Currency;
use crate::core::domain::measure_units::MeasureUnit;
use crate::core::infrastructure::error::CommandError;
use crate::core::infrastructure::unit_of_work::SqliteUnitOfWork;
use crate::state::AppState;
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

const SETTINGS_ID: i64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeValue {
    SteampunkLight,
    SteampunkDark,
    #[default]
    System,
}

impl FromStr for ThemeValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "steampunk-light" => Ok(ThemeValue::SteampunkLight),
            "steampunk-dark" => Ok(ThemeValue::SteampunkDark),
            "system" => Ok(ThemeValue::System),
            _ => Err(format!("Invalid theme value: {}", s)),
        }
    }
}

impl std::fmt::Display for ThemeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeValue::SteampunkLight => write!(f, "steampunk-light"),
            ThemeValue::SteampunkDark => write!(f, "steampunk-dark"),
            ThemeValue::System => write!(f, "system"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct SettingsDto {
    pub id: i64,
    pub currency: Currency,
    pub length_unit: MeasureUnit,
    pub favorite_scale: Scale,
    pub favorite_power_method: PowerMethod,
    pub language_code: String,
    pub theme: ThemeValue,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, Validate)]
#[garde(allow_unvalidated)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsPayload {
    pub currency: Currency,
    pub length_unit: MeasureUnit,
    pub favorite_scale: Scale,
    pub favorite_power_method: PowerMethod,
    pub language_code: String,
    pub theme: ThemeValue,
}

#[derive(Debug, FromRow)]
struct SettingsRow {
    id: i64,
    currency: String,
    length_unit: String,
    favorite_scale: String,
    favorite_power_method: String,
    language_code: String,
    theme: String,
}

impl TryFrom<SettingsRow> for SettingsDto {
    type Error = CommandError;

    fn try_from(row: SettingsRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.id,
            currency: parse_currency(&row.currency)?,
            length_unit: parse_length_unit(&row.length_unit)?,
            favorite_scale: parse_scale(&row.favorite_scale)?,
            favorite_power_method: parse_power_method(&row.favorite_power_method)?,
            language_code: row.language_code,
            theme: parse_theme(&row.theme)?,
        })
    }
}

pub struct SettingsRepository;

impl SettingsRepository {
    pub async fn get(unit_of_work: &mut SqliteUnitOfWork<'_>) -> Result<SettingsDto, CommandError> {
        let row = sqlx::query_as::<_, SettingsRow>(
            "SELECT id, currency, length_unit, favorite_scale, favorite_power_method, language_code, theme FROM settings WHERE id = ?1 LIMIT 1",
        )
        .bind(SETTINGS_ID)
        .fetch_optional(&mut *unit_of_work.tx)
        .await?;

        match row {
            Some(row) => row.try_into(),
            None => Err(CommandError::NotFound("Settings not found".into())),
        }
    }

    pub async fn upsert(
        unit_of_work: &mut SqliteUnitOfWork<'_>,
        payload: UpdateSettingsPayload,
    ) -> Result<SettingsDto, CommandError> {
        let row = sqlx::query_as::<_, SettingsRow>(
            "INSERT INTO settings (id, currency, length_unit, favorite_scale, favorite_power_method, language_code, theme)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
                 currency = excluded.currency,
                 length_unit = excluded.length_unit,
                 favorite_scale = excluded.favorite_scale,
                 favorite_power_method = excluded.favorite_power_method,
                 language_code = excluded.language_code,
                 theme = excluded.theme
             RETURNING id, currency, length_unit, favorite_scale, favorite_power_method, language_code, theme",
        )
        .bind(SETTINGS_ID)
        .bind(payload.currency.to_code())
        .bind(payload.length_unit.code())
        .bind(scale_code(&payload.favorite_scale))
        .bind(payload.favorite_power_method.to_string())
        .bind(payload.language_code)
        .bind(payload.theme.to_string())
        .fetch_one(&mut *unit_of_work.tx)
        .await?;

        row.try_into()
    }

    pub async fn ensure_default(pool: &sqlx::SqlitePool) -> Result<(), CommandError> {
        let mut unit_of_work = SqliteUnitOfWork::new(pool).await?;
        let row = sqlx::query_as::<_, SettingsRow>(
            "SELECT id, currency, length_unit, favorite_scale, favorite_power_method, language_code, theme FROM settings WHERE id = ?1 LIMIT 1",
        )
        .bind(SETTINGS_ID)
        .fetch_optional(&mut *unit_of_work.tx)
        .await?;

        if row.is_none() {
            let default_payload = UpdateSettingsPayload {
                currency: Currency::EUR,
                length_unit: MeasureUnit::Millimeters,
                favorite_scale: Scale::H0,
                favorite_power_method: PowerMethod::DC,
                language_code: "en".to_string(),
                theme: ThemeValue::System,
            };

            // ignore returned value; just ensure presence
            let _ = Self::upsert(&mut unit_of_work, default_payload).await?;
        }

        unit_of_work.commit().await?;
        Ok(())
    }
}

fn parse_currency(code: &str) -> Result<Currency, CommandError> {
    Currency::from_code(code)
        .map_err(|err| CommandError::validation_field("currency", err.to_string()))
}

fn parse_length_unit(value: &str) -> Result<MeasureUnit, CommandError> {
    MeasureUnit::from_str(value).map_err(|err| CommandError::validation_field("lengthUnit", err))
}

fn parse_scale(value: &str) -> Result<Scale, CommandError> {
    Scale::try_from(value)
        .map_err(|err| CommandError::validation_field("favoriteScale", err.to_string()))
}

fn parse_power_method(value: &str) -> Result<PowerMethod, CommandError> {
    PowerMethod::try_from(value)
        .map_err(|err| CommandError::validation_field("favoritePowerMethod", err.to_string()))
}

fn parse_theme(value: &str) -> Result<ThemeValue, CommandError> {
    ThemeValue::from_str(value).map_err(|err| CommandError::validation_field("theme", err))
}

fn scale_code(scale: &Scale) -> &'static str {
    match scale {
        Scale::H0 => "H0",
        Scale::H0m => "H0m",
        Scale::H0e => "H0e",
        Scale::N => "N",
        Scale::TT => "TT",
        Scale::Z => "Z",
        Scale::G => "G",
        Scale::Scale1 => "1",
        Scale::Scale0 => "0",
        Scale::Scale00 => "00",
    }
}

#[tauri::command]
#[specta::specta]
pub async fn get_settings(state: tauri::State<'_, AppState>) -> Result<SettingsDto, CommandError> {
    let mut unit_of_work = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(CommandError::from)?;

    let result = SettingsRepository::get(&mut unit_of_work).await?;
    unit_of_work.commit().await?;

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    payload: UpdateSettingsPayload,
) -> Result<SettingsDto, CommandError> {
    let mut unit_of_work = SqliteUnitOfWork::new(&state.db_pool())
        .await
        .map_err(CommandError::from)?;

    let updated = SettingsRepository::upsert(&mut unit_of_work, payload).await?;
    unit_of_work.commit().await?;

    Ok(updated)
}

/// Ensure settings defaults exist. Intended to be called during init.
pub async fn ensure_default_settings(pool: &sqlx::SqlitePool) -> Result<(), CommandError> {
    SettingsRepository::ensure_default(pool).await
}
