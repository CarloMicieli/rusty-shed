-- Migration 0014: Extend FTS5 search index with rolling stock fields and manufacturer name
-- Drops old triggers from migration 0013 (replaced by domain-event indexing in Rust)
-- Drops and recreates railway_model_search_idx with extended schema.

-- Drop old triggers from migration 0013 (replaced by domain-event indexing)
DROP TRIGGER IF EXISTS tr_rmt_fts_insert;
DROP TRIGGER IF EXISTS tr_rmt_fts_update;
DROP TRIGGER IF EXISTS tr_rmt_fts_delete;

-- Drop old FTS5 table (FTS5 does not support ALTER TABLE)
DROP TABLE IF EXISTS railway_model_search_idx;

-- Recreate with extended columns
CREATE VIRTUAL TABLE railway_model_search_idx USING fts5 (
    railway_model_id    UNINDEXED,   -- PK reference, not tokenised
    language_code       UNINDEXED,   -- language tag, not tokenised
    description,                     -- from railway_model_translations.description
    details,                         -- from railway_model_translations.details
    manufacturer_name,               -- from manufacturers.name
    rolling_stocks_text,             -- concat of road_number, series_code, livery, depot
    tokenize = 'unicode61'
);

-- Initial population from existing data
-- After this, all updates are driven by domain events in Rust (no triggers)
INSERT INTO railway_model_search_idx (
    railway_model_id,
    language_code,
    description,
    details,
    manufacturer_name,
    rolling_stocks_text
)
SELECT
    rmt.railway_model_id,
    rmt.language_code,
    COALESCE(rmt.description, ''),
    COALESCE(rmt.details, ''),
    COALESCE(m.name, ''),
    COALESCE(
        (SELECT group_concat(
            COALESCE(rs.road_number, '') || ' ' ||
            COALESCE(rs.series_code, '') || ' ' ||
            COALESCE(rs.livery, '') || ' ' ||
            COALESCE(rs.depot, ''),
            ' '
         )
         FROM rolling_stocks rs
         WHERE rs.railway_model_id = rmt.railway_model_id),
        ''
    )
FROM railway_model_translations rmt
JOIN railway_models rm ON rm.id = rmt.railway_model_id
JOIN manufacturers m   ON m.id  = rm.manufacturer_id;
