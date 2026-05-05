use crate::budget::domain::{
    BudgetConfigId, BudgetConfiguration, BudgetMode, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::budget::infrastructure::entities::{BudgetConfigRow, ExtraBudgetRow};
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::currency::Currency;
use crate::core::domain::metadata::Metadata;
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};
use tracing::warn;

fn parse_db_datetime_or_fallback(
    field: &str,
    value: &str,
    fallback: DateTime<Utc>,
) -> DateTime<Utc> {
    match DateTime::parse_from_rfc3339(value) {
        Ok(parsed) => parsed.with_timezone(&Utc),
        Err(error) => {
            warn!("Invalid {field} in budget_config row: {error}. Falling back to {fallback}");
            fallback
        }
    }
}

/// Map a BudgetConfigRow to BudgetConfiguration domain entity.
pub fn row_to_budget_config(row: BudgetConfigRow) -> Result<BudgetConfiguration, String> {
    let mode = match row.mode.as_str() {
        "YEARLY" => BudgetMode::Yearly,
        "MONTHLY" => BudgetMode::Monthly,
        _ => return Err(format!("Invalid budget mode: {}", row.mode)),
    };

    let currency =
        Currency::from_code(&row.currency).map_err(|e| format!("Invalid currency: {}", e))?;

    let last_reset_year =
        Year::try_from(row.last_reset_year).map_err(|e| format!("Invalid year: {}", e))?;

    let now = Utc::now();
    let created_at = parse_db_datetime_or_fallback("created_at", &row.created_at, now);
    let updated_at = parse_db_datetime_or_fallback("updated_at", &row.updated_at, created_at);

    let metadata = Metadata {
        version: row.version as u8,
        created_at,
        updated_at,
    };

    Ok(BudgetConfiguration {
        id: BudgetConfigId::new(row.id),
        mode,
        base_amount: MonetaryAmount::new(row.base_amount, currency),
        last_reset_year,
        metadata,
        pending_events: Vec::new(), // Events are not persisted in DB, will be populated by domain logic
    })
}

/// Map an ExtraBudgetRow to ExtraBudgetEntry domain entity.
pub fn row_to_extra_budget(row: ExtraBudgetRow) -> Result<ExtraBudgetEntry, String> {
    let currency =
        Currency::from_code(&row.currency).map_err(|e| format!("Invalid currency: {}", e))?;

    let created_at = DateTime::parse_from_rfc3339(&row.created_at)
        .map_err(|e| format!("Invalid created_at: {}", e))?
        .with_timezone(&Utc);

    let id = ExtraBudgetId::try_from(row.id.as_str())
        .map_err(|e| format!("Invalid extra budget ID: {}", e))?;

    let year = Year::try_from(row.year).map_err(|e| format!("Invalid year in DB row: {}", e))?;
    let month =
        Month::try_from(row.month as u8).map_err(|e| format!("Invalid month in DB row: {}", e))?;

    Ok(ExtraBudgetEntry {
        id,
        year,
        month,
        amount: MonetaryAmount::new(row.amount, currency),
        reason: row.reason,
        created_at,
        version: row.version as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_to_budget_config() {
        let row = BudgetConfigRow {
            id: 1,
            mode: "YEARLY".to_string(),
            base_amount: 120_000,
            currency: "USD".to_string(),
            last_reset_year: 2026,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "2026-01-01T00:00:00Z".to_string(),
            version: 0,
        };

        let config = row_to_budget_config(row).unwrap();
        assert_eq!(config.mode, BudgetMode::Yearly);
        assert_eq!(config.base_amount.amount, 120_000);
        assert_eq!(config.monthly_amount(), 10_000);
    }

    #[test]
    fn test_row_to_budget_config_falls_back_when_updated_at_is_invalid() {
        let row = BudgetConfigRow {
            id: 1,
            mode: "MONTHLY".to_string(),
            base_amount: 90_000,
            currency: "EUR".to_string(),
            last_reset_year: 2026,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_at: "".to_string(),
            version: 1,
        };

        let config = row_to_budget_config(row).expect("budget config should still map");
        assert_eq!(config.metadata.created_at, config.metadata.updated_at);
    }

    #[test]
    fn test_row_to_extra_budget_fails_on_invalid_currency() {
        let row = ExtraBudgetRow {
            id: "trn:extra-budget:11111111-1111-1111-1111-111111111111".to_string(),
            year: 2026,
            month: 4,
            amount: 5_000,
            currency: "INVALID".to_string(),
            reason: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            version: 0,
        };

        let result = row_to_extra_budget(row);

        assert!(result.is_err());
    }

    #[test]
    fn test_row_to_extra_budget_fails_on_invalid_month() {
        let row = ExtraBudgetRow {
            id: "trn:extra-budget:11111111-1111-1111-1111-111111111111".to_string(),
            year: 2026,
            month: 13,
            amount: 5_000,
            currency: "EUR".to_string(),
            reason: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            version: 0,
        };

        let result = row_to_extra_budget(row);

        assert!(result.is_err());
    }
}
