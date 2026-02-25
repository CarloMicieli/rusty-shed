# Tasks: Localized Railway Model Data

**Input**: Design documents from `/specs/029-localized-model-data/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: No dedicated test tasks — tests are embedded in the existing `#[sqlx::test]` / `#[cfg(test)]` pattern per project conventions.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Parallelizable — different files, no unresolved dependencies
- **[Story]**: User story label (US1–US5)

---

## Phase 1: Setup

**Purpose**: Schema foundation — must exist before any Rust code compiles against the new tables.

- [ ] T001 Create migration `src-tauri/migrations/0013_add_railway_model_translations.sql` — `CREATE TABLE railway_model_translations`, FTS5 virtual table `railway_model_search_idx`, three sync triggers (`tr_rmt_fts_insert`, `tr_rmt_fts_update`, `tr_rmt_fts_delete`), bulk `INSERT INTO railway_model_translations SELECT id,'en',description,details FROM railway_models`, bulk FTS5 populate, `ALTER TABLE railway_models DROP COLUMN description`, `ALTER TABLE railway_models DROP COLUMN details` (see `data-model.md` for exact SQL)

**Checkpoint**: Migration file written and parseable. Run `pnpm rust:build` to confirm SQLx compile-time checks pass against the new schema.

---

## Phase 2: Foundational (Domain Layer)

**Purpose**: Core domain types and trait definitions that every user story depends on. No user story can be implemented until this phase is complete.

⚠️ **CRITICAL**: Completing T001 first is required — SQLx `query!` macros verify against the live schema.

- [ ] T002 [P] Create `LocalizedField` value type in `src-tauri/src/catalog/domain/railway_model/localized_field.rs` — struct with `lang: String, value: String`; derives `Debug, Clone, PartialEq, Eq, Serialize, Deserialize, specta::Type`
- [ ] T003 [P] Create translation read structs in `src-tauri/src/catalog/domain/railway_model/railway_model_translation.rs` — `RailwayModelTranslations { railway_model_id, en: Option<RailwayModelTranslationEntry>, it: Option<RailwayModelTranslationEntry> }` and `RailwayModelTranslationEntry { description: Option<String>, details: Option<String> }`; both derive `Debug, Clone, Serialize, specta::Type`
- [ ] T004 [P] Add `TranslationUpserted` variant to `RailwayModelEvent` in `src-tauri/src/catalog/domain/railway_model/railway_model_event.rs` — fields: `event_id: Uuid, railway_model_id: RailwayModelId, timestamp: NaiveDateTime, lang: String, description: Option<String>, details: Option<String>`
- [ ] T005 Update `RailwayModel` aggregate in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` (depends on T002, T004): change `description: String` → `description: LocalizedField`; change `details: Option<String>` → `details: Option<LocalizedField>`; add `upsert_translation(lang: String, description: Option<String>, details: Option<String>)` method that pushes `TranslationUpserted`; update `update_description()` and `update_details()` to delegate to `upsert_translation()`
- [ ] T006 [P] Update `RailwayModelView` in `src-tauri/src/catalog/domain/railway_model/railway_model_view.rs` (depends on T002): add `description_lang: String` and `details_lang: Option<String>` fields; update `specta` type derive
- [ ] T007 [P] Update `RailwayModelRepository` trait in `src-tauri/src/catalog/domain/railway_model/repository.rs` (depends on T002, T003): change `find_by_id` signature to `find_by_id(&mut self, id: &RailwayModelId, lang: &str)`; add `find_view_by_id(&mut self, id: &RailwayModelId, lang: &str)`; add `find_translations(&mut self, id: &RailwayModelId) -> Result<Option<RailwayModelTranslations>, DomainError>`; add `search(&mut self, query: &str) -> Result<Vec<RailwayModelId>, DomainError>`
- [ ] T008 Update `src-tauri/src/catalog/domain/railway_model/mod.rs` (depends on T002, T003, T006): re-export `LocalizedField`, `RailwayModelTranslations`, `RailwayModelTranslationEntry`; confirm existing `RailwayModelView` re-export picks up new fields

**Checkpoint**: Run `pnpm rust:build` — all domain layer changes must compile cleanly. Infrastructure and interface layers will not compile yet (they still reference dropped columns via old signatures).

---

## Phase 3: User Story 1 — View Model in My Language (Priority: P1) 🎯 MVP

**Goal**: Any call to `get_railway_model_by_id` resolves description/details in the requested language, falls back to English when Italian is absent, and sets `description_lang`/`details_lang` so the frontend can render the fallback badge.

**Independent Test**: Switch app to Italian → open a model with only EN translation → English text appears with `(EN)` badge. Open a model with both translations → Italian text appears without badge. Open a model in English → no badge ever shown.

- [ ] T009 [US1] Update `find_by_id` and `find_view_by_id` in `src-tauri/src/catalog/infrastructure/railway_model/sqlite_railway_model_repository.rs`: replace direct `SELECT description, details FROM railway_models` with COALESCE double-LEFT-JOIN on `railway_model_translations` (see `research.md` R-003 for exact query); populate `resolved_lang` column; use runtime `sqlx::query()` (not `query!`) for COALESCE queries per research R-001
- [ ] T010 [P] [US1] Update `RailwayModelRow` in `src-tauri/src/catalog/infrastructure/entities.rs`: remove `description: String` and `details: Option<String>` fields; add `resolved_lang: String`, `description: Option<String>`, `details: Option<String>` (COALESCE may return NULL if both joins miss)
- [ ] T011 [P] [US1] Update `TryFrom<RailwayModelRow> for RailwayModel` in `src-tauri/src/catalog/infrastructure/mappers.rs`: construct `description: LocalizedField { lang: row.resolved_lang.clone(), value: row.description.unwrap_or_default() }`; construct `details: row.details.map(|v| LocalizedField { lang: row.resolved_lang.clone(), value: v })`; update `RailwayModelView` mapping to populate `description_lang` and `details_lang`
- [ ] T012 [US1] Update `GetRailwayModelById` use case in `src-tauri/src/catalog/application/get_railway_model_by_id.rs` (depends on T009–T011): add `lang: String` field to input type; pass lang to `repository.find_by_id(id, &lang)`; validate lang against `["en", "it"]`; default unknown values to `"en"`
- [ ] T013 [US1] Update `get_railway_model_by_id` Tauri command in `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T012): add `lang: String` parameter to the `#[tauri::command]` function; pass to use case
- [ ] T014 [P] [US1] Create `src/lib/components/LanguageFallbackBadge.svelte`: small inline badge (e.g., `(EN)`) displayed when content is shown in fallback language; accepts `lang: string` prop; uses Paraglide message key for label text; styled with Tailwind
- [ ] T015 [US1] Update `src/lib/components/RailwayModelCard.svelte` (depends on T014): import `LanguageFallbackBadge`; inject `LocaleService` via `getLocaleService()`; render badge after description text when `model.descriptionLang !== currentLocale`; same for details
- [ ] T016 [P] [US1] Update `src/lib/components/model-details/ModelDetailsHeader.svelte` (depends on T014): show `LanguageFallbackBadge` next to the description heading when `descriptionLang !== currentLocale`
- [ ] T017 [US1] Update all frontend callsites of `getRailwayModelById()` across `src/lib/` (depends on T013): pass `getLocaleService().currentLocale` as the `lang` argument; search for `getRailwayModelById` and update each caller

**Checkpoint**: `pnpm tauri dev` → load any model in Italian mode → English fallback badge visible. Load a model in English → no badge.

---

## Phase 4: User Story 2 — Add Translations When Creating (Priority: P2)

**Goal**: The create form accepts EN description (required) and IT description/details (optional). Both are persisted to `railway_model_translations`. The `get_railway_model_translations` command returns all stored translations for pre-populating the edit form.

**Independent Test**: Create model with EN description only → model loads in IT mode with EN badge. Create model with both EN+IT → model loads in IT mode with Italian text, no badge.

- [ ] T018 [US2] Update `sqlite_railway_model_repository.rs` (three changes): (1) `create()` — stop writing to dropped `description`/`details` columns; after inserting the `railway_models` row, insert EN translation via `INSERT INTO railway_model_translations (railway_model_id, language_code, description, details) VALUES (?, 'en', ?, ?)`; (2) `save()` — add `TranslationUpserted` arm: `INSERT OR REPLACE INTO railway_model_translations ... VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)`, or `DELETE FROM railway_model_translations WHERE railway_model_id=? AND language_code=?` when both description and details are None; (3) implement `find_translations()` — `SELECT language_code, description, details FROM railway_model_translations WHERE railway_model_id = ?` mapped to `RailwayModelTranslations`
- [ ] T019 [P] [US2] Create `src-tauri/src/catalog/application/get_railway_model_translations.rs` use case: accepts `railway_model_id: RailwayModelId`; calls `repository.find_translations(&id)`; returns `Option<RailwayModelTranslations>`
- [ ] T020 [P] [US2] Create `src-tauri/src/catalog/application/upsert_railway_model_translation.rs` use case: input fields `railway_model_id`, `lang: String`, `description: Option<String>`, `details: Option<String>`; validate lang; load aggregate; call `aggregate.upsert_translation()`; call `repository.save()`
- [ ] T021 [US2] Add `UpsertRailwayModelTranslationArgs` and `GetRailwayModelTranslationsArgs` to `src-tauri/src/catalog/interface/command_args.rs` (depends on T020): fields per `contracts/upsert_railway_model_translation.md` and `contracts/get_railway_model_translations.md`; derive `Debug, Clone, Deserialize, specta::Type, garde::Validate`
- [ ] T022 [US2] Add `get_railway_model_translations` and `upsert_railway_model_translation` handlers to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T018–T021): follow existing `#[tauri::command] #[specta::specta]` pattern; validate Args, map to Input, call use case
- [ ] T023 [US2] Register `get_railway_model_translations` and `upsert_railway_model_translation` in `src-tauri/src/lib.rs` invoke_handler list (depends on T022)
- [ ] T024 [P] [US2] Create `src/lib/features/catalogue/components/LocalizedFieldInput.svelte`: reusable textarea wrapper with a language label; accepts `lang: "en" | "it"`, `label: string`, `value: string`, `required: boolean` props; uses Tailwind + shadcn-svelte `Textarea`
- [ ] T025 [P] [US2] Create `src/lib/features/catalogue/components/TranslationsSection.svelte` (depends on T024): tabbed UI showing EN and IT translation inputs side by side or in tabs; exposes `enDescription`, `enDetails`, `itDescription`, `itDetails` bindable props; English tab required indicator; Italian tab all optional
- [ ] T026 [US2] Update `src/lib/features/catalogue/CreateRailwayModel.svelte` (depends on T025, T022): replace single `description` textarea with `TranslationsSection`; after model creation via `create_railway_model`, if IT fields are non-empty call `upsertRailwayModelTranslation` for `'it'`; pass EN description/details in `CreateRailwayModelArgs` as before (routed to EN translation by updated `create()`)

**Checkpoint**: Create a model with EN+IT translations → `get_railway_model_translations` returns both → display in IT mode shows Italian text.

---

## Phase 5: User Story 3 — Edit Existing Translations (Priority: P3)

**Goal**: The edit form fetches all stored translations and pre-populates EN/IT fields. Saving updates the targeted language translation without affecting the other.

**Independent Test**: Open edit form for a model with only EN translation → EN pre-populated, IT empty. Add IT description → save → model in IT mode shows Italian text. Clear IT → save → model in IT mode falls back to EN with badge.

- [ ] T027 [US3] Update `src-tauri/src/catalog/application/update_railway_model_text.rs` use case: add `lang: String` field to `UpdateRailwayModelTextInput`; instead of calling `model.update_description()`/`model.update_details()` directly, load existing translation for that language, merge the changed field, then call `model.upsert_translation(lang, merged_description, merged_details)`
- [ ] T028 [P] [US3] Update `UpdateRailwayModelTextArgs` in `src-tauri/src/catalog/interface/command_args.rs`: add `lang: String` field with `#[garde(pattern(r"^(en|it)$"))]`
- [ ] T029 [US3] Update `update_railway_model_text` command handler in `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T027, T028): pass `lang` from args to use case input
- [ ] T030 [P] [US3] Update all frontend callers of `updateRailwayModelText()` across `src/lib/` (search for `updateRailwayModelText`): add `lang: getLocaleService().currentLocale` to each call
- [ ] T031 [P] [US3] Update `src/lib/components/RailwayModelCard.svelte` edit path (depends on T025): on entering edit mode call `getRailwayModelTranslations(modelId)` to get all stored translations; render `TranslationsSection` pre-populated with EN/IT data; on save call `upsertRailwayModelTranslation` for each non-null language tab that was edited

**Checkpoint**: Edit a model's IT description → save → reload → IT text shown. Clear IT description → save → reload → EN fallback badge shown.

---

## Phase 6: User Story 4 — Search Models Across Languages (Priority: P4)

**Goal**: A search query matches railway models in either EN or IT, returning a list of model IDs ordered by FTS5 relevance rank.

**Independent Test**: Add model with unique EN term and unique IT term. Search each term → model appears in results. Search for a term that matches no model → empty list returned, no error.

- [ ] T032 [US4] Implement `search()` in `sqlite_railway_model_repository.rs`: use runtime `sqlx::query()` (not `query!`) for FTS5 MATCH query per research R-001 — `SELECT DISTINCT railway_model_id FROM railway_model_search_idx WHERE railway_model_search_idx MATCH ?1 ORDER BY rank LIMIT 200`; return `Vec<RailwayModelId>`
- [ ] T033 [P] [US4] Create `src-tauri/src/catalog/application/search_railway_models.rs` use case: input `query: String` (min 2 chars); validate; call `repository.search(&query)`; return `Vec<RailwayModelId>`
- [ ] T034 [US4] Add `SearchRailwayModelsArgs` to `src-tauri/src/catalog/interface/command_args.rs` (depends on T033): field `query: String`; `#[garde(length(min = 2, max = 500))]`; derives `Debug, Clone, Deserialize, specta::Type, garde::Validate`
- [ ] T035 [US4] Add `search_railway_models` command handler to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T032, T033, T034): standard pattern; returns `Result<Vec<RailwayModelId>, CommandError>`
- [ ] T036 [US4] Register `search_railway_models` in `src-tauri/src/lib.rs` invoke_handler (depends on T035)
- [ ] T037 [P] [US4] Wire `searchRailwayModels` to the frontend search UI: identify the existing search bar location in `src/lib/`; update the search handler to call `commands.searchRailwayModels({ query })` and display results as a list of model cards using the returned IDs with `getRailwayModelById(id, lang)`

**Checkpoint**: Type "locomotiva" in search (IT text in DB) → matching model appears in results list.

---

## Phase 7: User Story 5 — Non-Localized Fields Unaffected + i18n (Priority: P5)

**Goal**: All new UI strings use Paraglide message keys (no hardcoded text). Notes and rich-text fields are confirmed unaffected by the localization changes.

**Independent Test**: Enter Italian notes while app is in Italian → switch to English → same Italian note text displayed unchanged.

- [ ] T038 [P] [US5] Add Paraglide message keys in `messages/en.json` and `messages/it.json` for all new UI strings: fallback badge label (e.g., `"railway_model_content_in_english_fallback"`), TranslationsSection tab labels (`"translation_section_english"`, `"translation_section_italian"`), description/details field labels in translation context, required/optional field annotations
- [ ] T039 [P] [US5] Update `LanguageFallbackBadge.svelte` and `TranslationsSection.svelte` to use Paraglide message functions from `$lib/paraglide/messages.js` for all visible strings — no hardcoded text; replace any placeholder strings used during development with Paraglide calls

**Checkpoint**: `pnpm check` passes. All strings in new components use Paraglide.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final quality gate — all tools pass, bindings regenerated, full test suite green.

- [ ] T040 [P] Audit and fix remaining compilation errors from struct changes: search for direct access of `.description` and `.details` as `String` on `RailwayModel` in all Rust files not yet updated (run `pnpm rust:build` and fix each error) — focus on `application/save_railway_model.rs`, `application/testing.rs` (`FakeUow` mocks), `infrastructure/mappers.rs` (verify complete)
- [ ] T041 [P] Update Rust test fixtures and unit tests that previously set `railway_models.description`/`railway_models.details` columns: add `railway_model_translations` inserts to fixture SQL files in `src-tauri/fixtures/`; update `#[sqlx::test]` test functions in `sqlite_railway_model_repository.rs` that relied on the old schema
- [ ] T042 [P] Run `pnpm rust:clippy` (maps to `cargo clippy -D warnings --manifest-path src-tauri/Cargo.toml`) and fix all warnings in changed files — pay attention to `match` arms for the new `TranslationUpserted` event variant
- [ ] T043 [P] Run `pnpm check` (`svelte-check` + TypeScript) and fix all TypeScript errors from the updated `RailwayModelView` type (new `descriptionLang`/`detailsLang` fields) — update any destructured bindings or type assertions in `src/lib/`
- [ ] T044 Verify `src/lib/bindings.ts` has been regenerated with new specta types: confirm `LocalizedField`, `RailwayModelTranslations`, `UpsertRailwayModelTranslationArgs`, `SearchRailwayModelsArgs`, and updated `RailwayModelView` are present; run `cargo build --manifest-path src-tauri/Cargo.toml` if stale
- [ ] T045 Run full test suite: `pnpm rust:test` (all Rust tests including `#[sqlx::test]` migration tests) + `pnpm test` (Vitest) — all must pass before this feature branch is considered complete

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately
- **Phase 2 (Foundational)**: Depends on T001 — **blocks all user stories**
- **Phase 3 (US1)**: Depends on Phase 2 completion — can start immediately after
- **Phase 4 (US2)**: Depends on Phase 2 completion — can start in parallel with Phase 3
- **Phase 5 (US3)**: Depends on Phase 4 (reuses `upsert_railway_model_translation`)
- **Phase 6 (US4)**: Depends on Phase 2 (repository trait + foundational types)
- **Phase 7 (US5)**: Depends on Phase 3 and Phase 4 (new UI components must exist for i18n tasks)
- **Phase 8 (Polish)**: Depends on all phases complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no user story dependencies
- **US2 (P2)**: After Phase 2 — can run in parallel with US1 (different files)
- **US3 (P3)**: After US2 complete (reuses `TranslationsSection`, `upsert_railway_model_translation`)
- **US4 (P4)**: After Phase 2 — can run in parallel with US1/US2 (isolated FTS5 path)
- **US5 (P5)**: After US1 + US2 complete (i18n keys for their components)

### Within Each Phase

- Tasks marked [P] within a phase can be worked in parallel
- Infrastructure tasks before application tasks before interface tasks (within a user story)
- Backend command registration before frontend wiring

---

## Parallel Opportunities

### Phase 2 — Foundational

```
Parallel batch A: T002, T003, T004 (new files, no dependencies between them)
Then parallel batch B: T005, T006, T007, T008 (depend on batch A; different files)
```

### Phase 3 — US1

```
Parallel backend: T009, T010, T011 (different files, same Phase 2 dependency)
Then: T012 → T013 → T017 (sequential chain)
Parallel frontend: T014 → T015, T016 (can start alongside any backend task)
```

### Phase 4 — US2

```
Parallel: T018, T019, T020 (different files)
Then: T021 → T022 → T023 (sequential chain)
Parallel frontend: T024, T025 (different components); then T026
```

### Phase 6 — US4

```
Parallel: T032, T033 (different files)
Then: T034 → T035 → T036 (sequential chain)
Parallel frontend: T037 (independent of all Rust tasks)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Migration
2. Complete Phase 2: Domain foundations
3. Complete Phase 3: US1 — view with EN/IT fallback + badge
4. **STOP and VALIDATE**: Switch to Italian, confirm fallback works end-to-end
5. Deliver/demo: existing models display with EN fallback indicator

### Incremental Delivery

1. Phase 1 + Phase 2 → Foundation ready
2. Phase 3 (US1) → Language-aware display working — **demo**
3. Phase 4 (US2) → Create with translations working — **demo**
4. Phase 5 (US3) → Edit translations working — **demo**
5. Phase 6 (US4) → Cross-language search working — **demo**
6. Phase 7 (US5) → i18n complete, non-localized fields verified
7. Phase 8 (Polish) → Release-ready

### Parallel Team Strategy

With two developers after Phase 2:
- **Developer A**: US1 (Phase 3) — display + fallback badge
- **Developer B**: US2 (Phase 4) — create form + translation management
- US3 follows US2; US4 can run alongside either

---

## Notes

- **Never write directly to `railway_model_search_idx`** — FTS5 triggers maintain it automatically via `railway_model_translations` DML
- **FTS5 MATCH requires runtime `sqlx::query()`** (not `query!`) — see research R-001
- **COALESCE queries also use runtime `sqlx::query()`** due to `Option<String>` inference limitations
- **`description_lang` is the source of truth** for the fallback badge — always compare to `LocaleService.currentLocale`
- Commit after each completed phase checkpoint
- Run `pnpm rust:build` after T001 before writing any Rust code that references new schema
