-- Migration: Add railway_model_translations table and FTS5 search index
-- Moves description/details from railway_models to a per-language translations table.

-- 1. Create the translations table
CREATE TABLE railway_model_translations (
    railway_model_id TEXT NOT NULL,
    language_code    TEXT NOT NULL,
    description      TEXT,
    details          TEXT,
    created_at       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (railway_model_id, language_code),
    FOREIGN KEY (railway_model_id) REFERENCES railway_models (id) ON DELETE CASCADE
);

-- 2. Create the FTS5 virtual table for full-text search
CREATE VIRTUAL TABLE railway_model_search_idx USING fts5 (
    railway_model_id UNINDEXED,
    language_code    UNINDEXED,
    description,
    details,
    tokenize = 'unicode61'
);

-- 3. Triggers to keep FTS5 in sync with railway_model_translations

-- INSERT sync
CREATE TRIGGER tr_rmt_fts_insert
AFTER INSERT ON railway_model_translations
BEGIN
    INSERT INTO railway_model_search_idx (railway_model_id, language_code, description, details)
    VALUES (new.railway_model_id, new.language_code, new.description, new.details);
END;

-- UPDATE sync (FTS5 has no native UPDATE — delete old row, insert new row)
CREATE TRIGGER tr_rmt_fts_update
AFTER UPDATE ON railway_model_translations
BEGIN
    DELETE FROM railway_model_search_idx
    WHERE railway_model_id = old.railway_model_id
      AND language_code    = old.language_code;
    INSERT INTO railway_model_search_idx (railway_model_id, language_code, description, details)
    VALUES (new.railway_model_id, new.language_code, new.description, new.details);
END;

-- DELETE sync
CREATE TRIGGER tr_rmt_fts_delete
AFTER DELETE ON railway_model_translations
BEGIN
    DELETE FROM railway_model_search_idx
    WHERE railway_model_id = old.railway_model_id
      AND language_code    = old.language_code;
END;

-- 4. Data migration: copy existing EN text from railway_models to translations
INSERT INTO railway_model_translations (railway_model_id, language_code, description, details)
SELECT id, 'en', description, details
FROM railway_models
WHERE description IS NOT NULL;

-- 5. Populate FTS5 index for migrated rows
--    (triggers only fire for future DML; bulk-insert needed for existing data)
INSERT INTO railway_model_search_idx (railway_model_id, language_code, description, details)
SELECT railway_model_id, language_code, description, details
FROM railway_model_translations;

-- 6. Drop the old columns from railway_models
ALTER TABLE railway_models DROP COLUMN description;
ALTER TABLE railway_models DROP COLUMN details;
