-- Fixtures for dcc_inventory integration tests

PRAGMA foreign_keys = OFF;
BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS decoders (
  id TEXT PRIMARY KEY,
  manufacturer_id TEXT NOT NULL,
  product_code TEXT,
  decoder_type TEXT NOT NULL,
  protocol TEXT NOT NULL,
  decoder_interface TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS digital_rolling_stocks (
  id TEXT PRIMARY KEY,
  owned_rolling_stock_id TEXT NOT NULL,
  dcc_address INTEGER NOT NULL,
  installed_decoder_id TEXT
);

-- sample decoder
INSERT INTO decoders (id, manufacturer_id, product_code, decoder_type, protocol, decoder_interface)
VALUES ('trn:decoder:acme:d-100', 'trn:manufacturer:acme', 'D-100', 'PLAIN', 'DCC', 'PLUX_22');

-- sample digital rolling stock referencing decoder
INSERT INTO digital_rolling_stocks (id, owned_rolling_stock_id, dcc_address, installed_decoder_id)
VALUES ('trn:digital-rolling-stock:00000000-0000-0000-0000-000000000001', 'trn:owned-rolling-stock:11111111-1111-1111-1111-111111111111', 500, 'trn:decoder:acme:d-100');

COMMIT;
