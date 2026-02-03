use crate::budget::domain::ExtraBudgetId;
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
    pub year: i32,
    pub month: u8, // 1-12
    pub amount: MonetaryAmount,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub version: u32,
}

impl ExtraBudgetEntry {
    /// Create a new extra budget entry.
    pub fn new(
        year: i32,
        month: u8,
        amount: MonetaryAmount,
        reason: Option<String>,
    ) -> Result<Self, String> {
        // Validate month
        if !(1..=12).contains(&month) {
            return Err(format!(
                "Invalid month: {}. Must be between 1 and 12",
                month
            ));
        }

        // Validate year
        if !(2000..=2100).contains(&year) {
            return Err(format!(
                "Invalid year: {}. Must be between 2000 and 2100",
                year
            ));
        }

        // Validate amount is positive
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
        match self.month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Invalid",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::currency::Currency;

    #[test]
    fn test_valid_extra_budget() {
        let entry = ExtraBudgetEntry::new(
            2026,
            3,
            MonetaryAmount::new(5_000, Currency::USD),
            Some("Birthday gift".to_string()),
        );
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.year, 2026);
        assert_eq!(entry.month, 3);
        assert_eq!(entry.month_name(), "March");
    }

    #[test]
    fn test_invalid_month() {
        let entry = ExtraBudgetEntry::new(
            2026,
            13, // Invalid month
            MonetaryAmount::new(5_000, Currency::USD),
            None,
        );
        assert!(entry.is_err());
    }

    #[test]
    fn test_invalid_amount() {
        let entry = ExtraBudgetEntry::new(
            2026,
            3,
            MonetaryAmount::new(0, Currency::USD), // Non-positive
            None,
        );
        assert!(entry.is_err());
    }
}
