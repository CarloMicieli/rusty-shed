# Research: Budget Tracking Feature

**Feature**: 001-budget-tracking  
**Date**: January 30, 2026  
**Status**: Complete

## Executive Summary

This research document captures technology decisions, best practices, and design rationale for the Budget Tracking feature. All unknowns from the Technical Context have been resolved.

---

## 1. Roll-over Calculation Strategy

### Decision

Implement roll-over as a **derived calculation** rather than stored state.

### Rationale

- **Data integrity**: Stored roll-over values can become stale when historical purchases are modified
- **Simplicity**: Single source of truth - actual spending records
- **Recalculation**: When a past purchase is edited/deleted, recalculating the chain from that month forward is straightforward
- **Performance**: For a single user with ~12 months × 5 years = 60 records, in-memory calculation is negligible (<1ms)

### Implementation

```rust
fn calculate_rollover_chain(
    year: i32,
    config: &BudgetConfiguration,
    monthly_spending: &[MonthlySpending],
    extra_budgets: &[ExtraBudgetEntry],
) -> Vec<MonthlyBudgetRecord> {
    let mut rollover = 0i64;
    (1..=12).map(|month| {
        let base = config.monthly_amount();
        let extra = extra_budgets.iter()
            .filter(|e| e.year == year && e.month == month)
            .map(|e| e.amount)
            .sum::<i64>();
        let spent = monthly_spending.iter()
            .find(|s| s.month == month)
            .map(|s| s.total)
            .unwrap_or(0);
        let available = base + extra + rollover;
        let new_rollover = available - spent;
        rollover = new_rollover;
        MonthlyBudgetRecord { month, base, extra, spent, rollover: new_rollover, ... }
    }).collect()
}
```

### Alternatives Considered

1. **Stored roll-over per month**: Rejected because editing a January purchase would require cascading updates through December
2. **Event-sourced spending**: Overkill for single-user desktop app with local SQLite

---

## 2. Historical Data Aggregation Strategy

### Decision

Aggregate historical data into **quarterly summaries** at query time for display, while preserving monthly granularity in storage.

### Rationale

- **Flexibility**: Monthly data can always be re-aggregated differently if needed
- **Storage efficiency**: Individual purchase records already exist; no duplicate aggregation tables
- **Simplicity**: Use SQL GROUP BY for quarterly aggregation rather than maintaining separate materialized views

### Implementation

```sql
-- Quarterly spending summary
SELECT
    strftime('%Y', purchase_date) as year,
    CASE
        WHEN cast(strftime('%m', purchase_date) as integer) <= 3 THEN 'Q1'
        WHEN cast(strftime('%m', purchase_date) as integer) <= 6 THEN 'Q2'
        WHEN cast(strftime('%m', purchase_date) as integer) <= 9 THEN 'Q3'
        ELSE 'Q4'
    END as quarter,
    category,
    SUM(price_amount) as total_amount,
    currency as total_currency
FROM collection_items ci
JOIN purchase_info pi ON ci.id = pi.collection_item_id
WHERE pi.purchase_date >= date('now', '-5 years')
GROUP BY year, quarter, category, currency
ORDER BY year DESC, quarter DESC;
```

### Alternatives Considered

1. **Materialized quarterly summary table**: Rejected because it adds complexity with triggers/jobs to keep in sync
2. **Pre-computed on January 1st**: Rejected because historical edits would still require recalculation

---

## 3. Budget Configuration Storage

### Decision

Store budget configuration as a **singleton row** in a `budget_config` table, similar to the existing `settings` pattern.

### Rationale

- **Consistency**: Follows established pattern in `settings.rs` with `SETTINGS_ID: i64 = 1`
- **Simplicity**: Single user, single budget configuration
- **History**: No need to track budget configuration changes over time (user can always edit current)

### Schema

```sql
CREATE TABLE IF NOT EXISTS budget_config (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    mode TEXT NOT NULL CHECK (mode IN ('YEARLY', 'MONTHLY')),
    base_amount INTEGER NOT NULL CHECK (base_amount >= 0),
    currency TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 0
);
```

### Alternatives Considered

1. **Embed in settings table**: Rejected to keep concerns separated and allow independent evolution
2. **Per-year configuration**: Rejected as spec states budget resets annually but configuration persists

---

## 4. Extra Budget Entries Storage

### Decision

Store extra budget entries as **separate rows** in an `extra_budgets` table with year-month composite key logic.

### Rationale

- **Multiple entries per month**: Spec allows multiple one-time injections (gift + sale + bonus)
- **Audit trail**: Each entry can have a reason/description
- **Query simplicity**: Easy to SUM by year-month

### Schema

```sql
CREATE TABLE IF NOT EXISTS extra_budgets (
    id TEXT PRIMARY KEY,
    year INTEGER NOT NULL CHECK (year >= 2000 AND year <= 2100),
    month INTEGER NOT NULL CHECK (month >= 1 AND month <= 12),
    amount INTEGER NOT NULL CHECK (amount > 0),
    currency TEXT NOT NULL,
    reason TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_extra_budgets_year_month ON extra_budgets(year, month);
```

---

## 5. Category Mapping for Budget Tracking

### Decision

Reuse the existing `Category` enum from `catalog/domain/railway_model/category.rs` for budget category tracking.

### Rationale

- **Consistency**: Categories are already defined and used throughout the system
- **No duplication**: Avoid maintaining separate budget-specific category list
- **Type safety**: `Category` enum is already `sqlx::Type` and `specta::Type`

### Existing Categories (per spec)

1. Locomotives
2. TrainSets
3. StarterSets
4. FreightCars
5. PassengerCars
6. ElectricMultipleUnits
7. Railcars

### Note

The spec mentions "Railway Tracks" and "Decoders" as categories, but these appear to be separate inventory features (tracks_inventory, dcc_inventory). For MVP, budget tracking will focus on rolling stock categories from `collection_items`. Future enhancement may add tracks and decoders.

---

## 6. Spending Data Source

### Decision

Derive spending from **existing `collection_items` with `purchase_info`** data.

### Rationale

- **Single source of truth**: Purchases are already recorded when items are added to collection
- **No duplicate data entry**: Users don't need to enter purchases twice
- **Existing infrastructure**: `PurchaseInfo`, `PurchasedInfo` already include price and date

### Query Pattern

```sql
SELECT
    strftime('%Y', pi.purchase_date) as year,
    cast(strftime('%m', pi.purchase_date) as integer) as month,
    ci.category,
    SUM(pi.price_amount) as total,
    pi.price_currency as currency
FROM collection_items ci
JOIN purchase_info pi ON ci.id = pi.collection_item_id
WHERE pi.type = 'purchased'  -- Exclude sold items
GROUP BY year, month, category, currency;
```

---

## 7. Dashboard Widget Integration

### Decision

Add budget widgets to existing dashboard infrastructure via the `DashboardSummary` response type.

### Rationale

- **Single query**: Dashboard already loads summary data in one call
- **Consistency**: Follows existing `DashboardTotals`, `DashboardRecentItem` patterns
- **Performance**: One round-trip for all dashboard data

### Extension

```rust
// Add to DashboardSummary
pub struct DashboardSummary {
    pub totals: DashboardTotals,
    pub recent_items: Vec<DashboardRecentItem>,
    pub depot_entries: Vec<DashboardDepotEntry>,
    // NEW: Budget summary for dashboard widgets
    pub budget: Option<BudgetDashboardSummary>,
}

pub struct BudgetDashboardSummary {
    pub remaining_amount: i64,
    pub remaining_percentage: f64,
    pub monthly_spending: Vec<MonthlySpendingPoint>, // 12 months
    pub quarterly_activity: Vec<QuarterlyActivityPoint>, // 5 years × 4 quarters
}
```

---

## 8. Visualization Libraries

### Decision

Use **Chart.js** via existing Skeleton UI chart components for donut and bar charts; custom Svelte component for heatmap.

### Rationale

- **Consistency**: Skeleton UI already provides chart wrappers
- **Simplicity**: No new dependencies for basic charts
- **Customization**: Heatmap is simple enough to build with CSS grid + Tailwind

### Alternatives Considered

1. **D3.js**: Overkill for these simple visualizations
2. **Apache ECharts**: Would add significant bundle size
3. **Svelte-specific charting**: Less mature ecosystem

---

## 9. Annual Reset Mechanism

### Decision

Implement annual reset as a **check at query time** rather than a scheduled job.

### Rationale

- **Desktop app**: No always-running server to execute cron jobs
- **Simplicity**: When loading budget data for current year, check if config.last_reset_year < current_year
- **Idempotent**: Safe to run multiple times

### Implementation

```rust
async fn ensure_annual_reset(
    config: &BudgetConfiguration,
    current_year: i32,
    repo: &impl BudgetRepository,
) -> Result<()> {
    if config.last_reset_year < current_year {
        // Roll-over is derived, so reset is just updating last_reset_year
        repo.update_reset_year(current_year).await?;
    }
    Ok(())
}
```

---

## 10. Currency Handling

### Decision

Inherit currency from **budget configuration** (which inherits from app settings at creation time).

### Rationale

- **Consistency**: Spec states currency inherits from app settings
- **Immutability**: Once budget is configured, currency is fixed for that budget
- **Multi-currency edge case**: If user has purchases in different currencies, only matching currency is summed (warn user)

### Warning Pattern

```rust
// When calculating spending, check for currency mismatches
if spending_currency != config.currency {
    log::warn!(
        "Skipping purchase {} with currency {} (budget uses {})",
        purchase_id, spending_currency, config.currency
    );
}
```

---

## Summary Table

| Topic           | Decision                  | Key Benefit            |
| --------------- | ------------------------- | ---------------------- |
| Roll-over       | Derived calculation       | Data integrity         |
| Historical data | Query-time aggregation    | Flexibility            |
| Budget config   | Singleton table           | Simplicity             |
| Extra budgets   | Separate entries table    | Audit trail            |
| Categories      | Reuse existing enum       | Consistency            |
| Spending source | Collection purchase_info  | Single source of truth |
| Dashboard       | Extend DashboardSummary   | Performance            |
| Charts          | Chart.js + custom heatmap | Minimal dependencies   |
| Annual reset    | Query-time check          | Desktop-friendly       |
| Currency        | Inherit from config       | Consistency            |
