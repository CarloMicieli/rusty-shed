# Quickstart: Localized Railway Model Data

**Feature**: `029-localized-model-data` | **Date**: 2026-02-25

## Prerequisites

- Rust toolchain (1.93.0+) — `rustup show`
- pnpm 10+ — `pnpm --version`
- Tauri CLI — installed via `pnpm tauri`

## Running the App

```bash
# Start the full desktop app (Rust backend + Svelte frontend)
pnpm tauri dev
```

The migration `0013_add_railway_model_translations.sql` runs automatically on startup via `sqlx::migrate!()`. Existing `description`/`details` data is migrated to the `railway_model_translations` table as English translations.

## Switching Languages

The app supports English (`en`) and Italian (`it`). Switch the active language via the Settings page or by directly updating the `PARAGLIDE_LOCALE` cookie / `localStorage` key `PARAGLIDE_LOCALE` to `"it"`.

When the language is Italian, `get_railway_model_by_id` is called with `lang: "it"`. If no Italian translation exists, the English text is shown with a `LanguageFallbackBadge`.

## Key Dev Commands

```bash
# Run Rust tests (includes sqlx::test migration tests)
pnpm rust:test

# Run a single test
cargo test --manifest-path src-tauri/Cargo.toml -p rusty_shed_lib \
  catalog::infrastructure::railway_model::tests::find_by_id_resolves_it_translation

# Run frontend tests
pnpm test

# Rebuild TypeScript bindings (after changing Rust specta types)
pnpm tauri dev    # bindings regenerated on first build; or run:
cargo build --manifest-path src-tauri/Cargo.toml

# Check Rust code
pnpm rust:clippy

# Format all
pnpm format && pnpm rust:fmt
```

## Adding Test Data

SQL fixture for integration tests (place in `src-tauri/fixtures/`):

```sql
-- fixtures/railway_model_translations.sql
INSERT INTO railway_model_translations (railway_model_id, language_code, description, details)
VALUES
  ('trn:railway-model:marklin:39004', 'en', 'Class 01 Steam Locomotive', 'Digitally controlled, sound fitted'),
  ('trn:railway-model:marklin:39004', 'it', 'Locomotiva a vapore classe 01', 'Controllo digitale, con suono');
```

Use in Rust tests:

```rust
#[sqlx::test(
    migrations = "./migrations",
    fixtures("railway_model_translations")
)]
async fn find_by_id_returns_it_translation(pool: SqlitePool) { ... }
```

## Verifying the Fallback Indicator

1. Open a railway model that has only an English translation.
2. Switch app language to Italian.
3. The model detail view should display the English description with a `(EN)` badge visible.
4. Add an Italian translation via the edit form.
5. The `(EN)` badge should disappear.

## FTS5 Search Verification

```sql
-- Connect to the SQLite DB directly (path: app data dir)
-- Check FTS5 index is populated:
SELECT * FROM railway_model_search_idx LIMIT 5;

-- Test a cross-language search:
SELECT railway_model_id FROM railway_model_search_idx
WHERE railway_model_search_idx MATCH 'locomotiva';
```

## Architecture Notes for Implementers

- **Language fallback lives in SQL** (`COALESCE` double-join in `sqlite_railway_model_repository.rs`). Do not re-implement it in Rust or TypeScript.
- **FTS5 is trigger-maintained**. The application never writes to `railway_model_search_idx` directly — only via `railway_model_translations` DML.
- **`LocalizedField.lang`** tells the frontend which language was resolved. Always check `descriptionLang !== currentLocale` before rendering the fallback badge.
- **`update_railway_model_text`** now requires a `lang` parameter. Update all callsites to pass `getLocaleService().currentLocale`.
- **`create_railway_model`** description and details are implicitly stored as `'en'` in the translation table. The Args payload is unchanged, but the infrastructure layer routes to `railway_model_translations` instead of `railway_models.description`.
