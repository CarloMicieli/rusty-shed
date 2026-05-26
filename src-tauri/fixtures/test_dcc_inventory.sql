-- Fixtures for dcc_inventory integration tests

PRAGMA foreign_keys = ON;
BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS decoders (
  id TEXT PRIMARY KEY,
  manufacturer_id TEXT NOT NULL,
  product_code TEXT,
  decoder_type TEXT NOT NULL,
  protocol TEXT NOT NULL,
  decoder_interface TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS manufacturers (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS owned_rolling_stocks (
  id TEXT PRIMARY KEY,
  rolling_stock_id TEXT,
  collection_item_id TEXT NOT NULL,
  notes TEXT,
  dcc_address INTEGER,
  installed_decoder_id TEXT
);

CREATE TABLE IF NOT EXISTS rolling_stocks (
  id TEXT PRIMARY KEY,
  railway_model_id TEXT,
  category TEXT NOT NULL,
  railway_company_id TEXT,
  series_code TEXT,
  series TEXT,
  road_number TEXT,
  control TEXT,
  is_dummy INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS railway_companies (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS railway_models (
  id TEXT PRIMARY KEY,
  manufacturer_id TEXT NOT NULL,
  product_code TEXT NOT NULL,
  epoch TEXT NOT NULL,
  category TEXT NOT NULL,
  scale TEXT NOT NULL,
  power_method TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS railway_model_translations (
  railway_model_id TEXT NOT NULL,
  language_code TEXT NOT NULL,
  description TEXT NOT NULL,
  details TEXT,
  PRIMARY KEY (railway_model_id, language_code)
);

-- sample manufacturer
INSERT INTO manufacturers (id, name)
VALUES ('trn:manufacturer:acme', 'ACME');

-- sample decoder
INSERT INTO decoders (id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface)
VALUES ('trn:decoder:acme:d-100', 'trn:manufacturer:acme', 'D-100', 'PLAIN', 'DCC', 'PLUX_22');

-- sample railway company
INSERT INTO railway_companies (id, name)
VALUES ('trn:railway-company:test', 'Test Railway');

-- sample railway model
INSERT INTO railway_models (id, manufacturer_id, product_code, epoch, category, scale, power_method)
VALUES ('trn:railway-model:test', 'trn:manufacturer:acme', 'TEST-001', 'III', 'STARTER_SET', 'H0', 'AC');

INSERT INTO railway_model_translations (railway_model_id, language_code, description)
VALUES ('trn:railway-model:test', 'en', 'Test Model');

-- sample collection and collection item referenced by owned rolling stock
INSERT INTO collections (id, name)
VALUES ('trn:collection:test', 'Test Collection');

INSERT INTO collection_items (id, collection_id, railway_model_id, added_date)
VALUES ('trn:collection-item:test', 'trn:collection:test', 'trn:railway-model:test', '2026-01-01');

-- sample rolling stock
INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, series_code, series, road_number, control, is_dummy)
VALUES ('trn:rolling-stock:test', 'trn:railway-model:test', 'LOCOMOTIVE', 'trn:railway-company:test', 'E.428', 'E.428 Series', '001', 'DCC_READY', 0);

-- sample owned rolling stock
INSERT INTO owned_rolling_stocks (id, rolling_stock_id, collection_item_id, dcc_address, installed_decoder_id)
VALUES ('trn:owned-rolling-stock:11111111-1111-1111-1111-111111111111', 'trn:rolling-stock:test', 'trn:collection-item:test', 500, 'trn:decoder:acme:d-100');

COMMIT;
