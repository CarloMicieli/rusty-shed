/// Database row type for budget_config table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct BudgetConfigRow {
    pub id: i32,
    pub mode: String, // TEXT enum: "YEARLY" or "MONTHLY"
    pub base_amount: i64,
    pub currency: String,
    pub last_reset_year: i32,
    pub created_at: String, // ISO 8601 datetime string
    pub updated_at: String, // ISO 8601 datetime string
    pub version: i32,
}

/// Database row type for extra_budgets table.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ExtraBudgetRow {
    pub id: String, // UUID string
    pub year: i32,
    pub month: i32,
    pub amount: i64,
    pub currency: String,
    pub reason: Option<String>,
    pub created_at: String, // ISO 8601 datetime string
    pub version: i32,
}
