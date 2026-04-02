-- Migration 0010: Rebuild prototypes table with typed specification schema
--
-- The previous schema stored car_type / category / service_level as free-form
-- strings.  This migration converts to a single-table inheritance layout with:
--   • specification_type  — discriminator (LOCOMOTIVE | PASSENGER_CAR | FREIGHT_CAR |
--                           RAILCAR | ELECTRIC_MULTIPLE_UNIT)
--   • per-specification nullable columns for each variant's attributes
--   • friendly_name       — popular nickname separate from series_code
--
-- Existing IDs are preserved so all formation_elements foreign keys remain valid.

CREATE TABLE IF NOT EXISTS prototypes_new
(
    id                          TEXT    NOT NULL PRIMARY KEY,
    railway_company_id          TEXT    NOT NULL,
    series_code                 TEXT    NOT NULL,
    friendly_name               TEXT,
    is_motorized                INTEGER NOT NULL DEFAULT 0 CHECK (is_motorized IN (0, 1)),
    default_is_dummy            INTEGER NOT NULL DEFAULT 0 CHECK (default_is_dummy IN (0, 1)),
    is_custom                   INTEGER NOT NULL DEFAULT 0 CHECK (is_custom IN (0, 1)),
    notes                       TEXT,

    -- Discriminator: one of LOCOMOTIVE | PASSENGER_CAR | FREIGHT_CAR | RAILCAR | ELECTRIC_MULTIPLE_UNIT
    specification_type          TEXT    NOT NULL,

    -- Locomotive-specific (non-null only when specification_type = 'LOCOMOTIVE')
    locomotive_type             TEXT,
    locomotive_series           TEXT,

    -- PassengerCar-specific (non-null only when specification_type = 'PASSENGER_CAR')
    service_level               TEXT,
    passenger_car_type          TEXT,

    -- FreightCar-specific (non-null only when specification_type = 'FREIGHT_CAR')
    freight_car_type            TEXT,

    -- Railcar-specific (non-null only when specification_type = 'RAILCAR')
    railcar_type                TEXT,

    -- ElectricMultipleUnit-specific (non-null only when specification_type = 'ELECTRIC_MULTIPLE_UNIT')
    electric_multiple_unit_type TEXT,
    elements_count              INTEGER,
    is_permanently_coupled      INTEGER CHECK (is_permanently_coupled IN (0, 1)),

    created_at                  TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                  TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                     INTEGER NOT NULL DEFAULT 0,

    FOREIGN KEY (railway_company_id) REFERENCES railway_companies (id) ON DELETE RESTRICT
);

-- Migrate existing rows, mapping the old free-form string columns to the new schema.
-- Old `category` values: 'Locomotive', 'Passenger', 'Freight'
-- Old `service_level` values: '1st Class', '2nd Class', '3rd Class'
-- Default locomotive_type for all old Locomotive rows → ELECTRIC_LOCOMOTIVE
-- Default passenger_car_type for old Passenger rows → OPEN_COACH
INSERT INTO prototypes_new (id,
                            railway_company_id,
                            series_code,
                            is_motorized,
                            default_is_dummy,
                            is_custom,
                            notes,
                            specification_type,
                            locomotive_type,
                            service_level,
                            passenger_car_type,
                            created_at,
                            updated_at,
                            version)
SELECT id,
       railway_company_id,
       series_code,
       is_motorized,
       default_is_dummy,
       is_custom,
       notes,
       CASE category
           WHEN 'Locomotive' THEN 'LOCOMOTIVE'
           WHEN 'Passenger' THEN 'PASSENGER_CAR'
           WHEN 'Freight' THEN 'FREIGHT_CAR'
           ELSE 'LOCOMOTIVE'
           END,
       CASE WHEN category = 'Locomotive' THEN 'ELECTRIC_LOCOMOTIVE' ELSE NULL END,
       CASE service_level
           WHEN '1st Class' THEN 'FIRST'
           WHEN '2nd Class' THEN 'SECOND'
           WHEN '3rd Class' THEN 'THIRD'
           WHEN '1/2' THEN 'FIRST_SECOND'
           WHEN '2/3' THEN 'SECOND_THIRD'
           WHEN '1/2/3' THEN 'FIRST_SECOND_THIRD'
           ELSE NULL
           END,
       CASE WHEN category = 'Passenger' THEN 'OPEN_COACH' ELSE NULL END,
       created_at,
       updated_at,
       version
FROM prototypes;

DROP TABLE prototypes;
ALTER TABLE prototypes_new
    RENAME TO prototypes;

CREATE INDEX IF NOT EXISTS idx_prototypes_company_series ON prototypes (railway_company_id, series_code);
CREATE INDEX IF NOT EXISTS idx_prototypes_specification_type ON prototypes (specification_type);
CREATE INDEX IF NOT EXISTS idx_prototypes_is_custom ON prototypes (is_custom);
