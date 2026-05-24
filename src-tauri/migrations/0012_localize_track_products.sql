-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE track_product_translations (
    track_id                TEXT NOT NULL,
    language_code           TEXT NOT NULL,
    description             TEXT,
    details                 TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (track_id, language_code),
    FOREIGN KEY (track_id) REFERENCES track_products (track_id) ON DELETE CASCADE
);

INSERT INTO track_product_translations (track_id, language_code, description)
SELECT track_id, 'en', description
FROM track_products
WHERE description IS NOT NULL AND TRIM(description) != '';

PRAGMA foreign_keys = OFF;

CREATE TABLE track_products_new (
    id                                  TEXT PRIMARY KEY,
    track_id                            TEXT NOT NULL UNIQUE,
    manufacturer_id                     TEXT NOT NULL,
    product_code                        TEXT NOT NULL,
    with_roadbed                        INTEGER NOT NULL CHECK(with_roadbed IN (0,1)) DEFAULT 0,
    length_mm                           INTEGER,
    radius_mm                           INTEGER,
    track_code                          TEXT,
    track_type                          TEXT,
    created_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                             INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers (id) ON DELETE CASCADE
);

INSERT INTO track_products_new (
    id,
    track_id,
    manufacturer_id,
    product_code,
    with_roadbed,
    length_mm,
    radius_mm,
    track_code,
    track_type,
    created_at,
    updated_at,
    version
)
SELECT
    id,
    track_id,
    manufacturer_id,
    product_code,
    with_roadbed,
    length_mm,
    radius_mm,
    track_code,
    track_type,
    created_at,
    updated_at,
    version
FROM track_products;

DROP TABLE track_products;
ALTER TABLE track_products_new RENAME TO track_products;

CREATE INDEX idx_track_products_manufacturer_product_code
    ON track_products (manufacturer_id, product_code);

PRAGMA foreign_keys = ON;

CREATE VIRTUAL TABLE track_product_search_idx USING fts5 (
    track_id UNINDEXED,
    language_code UNINDEXED,
    description,
    details,
    track_code,
    track_type,
    tokenize = 'unicode61'
);

INSERT INTO track_product_search_idx (
    track_id,
    language_code,
    description,
    details,
    track_code,
    track_type
)
SELECT
    tpt.track_id,
    tpt.language_code,
    COALESCE(tpt.description, ''),
    COALESCE(tpt.details, ''),
    COALESCE(tp.track_code, ''),
    COALESCE(tp.track_type, '')
FROM track_product_translations tpt
JOIN track_products tp ON tp.track_id = tpt.track_id;