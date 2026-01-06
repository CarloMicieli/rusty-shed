-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:manufacturer:acme',
        'ACME',
        null,
        'IT',
        'ACTIVE',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);

INSERT INTO manufacturers (id, name, registered_company_name, country_code, created_at, updated_at)
VALUES ('trn:manufacturer:rivarossi',
        'Rivarossi',
        null,
        'IT',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);

INSERT INTO railway_companies (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:railway-company:fs',
        'FS',
        'Ferrovie dello Stato Italiane',
        'IT',
        'ACTIVE',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);

-- 1. Insert the FS E.444.005 "Tartaruga" (Electric Locomotive)
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category, availability_status, created_at, updated_at)
VALUES ( 'trn:railway-model:acme:60100',
         'trn:manufacturer:acme',
         '60100',
         'Locomotiva elettrica E.444.005 Tartaruga',
         'DC',
         'H0',
         'IV',
         'LOCOMOTIVES',
         'AVAILABLE',
         '2025-12-26 15:40:14', '2025-12-26 15:40:14');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id,
                            livery, length_millimeters, technical_lights, series_code, road_number,
                            locomotive_type, dcc_interface, control, is_dummy)
VALUES ('trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8',
        'trn:railway-model:acme:60100',
        'LOCOMOTIVE',
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

-- 2. Insert an FS UIC-Z1 Passenger Coach
INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status, created_at, updated_at)
VALUES ('trn:railway-model:rivarossi:hr4315',
        'trn:manufacturer:rivarossi',
        'HR4315',
        'Carrozza passeggeri UIC-Z1 1a classe',
        'DC',
        'H0',
        'V',
        'PASSENGER_CARS',
        'AVAILABLE',
        '2025-12-26 15:50:14', '2025-12-26 15:50:14');

INSERT INTO rolling_stocks (id, railway_model_id, category, railway_company_id, series_code,
                            livery, length_millimeters, passenger_car_type, service_level, is_dummy)
VALUES ('trn:rolling-stock:a709c07f-3458-40f8-8659-0d70250c0b70',
        'trn:railway-model:rivarossi:hr4315',
        'PASSENGER_CAR',
        'trn:railway-company:fs',
        'UIC-Z1',
        'XMPR',
        303.0,
        'OPEN_COACH',
        '1',
        0);


INSERT INTO collections(id, name, locomotives_count, passenger_cars_count, freight_cars_count, train_sets_count, railcars_count, electric_multiple_units_count, total_value_amount, total_value_currency)
VALUES ('trn:collection:1',
        'Test Collection',
        1,
        1,
        0,
        0,
        0,
        0,
        20000,
        'EUR');

INSERT INTO collection_items(id, collection_id, railway_model_id, added_date, purchase_condition, box_condition, model_condition, notes)
VALUES('trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730',
       'trn:collection:1',
       'trn:railway-model:acme:60100',
       '2025-12-26',
       'NEW',
       'ORIGINAL_MINT',
       'MINT',
       'My notes go here');

INSERT INTO owned_rolling_stocks(id, collection_item_id, rolling_stock_id, notes)
VALUES('trn:owned-rolling-stock:77122924-783e-4f3c-a6b5-f4caec9e695d',
       'trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730',
       'trn:rolling-stock:70300b1c-b1df-475f-a7be-291e435b1cf8',
       'My rolling stock notes go here');

INSERT INTO purchase_infos(id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency)
VALUES('trn:purchase:59adc26d-0274-4d6b-8c14-61e598d3fe0e',
       'trn:collection-item:d20a1a95-1ae4-4970-9e87-b4c84676e730',
       'PURCHASED',
       '2025-12-26',
       15000,
       'EUR');

INSERT INTO collection_items(id, collection_id, railway_model_id, added_date, purchase_condition, box_condition, model_condition, notes)
VALUES('trn:collection-item:a6d749a5-f3e9-44e1-9963-35ad90d4b83a',
       'trn:collection:1',
       'trn:railway-model:rivarossi:hr4315',
       '2025-12-26',
       'NEW',
       'ORIGINAL_MINT',
       'MINT',
       'My notes go here');

INSERT INTO owned_rolling_stocks(id, collection_item_id, rolling_stock_id, notes)
VALUES('trn:owned-rolling-stock:b693c971-909d-4ba1-bf06-de1e5ea309b4',
       'trn:collection-item:a6d749a5-f3e9-44e1-9963-35ad90d4b83a',
       'trn:rolling-stock:a709c07f-3458-40f8-8659-0d70250c0b70',
       'My rolling stock notes go here');

INSERT INTO purchase_infos(id, collection_item_id, purchase_type, purchase_date, purchased_price_amount, purchased_price_currency)
VALUES('trn:purchase:0af77d68-6f33-4991-b1ea-6cff9ff86811',
       'trn:collection-item:a6d749a5-f3e9-44e1-9963-35ad90d4b83a',
       'PURCHASED',
       '2025-12-27',
       5000,
       'EUR');
