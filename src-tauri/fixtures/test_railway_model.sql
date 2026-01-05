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
        'Ferrovie dello Stato',
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

INSERT INTO railway_models (id, manufacturer_id, product_code, description, power_method, scale, epoch, category,
                            availability_status)
VALUES ('trn:railway-model:acme:1234',
        'trn:manufacturer:acme',
        '1234',
        'Test Train Set',
        'DC',
        'H0',
        'IV',
        'TRAIN_SETS',
        'AVAILABLE');
