-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS formation_categories
(
    id                      TEXT NOT NULL PRIMARY KEY,
    name                    TEXT NOT NULL UNIQUE,
    is_custom               INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0,1)),
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS train_formations
(
    id                      TEXT NOT NULL PRIMARY KEY,
    name                    TEXT NOT NULL UNIQUE,
    category_id             TEXT,
    start_year              INTEGER,
    end_year                INTEGER,
    epoch                   TEXT,
    notes                   TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                 INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (category_id) REFERENCES formation_categories(id) ON DELETE SET NULL,
    CHECK (start_year IS NULL OR end_year IS NULL OR start_year <= end_year)
);

CREATE INDEX IF NOT EXISTS idx_train_formations_category_id ON train_formations (category_id);
CREATE INDEX IF NOT EXISTS idx_train_formations_name ON train_formations (name);

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
