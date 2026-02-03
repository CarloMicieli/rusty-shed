use serde::{Deserialize, Serialize};

/// Budget mode - determines how the budget is set (yearly or monthly).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, specta::Type, sqlx::Type,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
#[derive(Default)]
pub enum BudgetMode {
    /// Budget is configured as a yearly amount (divided by 12 for monthly).
    Yearly,
    /// Budget is configured as a monthly amount (multiplied by 12 for yearly).
    #[default]
    Monthly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_mode_serialization() {
        assert_eq!(
            serde_json::to_string(&BudgetMode::Yearly).unwrap(),
            "\"YEARLY\""
        );
        assert_eq!(
            serde_json::to_string(&BudgetMode::Monthly).unwrap(),
            "\"MONTHLY\""
        );
    }
}
