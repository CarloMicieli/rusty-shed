-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE IF NOT EXISTS coupler_types
(
  id                                    TEXT PRIMARY KEY,
  manufacturer                          TEXT NOT NULL,
  name                                  TEXT NOT NULL,
  compatible_socket                     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS decoders
(
  id                                    TEXT PRIMARY KEY,
  manufacturer_id                       TEXT NOT NULL,
  product_code                          TEXT,
  decoder_type                          TEXT NOT NULL,
  protocol                              TEXT NOT NULL,
  decoder_interface                     TEXT NOT NULL,
  version                               INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (manufacturer_id) REFERENCES manufacturers(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS sellers
(
  id                                    TEXT PRIMARY KEY NOT NULL,
  name                                  TEXT NOT NULL,
  type                                  TEXT NOT NULL,
  email                                 TEXT,
  phone                                 TEXT,
  website_url                           TEXT,
  street_address                        TEXT,
  extended_address                      TEXT,
  city                                  TEXT,
  state_region                          TEXT,
  postal_code                           TEXT,
  country_code                          TEXT,
  created_at                            TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at                            TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  version                               INTEGER NOT NULL DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_sellers_name ON sellers(name);

CREATE TABLE IF NOT EXISTS collections
(
  id                                    TEXT PRIMARY KEY,
  name                                  TEXT NOT NULL,
  electric_multiple_units_count         INTEGER NOT NULL DEFAULT 0,
  freight_cars_count                    INTEGER NOT NULL DEFAULT 0,
  locomotives_count                     INTEGER NOT NULL DEFAULT 0,
  passenger_cars_count                  INTEGER NOT NULL DEFAULT 0,
  railcars_count                        INTEGER NOT NULL DEFAULT 0,
  starter_sets_count                    INTEGER NOT NULL DEFAULT 0,
  train_sets_count                      INTEGER NOT NULL DEFAULT 0,
  total_value_amount                    INTEGER NOT NULL DEFAULT 0,
  total_value_currency                  TEXT    NOT NULL DEFAULT 'EUR',
  created_at                            TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at                            TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
  version                               INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS collection_items
(
  id                                    TEXT PRIMARY KEY,
  collection_id                         TEXT NOT NULL,
  railway_model_id                      TEXT NOT NULL,
  added_date                            TEXT NOT NULL CHECK(added_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
  removed_date                          TEXT CHECK(removed_date IS NULL OR removed_date GLOB '[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]'),
  purchase_condition                    TEXT,
  model_condition                       TEXT,
  box_condition                         TEXT,
  notes                                 TEXT,
  FOREIGN KEY (collection_id)    REFERENCES collections(id)    ON DELETE CASCADE,
  FOREIGN KEY (railway_model_id) REFERENCES railway_models(id) ON DELETE RESTRICT
);

CREATE TABLE IF NOT EXISTS owned_rolling_stocks
(
    id                                    TEXT PRIMARY KEY,
    collection_item_id                    TEXT NOT NULL,
    rolling_stock_id                      TEXT,
    notes                                 TEXT,
    dcc_address                           INTEGER,
    installed_decoder_id                  TEXT,
    current_coupler_id                    TEXT,
    FOREIGN KEY (collection_item_id)   REFERENCES collection_items(id)  ON DELETE CASCADE,
    FOREIGN KEY (rolling_stock_id)     REFERENCES rolling_stocks(id)    ON DELETE SET NULL,
    FOREIGN KEY (installed_decoder_id) REFERENCES decoders(id)          ON DELETE SET NULL,
    FOREIGN KEY (current_coupler_id)   REFERENCES coupler_types(id)     ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS purchase_infos
(
    id                                    TEXT PRIMARY KEY,
    collection_item_id                    TEXT NOT NULL,
    purchase_type                         TEXT,
    purchase_date                         TEXT NOT NULL,
    seller_id                             TEXT,
    buyer_id                              TEXT,
    sale_date                             TEXT,
    purchased_price_amount                INTEGER CHECK(purchased_price_amount > 0), -- Minor currency units (must be positive)
    purchased_price_currency              TEXT,
    sale_price_amount                     INTEGER CHECK(sale_price_amount >= 0), -- Minor currency units (must be positive)
    sale_price_currency                   TEXT,
    deposit_amount                        INTEGER CHECK(deposit_amount >= 0), -- Minor currency units (must be positive)
    deposit_currency                      TEXT,
    preorder_total_amount                 INTEGER CHECK(preorder_total_amount >= 0), -- Minor currency units (must be positive)
    preorder_total_currency               TEXT,
    expected_date                         TEXT,
    FOREIGN KEY (collection_item_id) REFERENCES collection_items(id) ON DELETE CASCADE,
    FOREIGN KEY (seller_id)          REFERENCES sellers(id)          ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_purchase_infos_collection_item ON purchase_infos(collection_item_id);
CREATE INDEX IF NOT EXISTS idx_purchase_infos_type            ON purchase_infos(purchase_type);
