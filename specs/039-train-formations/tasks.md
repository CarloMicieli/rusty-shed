# Tasks: Train Formations

**Feature**: `039-train-formations` | **Generated**: 2026-03-29
**Input**: spec.md, plan.md, data-model.md, contracts/tauri-ipc.md, quickstart.md, research.md
**Branch**: `039-train-formations`

---

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Maps to user story from spec.md (US1–US5)
- Every task includes an exact file path

---

## Phase 1: Setup

**Purpose**: Branch scaffolding and SQLite migration that all subsequent phases depend on.

- [x] T001 Create branch `039-train-formations` and verify `cargo build` + `pnpm check` pass cleanly on HEAD
- [x] T002 Create migration `src-tauri/migrations/0009_create_train_formations_schema.sql` from the SQL in `data-model.md` (tables: `prototypes`, `formation_categories`, `train_formations`, `formation_elements` — including `snapshot_series_code` + `snapshot_company_name` columns; ALTER TABLE `owned_rolling_stocks`)
- [x] T003 Run `sqlx migrate run` and verify all tables and indexes exist in the SQLite database

**Checkpoint**: Migration applied — all tables exist, FK constraints verified.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core Rust domain infrastructure shared by all user stories. MUST be complete before any US phase begins.

- [x] T004 Create `src-tauri/src/trains/mod.rs` and register the `trains` module in `src-tauri/src/lib.rs`
- [x] T005 [P] Create `src-tauri/src/trains/domain/formation/train_formation_event.rs` — `TrainFormationEvent` enum (10 variants) with `#[serde(tag = "type", content = "payload", rename_all = "camelCase")]` as specified in `quickstart.md` Phase A2a
- [x] T006 [P] Create `src-tauri/src/trains/domain/formation/formation_element.rs` — `FormationElement` value object (fields: `id`, `prototype_id`, `owned_rolling_stock_id`, `position_order`, `traction_override`; derive `Debug, Clone, Serialize, Deserialize`)
- [x] T007 Create `src-tauri/src/trains/domain/formation/train_formation.rs` — `TrainFormation` aggregate with `pending_events: Vec<TrainFormationEvent>`, all mutating methods (`create`, `rename`, `update_metadata`, `add_element`, `remove_element`, `reorder_elements`, `assign_rolling_stock`, `unassign_rolling_stock`, `set_traction_override`), `apply_event()`, and `take_events()` — full spec in `quickstart.md` Phase A2b (depends on T005, T006)
- [x] T008 [P] Create `src-tauri/src/trains/domain/formation/repositories.rs` — `TrainFormationRepository` trait (methods: `find_by_id`, `find_all`, `save`, `delete`)
- [x] T009 [P] Create `src-tauri/src/trains/domain/prototype/prototype.rs` — `Prototype` struct (fields match `data-model.md` entity; derive `Debug, Clone`)
- [x] T010 [P] Create `src-tauri/src/trains/domain/prototype/repositories.rs` — `PrototypeRepository` trait (methods: `find_by_id`, `search`, `save`, `find_all_grouped`)
- [x] T011 [P] Create `src-tauri/src/trains/domain/formation_category/formation_category.rs` — `FormationCategory` struct
- [x] T012 Create `src-tauri/src/trains/infrastructure/entities.rs` — SQLx row structs for all 4 tables (`TrainFormationRow`, `FormationElementRow`, `PrototypeRow`, `FormationCategoryRow`)
- [x] T013 Create `src-tauri/src/trains/infrastructure/mappers.rs` — row-to-domain and domain-to-view mapping functions; set `stock_not_found = snapshot_series_code.is_some() && owned_rolling_stock_id.is_none()` when mapping `FormationElementRow` → `FormationElementView` (depends on T012, T006, T009, T011)
- [x] T014 [P] Create `src-tauri/src/trains/infrastructure/seed_data.rs` — `insert_default_prototypes()` and `insert_default_categories()` using `INSERT OR IGNORE` (idempotent); call from app startup after migrations
- [x] T015 [P] Create `src-tauri/src/trains/interface/command_args.rs` — all `*Args` structs from `contracts/tauri-ipc.md` with `#[derive(Debug, Clone, serde::Deserialize, specta::Type, garde::Validate)]` and all `#[garde(...)]` annotations from the garde validation table in `contracts/tauri-ipc.md`

**Checkpoint**: `cargo check` passes — domain module compiles, entity structs defined, repository traits declared.

---

## Phase 3: User Story 1 — Create and Manage a Train Formation (Priority: P1) 🎯 MVP

**Goal**: Users can create, view, edit, and delete formation metadata (name, category, epoch, years, notes). No composition required.

**Independent Test**: Create formation "Gottardo 1974", category "EuroCity", years 1970–1982, epoch IV, add notes → verify all fields persist and render correctly on the detail page.

### Implementation

- [x] T016 Create `src-tauri/src/trains/infrastructure/train_formation_repo.rs` — `SqlxTrainFormationRepository` implementing `TrainFormationRepository`; implement `save` (INSERT + UPDATE with optimistic version), `find_by_id`, `find_all` (returns `TrainFormationSummary` view), `delete` (cascade via DB) (depends on T012, T013)
- [x] T017 [P] [US1] Create `src-tauri/src/trains/application/create_train_formation.rs` — use case struct; validates via aggregate `TrainFormation::create()`, calls repo `save()`, drains events via `take_events()` (depends on T007, T016)
- [x] T018 [P] [US1] Create `src-tauri/src/trains/application/update_train_formation.rs` — calls `formation.rename()` + `formation.update_metadata()`, drains events (depends on T007, T016)
- [x] T019 [P] [US1] Create `src-tauri/src/trains/application/delete_train_formation.rs` — verifies existence, calls repo `delete()`, emits `Deleted` event (depends on T016)
- [x] T020 [P] [US1] Create `src-tauri/src/trains/application/get_train_formation.rs` and `get_train_formations.rs` — read use cases returning `TrainFormationDetail` and `Vec<TrainFormationSummary>` (depends on T016)
- [x] T021 [US1] Create `src-tauri/src/trains/interface/command_handlers.rs` — implement `create_train_formation`, `update_train_formation`, `delete_train_formation`, `get_train_formation`, `get_train_formations` Tauri commands with `garde` validation + `CommandError` return type per `contracts/tauri-ipc.md` (depends on T015, T017–T020)
- [x] T022 [US1] Register all 5 US1 commands in `src-tauri/src/lib.rs` (or the central Tauri builder); run `cargo build` (depends on T021)
- [x] T023 [US1] Run `pnpm tauri dev` once to trigger `tauri-specta` binding regeneration; verify `src/lib/bindings.ts` contains `CreateTrainFormationArgs`, `TrainFormationSummary`, `TrainFormationDetail`
- [x] T024 [P] [US1] Create `src/lib/features/train-formations/services/formations.service.ts` — `safeInvoke` wrappers for US1 commands (depends on T023)
- [x] T025 [P] [US1] Create `src/lib/features/train-formations/TrainFormationState.svelte.ts` — Svelte 5 class with `$state` formations list, `load()`, `create()`, `update()`, `delete()` methods (depends on T024)
- [x] T026 [US1] Create `src/routes/train-formations/+page.svelte` — formation list page using `TrainFormationState`; renders `FormationList.svelte` (depends on T025)
- [x] T027 [US1] Create `src/routes/train-formations/[id]/+page.svelte` — formation detail page; renders `FormationBuilder.svelte` (depends on T025)
- [x] T028 [P] [US1] Create `src/lib/features/train-formations/components/FormationList.svelte` — empty-state + list of `FormationCard` (depends on T025)
- [x] T029 [P] [US1] Create `src/lib/features/train-formations/components/FormationCard.svelte` — shows name, category, epoch, owned/planned counts (depends on T023)
- [x] T030 [P] [US1] Create `src/lib/features/train-formations/components/FormationForm.svelte` — create/edit form with name, category, start_year, end_year, epoch, notes fields; `garde`-aligned client-side validation (name required, year range); uses Superforms + Zod per web-form skill (depends on T023)
- [x] T031 [US1] Add `formations_*` i18n keys to `messages/en.json` and `messages/it.json` — all keys from `quickstart.md` Phase C4 minimum key set
- [x] T032 [US1] Add Train Formations entry to `src/lib/components/navigation/config.ts` — `isPrimary: false` (→ "More" menu on mobile), `href: '/train-formations'`, `icon: Combine`
- [x] T033 [US1] Write inline Rust domain unit tests in `src-tauri/src/trains/domain/formation/train_formation.rs` — 5 aggregate invariant tests from `quickstart.md` Phase F1 (empty name, year range, open-ended, null years, same year)

**Checkpoint (US1)**: `cargo test` passes all T033 tests. `pnpm check` passes. Formation list and detail pages render. CRUD operations persist through app restart.

---

## Phase 4: User Story 2 — Manage Formation Categories (Priority: P2)

**Goal**: Users can select from seeded formation categories and add custom ones that persist globally.

**Independent Test**: Add custom category "Regionale"; create a formation using it; reopen formation form — custom category appears alongside built-in entries.

### Implementation

- [ ] T034 [P] [US2] Create `src-tauri/src/trains/infrastructure/prototype_repo.rs` (partial) — implement only `FormationCategoryRepository` portion: `find_all`, `save` (for custom categories) (depends on T012, T013)
- [x] T035 [P] [US2] Create `src-tauri/src/trains/application/get_formation_categories.rs` — returns `Vec<FormationCategoryView>` (seeded + custom) (depends on T034)
- [x] T036 [P] [US2] Create `src-tauri/src/trains/application/create_formation_category.rs` — validates unique name, saves `is_custom=true` record (depends on T034)
- [x] T037 [US2] Add `get_formation_categories` and `create_formation_category` Tauri commands to `src-tauri/src/trains/interface/command_handlers.rs` and register in `src-tauri/src/lib.rs` (depends on T035, T036)
- [x] T038 [US2] Re-run `pnpm tauri dev` to update `src/lib/bindings.ts` with `FormationCategoryView` and `CreateFormationCategoryArgs`
- [x] T039 [US2] Update `src/lib/features/train-formations/components/FormationForm.svelte` — wire category picker to `get_formation_categories`; add inline "+ new category" action that calls `create_formation_category` (depends on T038, T030)
- [x] T040 [US2] Verify seed data in `src-tauri/src/trains/infrastructure/seed_data.rs` includes minimum 5 default categories (EuroCity, Intercity, TEE, Express, Regional) per FR-006

**Checkpoint (US2)**: Custom categories persist across restarts. Formation form populates category picker. Seed categories always present after re-migration.

---

## Phase 5: User Story 3 — Build a Formation Composition (Priority: P3)

**Goal**: Users can search the Prototype library, add units to a formation, drag-and-drop reorder, and remove units.

**Independent Test**: Open a formation; search "Re 4/4"; add it; add two coaches; verify 3 cells in insertion order; drag cell to swap positions; verify new order persists after reload.

### Implementation

- [ ] T041 [P] [US3] Complete `src-tauri/src/trains/infrastructure/prototype_repo.rs` — implement full `PrototypeRepository`: `find_all_grouped`, `search` (real-time filter by `series_code`/`car_type`), `find_by_id`, `save` for custom prototypes (depends on T012, T013)
- [x] T042 [P] [US3] Create `src-tauri/src/trains/application/get_prototypes.rs` — returns `Vec<PrototypeGroupView>` filtered by optional `search_query` (depends on T041)
- [x] T043 [P] [US3] Create `src-tauri/src/trains/application/create_custom_prototype.rs` — validates `car_type` against enum, checks company exists, saves `is_custom=true` (depends on T041)
- [x] T044 [P] [US3] Create `src-tauri/src/trains/application/add_formation_element.rs` — calls `formation.add_element()`, drains events; appends at end (position = current max + 1) (depends on T007, T016)
- [x] T045 [P] [US3] Create `src-tauri/src/trains/application/remove_formation_element.rs` — calls `formation.remove_element()`, drains events; shifts `position_order` of subsequent elements atomically (depends on T007, T016)
- [x] T046 [P] [US3] Create `src-tauri/src/trains/application/reorder_formation_elements.rs` — calls `formation.reorder_elements()`, drains `ElementsReordered` event; repo performs bulk `position_order` UPDATE in a single transaction (depends on T007, T016)
- [x] T047 [US3] Add `get_prototypes`, `create_custom_prototype`, `add_formation_element`, `remove_formation_element`, `reorder_formation_elements` commands to `command_handlers.rs` and register in `src-tauri/src/lib.rs` (depends on T042–T046)
- [x] T048 [US3] Re-run `pnpm tauri dev` to update bindings; verify `AddFormationElementArgs`, `ReorderFormationElementsArgs`, `PrototypeGroupView` in `src/lib/bindings.ts`
- [x] T049 [US3] Add `pnpm add svelte-dnd-action` (approved 2026-03-29 per `quickstart.md` Phase D1)
- [x] T050 [P] [US3] Create `src/lib/features/train-formations/components/icons/Locomotive.svelte` — SVG from `quickstart.md` Phase C3 (stroke="currentColor", $props() rune)
- [x] T051 [P] [US3] Create `src/lib/features/train-formations/components/icons/Coach.svelte` — SVG from `quickstart.md` Phase C3
- [x] T052 [P] [US3] Create `src/lib/features/train-formations/components/icons/Wagon.svelte` — SVG from `quickstart.md` Phase C3
- [x] T053 [US3] Create `src/lib/features/train-formations/components/icons/PrototypeIcon.svelte` — dispatcher mapping `car_type` → icon component + ownership styling; `$derived` for component selection; `iconMap` covers all 9 `car_type` values per `quickstart.md` Phase C3 (depends on T050–T052)
- [x] T054 [P] [US3] Create `src/lib/features/train-formations/domain/traction.ts` — `isTractionSlot()` and `hasTraction()` pure functions for frontend traction evaluation
- [x] T055 [P] [US3] Create `src/lib/features/train-formations/components/FormationCell.svelte` — renders `PrototypeIcon`, series code, `OwnershipBadge`; "Quick Assign" when `ownedCount === 1` (depends on T053)
- [x] T056 [US3] Create `src/lib/features/train-formations/components/FormationTrack.svelte` — horizontal scroll container with `svelte-dnd-action` `dndzone`; optimistic `consider` (local state only) + `finalize` (single `reorderFormationElements` call); `animate:flip` per `quickstart.md` Phase D2 (depends on T049, T055)
- [x] T057 [P] [US3] Create `src/lib/features/train-formations/components/AddStockDrawer.svelte` — side drawer; calls `getPrototypes(searchQuery)`; groups results by railway company; shows `+ Add Prototype` when search is empty (depends on T048)
- [x] T058 [P] [US3] Create `src/lib/features/train-formations/components/PrototypeSearchResults.svelte` — renders grouped search results inside `AddStockDrawer` (depends on T048)
- [x] T059 [P] [US3] Create `src/lib/features/train-formations/components/CreatePrototypeForm.svelte` — inline prototype creation from drawer; shown when "+ Add Prototype" clicked (depends on T048)
- [x] T060 [US3] Update `src/lib/features/train-formations/TrainFormationState.svelte.ts` — add `addElement()`, `removeElement()`, `reorderElements()` methods; optimistic local reorder state (depends on T056, T048)
- [x] T061 [US3] Wire `FormationBuilder.svelte` as the host component for the detail page: `IdentityCard` (sticky) + `FormationTrack` + `AddStockDrawer` toggle (depends on T056, T057)
- [x] T062 [US3] Write Rust integration tests in `src-tauri/src/trains/infrastructure/train_formation_repo.rs` — 9 `formation_element_repo` scenarios from `quickstart.md` Phase F2 (append order, shift on remove, atomic reorder, mismatch rejection, duplicate prototype, assign/unassign, ON DELETE SET NULL, traction override values) (depends on T044–T046)

**Checkpoint (US3)**: Add/remove/reorder elements persists. DnD works. Drawer filters prototypes. `cargo test` passes T062 block.

---

## Phase 6: User Story 4 — Visualize Composition with Ownership Status (Priority: P4)

**Goal**: Track view renders horizontal cell grid with icons, badges, ownership indicator, sticky IdentityCard.

**Independent Test**: Open a formation with 5 units (3 owned, 2 unowned); verify all 5 cells render with SVG icon, railway badge, series code, service level, distinct owned/unowned visual state; IdentityCard stays fixed on horizontal scroll.

### Implementation

- [x] T063 [P] [US4] Update SQL query in `train_formation_repo.rs` to include `owned_count_for_prototype` subquery per `quickstart.md` Phase E1 — counts `owned_rolling_stocks` matching `prototype_id` per element
- [x] T064 [P] [US4] Create `src-tauri/src/trains/application/assign_rolling_stock_to_element.rs` — calls `formation.assign_rolling_stock()` or `unassign_rolling_stock()`, drains event; when assigning, fetches `Prototype.series_code` + `RailwayCompany.name` and persists them into `snapshot_series_code` / `snapshot_company_name` (FR-020 tombstone); when explicitly unassigning, clears snapshots (depends on T007, T016)
- [x] T065 [US4] Add `assign_rolling_stock_to_element` command to `command_handlers.rs` and register in `src-tauri/src/lib.rs` (depends on T064)
- [x] T066 [US4] Re-run `pnpm tauri dev` to update bindings; verify `FormationElementView.owned_count_for_prototype` and `AssignRollingStockToElementArgs` in `src/lib/bindings.ts`
- [x] T067 [P] [US4] Create `src/lib/features/train-formations/components/OwnershipBadge.svelte` — shows "N owned" / "Assigned" / "Planned" states; `$derived` label + variant per `quickstart.md` Phase E2 (depends on T066)
- [x] T068 [P] [US4] Create `src/lib/features/train-formations/components/AssignModelPicker.svelte` — modal/drawer to pick specific owned model from matching `owned_rolling_stocks`; calls `assignRollingStockToElement` (depends on T066)
- [x] T069 [US4] Update `src/lib/features/train-formations/components/FormationCell.svelte` to use `OwnershipBadge`, show railway company badge overlay (absolute top-left in `relative` wrapper), show "stock not found" indicator when `owned_rolling_stock_id` references a deleted model (depends on T067, T068, T055)
- [x] T070 [US4] Create `src/lib/features/train-formations/components/IdentityCard.svelte` — sticky left column: formation name, category, epoch badge; traction warning placeholder (fully connected in US5) (depends on T066)

**Checkpoint (US4)**: Cells show correct icons and ownership state. IdentityCard visible while scrolling. Owned/unowned cells visually distinct. Quick Assign works for `ownedCount === 1` slots.

---

## Phase 7: User Story 5 — Motorization Validation (Priority: P5)

**Goal**: "No Traction" warning appears when composition has zero effective traction slots; disappears reactively when a traction unit is added.

**Independent Test**: Formation with only Coaches → warning visible. Add non-dummy Locomotive → warning disappears immediately (<500ms, SC-003).

### Implementation

- [x] T071 [P] [US5] Create `src-tauri/src/trains/application/set_traction_override.rs` — calls `formation.set_traction_override()`, drains event (depends on T007, T016)
- [x] T072 [US5] Add `set_traction_override` command to `command_handlers.rs` and register; update bindings with `pnpm tauri dev` (depends on T071)
- [x] T073 [P] [US5] Create `src/lib/features/train-formations/components/TractionWarning.svelte` — shows amber/copper warning icon + Paraglide tooltip; hidden when `hasTraction=true` (depends on T054)
- [x] T074 [US5] Wire `hasTraction` into `TrainFormationState.svelte.ts` as a `$derived` value computed from `elements` array using `hasTraction()` from `traction.ts` (depends on T060, T054)
- [x] T075 [US5] Connect `TractionWarning` to `IdentityCard.svelte` via `hasTraction` prop from state; verify warning is reactive to composition changes without page reload (SC-003) (depends on T073, T074, T070)
- [x] T076 [US5] Add per-cell traction override toggle to `FormationCell.svelte` for dummy motorized units (FR-018); calls `setTractionOverride` command (depends on T072, T055)
- [x] T077 [US5] Write Rust domain unit tests — 8 traction evaluation scenarios from `quickstart.md` Phase F1 traction table (coach-only, Locomotive counts, PowerCar counts, default_is_dummy excluded, override force-include Coach, override force-exclude Loco, override exclude all, empty composition)

**Checkpoint (US5)**: Traction warning reactive. Override toggle works. All F1 traction tests pass (`cargo test`).

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Validation gate checks, test completeness, design consistency, and final integration.

- [ ] T078 [P] Apply `designer` skill to all Svelte components — FormationBuilder, FormationCell, IdentityCard use Iron/Copper palette; FormationCard / FormationList use Parchment/Brass; TractionWarning uses amber/copper glow per `quickstart.md` Design Notes
- [x] T079 [P] Write frontend Vitest tests in `src/__tests__/lib/features/train-formations/` — `domain/traction.test.ts` (11 named tests), `components/icons/PrototypeIcon.test.ts` (17 named tests), `components/OwnershipBadge.test.ts` (5), `components/FormationCell.test.ts` (7), `components/TractionWarning.test.ts` (3), `components/FormationForm.test.ts` (6), `components/AddStockDrawer.test.ts` (8), `TrainFormationState.test.ts` (7), `components/FormationList.test.ts` (4) — all per `quickstart.md` Phase F4; follow svelte-test-writer skill rules (`cleanup()`, `mockResolvedValue`, `waitFor`)
- [x] T080 [P] Write Rust integration tests for `train_formation_repo` (5 scenarios) and `prototype_repo` (5 scenarios) from `quickstart.md` Phase F2 not yet covered by T062
- [x] T081 [P] Write Rust use-case boundary tests (8 scenarios from `quickstart.md` Phase F3) in `src-tauri/src/trains/application/`
- [x] T082 Verify all i18n keys in `messages/en.json` + `messages/it.json` are complete — Italian translations required for all `formations_*` keys per `quickstart.md` Phase C4
- [x] T083 Run full validation sequence: `cargo fmt --check`, `cargo clippy -- -D warnings`, `pnpm format --check`, `pnpm lint`, `pnpm check`, `pnpm test`, `cargo test --manifest-path src-tauri/Cargo.toml` — all pass with zero errors/warnings (Gate G from `quickstart.md`)
- [ ] T084 Run `pnpm test:coverage` and verify domain/use-case coverage ≥80%, UI components ≥60% (Gate F from `quickstart.md`)

**Checkpoint (Done)**: 80 of 84 tasks complete. Remaining work: T034/T041 dedicated prototype repository extraction, T078 design polish, and T084 coverage verification.

---

## Dependency Graph (Story Completion Order)

```
Phase 1 (T001–T003)
  └─► Phase 2 (T004–T015)  ← BLOCKS everything
        ├─► US1 / Phase 3 (T016–T033)  ← MVP
        │     └─► US2 / Phase 4 (T034–T040)
        │           └─► US3 / Phase 5 (T041–T062)
        │                 └─► US4 / Phase 6 (T063–T070)
        │                       └─► US5 / Phase 7 (T071–T077)
        │                             └─► Phase 8 (T078–T084)
        └─► (US2, US3, US4 can begin as soon as Phase 2 + their
             direct predecessor US is done)
```

## Parallel Execution (within each phase)

- **Phase 2**: T005, T006, T008, T009, T010, T011, T015 can all run in parallel after T004
- **Phase 3 (US1)**: T017–T020 parallel after T016; T024–T030 parallel after T023
- **Phase 5 (US3)**: T042–T046 parallel after T041; T050–T052 parallel (no deps); T057–T059 parallel after T048
- **Phase 8**: T078–T082 fully parallel; T083 runs last after all prior tasks complete

## Implementation Strategy

**MVP scope**: Complete Phases 1–3 (T001–T033) first. This delivers a functional formation metadata CRUD with Tauri IPC, Svelte UI, navigation entry, and domain aggregate unit tests — independently demonstrable.

**Phase 4 (US2)** is a small add-on to Phase 3 (categories only).

**Phase 5 (US3)** is the largest phase — split across 2 sessions if needed: first complete the Rust backend (T041–T047), then the Svelte frontend (T048–T061).

**Phase 6–7 (US4–US5)** build on the composition rendering established in US3.
