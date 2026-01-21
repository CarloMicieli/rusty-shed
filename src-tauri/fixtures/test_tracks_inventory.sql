-- Fixture for tracks inventory tests

INSERT INTO manufacturers (id, name, created_at, updated_at)
VALUES ('trn:manufacturer:acme', 'ACME', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO sellers (id, name, type, website_url, country_code, created_at, updated_at)
VALUES ('trn:seller:model-train-shop', 'Model Train Shop', 'SHOP', 'https://www.modeltrainshop.com', 'US', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO track_products (id, track_id, manufacturer_id, product_code, with_roadbed, created_at, updated_at)
VALUES ('trn:track-product:acme:60100', 'trn:track:acme:60100', 'trn:manufacturer:acme', '60100', 0, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO track_inventories (id, name, description, created_at, updated_at)
VALUES ('trn:track-inventory:00000000-0000-0000-0000-000000000001', 'Test Inventory', 'Created for add_purchase tests', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO track_inventory_items (inventory_id, track_id, quantity)
VALUES ('trn:track-inventory:00000000-0000-0000-0000-000000000001', 'trn:track:acme:60100', 1);
