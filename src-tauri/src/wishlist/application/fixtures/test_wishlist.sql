-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

-- Add manufacturer and railway_model referenced by the wishlist item
INSERT INTO manufacturers (id, name) VALUES ('trn:manufacturer:acme', 'ACME');

INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category)
VALUES ('trn:railway-model:acme:60100', 'trn:manufacturer:acme', '60100', 'Test model', 'DC', 'H0', 'IV', 'Locomotives');

INSERT INTO wishlists(id, name, notes, is_default, created_at, updated_at)
VALUES ('58fb6f1d-d838-44b5-b65c-21e5388ca4c9', 'Test Wishlist', 'Notes', 0, '2025-12-26T00:00:00Z', '2025-12-26T00:00:00Z');

INSERT INTO wishlist_items(
    id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date, notes
) VALUES (
    '2af7578c-8857-4894-8c93-0be4b579ff25',
    '58fb6f1d-d838-44b5-b65c-21e5388ca4c9',
    'trn:railway-model:acme:60100',
    'NORMAL',
    'WANTED',
    12345,
    'EUR',
    '2025-12-26',
    'Fixture item notes'
);
