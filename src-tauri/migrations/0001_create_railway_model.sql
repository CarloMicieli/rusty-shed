-- Create railway_models table
CREATE TABLE IF NOT EXISTS railway_models (
    id TEXT PRIMARY KEY,
    manufacturer TEXT NOT NULL,
    product_code TEXT NOT NULL,
    description TEXT
);

