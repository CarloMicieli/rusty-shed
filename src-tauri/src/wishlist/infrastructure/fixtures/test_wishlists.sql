-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:manufacturer:acme',
        'ACME',
        null,
        'IT',
        'ACTIVE',
        '2025-12-26T15:50:13.995315135+00:00',
        '2025-12-26 15:50:14');

INSERT INTO railway_companies (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:railway-company:fs',
        'FS',
        'Ferrovie dello Stato Italiane (Trenitalia)',
        'IT',
        'ACTIVE',
        '2025-12-26T15:50:13.995410967+00:00',
        '2025-12-26 15:50:14');

-- 1. Insert the FS E.444.005 "Tartaruga" (Electric Locomotive)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:acme:60100',
        'trn:manufacturer:acme',
        '60100',
        'Locomotiva elettrica E.444.005 Tartaruga',
        'DC',
        'H0',
        'IV',
        'LOCOMOTIVES',
        'AVAILABLE');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, technical_lights, series_code, road_number,
                            locomotive_type, dcc_interface, control, is_dummy)
VALUES ('trn:rolling-stock:d98fbfa1-ffff-4b9e-807b-30a8f201516d',
        'trn:railway-model:acme:60100',
        'ELECTRIC_LOCOMOTIVE',
        'trn:railway-company:fs',
        'Grigio nebbia / Blu orientale',
        195.0,
        'White/Red directional',
        'E.444',
        'E.444.005',
        'ELECTRIC_LOCOMOTIVE',
        'MTC_21',
        'DCC_READY',
        0);


INSERT INTO wishlists(id, name, notes, is_default, created_at, updated_at)
VALUES ('trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9',
        'Test Wishlist 1',
        'Notes',
        0, '2025-12-12T00:00:00Z',
        '2025-12-27T00:00:00Z');

INSERT INTO wishlist_items(
    id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date, notes
) VALUES (
    'trn:wishlist-item:2af7578c-8857-4894-8c93-0be4b579ff25',
    'trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9',
    'trn:railway-model:acme:60100',
    'NORMAL',
    'WANTED',
    17500,
    'EUR',
    '2025-12-13',
    'Fixture item notes'
);

INSERT INTO wishlist_items(
    id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date, notes
) VALUES (
     'trn:wishlist-item:d3aae962-b6da-45a2-addd-9f58dee5951b',
     'trn:wishlist:58fb6f1d-d838-44b5-b65c-21e5388ca4c9',
     'trn:railway-model:acme:60100',
     'NORMAL',
     'WANTED',
     15000,
     'EUR',
     '2025-12-26',
     'Fixture item notes'
 );

INSERT INTO wishlists(id, name, notes, is_default, created_at, updated_at)
VALUES ('trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5',
        'Test Wishlist 2',
        'Notes',
        1,
        '2025-12-12T00:00:00Z',
        '2025-12-26T00:00:00Z');

INSERT INTO wishlist_items(
    id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date, notes
) VALUES (
     'trn:wishlist-item:2835c4bd-25e5-4061-9b8f-ef5cc26b9137',
     'trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5',
     'trn:railway-model:acme:60100',
     'NORMAL',
     'WANTED',
     17500,
     'USD',
     '2025-12-13',
     'Fixture item notes'
 );

INSERT INTO wishlist_items(
    id, wishlist_id, railway_model_id, priority, status, desired_price_amount, desired_price_currency, added_date, notes
) VALUES (
     'trn:wishlist-item:69808a0c-7715-4438-8dcb-5d8df28c89e4',
     'trn:wishlist:c9950910-96e1-47ae-8097-cd0ebbaa83f5',
     'trn:railway-model:acme:60100',
     'NORMAL',
     'WANTED',
     15000,
     'EUR',
     '2025-12-26',
     'Fixture item notes'
 );