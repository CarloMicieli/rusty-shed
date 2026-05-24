-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS manufacturers
(
    id                                   TEXT PRIMARY KEY,
    name                                 TEXT NOT NULL,
    registered_company_name              TEXT,
    status                               TEXT NOT NULL DEFAULT 'ACTIVE',
    street_address                       TEXT,
    extended_address                     TEXT,
    city                                 TEXT,
    state_region                         TEXT,
    postal_code                          TEXT,
    country_code                         TEXT,
    website_url                          TEXT,
    created_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                              INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_manufacturers_name ON manufacturers (name);

CREATE TABLE IF NOT EXISTS railway_companies
(
    id                                   TEXT PRIMARY KEY,
    name                                 TEXT NOT NULL,
    registered_company_name              TEXT,
    country_code                         TEXT,
    status                               TEXT,
    operating_since                      TEXT,
    operating_until                      TEXT,
    created_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                              INTEGER NOT NULL DEFAULT 0
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_railway_companies_name ON railway_companies (name);

CREATE TABLE IF NOT EXISTS railway_models
(
    id                                   TEXT PRIMARY KEY,
    manufacturer_id                      TEXT NOT NULL,
    product_code                         TEXT NOT NULL,
    power_method                         TEXT NOT NULL,
    scale                                TEXT NOT NULL,
    epoch                                TEXT NOT NULL,
    category                             TEXT NOT NULL,
    delivery_date                        TEXT,
    availability_status                  TEXT,
    created_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                           TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                              INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_railway_model_product_code ON railway_models (product_code);

CREATE TABLE IF NOT EXISTS prototypes
(
    id                                   TEXT    NOT NULL PRIMARY KEY,
    railway_company_id                   TEXT    NOT NULL,
    series_code                          TEXT    NOT NULL,
    friendly_name                        TEXT,
    is_motorized                         INTEGER NOT NULL DEFAULT 0 CHECK (is_motorized IN (0, 1)),
    default_is_dummy                     INTEGER NOT NULL DEFAULT 0 CHECK (default_is_dummy IN (0, 1)),
    is_custom                            INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0, 1)),
    notes                                TEXT,
    specification_type                   TEXT    NOT NULL,
    locomotive_type                      TEXT,
    locomotive_series                    TEXT,
    service_level                        TEXT,
    passenger_car_type                   TEXT,
    freight_car_type                     TEXT,
    railcar_type                         TEXT,
    electric_multiple_unit_type          TEXT,
    elements_count                       INTEGER,
    is_permanently_coupled               INTEGER CHECK (is_permanently_coupled IN (0, 1)),
    created_at                           TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                           TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                              INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (railway_company_id) REFERENCES railway_companies (id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_prototypes_company_series ON prototypes (railway_company_id, series_code);
CREATE INDEX IF NOT EXISTS idx_prototypes_specification_type ON prototypes (specification_type);
CREATE INDEX IF NOT EXISTS idx_prototypes_is_custom ON prototypes (is_custom);

CREATE TABLE IF NOT EXISTS rolling_stocks
(
    id                                   TEXT PRIMARY KEY,
    railway_model_id                     TEXT NOT NULL,
    category                             TEXT NOT NULL,
    railway_company_id                   TEXT NOT NULL,
    series_code                          TEXT NOT NULL,
    prototype_id                         TEXT,
    series                               TEXT,
    road_number                          TEXT,
    friendly_name                        TEXT,
    depot                                TEXT,
    livery                               TEXT,
    electric_multiple_unit_type          TEXT,
    freight_car_type                     TEXT,
    locomotive_type                      TEXT,
    passenger_car_type                   TEXT,
    railcar_type                         TEXT,
    service_level                        TEXT,
    length_inches                        REAL,
    length_millimeters                   REAL,
    technical_minimum_radius_mm          REAL,
    technical_coupling_socket            TEXT,
    technical_coupling_close_couplers    TEXT,
    technical_coupling_digital_shunting  TEXT,
    technical_flywheel_fitted            TEXT,
    technical_body_shell                 TEXT,
    technical_chassis                    TEXT,
    technical_interior_lights            TEXT,
    technical_lights                     TEXT,
    technical_sprung_buffers             TEXT,
    dcc_interface                        TEXT,
    control                              TEXT,
    is_dummy                             INTEGER NOT NULL CHECK (is_dummy IN (0, 1)) DEFAULT 0,
    FOREIGN KEY (prototype_id)       REFERENCES prototypes (id) ON DELETE SET NULL,
    FOREIGN KEY (railway_model_id)   REFERENCES railway_models (id)    ON DELETE CASCADE,
    FOREIGN KEY (railway_company_id) REFERENCES railway_companies (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_rolling_stock_road_number       ON rolling_stocks (road_number);
CREATE INDEX IF NOT EXISTS idx_rolling_stock_railway_model_id  ON rolling_stocks (railway_model_id);
CREATE INDEX IF NOT EXISTS idx_rolling_stock_prototype_id      ON rolling_stocks (prototype_id);