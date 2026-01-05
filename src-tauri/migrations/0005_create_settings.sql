-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS settings (
  id                                    INTEGER PRIMARY KEY CHECK (id = 1),
  currency                              TEXT NOT NULL,
  length_unit                           TEXT NOT NULL,
  favorite_scale                        TEXT NOT NULL,
  favorite_power_method                 TEXT NOT NULL,
  language_code                         TEXT NOT NULL
);

-- Seed a single default row if it does not exist
INSERT INTO settings (id, currency, length_unit, favorite_scale, favorite_power_method, language_code)
SELECT 1, 'EUR', 'MILLIMETERS', 'H0', 'DC', 'en'
WHERE NOT EXISTS (SELECT 1 FROM settings WHERE id = 1);
