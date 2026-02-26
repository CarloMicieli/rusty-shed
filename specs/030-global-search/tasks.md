# Tasks: Global Search

**Input**: Design documents from `/specs/030-global-search/`
**Prerequisites**: plan.md ✅ · spec.md ✅ · research.md ✅ · data-model.md ✅ · contracts/ ✅ · quickstart.md ✅

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1–US4)

---

## Phase 1: Setup (Module Scaffolding)

**Purpose**: Create empty module structure so Phase 2 tasks can work in parallel without file conflicts.

- [x] T001 Create Rust `search` domain module tree — add empty `mod.rs` files for `src-tauri/src/search/`, `src-tauri/src/search/domain/`, `src-tauri/src/search/application/`, `src-tauri/src/search/infrastructure/`, `src-tauri/src/search/interface/`
- [x] T002 [P] Create frontend feature module tree — add empty `index.ts`, `SearchService.svelte.ts`, and `components/` directory under `src/lib/features/search/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Database schema, domain types, and shared infrastructure that ALL user stories depend on.

**⚠️ CRITICAL**: No user story work can begin until T004 passes.

- [x] T003 Write `src-tauri/migrations/0014_extend_railway_model_search_idx.sql` — drop old triggers (`tr_rmt_fts_insert/update/delete`), drop old FTS5 table, recreate `railway_model_search_idx` with columns `(railway_model_id UNINDEXED, language_code UNINDEXED, description, details, manufacturer_name, rolling_stocks_text)` using `tokenize='unicode61'`, and add initial population INSERT-SELECT from `railway_model_translations JOIN railway_models JOIN manufacturers LEFT JOIN rolling_stocks` using `group_concat` for rolling stock fields. No triggers. See data-model.md for exact SQL.
- [x] T004 Run `pnpm rust:build` to apply migration 0014 and confirm the project compiles — verify FTS5 table is created with correct schema
- [x] T005 [P] Define `GlobalSearchResult` struct and `SearchSource` enum in `src-tauri/src/search/domain/global_search_result.rs` — fields: `railway_model_id: RailwayModelId`, `source: SearchSource`, `item_id: String`, `parent_id: Option<String>`, `display_name: String`, `manufacturer_name: String`. See data-model.md.
- [x] T006 [P] Define `GlobalSearchRepository` async trait in `src-tauri/src/search/domain/repository.rs` — single method `search(&mut self, query: &str, lang: &str) -> Result<Vec<GlobalSearchResult>, DomainError>`
- [x] T007 [P] Define `GlobalSearchArgs` (garde + specta + serde Deserialize) and `GlobalSearchResultView` (specta + serde Serialize) in `src-tauri/src/search/interface/command_args.rs` — `GlobalSearchArgs` fields: `query: String` (min 2, max 500), `lang: String` (min 2, max 10); `GlobalSearchResultView` fields: `railway_model_id`, `source` (String), `item_id`, `parent_id` (Option<String>), `display_name`, `manufacturer_name`. See contracts/global_search.md.
- [x] T008 Add `GlobalSearchUowExt` trait to `src-tauri/src/core/infrastructure/unit_of_work.rs` — method `global_search_repo(&mut self) -> &mut impl GlobalSearchRepository`; implement for `SqliteUnitOfWork`
- [x] T009 Declare the `search` module in `src-tauri/src/lib.rs` — add `mod search;` and wire sub-module exports; ensure `search::interface::command_handlers` is accessible for command registration

**Checkpoint**: Migration applied ✅ · Domain types compiled ✅ · UoW extension in place ✅

---

## Phase 3: User Stories 1 & 2 — Core Search + Debounce (Priority: P1) 🎯 MVP

**Goal**: A collector can type in the header search bar, press Enter, and see a results page with matched items from both collection and wishlist, each linking to the correct detail page. The search does not fire on every keystroke (300 ms debounce preserved).

**Independent Test**: Run the app, type "A.C.M.E" in the header search bar, press Enter, verify `/search?q=A.C.M.E` loads with results showing Collection and/or Wishlist badges and clicking a result navigates correctly.

- [x] T010 [P] [US1] Implement `GlobalSearch` use case in `src-tauri/src/search/application/global_search.rs` — define `GlobalSearchInput { query: String, lang: String }`; implement `GlobalSearch::execute(uow, input)` which transforms the query to prefix format (`"term"*`) and calls `uow.global_search_repo().search(&fts_query, &input.lang)`
- [x] T011 [P] [US1] Implement `rebuild_search_index(model_id, executor)` helper in `src-tauri/src/search/infrastructure/sqlite_global_search_repository.rs` — executes DELETE FROM `railway_model_search_idx` WHERE `railway_model_id = ?1`, then re-INSERT via JOIN query (see data-model.md "Rebuild SQL" section)
- [x] T012 [US1] Implement `SqliteGlobalSearchRepository::search()` in `src-tauri/src/search/infrastructure/sqlite_global_search_repository.rs` — execute the FTS5 MATCH query with LEFT JOINs on `collection_items` and `wishlist_items`, filter `removed_date IS NULL`, ORDER BY `bm25()`, LIMIT 50; map rows to `GlobalSearchResult`, emitting two results per row when both `collection_item_id` and `wishlist_item_id` are non-null (see data-model.md "Core Global Search SQL")
- [x] T013 [P] [US1] Hook `rebuild_search_index` into `src-tauri/src/catalog/infrastructure/railway_model/sqlite_railway_model_repository.rs` — after all domain events are drained and their SQL mutations applied (within the same transaction), call `rebuild_search_index(model.id, &mut tx)` before `tx.commit()`; for the `Deleted` event, DELETE FTS5 rows first, then delete the model row
- [x] T014 [P] [US1] Hook `rebuild_search_index` for manufacturer name changes in the manufacturer infrastructure repository (`src-tauri/src/catalog/infrastructure/manufacturer/`) — when a `ManufacturerNameUpdated` event is drained, call `rebuild_search_index` for every `railway_model_id` associated with that manufacturer, within the same transaction
- [x] T015 [US1] Implement `global_search` Tauri command in `src-tauri/src/search/interface/command_handlers.rs` — validate `args` with `args.validate()`; map `GlobalSearchArgs` → `GlobalSearchInput`; call `GlobalSearch::execute`; map result to `Vec<GlobalSearchResultView>`
- [x] T016 [US1] Register `global_search` in `src-tauri/src/lib.rs` — add to `.invoke_handler(tauri_specta::collect_commands![..., search_command_handlers::global_search])` and to the specta builder so TypeScript types are generated
- [x] T017 [US1] Regenerate `src/lib/bindings.ts` — run `pnpm tauri dev`, wait for specta to write bindings, then Ctrl+C; verify `globalSearch` and `GlobalSearchResultView` appear in the file
- [x] T018 [P] [US1] Add Paraglide message keys to `messages/en.json` and `messages/it.json` — keys: `search_page_title`, `search_results_for`, `search_source_collection`, `search_source_wishlist`, `search_no_results_title`, `search_no_results_body`, `search_add_new_model`, `search_result_count` (see contracts/global_search.md for English values; provide Italian equivalents in it.json)
- [x] T019 [US1] [US2] Update `src/lib/components/SearchBar.svelte` — change Enter key handler to call `goto(\`/search?q=\${encodeURIComponent(query.trim())}\`)`when`query.trim().length >= 2`; confirm the existing 300 ms debounce on the inline-preview path is preserved (do not remove it)
- [x] T020 [P] [US1] Implement `SearchService.svelte.ts` in `src/lib/features/search/` — expose `results: GlobalSearchResultView[]`, `isLoading: boolean`, `hasSearched: boolean`; add `search(query, lang)` method that calls `commands.globalSearch({ query, lang })` and updates state; export `setSearchContext` / `getSearchContext`
- [x] T021 [US1] Create `src/routes/search/+page.ts` — export `load` function that reads `url.searchParams.get('q')`, returns early with `{ results: [], query: '', hasSearched: false }` if query length < 2, otherwise calls `commands.globalSearch({ query: q, lang: getLocale() })` and returns `{ results: result.data, query: q, hasSearched: true }`
- [x] T022 [US1] Create `src/routes/search/+page.svelte` — display page title using `m.search_page_title()`, render `SearchResultCard` for each result grouped by source (Collection section first, then Wishlist section); wire `data` from `+page.ts` load function; use `SearchEmptyState` when `hasSearched && results.length === 0` (placeholder until Phase 4)
- [x] T023 [P] [US1] Create `src/lib/features/search/components/SearchResultCard.svelte` — props: `result: GlobalSearchResultView`; displays `displayName`, `manufacturerName`, source badge using `m.search_source_collection()` / `m.search_source_wishlist()`; clicking navigates to `/collection/{result.itemId}` (Collection) or `/wishlists/{result.parentId}/items/{result.itemId}` (Wishlist)
- [x] T024 [P] [US1] Write Vitest test for `SearchResultCard.svelte` in `src/__tests__/features/search/SearchResultCard.test.ts` — test: Collection badge renders and link points to `/collection/{itemId}`; Wishlist badge renders and link points to `/wishlists/{parentId}/items/{itemId}`
- [x] T025 [P] [US1] Write `#[sqlx::test(migrations = "./migrations")]` test for `SqliteGlobalSearchRepository` in `src-tauri/src/search/` — test cases: match by description, match by road_number, match by manufacturer_name, no result when model not in collection/wishlist, no result when `removed_date` is set, delete event removes FTS5 rows

**Checkpoint**: User Stories 1 & 2 fully functional ✅ — search page reachable, results displayed, correct routing, debounce preserved.

---

## Phase 4: User Story 3 — Empty State (Priority: P2)

**Goal**: When no items match the search term, the user sees "No models found" and an option to add a new model.

**Independent Test**: Search for "zzz_no_match" — verify the empty state message and "Add a new model" link appear instead of a results list.

- [x] T026 [P] [US3] Create `src/lib/features/search/components/SearchEmptyState.svelte` — props: `query: string`; displays `m.search_no_results_title()`, `m.search_no_results_body({ query })`, and a link/button using `m.search_add_new_model()` that navigates to `/catalogue/new-model`
- [x] T027 [US3] Replace the empty-state placeholder in `src/routes/search/+page.svelte` with `<SearchEmptyState {query} />` — shown when `data.hasSearched && data.results.length === 0`
- [x] T028 [P] [US3] Write Vitest test for `SearchEmptyState.svelte` in `src/__tests__/features/search/SearchEmptyState.test.ts` — test: "No models found" text is visible; "Add a new model" link points to `/catalogue/new-model`

**Checkpoint**: Empty state visible and functional ✅ — US1 + US2 + US3 all independently testable.

---

## Phase 5: User Story 4 — Loading Indicator (Priority: P3)

**Goal**: A loading indicator is visible in the search bar while a query is in progress; the search results page shows a loading state while the load function is executing.

**Independent Test**: Throttle the Tauri command response artificially in dev — verify the spinner appears in the search bar and the page shows a loading skeleton, then disappears when results render.

- [x] T029 [US4] Add loading spinner to `src/lib/components/SearchBar.svelte` — show a subtle spinner (shadcn-svelte `Loader2` icon or equivalent) inside the search input while `isSearching` is true (the existing `isSearching` reactive state already tracks the inline-preview fetch; the same flag covers the debounce-to-navigation window)
- [x] T030 [US4] Add loading feedback to `src/routes/search/+page.svelte` — use SvelteKit's `$navigating` store or a page-level loading skeleton to show a loading state while `+page.ts` is executing; hide it once `data` is available

**Checkpoint**: All four user stories complete ✅

---

## Phase 6: Polish & Cross-Cutting Concerns

- [x] T031 [P] Run frontend quality checks: `pnpm format && pnpm lint && pnpm check && pnpm test` — all must pass with no errors
- [x] T032 [P] Run Rust quality checks: `pnpm rust:fmt && pnpm rust:clippy && pnpm rust:test` — clippy must pass with zero warnings (`-D warnings`); all sqlx::test cases must pass
- [x] T033 Export new search components from `src/lib/features/search/index.ts` barrel — `SearchService`, `SearchResultCard`, `SearchEmptyState`; verify no unused imports in any modified file
- [x] T034 Manual smoke test per quickstart.md Step 7 — verify all six scenarios: normal search, Collection routing, Wishlist routing, empty state, 1-char guard, cross-collection+wishlist result

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 — **BLOCKS all user story phases**; T004 (build verification) is the hard gate
- **Phase 3 (US1 + US2)**: Depends on Phase 2 complete — especially T004, T005, T006, T007, T008, T009
- **Phase 4 (US3)**: Depends on T022 (search page exists); independent of Phase 3 implementation details
- **Phase 5 (US4)**: Depends on T019 (SearchBar updated) and T022 (search page exists)
- **Phase 6 (Polish)**: Depends on all desired user story phases

### Key Task Dependencies Within Phase 3

```
T005, T006, T007, T008, T009  (Phase 2 foundations)
  ↓
T010 [use case]   T011 [rebuild helper]   T023 [card component]   T018 [messages]
       ↓                 ↓
     T015             T012 [search query]
  [command]            T013 [catalog hook]
       ↓               T014 [manufacturer hook]
     T016
  [register]
       ↓
     T017
  [regen bindings]
       ↓
  T019, T020, T021
       ↓
     T022 [page]
```

### Parallel Opportunities Per Phase

**Phase 2 (after T004)**:

```
T005 · T006 · T007  — run together (different files)
```

**Phase 3 (after Phase 2 complete)**:

```
T010 · T011 · T018 · T023 · T024 · T025  — run together (different files)
T013 · T014  — run together after T011
T015          — after T010 + T012
T016          — after T015
T017          — after T016
T019 · T020   — after T017
T021          — after T017
T022          — after T021 + T023
```

**Phase 4 (after T022)**:

```
T026 · T028  — run together
T027         — after T026
```

---

## Parallel Example: User Story 1

```text
# After Phase 2 completes, these can start simultaneously:
T010  Implement GlobalSearch use case
T011  Implement rebuild_search_index helper SQL
T018  Add Paraglide message keys
T023  Create SearchResultCard component
T024  Write Vitest test for SearchResultCard
T025  Write sqlx::test for GlobalSearchRepository

# Then T012, T013, T014 can run in parallel (all depend on T011)
# Then T015 (depends on T010 + T012)
# Then T016 → T017 → T019, T020, T021 → T022
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 only)

1. Complete Phase 1: Setup (T001–T002)
2. Complete Phase 2: Foundational (T003–T009) — **do not skip T004**
3. Complete Phase 3: US1 + US2 (T010–T025)
4. **STOP AND VALIDATE**: open the app, search for a known brand, press Enter, verify results page and routing
5. Ship MVP

### Incremental Delivery

1. Phase 1 + Phase 2 → foundation ready
2. Phase 3 → search works end-to-end (MVP)
3. Phase 4 → empty state prevents dead ends
4. Phase 5 → loading feedback improves perceived performance
5. Phase 6 → quality gate before merge

---

## Notes

- `[P]` tasks touch different files and have no mutual blocking dependencies — safe to run in parallel
- `[US1]` through `[US4]` map directly to User Stories in spec.md
- T013 and T014 modify **existing** catalog infrastructure files — read them before editing to avoid overwriting unrelated logic
- After T017 (bindings regen), commit `bindings.ts` so the file is tracked and diffs are visible in review
- FTS5 note: `bm25()` returns negative scores; ORDER BY `rank` (ascending) puts the best match first — do not add `DESC`
- The `removed_date IS NULL` filter in T012 is critical for correctness; verify it is present in the final SQL
