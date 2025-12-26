CREATE TABLE IF NOT EXISTS manufacturers
(
    id                      TEXT PRIMARY KEY,
    name                    TEXT NOT NULL,
    registered_company_name TEXT,
    country_code            TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_manufacturers_name ON manufacturers (name);

CREATE TABLE IF NOT EXISTS railway_companies
(
    id                      TEXT PRIMARY KEY,
    name                    TEXT NOT NULL,
    registered_company_name TEXT,
    country_code            TEXT,
    status                  TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_railway_companies_name ON railway_companies (name);

CREATE TABLE IF NOT EXISTS railway_models
(
    id                  TEXT PRIMARY KEY,
    manufacturer_id     TEXT NOT NULL,
    product_code        TEXT NOT NULL,
    description         TEXT NOT NULL,
    details             TEXT,
    power_method        TEXT NOT NULL,
    scale               TEXT NOT NULL,
    epoch               TEXT NOT NULL,
    category            TEXT NOT NULL,
    delivery_date       TEXT,
    availability_status TEXT,
    created_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_railway_model_product_code ON railway_models (product_code);

CREATE TABLE IF NOT EXISTS rolling_stocks
(
    id                          TEXT PRIMARY KEY,
    railway_model_id            TEXT    NOT NULL,
    category                    TEXT    NOT NULL,
    railway_company_id          TEXT    NOT NULL,
    railway_display             TEXT,
    livery                      TEXT,
    length_inches               REAL,
    length_millimeters          REAL,
    technical_minimum_radius_mm REAL,
    technical_coupling          TEXT,
    technical_flywheel_fitted   TEXT,
    technical_body_shell        TEXT,
    technical_chassis           TEXT,
    technical_interior_lights   TEXT,
    technical_lights            TEXT,
    technical_sprung_buffers    TEXT,
    type_name                   TEXT,
    class_name                  TEXT,
    road_number                 TEXT,
    series                      TEXT,
    depot                       TEXT,
    electric_multiple_unit_type TEXT,
    freight_car_type            TEXT,
    locomotive_type             TEXT,
    passenger_car_type          TEXT,
    railcar_type                TEXT,
    service_level               TEXT,
    dcc_interface               TEXT,
    control                     TEXT,
    is_dummy                    INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (railway_model_id) REFERENCES railway_models (id) ON DELETE CASCADE,
    FOREIGN KEY (railway_company_id) REFERENCES railway_companies (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_rolling_stock_road_number ON rolling_stocks (road_number);
CREATE INDEX IF NOT EXISTS idx_rolling_stock_railway_model_id ON rolling_stocks (railway_model_id);
