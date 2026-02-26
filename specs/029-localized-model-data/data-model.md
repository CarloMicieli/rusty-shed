# Data Model: Localized Railway Model Data

**Feature**: `029-localized-model-data` | **Date**: 2026-02-25

## Database Schema Changes

### Modified Table: `railway_models`

Columns `description TEXT NOT NULL` and `details TEXT` are **dropped** in migration `0013`. All other columns remain unchanged.

```sql
-- BEFORE (columns to be removed):
description TEXT NOT NULL,
details TEXT,

-- AFTER: these columns no longer exist on railway_models.
-- All text content lives in railway_model_translations.
```

---

### New Table: `railway_model_translations`

```sql
CREATE TABLE railway_model_translations (
    railway_model_id TEXT NOT NULL,
    language_code    TEXT NOT NULL,    -- 'en' or 'it'
    description      TEXT,             -- NULL = not provided for this language
    details          TEXT,             -- NULL = not provided for this language
    created_at       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at       TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (railway_model_id, language_code),
    FOREIGN KEY (railway_model_id) REFERENCES railway_models (id) ON DELETE CASCADE
);
```

**Constraints**:

- `(railway_model_id, language_code)` composite primary key — one row per model per language.
- `ON DELETE CASCADE` — deleting a railway model removes all its translations automatically.
- `language_code` is a free-text column validated at the application boundary (not a DB enum). Accepted values: `'en'`, `'it'`.
- A row with both `description IS NULL` and `details IS NULL` is treated as "translation absent" at the application layer and should be deleted rather than stored.

---

### New Virtual Table: `railway_model_search_idx` (FTS5)

```sql
CREATE VIRTUAL TABLE railway_model_search_idx USING fts5 (
    railway_model_id UNINDEXED,
    language_code    UNINDEXED,
    description,
    details,
    tokenize = 'unicode61'
);
```

**Notes**:

- `UNINDEXED` columns are stored but not tokenized; they allow the FTS5 result to carry the model ID and language for JOIN-back to `railway_models`.
- `tokenize = 'unicode61'` handles accented characters in Italian (e.g., "locomotiva", "ferrovia").
- The FTS5 table is kept in sync by three triggers (see below); no application-layer writes are needed.

---

### Triggers (FTS5 Sync)

```sql
-- INSERT sync
CREATE TRIGGER tr_rmt_fts_insert
AFTER INSERT ON railway_model_translations
BEGIN
    INSERT INTO railway_model_search_idx (railway_model_id, language_code, description, details)
    VALUES (new.railway_model_id, new.language_code, new.description, new.details);
END;

-- UPDATE sync (delete old row, insert new row — FTS5 has no native UPDATE)
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
```

---

### Migration `0013_add_railway_model_translations.sql` — Execution Order

1. `CREATE TABLE railway_model_translations` (above)
2. `CREATE VIRTUAL TABLE railway_model_search_idx` (above)
3. `CREATE TRIGGER tr_rmt_fts_insert` / `tr_rmt_fts_update` / `tr_rmt_fts_delete`
4. **Data migration** — copy existing EN text:
   ```sql
   INSERT INTO railway_model_translations (railway_model_id, language_code, description, details)
   SELECT id, 'en', description, details
   FROM railway_models
   WHERE description IS NOT NULL;
   ```
5. **Populate FTS5 index** for migrated rows (triggers only fire for future writes):
   ```sql
   INSERT INTO railway_model_search_idx (railway_model_id, language_code, description, details)
   SELECT railway_model_id, language_code, description, details
   FROM railway_model_translations;
   ```
6. **Drop old columns**:
   ```sql
   ALTER TABLE railway_models DROP COLUMN description;
   ALTER TABLE railway_models DROP COLUMN details;
   ```

---

## Rust Domain Types

### New: `LocalizedField` (value object)

Lives in `src-tauri/src/catalog/domain/railway_model/localized_field.rs`.

```rust
/// A text value resolved to a specific language code.
///
/// The `lang` field records which language was actually resolved,
/// enabling the UI to show a fallback indicator when `lang` differs
/// from the user's requested language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub struct LocalizedField {
    pub lang: String,    // "en" or "it" — the language that was resolved
    pub value: String,   // the text content
}
```

---

### Modified: `RailwayModel` aggregate fields

```rust
// Before:
pub description: String,
pub details: Option<String>,

// After:
pub description: LocalizedField,
pub details: Option<LocalizedField>,
```

`description.lang` indicates the resolved language (may differ from requested when fallback applies).

---

### New: `RailwayModelTranslation` (read struct)

Lives in `src-tauri/src/catalog/domain/railway_model/railway_model_translation.rs`. Used by `get_railway_model_translations` to return all stored translations for the edit form.

```rust
/// All stored translations for a single railway model.
/// Used by the edit form to pre-populate language-specific input fields.
#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct RailwayModelTranslations {
    pub railway_model_id: RailwayModelId,
    pub en: Option<RailwayModelTranslationEntry>,
    pub it: Option<RailwayModelTranslationEntry>,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct RailwayModelTranslationEntry {
    pub description: Option<String>,
    pub details: Option<String>,
}
```

---

### Modified: `RailwayModelView`

```rust
// Added fields:
pub description_lang: String,        // "en" or "it" — actual resolved lang
pub details_lang: Option<String>,    // "en" or "it" — actual resolved lang (None if no details)
```

The `description` and `details` fields remain `String` / `Option<String>` for backwards compatibility; they now hold the resolved text.

---

### New: `RailwayModelEvent::TranslationUpserted`

```rust
RailwayModelEvent::TranslationUpserted {
    event_id: Uuid,
    railway_model_id: RailwayModelId,
    timestamp: NaiveDateTime,
    lang: String,                // "en" or "it"
    description: Option<String>, // None = leave as-is; Some("") = clear
    details: Option<String>,     // None = leave as-is; Some("") = clear
}
```

---

### Modified: `RailwayModelRepository` trait

```rust
#[async_trait]
pub trait RailwayModelRepository: Send + Sync {
    // EXISTING — signature changed: added `lang` parameter
    async fn find_by_id(
        &mut self,
        id: &RailwayModelId,
        lang: &str,               // <-- NEW
    ) -> Result<Option<RailwayModel>, DomainError>;

    // EXISTING — unchanged
    async fn create(&mut self, params: &RailwayModelParams) -> Result<RailwayModelId, DomainError>;
    async fn save(&mut self, aggregate: &mut RailwayModel) -> Result<(), DomainError>;

    // NEW
    async fn find_translations(
        &mut self,
        id: &RailwayModelId,
    ) -> Result<Option<RailwayModelTranslations>, DomainError>;

    // NEW — FTS5 full-text search
    async fn search(
        &mut self,
        query: &str,
        lang: &str,
    ) -> Result<Vec<RailwayModelId>, DomainError>;
}
```

---

## Validation Rules

| Rule                                                                           | Enforced At                                                |
| ------------------------------------------------------------------------------ | ---------------------------------------------------------- |
| `lang` must be `"en"` or `"it"`                                                | Rust IPC boundary (`garde` validator on `Args`)            |
| `description` (EN) must not be empty on create                                 | Domain: `RailwayModel::new()` / application use case       |
| `description` (IT) may be `None` or non-empty; empty string treated as remove  | Application use case                                       |
| A translation with both `description IS NULL` and `details IS NULL` is deleted | Application use case before emitting `TranslationUpserted` |
| FTS5 index kept in sync by DB triggers — no application rule needed            | Database triggers                                          |

---

## State Transitions

```
[New Model Created]
    → EN translation inserted (description required)
    → IT translation absent

[IT Translation Added]
    → TranslationUpserted { lang: "it", description, details }
    → railway_model_translations upserted
    → FTS5 trigger fires → search_idx updated

[Translation Field Cleared]
    → If both fields become None → DELETE railway_model_translations WHERE lang = 'it'
    → FTS5 delete trigger fires → search_idx updated

[Model Deleted]
    → ON DELETE CASCADE removes railway_model_translations rows
    → FTS5 delete trigger fires → search_idx updated
```
