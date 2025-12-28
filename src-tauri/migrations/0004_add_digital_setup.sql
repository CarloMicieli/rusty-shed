-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS decoders (
    id TEXT PRIMARY KEY,
    manufacturer_id TEXT NOT NULL REFERENCES manufacturers(id),
    product_code TEXT,
    decoder_type TEXT NOT NULL,
    protocol TEXT NOT NULL,
    decoder_interface TEXT NOT NULL
);

ALTER TABLE owned_rolling_stocks ADD COLUMN dcc_address INTEGER;
ALTER TABLE owned_rolling_stocks ADD COLUMN installed_decoder_id TEXT;
