-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:manufacturer:acme', 'ACME', null, 'IT',  'ACTIVE', '2025-12-26T15:50:13.995315135+00:00', '2025-12-26 15:50:14');

INSERT INTO manufacturers (id, name, registered_company_name, country_code, created_at, updated_at)
VALUES ('trn:manufacturer:rivarossi', 'Rivarossi', null, 'IT', '2025-12-26T15:50:13.995315135+00:00',
        '2025-12-26 15:50:14');

INSERT INTO railway_companies (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:railway-company:fs', 'FS', 'Ferrovie dello Stato Italiane (Trenitalia)', 'IT', 'ACTIVE',
        '2025-12-26T15:50:13.995410967+00:00', '2025-12-26 15:50:14');

-- 1. Insert the FS E.444.005 "Tartaruga" (Electric Locomotive)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:acme:60100', 'trn:manufacturer:acme', '60100', 'Locomotiva elettrica E.444.005 Tartaruga',
        'DC', 'H0', 'IV', 'Locomotives', 'Available');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, technical_lights, class_name, road_number,
                            locomotive_type, dcc_interface, control, is_dummy)
VALUES ('rs-001', 'trn:railway-model:acme:60100', 'Locomotive', 'trn:railway-company:fs',
        'Grigio nebbia / Blu orientale', 195.0, 'White/Red directional', 'E.444', 'E.444.005',
        'Electric', 'MTC21', 'Analog', 0);

-- 2. Insert an FS UIC-Z1 Passenger Coach (Biglietto proiettile style)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:rivarossi:hr4315', 'trn:manufacturer:rivarossi', 'HR4315',
        'Carrozza passeggeri UIC-Z1 1a classe', 'DC', 'H0', 'V', 'Passenger_Cars', 'Available');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, passenger_car_type, service_level, is_dummy)
VALUES ('rs-002', 'trn:railway-model:rivarossi:hr4315', 'Passenger_Car', 'trn:railway-company:fs',
        'XMPR', 303.0, 'Express Train', 'First', 0);

INSERT INTO collections(id, name)
VALUES ('052cb8be-cc5c-460d-b72c-6cec595b91d7', 'Test Collection');

INSERT INTO collection_items(id, collection_id, railway_model_id, conditions, notes)
VALUES('d20a1a95-1ae4-4970-9e87-b4c84676e730', '052cb8be-cc5c-460d-b72c-6cec595b91d7', 'trn:railway-model:acme:60100', 'new', 'My notes go here');

INSERT INTO owned_rolling_stocks(id, collection_item_id, rolling_stock_id, notes)
VALUES('d3606635-4c4e-462b-ae9f-02c7ce47bc770', 'd20a1a95-1ae4-4970-9e87-b4c84676e730', 'rs-001', 'My rolling stock notes go here');

INSERT INTO purchase_infos(id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency)
VALUES('59adc26d-0274-4d6b-8c14-61e598d3fe0e', 'd20a1a95-1ae4-4970-9e87-b4c84676e730', 'purchased', '2025-12-26', 17500, 'EUR');

INSERT INTO maintenance_cards (id, owned_rolling_stock_id, last_maintenance_date, next_maintenance_date, created_at, updated_at)
VALUES
  ('11111111-1111-1111-1111-111111111111', 'd3606635-4c4e-462b-ae9f-02c7ce47bc770', '2025-01-01', '2025-07-01', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
  ('22222222-2222-2222-2222-222222222222', 'd3606635-4c4e-462b-ae9f-02c7ce47bc770', NULL, '2025-12-31', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

INSERT INTO maintenance_events (id, maintenance_card_id, date_performed, maintenance_type, notes)
VALUES
  ('33333333-3333-3333-3333-333333333333', 'd3606635-4c4e-462b-ae9f-02c7ce47bc770', '2025-01-01', 'inspection', 'Initial inspection'),
  ('44444444-4444-4444-4444-444444444444', 'd3606635-4c4e-462b-ae9f-02c7ce47bc770', '2025-03-01', 'oil_change', 'Changed oil');
