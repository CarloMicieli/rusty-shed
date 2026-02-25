# Phase 0 Research: Localized Railway Model Data

**Feature**: `029-localized-model-data` | **Date**: 2026-02-25

## Research Questions

Five technical unknowns were identified from the Technical Context and spec requirements. Each is resolved below.

---

### R-001: SQLx Compile-Time Verification and FTS5 Virtual Tables

**Question**: Can `sqlx::query!` (compile-time checked) be used with SQLite FTS5 virtual tables (`railway_model_search_idx`)? Does it support FTS5 `MATCH` syntax?

**Research**: SQLx's compile-time `query!` macro introspects the database schema at build time using `EXPLAIN QUERY PLAN`. FTS5 virtual tables expose themselves as regular tables for basic `SELECT` queries, so `sqlx::query!` can read from them. However, FTS5 `MATCH` queries use non-standard SQL syntax that confuses the SQLx type-inference engine, causing compilation failures or incorrect type inference for `MATCH` predicates.

**Decision**: Use `sqlx::query()` (runtime, no `!`) for all FTS5 `MATCH` search queries. Use `sqlx::query!` (compile-time) for all regular `INSERT`, `UPDATE`, `DELETE`, and `SELECT` operations on `railway_model_translations`.

**Rationale**: This is the established community pattern. Compile-time safety is preserved for the high-traffic CRUD path (reads, upserts); only the FTS5 search path uses runtime queries, which are deterministic and covered by integration tests using `#[sqlx::test(migrations)]`.

**Alternatives considered**:
- Use `sqlx::query!` for FTS5 — rejected: known incompatibility with `MATCH` predicates.
- Use a separate search library (e.g., `tantivy`) — rejected: over-engineering; SQLite FTS5 meets the <2 s SC-004 target at 10 000 rows and avoids an additional dependency.

---

### R-002: Language Code Passing from Frontend to Tauri IPC

**Question**: How does the frontend reliably pass the current user language to Tauri commands that need it (e.g., `get_railway_model_by_id`)?

**Research**: The project already has a `LocaleService` (`src/lib/shared/services/LocaleService.svelte.ts`) backed by Paraglide's `getLocale()`. It exposes a reactive `currentLocale: AvailableLanguageTag` property (typed as `"en" | "it"`). It is accessed via Svelte context DI with `getLocaleService()`. Paraglide configures two locales: `"en"` (base) and `"it"`.

**Decision**: Controllers that invoke locale-sensitive Tauri commands call `getLocaleService().currentLocale` and pass it as a `lang` string parameter. The Rust command handler accepts `lang: String`, validates it against the allowed set `["en", "it"]`, and defaults to `"en"` for any unknown value.

**Rationale**: Reuses the existing `LocaleService` without new state. The Tauri command validates the input at the boundary, so the fallback to `"en"` ensures robustness against future locale additions that are not yet supported by the translations system.

**Alternatives considered**:
- Detect language server-side from a stored setting — rejected: would require an extra DB read per command; the frontend already knows the active locale.
- Read locale from a persistent setting table — acceptable but adds indirection; deferred to a future settings-driven preference feature.

---

### R-003: COALESCE Double-Join Pattern in SQLx

**Question**: Does SQLx support the double-`LEFT JOIN` on the same table pattern needed for EN fallback, and how should the query be typed?

**Research**: SQLx fully supports aliases and multiple `LEFT JOIN`s on the same table. The key design concern is the `COALESCE` return type: SQLx infers it as `Option<String>` when either join leg may produce `NULL`. The result column must be handled with `.ok_or(...)` or `unwrap_or_default()` in the mapper.

**Decision**: Use the following query pattern, relying on runtime `sqlx::query()` for simplicity (avoids the need for complex `query_as!` type annotations for COALESCE columns):

```sql
SELECT
    rm.id,
    rm.manufacturer_id,
    m.name AS manufacturer_name,
    rm.product_code,
    COALESCE(t_req.language_code, t_en.language_code, 'en') AS resolved_lang,
    COALESCE(t_req.description,  t_en.description)          AS description,
    COALESCE(t_req.details,      t_en.details)               AS details
FROM railway_models rm
JOIN manufacturers m ON m.id = rm.manufacturer_id
LEFT JOIN railway_model_translations t_req
    ON t_req.railway_model_id = rm.id AND t_req.language_code = ?1
LEFT JOIN railway_model_translations t_en
    ON t_en.railway_model_id = rm.id AND t_en.language_code = 'en'
WHERE rm.id = ?2
```

The `resolved_lang` column tells the mapper which language was actually resolved, enabling the fallback indicator in the UI.

**Rationale**: Single SQL round-trip for the common read path. `COALESCE` is O(1) at the DB layer. The `resolved_lang` metadata avoids a second query to determine the display language.

**Alternatives considered**:
- Two separate queries (fetch requested lang, then fallback if NULL) — rejected: two round-trips; more complex Rust control flow.
- Store denormalized `resolved_lang` in the aggregate — rejected: violates separation of concerns; the resolved language changes with the user's locale preference.

---

### R-004: Domain Event Design for Translation Mutations

**Question**: How should the new `TranslationUpserted` event fit into the existing `RailwayModelEvent` enum and the `save()` repository dispatch?

**Research**: The existing `RailwayModelEvent` has five variants: `RailwayModelCreated`, `RailwayModelUpdated`, `RollingStockAdded`, `RollingStockUpdated`, `RollingStockRemoved`. The `save()` method pattern-matches on these and executes targeted SQL. The `RailwayModelUpdated` variant carries a `changed: serde_json::Value` JSON patch; the repository detects field sets from the JSON keys.

**Decision**: Add one new variant:

```rust
RailwayModelEvent::TranslationUpserted {
    event_id: Uuid,
    railway_model_id: RailwayModelId,
    timestamp: NaiveDateTime,
    lang: String,              // "en" or "it"
    description: Option<String>,
    details: Option<String>,
}
```

The aggregate method `upsert_translation(lang, description, details)` pushes this event. The repository's `save()` handles it with an `INSERT OR REPLACE INTO railway_model_translations` (SQLite upsert). No separate `TranslationRemoved` event is needed: setting both `description` and `details` to `None` for a given lang triggers a `DELETE` instead of an upsert (handled by repository logic on the event payload).

**Rationale**: Follows the same event-per-mutation pattern already used for rolling stocks. One event covers both create and update (upsert semantics). The repository remains the only place with SQL knowledge.

**Alternatives considered**:
- Re-use `RailwayModelUpdated` with a JSON key like `"translations"` — rejected: the existing `update_from_patch` logic would need special-casing; a dedicated event is cleaner and easier to test.
- Separate `TranslationCreated` and `TranslationUpdated` events — rejected: over-engineering; the domain has no need to distinguish create from update for translations.

---

### R-005: Data Migration Strategy for Existing `description`/`details` Columns

**Question**: How should existing railway model records migrate from `railway_models.description` and `railway_models.details` to the new `railway_model_translations` table? Can SQLite drop those columns after migration?

**Research**: SQLite 3.35.0+ supports `ALTER TABLE ... DROP COLUMN`. Tauri 2 bundles SQLite ≥ 3.35 (bundled via `libsqlite3-sys`). The migration can therefore directly `INSERT INTO railway_model_translations SELECT ... FROM railway_models` and then drop the columns in the same migration file.

**Decision**: Migration `0013_add_railway_model_translations.sql` will:
1. Create `railway_model_translations` table.
2. Create FTS5 virtual table `railway_model_search_idx`.
3. Create triggers (`AFTER INSERT`, `AFTER UPDATE`, `AFTER DELETE` on `railway_model_translations`).
4. Bulk-insert existing `description`/`details` as `'en'` translations (only where `description IS NOT NULL`).
5. Populate FTS5 index via a bulk `INSERT INTO railway_model_search_idx SELECT ...` (triggers only fire on future inserts).
6. Drop `description` and `details` columns from `railway_models`.

All six steps are in one migration file to keep the schema transition atomic. The FTS5 `DELETE` trigger must handle soft-delete re-indexing correctly.

**Rationale**: Single migration keeps the transition atomic and reversible (via sqlx `down` migrations if needed). Dropping the old columns removes the risk of stale data divergence between the old columns and the new table.

**Alternatives considered**:
- Keep `railway_models.description`/`details` as a denormalized cache — rejected: creates a dual source of truth; the COALESCE query already provides the EN fallback efficiently.
- Multiple migration files — rejected: split migrations risk partial state if one fails; SQLite migrations run in a transaction anyway.
