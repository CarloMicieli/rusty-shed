-- Enforce 1:1 relationship: one maintenance card per owned rolling stock.
CREATE UNIQUE INDEX IF NOT EXISTS idx_maintenance_cards_owned_rolling_stock_id
    ON maintenance_cards (owned_rolling_stock_id);
