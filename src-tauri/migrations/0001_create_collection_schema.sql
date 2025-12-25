-- Create collections table
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
    total_value_currency TEXT NOT NULL DEFAULT 'EUR'
);

-- Create collection_items table
CREATE TABLE IF NOT EXISTS collection_items (
    id TEXT PRIMARY KEY,
    collection_id TEXT NOT NULL,
    railway_model_id TEXT,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    description TEXT,
    power_method TEXT,
    scale TEXT,
    epoch TEXT,
    FOREIGN KEY(collection_id) REFERENCES collections(id) ON DELETE CASCADE
);

-- Create owned_rolling_stocks table (trimmed to only fields used by the collecting view)
CREATE TABLE IF NOT EXISTS owned_rolling_stocks (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    catalog_rolling_stock_id TEXT,
    notes TEXT,
    railway_name TEXT,
    railway_registered_name TEXT,
    railway_country_code TEXT,
    epoch TEXT,
    FOREIGN KEY(item_id) REFERENCES collection_items(id) ON DELETE CASCADE
);

-- Create purchase_infos table
CREATE TABLE IF NOT EXISTS purchase_infos (
    id TEXT PRIMARY KEY,
    item_id TEXT NOT NULL,
    date TEXT,
    price_amount INTEGER,
    price_currency TEXT,
    seller TEXT,
    FOREIGN KEY(item_id) REFERENCES collection_items(id) ON DELETE CASCADE
);
