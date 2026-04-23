-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS budget_config
(
    id                  INTEGER PRIMARY KEY CHECK(id = 1), -- Singleton constraint
    mode                TEXT NOT NULL CHECK(mode IN ('YEARLY', 'MONTHLY')),
    base_amount         INTEGER NOT NULL CHECK(base_amount >= 0), -- Minor currency units (cents)
    currency            TEXT NOT NULL, -- ISO 4217 currency code (e.g., 'USD', 'EUR')
    last_reset_year     INTEGER NOT NULL, -- Last year when annual reset was performed
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version             INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS extra_budgets
(
    id                  TEXT PRIMARY KEY,
    year                INTEGER NOT NULL CHECK(year >= 2000 AND year <= 2100),
    month               INTEGER NOT NULL CHECK(month >= 1 AND month <= 12),
    amount              INTEGER NOT NULL CHECK(amount > 0), -- Minor currency units (must be positive)
    currency            TEXT NOT NULL, -- ISO 4217 currency code
    reason              TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version             INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_extra_budgets_year_month
    ON extra_budgets(year, month);
