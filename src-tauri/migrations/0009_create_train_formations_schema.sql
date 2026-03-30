-- ──────────────────────────────────────────────
-- Migration: Train Formations (Feature 039)
-- ──────────────────────────────────────────────

-- 1. Prototype Library (master catalog)
CREATE TABLE IF NOT EXISTS prototypes
(
    id                  TEXT NOT NULL PRIMARY KEY,
    railway_company_id   TEXT NOT NULL,
    series_code         TEXT NOT NULL,
    car_type            TEXT NOT NULL,
    service_level       TEXT,
    category            TEXT NOT NULL,
    is_motorized        INTEGER NOT NULL DEFAULT 0 CHECK (is_motorized IN (0,1)),
    default_is_dummy    INTEGER NOT NULL DEFAULT 0 CHECK (default_is_dummy IN (0,1)),
    is_custom           INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0,1)),
    notes               TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version             INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (railway_company_id) REFERENCES railway_companies(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_prototypes_company_series ON prototypes (railway_company_id, series_code);
CREATE INDEX IF NOT EXISTS idx_prototypes_car_type ON prototypes (car_type);
CREATE INDEX IF NOT EXISTS idx_prototypes_is_custom ON prototypes (is_custom);

-- 2. Formation Categories
CREATE TABLE IF NOT EXISTS formation_categories
(
    id          TEXT NOT NULL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    is_custom   INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0,1)),
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 3. Train Formations (header)
CREATE TABLE IF NOT EXISTS train_formations
(
    id          TEXT NOT NULL PRIMARY KEY,
    name        TEXT NOT NULL UNIQUE,
    category_id TEXT,
    start_year  INTEGER,
    end_year    INTEGER,
    epoch       TEXT,
    notes       TEXT,
    created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version     INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (category_id) REFERENCES formation_categories(id) ON DELETE SET NULL,
    CHECK (start_year IS NULL OR end_year IS NULL OR start_year <= end_year)
);

CREATE INDEX IF NOT EXISTS idx_train_formations_category_id ON train_formations (category_id);
CREATE INDEX IF NOT EXISTS idx_train_formations_name ON train_formations (name);

-- 4. Formation Elements (composition slots)
CREATE TABLE IF NOT EXISTS formation_elements
(
    id                      TEXT NOT NULL PRIMARY KEY,
    formation_id            TEXT NOT NULL,
    prototype_id            TEXT NOT NULL,
    owned_rolling_stock_id  TEXT,
    snapshot_series_code    TEXT,
    snapshot_company_name   TEXT,
    position_order          INTEGER NOT NULL,
    traction_override       INTEGER NOT NULL DEFAULT 0,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (formation_id)           REFERENCES train_formations(id)      ON DELETE CASCADE,
    FOREIGN KEY (prototype_id)           REFERENCES prototypes(id)            ON DELETE RESTRICT,
    FOREIGN KEY (owned_rolling_stock_id) REFERENCES owned_rolling_stocks(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_formation_elements_formation ON formation_elements (formation_id, position_order);
CREATE INDEX IF NOT EXISTS idx_formation_elements_prototype ON formation_elements (prototype_id);

-- 5. Add prototype_id to owned_rolling_stocks (additive, non-breaking)
ALTER TABLE owned_rolling_stocks ADD COLUMN prototype_id TEXT REFERENCES prototypes(id) ON DELETE SET NULL;
CREATE INDEX IF NOT EXISTS idx_owned_rolling_stocks_prototype_id ON owned_rolling_stocks (prototype_id);

-- 6. Seed: Formation Categories
INSERT OR IGNORE INTO formation_categories (id, name, is_custom) VALUES
    ('trn:formation-category:eurocity',  'EuroCity',   0),
    ('trn:formation-category:intercity', 'Intercity',  0),
    ('trn:formation-category:tee',       'TEE',        0),
    ('trn:formation-category:express',   'Express',    0),
    ('trn:formation-category:regional',  'Regional',   0),
    ('trn:formation-category:freight',   'Freight',    0),
    ('trn:formation-category:special',   'Special',    0),
    ('trn:formation-category:thematic',  'Thematic',   0);
