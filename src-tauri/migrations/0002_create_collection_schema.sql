CREATE TABLE IF NOT EXISTS collections (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    locomotives_count INTEGER NOT NULL DEFAULT 0,
    passenger_cars_count INTEGER NOT NULL DEFAULT 0,
    freight_cars_count INTEGER NOT NULL DEFAULT 0,
    train_sets_count INTEGER NOT NULL DEFAULT 0,
    railcars_count INTEGER NOT NULL DEFAULT 0,
    electric_multiple_units_count INTEGER NOT NULL DEFAULT 0,
    total_value_amount INTEGER NOT NULL DEFAULT 0,
    total_value_currency TEXT NOT NULL DEFAULT 'EUR',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS collection_items (
    id TEXT PRIMARY KEY,
    collection_id TEXT NOT NULL,
    railway_model_id TEXT,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    description TEXT,
    conditions TEXT,
    power_method TEXT,
    scale TEXT,
    epoch TEXT,
    FOREIGN KEY(collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS owned_rolling_stocks (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    catalog_rolling_stock_id TEXT,
    notes TEXT,
    railway_id TEXT NOT NULL,
    epoch TEXT,
    FOREIGN KEY(item_id) REFERENCES collection_items(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS purchase_infos (
     purchase_id TEXT PRIMARY KEY,
     collection_item_id TEXT NOT NULL,
     purchase_type TEXT,
     purchase_date TEXT NOT NULL,
     seller_id TEXT,
     buyer_id TEXT,
     sale_date TEXT,
     purchased_price_amount INTEGER,
     purchased_price_currency TEXT,
     sale_price_amount INTEGER,
     sale_price_currency TEXT,
     deposit_amount INTEGER,
     deposit_currency TEXT,
     preorder_total_amount INTEGER,
     preorder_total_currency TEXT,
     expected_date TEXT,
     FOREIGN KEY(collection_item_id) REFERENCES collection_items(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_purchase_infos_collection_item ON purchase_infos(collection_item_id);
CREATE INDEX IF NOT EXISTS idx_purchase_infos_type ON purchase_infos(purchase_type);
