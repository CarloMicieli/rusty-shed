-- Ensure quick-add entity names are unique regardless of case.
-- Note: SQLite LOWER() case-folding is ASCII-oriented without ICU extension.

DROP INDEX IF EXISTS idx_manufacturers_name;
CREATE UNIQUE INDEX IF NOT EXISTS idx_manufacturers_name_ci
    ON manufacturers (LOWER(name));

DROP INDEX IF EXISTS idx_sellers_name;
CREATE UNIQUE INDEX IF NOT EXISTS idx_sellers_name_ci
    ON sellers (LOWER(name));
