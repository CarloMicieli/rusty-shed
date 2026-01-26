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
                            livery, length_millimeters, technical_lights, series_code, road_number,
                            locomotive_type, dcc_interface, control, is_dummy)
VALUES ('trn:rolling-stock:rs-001', 'trn:railway-model:acme:60100', 'Locomotive', 'trn:railway-company:fs',
        'Grigio nebbia / Blu orientale', 195.0, 'White/Red directional', 'E.444', 'E.444.005',
        'Electric', 'MTC21', 'Analog', 0);

-- 2. Insert an FS UIC-Z1 Passenger Coach (Biglietto proiettile style)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:rivarossi:hr4315', 'trn:manufacturer:rivarossi', 'HR4315',
        'Carrozza passeggeri UIC-Z1 1a classe', 'DC', 'H0', 'V', 'Passenger_Cars', 'Available');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, passenger_car_type, service_level, series_code, is_dummy)
VALUES ('trn:rolling-stock:rs-002', 'trn:railway-model:rivarossi:hr4315', 'Passenger_Car', 'trn:railway-company:fs',
        'XMPR', 303.0, 'Express Train', 'First', 'UIC-Z1', 0);

INSERT INTO collections(id, name)
VALUES ('trn:collection:1', 'Test Collection');

INSERT INTO collection_items(id, collection_id, railway_model_id, added_date, purchase_condition, notes)
VALUES (
        'trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730', 
        'trn:collection:1', 
        'trn:railway-model:acme:60100', 
        '2025-12-26', 
        'new', 
        'My notes go here'
);

INSERT INTO owned_rolling_stocks(id, collection_item_id, rolling_stock_id, notes)
VALUES (
        'trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-02c7ce47bc70',
        'trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730',
        'trn:rolling-stock:rs-001', 
        'My rolling stock notes go here'
);

INSERT INTO maintenance_cards (id, owned_rolling_stock_id, last_maintenance_date, next_maintenance_date, created_at, updated_at)
VALUES (
        'trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2',
        'trn:owned-rolling-stock:d3606635-4c4e-462b-ae9f-02c7ce47bc70',
        '2025-01-01', 
        '2025-07-01', 
        CURRENT_TIMESTAMP, 
        CURRENT_TIMESTAMP);

INSERT INTO maintenance_events (id, maintenance_card_id, date_performed, maintenance_type, notes)
VALUES
  ('trn:maintenance-event:fc82c5f0-6a42-4302-bb41-22a6e67868a0', 'trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2', '2025-01-01', 'inspection', 'Initial inspection'),
  ('trn:maintenance-event:ad4f1aa7-1142-43eb-afb4-cb56871ac29d', 'trn:maintenance-card:3284cc76-1472-4b12-a7d4-62043416adc2', '2025-03-01', 'oil_change', 'Changed oil');