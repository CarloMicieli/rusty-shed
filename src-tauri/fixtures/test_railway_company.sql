-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO railway_companies (
        id, name, registered_company_name, country_code, status, operating_since, operating_until, created_at, updated_at)
VALUES ('trn:railway-company:fs', 
        'FS', 
        'Ferrovie dello Stato', 
        'IT', 
        'ACTIVE',
        '1905-07-01',
        NULL,
        CURRENT_TIMESTAMP, 
        CURRENT_TIMESTAMP);

INSERT INTO railway_companies (
    id, name, registered_company_name, country_code, status, operating_since, operating_until, created_at, updated_at)
VALUES ('trn:railway-company:drg',
        'DRG',
        'Deutsche Reichsbahn-Gesellschaft',
        'DE',
        'MERGED',
        '1920-04-01',
        '1945-05-23',
        CURRENT_TIMESTAMP,
        CURRENT_TIMESTAMP);
