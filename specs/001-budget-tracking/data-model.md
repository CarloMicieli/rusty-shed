# Data Model: Budget Tracking

**Feature**: 001-budget-tracking  
**Date**: January 30, 2026  
**Status**: Draft

---

## Entity Relationship Diagram

```
┌─────────────────────────┐
│   budget_config         │
│   (singleton, id=1)     │
├─────────────────────────┤
│ id: INTEGER PK          │
│ mode: TEXT              │──┐
│ base_amount: INTEGER    │  │ Currency inherited
│ currency: TEXT          │  │ from settings
│ last_reset_year: INT    │  │
│ created_at: TEXT        │  │
│ updated_at: TEXT        │  │
│ version: INTEGER        │  │
└─────────────────────────┘  │
                             │
┌─────────────────────────┐  │
│   extra_budgets         │  │
├─────────────────────────┤  │
│ id: TEXT PK             │  │
│ year: INTEGER           │  │
│ month: INTEGER          │  │
│ amount: INTEGER         │◀─┤ Same currency
│ currency: TEXT          │  │
│ reason: TEXT (nullable) │  │
│ created_at: TEXT        │  │
│ version: INTEGER        │  │
└─────────────────────────┘  │
                             │
         ┌───────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────┐
│                 collection_items                     │
│              (existing, not modified)                │
├─────────────────────────────────────────────────────┤
│ id, category, purchase_info (price, date, etc.)     │
│                                                      │
│ → Spending is DERIVED from purchase_info.price      │
│   grouped by month/year/category                    │
└─────────────────────────────────────────────────────┘
```

---

## Tables

### 1. `budget_config` (New)

Budget configuration singleton - stores the user's budget settings.

| Column            | Type    | Constraints                            | Description                               |
| ----------------- | ------- | -------------------------------------- | ----------------------------------------- |
| `id`              | INTEGER | PK, CHECK(id = 1)                      | Singleton constraint                      |
| `mode`            | TEXT    | NOT NULL, CHECK(IN 'YEARLY','MONTHLY') | Budget input mode                         |
| `base_amount`     | INTEGER | NOT NULL, CHECK(>= 0)                  | Amount in minor currency units (cents)    |
| `currency`        | TEXT    | NOT NULL                               | ISO 4217 currency code                    |
| `last_reset_year` | INTEGER | NOT NULL                               | Last year when annual reset was performed |
| `created_at`      | TEXT    | NOT NULL, DEFAULT CURRENT_TIMESTAMP    | Creation timestamp                        |
| `updated_at`      | TEXT    | NOT NULL, DEFAULT CURRENT_TIMESTAMP    | Last update timestamp                     |
| `version`         | INTEGER | NOT NULL, DEFAULT 0                    | Optimistic locking version                |

**Indexes**: None (singleton table)

### 2. `extra_budgets` (New)

One-time budget injections for specific months.

| Column       | Type    | Constraints                          | Description                                  |
| ------------ | ------- | ------------------------------------ | -------------------------------------------- |
| `id`         | TEXT    | PK                                   | UUID                                         |
| `year`       | INTEGER | NOT NULL, CHECK(>= 2000 AND <= 2100) | Target year                                  |
| `month`      | INTEGER | NOT NULL, CHECK(>= 1 AND <= 12)      | Target month (1-12)                          |
| `amount`     | INTEGER | NOT NULL, CHECK(> 0)                 | Amount in minor currency units               |
| `currency`   | TEXT    | NOT NULL                             | ISO 4217 currency code                       |
| `reason`     | TEXT    | NULL                                 | Optional description (e.g., "Birthday gift") |
| `created_at` | TEXT    | NOT NULL, DEFAULT CURRENT_TIMESTAMP  | Creation timestamp                           |
| `version`    | INTEGER | NOT NULL, DEFAULT 0                  | Optimistic locking version                   |

**Indexes**:

- `idx_extra_budgets_year_month` ON (year, month)

---

## Domain Entities (Rust)

### BudgetConfiguration (Aggregate Root)

```rust
/// The user's budget configuration.
/// Singleton aggregate - only one configuration exists per user.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BudgetConfiguration {
    pub id: BudgetConfigId,
    pub mode: BudgetMode,
    pub base_amount: MonetaryAmount,
    pub last_reset_year: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,

    #[serde(skip)]
    pending_events: Vec<BudgetEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type, sqlx::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[sqlx(type_name = "TEXT", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BudgetMode {
    Yearly,
    Monthly,
}

impl BudgetConfiguration {
    /// Returns the monthly budget amount.
    /// For YEARLY mode, divides by 12.
    pub fn monthly_amount(&self) -> i64 {
        match self.mode {
            BudgetMode::Yearly => self.base_amount.amount / 12,
            BudgetMode::Monthly => self.base_amount.amount,
        }
    }

    /// Returns the yearly budget amount.
    /// For MONTHLY mode, multiplies by 12.
    pub fn yearly_amount(&self) -> i64 {
        match self.mode {
            BudgetMode::Yearly => self.base_amount.amount,
            BudgetMode::Monthly => self.base_amount.amount * 12,
        }
    }
}
```

### ExtraBudgetEntry (Value Object)

```rust
/// A one-time budget injection for a specific month.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct ExtraBudgetEntry {
    pub id: ExtraBudgetId,
    pub year: i32,
    pub month: u8,  // 1-12
    pub amount: MonetaryAmount,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub version: u32,
}
```

### MonthlyBudgetRecord (Read Model / Projection)

```rust
/// Computed budget status for a single month.
/// This is a read model, not persisted directly.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyBudgetRecord {
    pub year: i32,
    pub month: u8,  // 1-12
    pub base_budget: i64,        // Monthly allocation
    pub extra_budget: i64,       // Sum of extra budgets for this month
    pub actual_spend: i64,       // Sum of purchases this month
    pub rollover_in: i64,        // Rollover from previous month
    pub rollover_out: i64,       // Rollover to next month
    pub status: MonthStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonthStatus {
    /// Future month, not yet reached
    Projected,
    /// Current month, in progress
    InProgress,
    /// Past month, completed
    Completed,
}

impl MonthlyBudgetRecord {
    /// Total available budget for this month (before spending)
    pub fn available(&self) -> i64 {
        self.base_budget + self.extra_budget + self.rollover_in
    }

    /// Remaining budget after spending
    pub fn remaining(&self) -> i64 {
        self.available() - self.actual_spend
    }

    /// Remaining as percentage (0.0 to 1.0+, can exceed 1.0 if rollover unused)
    pub fn remaining_percentage(&self) -> f64 {
        let available = self.available();
        if available == 0 {
            return 0.0;
        }
        self.remaining() as f64 / available as f64
    }
}
```

### QuarterlySummary (Read Model)

```rust
/// Aggregated spending data for a quarter (historical view).
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlySummary {
    pub year: i32,
    pub quarter: Quarter,
    pub total_spending: i64,
    pub currency: Currency,
    pub category_breakdown: Vec<CategorySpending>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum Quarter {
    Q1, Q2, Q3, Q4,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CategorySpending {
    pub category: Category,
    pub amount: i64,
    pub currency: Currency,
}
```

### BudgetDashboardSummary (Read Model)

```rust
/// Budget summary for dashboard widgets.
#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct BudgetDashboardSummary {
    /// Current month's remaining budget amount
    pub remaining_amount: i64,
    /// Remaining as percentage (0.0 to 1.0)
    pub remaining_percentage: f64,
    /// Total available this month (base + extra + rollover)
    pub total_available: i64,
    /// Currency for all amounts
    pub currency: Currency,
    /// Monthly spending for bar chart (12 data points)
    pub monthly_spending: Vec<MonthlySpendingPoint>,
    /// Monthly budget goal line amount
    pub monthly_goal: i64,
    /// Quarterly activity for heatmap (up to 20 data points: 5 years × 4 quarters)
    pub quarterly_activity: Vec<QuarterlyActivityPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct MonthlySpendingPoint {
    pub month: u8,  // 1-12
    pub amount: i64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct QuarterlyActivityPoint {
    pub year: i32,
    pub quarter: Quarter,
    pub spending_level: SpendingLevel,
    pub amount: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpendingLevel {
    None,    // 0
    Low,     // 1-33% of max
    Medium,  // 34-66% of max
    High,    // 67-100% of max
}
```

---

## Domain Events

```rust
/// Events emitted by budget aggregates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetEvent {
    /// Budget configuration was created or updated
    BudgetConfigured {
        mode: BudgetMode,
        base_amount: i64,
        currency: Currency,
        timestamp: DateTime<Utc>,
    },

    /// Extra budget was added to a month
    ExtraBudgetAdded {
        id: ExtraBudgetId,
        year: i32,
        month: u8,
        amount: i64,
        currency: Currency,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },

    /// Extra budget was removed
    ExtraBudgetRemoved {
        id: ExtraBudgetId,
        year: i32,
        month: u8,
        timestamp: DateTime<Utc>,
    },

    /// Annual reset was performed
    AnnualResetPerformed {
        year: i32,
        timestamp: DateTime<Utc>,
    },
}
```

---

## Validation Rules

### BudgetConfiguration

| Field         | Rule                          |
| ------------- | ----------------------------- |
| `mode`        | Must be 'YEARLY' or 'MONTHLY' |
| `base_amount` | Must be >= 0                  |
| `currency`    | Must be valid ISO 4217 code   |

### ExtraBudgetEntry

| Field      | Rule                                     |
| ---------- | ---------------------------------------- |
| `year`     | Must be between 2000 and 2100            |
| `month`    | Must be between 1 and 12                 |
| `amount`   | Must be > 0                              |
| `currency` | Must match budget configuration currency |
| `reason`   | Optional, max 500 characters             |

---

## State Transitions

### Budget Configuration Lifecycle

```
[No Config] ──create()──▶ [Configured]
                              │
                              ├── update() ──▶ [Configured]
                              │
                              └── (annual reset check on load)
```

### Monthly Budget Status

```
[Projected] ──(month arrives)──▶ [InProgress] ──(month ends)──▶ [Completed]
     │                                │                              │
     │                                │                              │
     ▼                                ▼                              ▼
  Future months              Current month only           Past months
  (Apr-Dec if now=Mar)       (exactly one)               (Jan-Feb if now=Mar)
```

---

## Relationships

### With Existing Entities

| This Feature                 | Relationship  | Existing Entity                     |
| ---------------------------- | ------------- | ----------------------------------- |
| `budget_config.currency`     | inherits from | `settings.currency`                 |
| Monthly spending calculation | reads from    | `collection_items.purchase_info`    |
| Category breakdown           | uses          | `catalog::Category` enum            |
| Currency amounts             | uses          | `core::MonetaryAmount` value object |

### Internal Relationships

| Entity A        | Relationship    | Entity B                           |
| --------------- | --------------- | ---------------------------------- |
| `budget_config` | 1:N             | `extra_budgets` (via year context) |
| `budget_config` | derives         | `MonthlyBudgetRecord` (read model) |
| `extra_budgets` | aggregates into | `MonthlyBudgetRecord.extra_budget` |
