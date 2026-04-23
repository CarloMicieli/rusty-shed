-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS track_products
(
    id                                  TEXT PRIMARY KEY,
    track_id                            TEXT NOT NULL UNIQUE,
    manufacturer_id                     TEXT NOT NULL,
    product_code                        TEXT NOT NULL,
    with_roadbed                        INTEGER NOT NULL CHECK(with_roadbed IN (0,1)) DEFAULT 0,
    length_mm                           INTEGER,
    radius_mm                           INTEGER,
    track_code                          TEXT,
    track_type                          TEXT,
    description                         TEXT,
    created_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                             INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (manufacturer_id) REFERENCES manufacturers (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_track_products_manufacturer_product_code ON track_products (manufacturer_id, product_code);

CREATE TABLE IF NOT EXISTS track_inventories
(
    id                                  TEXT PRIMARY KEY,
    name                                TEXT NOT NULL,
    description                         TEXT,
    created_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    version                             INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS track_inventory_items
(
    inventory_id                        TEXT NOT NULL,
    track_id                            TEXT NOT NULL,
    quantity                            INTEGER NOT NULL DEFAULT 0,
    required                            INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (inventory_id, track_id),
    FOREIGN KEY (inventory_id)  REFERENCES track_inventories (id)    ON DELETE CASCADE,
    FOREIGN KEY (track_id)      REFERENCES track_products (track_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_track_inventory_items_inventory_id ON track_inventory_items (inventory_id);

CREATE INDEX idx_track_inventory_items_shortage 
ON track_inventory_items(inventory_id, track_id) 
WHERE quantity < required;

CREATE TABLE IF NOT EXISTS track_purchases
(
    id                                  TEXT PRIMARY KEY,
    inventory_id                        TEXT NOT NULL,
    track_id                            TEXT NOT NULL,
    quantity                            INTEGER NOT NULL,
    price_amount                        INTEGER NOT NULL CHECK(price_amount > 0), -- Minor currency units (must be positive)
    price_currency                      TEXT NOT NULL,
    seller_id                           TEXT,
    purchase_date                       TEXT NOT NULL,
    created_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (inventory_id)  REFERENCES track_inventories (id)    ON DELETE CASCADE,
    FOREIGN KEY (track_id)      REFERENCES track_products (track_id) ON DELETE RESTRICT,
    FOREIGN KEY (seller_id)     REFERENCES sellers (id)              ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_track_purchases_inventory_id ON track_purchases (inventory_id);
CREATE INDEX IF NOT EXISTS idx_track_purchases_track_id ON track_purchases (track_id);
