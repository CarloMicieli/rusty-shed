-- Feature 041 foundation: protection flag and canonical case-insensitive indexes.

ALTER TABLE manufacturers
    ADD COLUMN is_system_seeded INTEGER NOT NULL DEFAULT 0 CHECK (is_system_seeded IN (0, 1));

ALTER TABLE sellers
    ADD COLUMN is_system_seeded INTEGER NOT NULL DEFAULT 0 CHECK (is_system_seeded IN (0, 1));

CREATE UNIQUE INDEX IF NOT EXISTS idx_manufacturers_name_ci
    ON manufacturers (LOWER(name));

CREATE UNIQUE INDEX IF NOT EXISTS idx_sellers_name_ci
    ON sellers (LOWER(name));
