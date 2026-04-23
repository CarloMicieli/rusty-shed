-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS decoders
(
    id                      TEXT PRIMARY KEY,
    manufacturer_id         TEXT NOT NULL,
    product_code            TEXT NOT NULL,
    decoder_type            TEXT NOT NULL,
    protocol                TEXT NOT NULL,
    decoder_interface       TEXT,
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_decoders_manufacturer ON decoders (manufacturer_id);
CREATE INDEX IF NOT EXISTS idx_decoders_type ON decoders (decoder_type);

CREATE TABLE IF NOT EXISTS digital_rolling_stocks
(
    id                      TEXT PRIMARY KEY,
    owned_rolling_stock_id  TEXT NOT NULL,
    dcc_address             INTEGER NOT NULL CHECK (dcc_address >= 1 AND dcc_address <= 9999),
    installed_decoder_id    TEXT,
    FOREIGN KEY (owned_rolling_stock_id) REFERENCES owned_rolling_stocks (id)   ON DELETE CASCADE,
    FOREIGN KEY (installed_decoder_id)   REFERENCES decoders (id)               ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_digital_rolling_stocks_dcc_address ON digital_rolling_stocks (dcc_address);
CREATE INDEX IF NOT EXISTS idx_digital_rolling_stocks_owned ON digital_rolling_stocks (owned_rolling_stock_id);
CREATE INDEX IF NOT EXISTS idx_digital_rolling_stocks_decoder ON digital_rolling_stocks (installed_decoder_id);
