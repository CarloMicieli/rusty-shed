-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO sellers (id, name, type, website_url, country_code, created_at, updated_at)
VALUES ('trn:seller:model-train-shop',
        'Model Train Shop',
        'SHOP',
        'https://www.modeltrainshop.com',
        'US',
        CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);