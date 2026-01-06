-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO manufacturers (id, name, registered_company_name, status, country_code, website_url, created_at, updated_at)
VALUES ('trn:manufacturer:acme',
        'ACME',
        'Anonima Costruzioni Modellistiche Esatte S.r.l.',
        'ACTIVE',
        'IT',
        'https://www.acmetreni.com',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);

INSERT INTO manufacturers (id, name, registered_company_name, status, country_code, website_url, created_at, updated_at)
VALUES ('trn:manufacturer:roco',
        'Roco',
        'Modelleisenbahn München GmbH',
        'ACTIVE',
        'AT',
        'https://www.roco.cc',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);
