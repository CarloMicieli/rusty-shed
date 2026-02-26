# Research: Global Search

**Branch**: `030-global-search` | **Date**: 2026-02-26

---

## 1. Existing FTS5 Infrastructure

**Decision**: Extend `railway_model_search_idx` via a new migration rather than creating a parallel virtual table.

**Rationale**: Migration 0013 already establishes `railway_model_search_idx` with `(railway_model_id, language_code, description, details)`. The extension adds `manufacturer_name` and `rolling_stocks_text` (a space-concatenated aggregate of `road_number`, `livery`, `depot`, `series_code`). This keeps the search index unified — one row per `(railway_model_id, language_code)` — and avoids maintaining a separate virtual table.

**Note on migration 0013 triggers**: The existing triggers (`tr_rmt_fts_insert`, `tr_rmt_fts_update`, `tr_rmt_fts_delete`) from migration 0013 are removed in migration 0014 because they are replaced by the domain-event indexing strategy (see Decision 2 below). Removing them prevents double-write conflicts.

**Alternatives considered**:

- _Separate `global_search_idx` table_ — rejected: duplicates index maintenance logic; harder to keep consistent.
- _Content table / external content FTS5_ — rejected: adds complexity without benefit for a desktop single-user app.
- _Search at query time with LIKE across all tables_ — rejected: does not scale past ~500 rows; user requirement specifies 5,000 items remain responsive.

---

## 2. FTS5 Index Synchronisation Strategy

**Decision**: The FTS5 index is updated by the **repository infrastructure layer**, triggered by domain events, not by SQLite triggers. No database-level triggers are created or retained.

**Rationale**: The constitution's Domain Event Tracking law requires aggregates to record domain events and repositories to drain them atomically in a transaction. This means the repository already has a natural, explicit hook after every successful save. Updating the FTS5 index at that point — within the same transaction — gives us:

- **Atomicity**: if the save rolls back, the index update rolls back with it. No ghost entries.
- **Explicitness**: the update path is visible in Rust code, not hidden in the database layer.
- **Testability**: the index-update SQL can be unit-tested in isolation with `#[sqlx::test]`, unlike triggers which require careful fixture ordering.
- **Alignment with architecture**: mirrors the "Acknowledge then Index" pattern described in the feature design notes.

**Mechanism**: `SqliteRailwayModelRepository::save()` drains domain events (e.g., `RailwayModelCreated`, `TranslationUpserted`, `RollingStockAdded`). After processing each event's SQL mutations, it calls `self.rebuild_search_index(model_id, executor).await` which does an atomic DELETE + re-INSERT for all language rows of that model.

**Rolling stock fields**: `road_number`, `livery`, `depot`, `series_code` are concatenated into a single `rolling_stocks_text` column using a subquery with `group_concat`. One FTS5 row per `(railway_model_id, language_code)` — not one row per rolling stock. Concatenation is invisible to the FTS5 tokeniser; all tokens are indexed regardless of position.

**Manufacturer name**: Indexed in a dedicated `manufacturer_name` column. Since `Manufacturer` is a separate aggregate, its repository's save path must similarly rebuild the FTS5 rows for all `railway_models` associated with that manufacturer when a `ManufacturerNameUpdated` event is processed.

**Alternatives considered**:

- _SQLite AFTER INSERT/UPDATE/DELETE triggers_ — rejected: violates the Domain Event Tracking architectural law; hides side-effects in the database layer; makes testing harder; original migration 0013 triggers are removed for the same reason.
- _Background async task_ — rejected: introduces eventual consistency; a collector who just edits a road number expects it to be findable immediately.
- _One FTS5 row per rolling stock_ — rejected: requires de-duplication on the query result path; the per-model granularity is sufficient.

---

## 3. Manufacturer Name Indexing

**Decision**: Include `manufacturer_name` as a separate FTS5 column populated from `manufacturers.name`.

**Rationale**: "A.C.M.E." is a common search term for collectors. Brand names contain punctuation; the `unicode61` tokeniser handles dots and hyphens — partial matches like "acme" or "A.C.M" will resolve correctly because unicode61 strips punctuation from tokens. A dedicated column allows field-specific boosting in the future if needed (e.g., `manufacturer_name:acme`).

**Manufacturer index update**: Manufacturers rarely change their name. When a `ManufacturerNameUpdated` domain event is processed by the manufacturer repository, a bulk `rebuild_search_index` for all `railway_model_id` values linked to that manufacturer is executed within the same transaction. The cost is proportional to model count per manufacturer, which is bounded.

**Alternatives considered**:

- _Concatenate manufacturer into `rolling_stocks_text`_ — rejected: mixing entity types in one column reduces future extensibility (field-specific search weighting).
- _Join at query time without indexing manufacturer name_ — rejected: `manufacturers.name` is not a FTS5-indexed column; a LIKE would be required, defeating the performance goal.

---

## 4. Context Determination (Collection vs. Wishlist)

**Decision**: The `global_search` SQL query joins FTS5 results with `collection_items` and `wishlist_items` at query time using LEFT JOINs, returning one row per context per model.

**Rationale**: A model can appear in both collection and wishlist simultaneously (confirmed by spec assumption). The LEFT JOIN approach is correct: if a model is in both, two result rows are produced. SQLite query planner can use the existing indexes on `collection_items.railway_model_id` and `wishlist_items.railway_model_id` (both defined in migrations 0002/0003).

**Query pattern (simplified)**:

```sql
SELECT DISTINCT
    si.railway_model_id,
    ci.id            AS collection_item_id,
    wi.id            AS wishlist_item_id,
    bm25(railway_model_search_idx) AS rank
FROM railway_model_search_idx si
LEFT JOIN collection_items ci ON ci.railway_model_id = si.railway_model_id
                              AND ci.removed_date IS NULL
LEFT JOIN wishlist_items wi   ON wi.railway_model_id = si.railway_model_id
                              AND wi.removed_date IS NULL
WHERE railway_model_search_idx MATCH ?1
  AND (ci.id IS NOT NULL OR wi.id IS NOT NULL)
ORDER BY rank
LIMIT 50
```

A Rust-side mapper converts each row: if both `collection_item_id` and `wishlist_item_id` are non-null, two `GlobalSearchResult` values are emitted.

**Alternatives considered**:

- _Store context in FTS5 index_ — rejected: would require per-context rows in the FTS5 table; triggers become complex when an item is added to/removed from a collection or wishlist.
- _Two separate Tauri commands (searchCollection / searchWishlist)_ — rejected: the spec requires a single unified result list from one query.

---

## 5. Existing `search_railway_models` Command

**Finding**: A `search_railway_models` Tauri command already exists (catalog domain). It returns `Vec<RailwayModelId>` — just IDs, no context. The `SearchBar.svelte` component currently calls this and then does a second round-trip `getRailwayModelById` for each result.

**Decision**: Add a new `global_search` command rather than modifying `search_railway_models`. The existing catalog command remains unchanged (it is used for other purposes such as the add-model flow).

**Frontend `SearchBar` change**: On Enter key press, navigate to `/search?q={query}` instead of attempting inline result display. The existing inline search preview behaviour (desktop popover) can be preserved for quick-look-up while typing — the Enter action changes from inline selection to page navigation.

---

## 6. `/search` Page Architecture

**Decision**: SvelteKit route at `src/routes/search/+page.ts` (load function reads `url.searchParams.get('q')`) + `+page.svelte` (renders results). The load function calls the `global_search` Tauri command via `commands.globalSearch(...)` and returns the results to the page component.

**Debounce location**: The debounce (300 ms) lives in the `SearchBar` component on the typing path (inline preview). The `/search` page does not debounce — it fires one query immediately on load with the `q` parameter.

**Rationale**: SvelteKit's `load` function is the idiomatic place for data fetching in page components. This keeps the page component reactive to URL changes (back/forward navigation preserves search state).

**Alternatives considered**:

- _`SearchService.svelte.ts` calls command directly in `$effect`_ — rejected: SvelteKit `load` is more testable and integrates with the router's navigation lifecycle.
- _Global search context in layout_ — rejected: search is a page-level concern, not an app-level concern.

---

## 7. Language Handling

**Decision**: The `global_search` command accepts a `lang` string parameter (e.g., `"en"`, `"it"`). The FTS5 query filters by `language_code = lang` (exact match, no COALESCE fallback at the FTS layer). If no results are found for `lang`, the Rust use-case falls back to `"en"` with a second query.

**Rationale**: This matches the pattern used by `get_railway_model_by_id`. Consistent language handling prevents missing results for users of non-English locales.

**Alternatives considered**:

- _Concatenate all language translations into one FTS5 row_ — feasible but increases index size; mixed-language tokens can cause false positives for multi-language collections.
- _Query both languages simultaneously with OR_ — rejected: `MATCH` operators in FTS5 don't support parameterised language-OR cleanly; two separate queries are simpler and deterministic.

---

## 8. `removed_date` Filtering

**Decision**: Both `collection_items.removed_date IS NULL` and `wishlist_items.removed_date IS NULL` conditions are included in the search JOIN to exclude soft-deleted / removed items.

**Rationale**: Items with a `removed_date` are historical records (e.g., a model that was sold). Showing them in search would confuse the user since clicking would navigate to a detail page for an item they no longer own.

---

## 9. Result Display on `/search` Page

**Decision**: Results are grouped by source context (Collection first, then Wishlist) with a section header for each group. Each result card shows: model name/description, manufacturer, source badge (Collection | Wishlist), and clicking navigates to the item's detail page.

**Alternatives considered**:

- _Interleaved results sorted by rank only_ — rejected: harder for the user to understand which items are in their collection vs. wishlist at a glance.
- _Separate tabs per source_ — possible future enhancement, deferred to avoid scope creep.

---

## 10. Specta Type Generation

**Decision**: `GlobalSearchResultView` and `GlobalSearchArgs` derive `specta::Type` so that TypeScript types are auto-generated into `src/lib/bindings.ts` when `pnpm tauri dev` is run. No manual type definitions in TypeScript.

**Process**: After adding the new Rust command, run `pnpm tauri dev` once to trigger specta code generation, then commit the updated `bindings.ts`.
