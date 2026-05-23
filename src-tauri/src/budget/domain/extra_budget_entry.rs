use crate::budget::domain::ExtraBudgetId;
use crate::core::domain::calendar::{Month, Year};
use crate::core::domain::monetary_amount::MonetaryAmount;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Extra Budget Entry value object.
///
/// Represents a one-time budget injection for a specific month.
/// For example, a birthday gift or bonus that increases the budget for that month.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExtraBudgetEntry {
    pub id: ExtraBudgetId,
    pub year: Year,
    pub month: Month,
    pub amount: MonetaryAmount,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub version: u32,
}

impl ExtraBudgetEntry {
    /// Create a new extra budget entry.
    pub fn new(
        year: Year,
        month: Month,
        amount: MonetaryAmount,
        reason: Option<String>,
    ) -> Result<Self, String> {
        if amount.amount <= 0 {
            return Err("Amount must be positive".to_string());
        }

        Ok(Self {
            id: ExtraBudgetId::default(),
            year,
            month,
            amount,
            reason,
            created_at: Utc::now(),
            version: 0,
        })
    }

    /// Format the month name for display.
    pub fn month_name(&self) -> &'static str {
        self.month.name()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::calendar::{Month, Year};
    use crate::core::domain::currency::Currency;

    #[test]
    fn test_valid_extra_budget() {
        let year = Year::try_from(2026).unwrap();
        let month = Month::try_from(3).unwrap();
        let entry = ExtraBudgetEntry::new(
            year,
            month,
            MonetaryAmount::new(5_000, Currency::USD),
            Some("Birthday gift".to_string()),
        );
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.year.value(), 2026);
        assert_eq!(entry.month.value(), 3);
        assert_eq!(entry.month_name(), "March");
    }

    #[test]
    fn test_invalid_amount() {
        let year = Year::try_from(2026).unwrap();
        let month = Month::try_from(3).unwrap();
        let entry = ExtraBudgetEntry::new(
            year,
            month,
            MonetaryAmount::new(0, Currency::USD), // Non-positive
            None,
        );
        assert!(entry.is_err());
    }
}
