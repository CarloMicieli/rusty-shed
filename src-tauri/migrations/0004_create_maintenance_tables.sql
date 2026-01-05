-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS maintenance_cards
(
    id                                  TEXT PRIMARY KEY,
    owned_rolling_stock_id              TEXT NOT NULL,
    last_maintenance_date               TEXT,
    next_maintenance_date               TEXT,
    created_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at                          TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (owned_rolling_stock_id) REFERENCES owned_rolling_stocks (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS maintenance_events
(
    id                                  TEXT PRIMARY KEY,
    maintenance_card_id                 TEXT NOT NULL,
    date_performed                      TEXT NOT NULL,
    maintenance_type                    TEXT,
    notes                               TEXT,
    FOREIGN KEY (maintenance_card_id) REFERENCES maintenance_cards (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_maintenance_events_card
    ON maintenance_events (maintenance_card_id, date_performed DESC);
