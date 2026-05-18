# Tasks: Centralized Entity Management

**Input**: Design documents from `/specs/041-entity-management/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/openapi.yaml, quickstart.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare feature scaffolding and shared type contracts

- [X] T001 Create Settings Library feature module scaffolding in `src/lib/features/settings/components/library/`
- [X] T002 Create buyer bounded-context module scaffolding in `src-tauri/src/buyers/`
- [X] T003 [P] Create feature test directories in `src/__tests__/settings/library/` and `src-tauri/tests/library/`
- [X] T004 [P] Add feature-level i18n key placeholders in `messages/en.json` and `messages/it.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend and shared UI foundations required by all stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T005 Add migrations for `is_system_seeded` and case-insensitive unique indexes in `src-tauri/migrations/`
- [X] T006 Add shared usage-count query primitives for manufacturers and canonical parties in `src-tauri/src/core/`
- [X] T007 Implement canonical shared-party repository operations in `src-tauri/src/sellers/infrastructure/`
- [X] T008 Implement buyer command registration and module wiring in `src-tauri/src/lib.rs` and `src-tauri/src/buyers/mod.rs`
- [X] T009 [P] Extend specta IPC type exports for new buyer/seller/manufacturer management DTOs in `src-tauri/src/bin/gen_types.rs`
- [X] T010 Extend shared entity form to support `mode: 'QUICK' | 'FULL'` in `src/lib/features/quick-add/QuickAddEntityForm.svelte`
- [X] T011 Add common Library state store (tabs/search/loading/errors) in `src/lib/features/settings/SettingsState.svelte.ts`
- [X] T012 Add centralized protection/lock badge utility in `src/lib/features/settings/types.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Browse and Search All Entities (Priority: P1) 🎯 MVP

**Goal**: Add Settings > Library with Manufacturers/Sellers/Buyers tabs and realtime search

**Independent Test**: Open Settings Library, switch among all tabs, filter by partial name/country, and verify empty-state rendering

### Tests for User Story 1

- [X] T013 [P] [US1] Add Library page render/search integration tests in `src/__tests__/settings/library/LibraryView.test.ts`
- [X] T014 [P] [US1] Add backend list/query command tests for manufacturers/sellers/buyers in `src-tauri/tests/library/list_entities_tests.rs`

### Implementation for User Story 1

- [X] T015 [P] [US1] Implement Library section host component in `src/lib/features/settings/components/library/LibrarySection.svelte`
- [X] T016 [P] [US1] Implement tabbed entity list component in `src/lib/features/settings/components/library/EntityTabs.svelte`
- [X] T017 [P] [US1] Implement shared search input and empty state in `src/lib/features/settings/components/library/EntitySearch.svelte`
- [X] T018 [US1] Integrate Library section into Settings route in `src/routes/settings/+page.svelte`
- [X] T019 [US1] Implement frontend query calls (`getManufacturers`, `getSellers`, `getBuyers`) in `src/lib/services/`
- [X] T020 [US1] Implement buyer list/query command handlers in `src-tauri/src/buyers/interface/command_handlers.rs`
- [X] T021 [US1] Regenerate and consume bindings for new list/query commands in `src/lib/bindings.ts`

**Checkpoint**: User Story 1 is independently functional and testable

---

## Phase 4: User Story 2 - Add a New Entity from Settings (Priority: P1)

**Goal**: Enable full-field Add New drawer for each tab with duplicate prevention

**Independent Test**: From each tab, create a unique entity via FULL form and verify immediate appearance without page reload

### Tests for User Story 2

- [X] T022 [P] [US2] Add FULL-mode form validation/duplicate tests, including a timing assertion that duplicate warning appears within 500ms after input settles, in `src/__tests__/settings/library/EntityFormFullMode.test.ts`
- [X] T023 [P] [US2] Add backend create command tests for manufacturer/seller/buyer surfaces and verify canonical shared-party visibility semantics across buyer/seller list queries in `src-tauri/tests/library/create_entities_tests.rs`

### Implementation for User Story 2

- [X] T024 [P] [US2] Extend shared entity form fields for FULL mode (Name, Website, Country, Notes) in `src/lib/features/quick-add/QuickAddEntityForm.svelte`
- [X] T025 [P] [US2] Implement Add New trigger and drawer orchestration per tab in `src/lib/features/settings/components/library/LibrarySection.svelte`
- [X] T026 [US2] Implement create commands for missing entity surfaces (`create_buyer`, updated `create_manufacturer`/`create_seller`) in `src-tauri/src/buyers/interface/command_handlers.rs` and `src-tauri/src/catalog/interface/manufacturers.rs` and `src-tauri/src/sellers/interface/command_handlers.rs
- [X] T027 [US2] Implement immediate insertion/update of canonical shared party state so newly created buyer/seller appears in both Buyers and Sellers tabs without reload in `src/lib/features/settings/SettingsState.svelte.ts`
- [X] T028 [US2] Add create success/error toasts and messages in `messages/en.json` and `messages/it.json`

**Checkpoint**: User Story 2 is independently functional and testable

---

## Phase 5: User Story 3 - Edit a User-Created Entity (Priority: P1)

**Goal**: Support edit flows with protection semantics and canonical cross-tab update behavior

**Independent Test**: Edit a user-created shared party in one tab and verify updates appear in both tabs; verify protected system-seeded name cannot be edited

### Tests for User Story 3

- [X] T029 [P] [US3] Add edit/protection UI tests in `src/__tests__/settings/library/EditEntityProtection.test.ts`
- [X] T030 [P] [US3] Add backend update/protection tests in `src-tauri/tests/library/update_entities_tests.rs`

### Implementation for User Story 3

- [X] T031 [P] [US3] Implement edit action and prefilled FULL form in `src/lib/features/settings/components/library/EntityRowActions.svelte`
- [X] T032 [US3] Implement backend update handlers enforcing `is_system_seeded` name protection in `src-tauri/src/catalog/interface/manufacturers.rs` and `src-tauri/src/sellers/interface/command_handlers.rs` and `src-tauri/src/buyers/interface/command_handlers.rs`
- [X] T033 [US3] Implement canonical shared-party update propagation across Buyers/Sellers tabs in `src/lib/features/settings/SettingsState.svelte.ts`
- [X] T034 [US3] Add protected badge rendering and disabled controls in `src/lib/features/settings/components/library/EntityTable.svelte`

**Checkpoint**: User Story 3 is independently functional and testable

---

## Phase 6: User Story 4 - Delete an Unused User-Created Entity (Priority: P2)

**Goal**: Allow deletion only when user-created and usage_count is zero, with confirmation and backend revalidation

**Independent Test**: Delete an unused user-created entity successfully; confirm delete is blocked for protected/in-use entities with explicit reason

### Tests for User Story 4

- [X] T035 [P] [US4] Add delete visibility/lock UI tests in `src/__tests__/settings/library/DeleteLocks.test.ts`
- [X] T036 [P] [US4] Add backend delete revalidation tests (protected + usage>0) in `src-tauri/tests/library/delete_entities_tests.rs`

### Implementation for User Story 4

- [X] T037 [P] [US4] Implement delete confirmation modal flow that displays entity name and affected linked-item count (or zero) in `src/lib/features/settings/components/library/DeleteEntityDialog.svelte`
- [X] T038 [US4] Implement backend delete commands with execution-time revalidation (`is_system_seeded=false && usage_count=0`) in `src-tauri/src/catalog/interface/manufacturers.rs` and `src-tauri/src/sellers/interface/command_handlers.rs` and `src-tauri/src/buyers/interface/command_handlers.rs`
- [X] T039 [US4] Implement shared-party total usage lock logic for buyer/seller records in `src-tauri/src/sellers/application/`
- [X] T040 [US4] Surface backend rejection reasons to UI messages in `messages/en.json` and `messages/it.json`
- [X] T055 [P] [US4] Add interaction-budget test asserting unused user-created delete completes in <=3 clicks from list row action in `src/__tests__/settings/library/DeleteLocks.test.ts`

**Checkpoint**: User Story 4 is independently functional and testable

---

## Phase 7: User Story 5 - Merge Two Duplicate Entities (Priority: P3)

**Goal**: Merge two user-created entities of same type with canonical target selection and atomic relinking

**Independent Test**: Merge two records and verify source removal plus full relinking; verify merge blocked when protected rows are included

### Tests for User Story 5

- [X] T041 [P] [US5] Add merge modal selection/validation UI tests in `src/__tests__/settings/library/MergeEntities.test.ts`
- [X] T042 [P] [US5] Add backend merge transaction tests (success + rollback + protected block) in `src-tauri/tests/library/merge_entities_tests.rs`

### Implementation for User Story 5

- [X] T043 [P] [US5] Implement merge selection modal in `src/lib/features/settings/components/library/MergeEntityDialog.svelte`
- [X] T044 [US5] Implement manufacturer merge command and relink transaction in `src-tauri/src/catalog/application/`
- [X] T045 [US5] Implement canonical shared-party merge command relinking buyer and seller references in one transaction in `src-tauri/src/sellers/application/` and `src-tauri/src/buyers/application/`
- [X] T046 [US5] Wire merge actions into tab lists and refresh state in `src/lib/features/settings/SettingsState.svelte.ts`

**Checkpoint**: User Story 5 is independently functional and testable

---

## Phase 8: User Story 6 - Mobile Responsive Table (Priority: P3)

**Goal**: Provide card-based responsive Library layout under 768px with full action parity

**Independent Test**: In mobile viewport, entities render as cards and Edit/Delete/Merge flows behave identically to desktop

### Tests for User Story 6

- [ ] T047 [P] [US6] Add responsive card rendering/action parity tests in `src/__tests__/settings/library/LibraryResponsive.test.ts`

### Implementation for User Story 6

- [ ] T048 [P] [US6] Implement mobile card layout variant in `src/lib/features/settings/components/library/EntityCards.svelte`
- [ ] T049 [US6] Implement desktop/mobile layout switch logic in `src/lib/features/settings/components/library/EntityTabs.svelte`
- [ ] T050 [US6] Ensure drawer/modal interactions remain accessible in mobile layout in `src/lib/features/settings/components/library/`

**Checkpoint**: User Story 6 is independently functional and testable

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final hardening across all stories

- [ ] T051 [P] Run full validation pipeline (`pnpm prepare`, `pnpm specta:generate`, `pnpm check`, `pnpm lint`, `pnpm test`, `pnpm rust:test`, `pnpm rust:clippy`) and address regressions
- [ ] T052 [P] Update architecture and feature docs for Settings Library behavior in `docs/FEATURE_IMPLEMENTATION.md` and `docs/FRONTEND_ARCHITECTURE.md`
- [ ] T053 Add final telemetry/logging for critical entity commands in `src-tauri/src/*/interface/command_handlers.rs`
- [ ] T054 Run quickstart verification scenarios from `specs/041-entity-management/quickstart.md`
- [ ] T056 [P] Measure duplicate-warning latency under FULL form input and assert p95 <= 500ms in `src/__tests__/settings/library/EntityFormFullMode.perf.test.ts`
- [ ] T057 [P] Benchmark UI-critical list queries for manufacturers/sellers/buyers and assert common-case <200ms in `src-tauri/tests/library/list_entities_perf_tests.rs`
- [ ] T058 Record profiling evidence and mitigation notes for any missed thresholds in `specs/041-entity-management/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: can start immediately
- **Phase 2 (Foundational)**: depends on Phase 1; blocks all user stories
- **Phases 3–8 (User Stories)**: depend on Phase 2 completion
- **Phase 9 (Polish)**: depends on completion of desired user stories

### User Story Dependencies

- **US1 (P1)**: starts after Foundational; no dependency on other stories
- **US2 (P1)**: starts after Foundational; can proceed parallel with US1 once shared form baseline is ready
- **US3 (P1)**: depends on US1 list UI and US2 form/edit surfaces
- **US4 (P2)**: depends on US1 list state and US3 protection display patterns
- **US5 (P3)**: depends on US1 list selection mechanics and US4 backend lock/protection checks
- **US6 (P3)**: depends on US1 list components; otherwise independent from CRUD logic

### Within Each User Story

- Tests first, then implementation
- Backend invariants before frontend orchestration when behavior is safety-critical
- Story must be independently verifiable before moving on

## Parallel Opportunities

- Phase 1 tasks marked `[P]` can run concurrently
- Foundational backend and frontend tasks T009/T010/T011/T012 can run in parallel after migrations begin
- For each story, `[P]` test tasks can run in parallel
- UI component tasks in separate files can run in parallel with backend command/service tasks

## Parallel Example: User Story 1

```bash
# Parallel test work
Task: T013 [US1] src/__tests__/settings/library/LibraryView.test.ts
Task: T014 [US1] src-tauri/tests/library/list_entities_tests.rs

# Parallel implementation work
Task: T015 [US1] src/lib/features/settings/components/library/LibrarySection.svelte
Task: T016 [US1] src/lib/features/settings/components/library/EntityTabs.svelte
Task: T020 [US1] src-tauri/src/buyers/interface/command_handlers.rs
```

## Parallel Example: User Story 5

```bash
# Parallel backend merge work per bounded context
Task: T044 [US5] src-tauri/src/catalog/application/
Task: T045 [US5] src-tauri/src/sellers/application/ and src-tauri/src/buyers/application/

# Parallel frontend merge UX
Task: T043 [US5] src/lib/features/settings/components/library/MergeEntityDialog.svelte
```

## Implementation Strategy

### MVP First (P1 stories)

1. Complete Setup (Phase 1)
2. Complete Foundational (Phase 2)
3. Deliver US1 → validate Library navigation/search
4. Deliver US2 → validate add flow in all tabs
5. Deliver US3 → validate edit and protected behavior

### Incremental Delivery

1. Add US4 delete locking/revalidation
2. Add US5 merge transactions
3. Add US6 responsive card mode
4. Final polish and full pipeline validation

### Team Parallel Strategy

1. Backend team: T005–T009 then US4/US5 command logic
2. Frontend team: T010–T012 then US1/US2/US6 UI
3. Integration team: bindings, i18n, and end-to-end verification tasks

