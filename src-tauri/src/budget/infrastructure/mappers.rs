use crate::budget::domain::{
    BudgetConfigId, BudgetConfiguration, BudgetMode, ExtraBudgetEntry, ExtraBudgetId,
};
use crate::budget::infrastructure::entities::{BudgetConfigRow, ExtraBudgetRow};
use crate::core::domain::currency::Currency;
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};

/// Map a BudgetConfigRow to BudgetConfiguration domain entity.
pub fn row_to_budget_config(row: BudgetConfigRow) -> Result<BudgetConfiguration, String> {
    let mode = match row.mode.as_str() {
        "YEARLY" => BudgetMode::Yearly,
        "MONTHLY" => BudgetMode::Monthly,
        _ => return Err(format!("Invalid budget mode: {}", row.mode)),
    };

    let currency =
        Currency::from_code(&row.currency).map_err(|e| format!("Invalid currency: {}", e))?;

    let created_at = DateTime::parse_from_rfc3339(&row.created_at)
        .map_err(|e| format!("Invalid created_at: {}", e))?
        .with_timezone(&Utc);

    let updated_at = DateTime::parse_from_rfc3339(&row.updated_at)
        .map_err(|e| format!("Invalid updated_at: {}", e))?
        .with_timezone(&Utc);

    Ok(BudgetConfiguration {
        id: BudgetConfigId::new(row.id),
        mode,
        base_amount: MonetaryAmount::new(row.base_amount, currency),
        last_reset_year: row.last_reset_year,
        created_at,
        updated_at,
        version: row.version as u32,
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

    Ok(ExtraBudgetEntry {
        id,
        year: row.year,
        month: row.month as u8,
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
}
