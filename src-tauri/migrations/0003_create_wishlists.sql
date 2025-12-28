-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS wishlists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    notes TEXT,
    is_default INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS wishlist_items (
    id TEXT PRIMARY KEY,
    wishlist_id TEXT NOT NULL,
    railway_model_id TEXT,
    priority TEXT NOT NULL DEFAULT 'NORMAL' CHECK(priority IN ('LOW','NORMAL','HIGH')),
    status TEXT NOT NULL DEFAULT 'WANTED' CHECK(status IN ('WANTED','ON_ORDER','PURCHASED','IGNORED')),
    desired_price_amount INTEGER,
    desired_price_currency TEXT,
    -- date-only fields (YYYY-MM-DD). Enforce format with GLOB checks.
    added_date TEXT NOT NULL CHECK(added_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
    removed_date TEXT CHECK(removed_date IS NULL OR removed_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
    notes TEXT,
    purchased_at TEXT,
    purchased_price_amount INTEGER,
    purchased_price_currency TEXT,
    FOREIGN KEY (wishlist_id) REFERENCES wishlists (id) ON DELETE CASCADE,
    FOREIGN KEY (railway_model_id) REFERENCES railway_models (id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS idx_wishlist_items_wishlist_id ON wishlist_items (wishlist_id);
CREATE INDEX IF NOT EXISTS idx_wishlist_items_railway_model_id ON wishlist_items (railway_model_id);

