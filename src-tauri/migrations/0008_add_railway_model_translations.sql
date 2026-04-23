-- noinspection SqlResolveForFile
-- noinspection SqlNoDataSourceInspectionForFile
-- noinspection SqlResolveInspectionForFile

CREATE TABLE railway_model_translations (
    railway_model_id        TEXT NOT NULL,
    language_code           TEXT NOT NULL,
    description             TEXT,
    details                 TEXT,
    created_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at              TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (railway_model_id, language_code),
    FOREIGN KEY (railway_model_id) REFERENCES railway_models (id) ON DELETE CASCADE
);

CREATE VIRTUAL TABLE railway_model_search_idx USING fts5 (
    railway_model_id    UNINDEXED,   -- PK reference, not tokenised
    language_code       UNINDEXED,   -- language tag, not tokenised
    description,                     -- from railway_model_translations.description
    details,                         -- from railway_model_translations.details
    manufacturer_name,               -- from manufacturers.name
    rolling_stocks_text,             -- concat of road_number, series_code, livery, depot
    tokenize = 'unicode61'
);