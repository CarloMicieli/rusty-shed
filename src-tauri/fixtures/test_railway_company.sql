-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

INSERT INTO railway_companies (id, name, registered_company_name, country_code, status, created_at, updated_at)
VALUES ('trn:railway-company:fs', 
        'FS', 
        'Ferrovie dello Stato', 
        'IT', 
        'ACTIVE', 
        CURRENT_TIMESTAMP, 
        CURRENT_TIMESTAMP);
