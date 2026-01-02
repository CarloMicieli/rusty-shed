-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:manufacturer:acme', 'ACME', null, 'IT',  'ACTIVE', '2025-12-26T15:50:13.995315135+00:00', '2025-12-26 15:50:14');

INSERT INTO railway_companies (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:railway-company:fs', 'FS', 'Ferrovie dello Stato Italiane (Trenitalia)', 'IT', 'ACTIVE',
        '2025-12-26T15:50:13.995410967+00:00', '2025-12-26 15:50:14');

-- 1. Insert the FS E.444.005 "Tartaruga" (Electric Locomotive)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:acme:60100', 'trn:manufacturer:acme', '60100', 'Locomotiva elettrica E.444.005 Tartaruga',
        'DC', 'H0', 'IV', 'Locomotives', 'Available');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, technical_lights, series_code, road_number,
                            locomotive_type, dcc_interface, control, is_dummy)
VALUES ('rs-001', 'trn:railway-model:acme:60100', 'Locomotive', 'trn:railway-company:fs',
        'Grigio nebbia / Blu orientale', 195.0, 'White/Red directional', 'E.444', 'E.444.005',
        'Electric', 'MTC21', 'Analog', 0);


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
