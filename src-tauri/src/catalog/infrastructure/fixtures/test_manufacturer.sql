-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, status, country_code, website_url, created_at, updated_at)
VALUES ('trn:manufacturer:acme',
        'ACME',
        'ACME Corporation',
        'ACTIVE',
        'IT',
        'https://www.acmetreni.com',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);
