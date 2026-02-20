# Tasks: Rolling Stock Progressive Editing

**Input**: Design documents from `/specs/024-rolling-stock-edit/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Included — required by constitution check (domain layer 80%+, commands 80%+, UI components 60%+) and explicitly enumerated per contract.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1, US2, US3, US4)
- Exact file paths included in every description

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the i18n message key infrastructure used by all four user stories.

- [x] T001 Add all 33 Paraglide i18n message keys from research.md §8 (edit*field_save, edit_field_cancel, edit_field_placeholder_empty, badge_picker_close, railway_model_field*_, rolling*stock_field*_, rolling*stock_edit_specs_button, specs_drawer*\*) to `messages/en.json` and run `pnpm prepare` to regenerate the compiled message module

---

## Phase 2: Foundational (Blocking Prerequisites for US2, US3, US4)

**Purpose**: Shared Rust domain event infrastructure required by every Rolling Stock mutation. US1 (model text editing) does not need this phase and can start independently, but US2–US4 are blocked until this phase is complete.

**⚠️ CRITICAL**: US2, US3, and US4 work cannot begin until this phase is complete

- [x] T002 Add `RollingStockUpdated { event_id: Uuid, railway_model_id: RailwayModelId, rolling_stock_id: RollingStockId, timestamp: NaiveDateTime, changed: serde_json::Value }` variant to the `RailwayModelEvent` enum in `src-tauri/src/catalog/domain/railway_model/railway_model_event.rs`
- [x] T003 [P] Add `apply_identification_patch(series_code, road_number, livery, depot) -> serde_json::Value`, `apply_railway_company(company_id: RailwayCompanyId) -> serde_json::Value`, and `apply_specifications(spec: RollingStockSpecPatch) -> serde_json::Value` mutation helper methods to all relevant variants of the `RollingStock` enum in `src-tauri/src/catalog/domain/railway_model/rolling_stock.rs` (depends on T002)
- [x] T004 [P] Add `update_rolling_stock_from_patch(&self, rolling_stock_id: &RollingStockId, changed: &serde_json::Value)` private SQL helper and add a `RollingStockUpdated` arm to the event `match` block in `save()` within `src-tauri/src/catalog/infrastructure/railway_model/sqlite_railway_model_repository.rs`; the helper must generate the full 14-column UPDATE SQL from data-model.md (depends on T002)

**Checkpoint**: Foundation ready — US2, US3, US4 can now begin. US1 can proceed independently from Phase 1.

---

## Phase 3: User Story 1 — In-Place Text Editing for Model Description and Details (Priority: P1) 🎯 MVP

**Goal**: Allow collectors to click directly on the description or details text of a Railway Model detail page, edit in place, and blur-to-save, with Escape-to-cancel and hover affordance.

**Independent Test**: Navigate to a Railway Model detail page → click description → edit text → click outside → navigate away → return → verify updated text persists. Repeat for details field.

**Note**: US1 does NOT require Phase 2 (Foundational) — `update_description` and `update_details` are existing domain methods.

### Tests for User Story 1

> Write these tests BEFORE implementing T007–T012 and verify they fail first

- [x] T005 [P] [US1] Write `#[cfg(test)]` unit tests in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` covering: `update_description` emits `RailwayModelUpdated` with `{"description": "new val"}`; calling `update_description("")` returns an error; `update_details(None)` emits `{"details": null}`
- [x] T006 [P] [US1] Create Vitest component test file `src/__tests__/components/InPlaceEdit.test.ts` covering: blur triggers `onSave` callback with new value; Escape key restores original value without calling `onSave`; failed `onSave` (rejected promise) shows error message and preserves unsaved value in editable state

### Implementation for User Story 1

- [x] T007 [P] [US1] Create `UpdateRailwayModelText` use case in `src-tauri/src/catalog/application/update_railway_model_text.rs` — load `RailwayModel` by id, match on `RailwayModelTextField` (Description → `model.update_description(value)`, Details → `model.update_details(if value.is_empty() { None } else { Some(value) })`), save aggregate
- [x] T008 [P] [US1] Create `InPlaceEdit.svelte` reusable click-to-edit primitive in `src/lib/components/InPlaceEdit.svelte` — implement the four visual states from plan.md (idle, hover with amber-muted affordance, active edit with amber border, floating Save/Cancel pill), props: `value: string`, `placeholder?: string`, `multiline?: boolean`, `onSave: (value: string) => Promise<void>`, use `$state` for `isEditing`, `editValue`, `isSaving`, `error`
- [x] T009 [US1] Add `RailwayModelTextField` enum and `UpdateRailwayModelTextArgs` struct with `#[derive(Debug, Clone, Deserialize, Validate, Type)]` and `update_railway_model_text` `#[tauri::command] #[specta::specta]` handler to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T007)
- [x] T010 [US1] Register `update_railway_model_text` in the `generate!()` macro and `invoke_handler!()` call in `src-tauri/src/lib.rs` (depends on T009)
- [x] T011 [US1] Run `cargo build --manifest-path src-tauri/Cargo.toml` to regenerate `src/lib/bindings.ts` with the new `updateRailwayModelText` command and `RailwayModelTextField` type (depends on T010)
- [x] T012 [US1] Update `src/lib/components/RailwayModelCard.svelte` — replace the static description and details text elements with `<InPlaceEdit>` components; wire `onSave` callbacks to `commands.updateRailwayModelText({ railwayModelId, field: 'Description' | 'Details', value })`; handle error result by allowing `InPlaceEdit` to remain editable (depends on T008 + T011)

**Checkpoint**: User Story 1 fully functional — collectors can edit model description and details in place. Verify SC-001 (change visible <3s after blur).

---

## Phase 4: User Story 2 — In-Place Text Editing for Rolling Stock Identification Fields (Priority: P2)

**Goal**: Allow collectors to click directly on Series Code, Road Number, Livery, or Depot on a rolling stock card and edit each field individually without opening the drawer, using the same interaction model as US1.

**Independent Test**: On a rolling stock listing → click Road Number on any card → edit value → click outside → verify updated value on card and persists after page refresh. Repeat for Series Code, Livery, and Depot.

**Depends on**: Phase 2 (Foundational) — requires `RollingStockUpdated` event and repository handler

### Tests for User Story 2

> Write these tests BEFORE implementing T015–T021 and verify they fail first

- [x] T013 [P] [US2] Write `#[cfg(test)]` unit tests in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` covering: `update_rolling_stock_identification` with `RoadNumber` emits `RollingStockUpdated` with `{"road_number": "123"}`; clearing road number (empty string) emits `{"road_number": null}`; empty `SeriesCode` returns `DomainError::Validation`
- [x] T014 [P] [US2] Create Vitest test file `src/__tests__/components/RollingStockCard.test.ts` covering: InPlaceEdit on series code field triggers save callback; after a successful card save, subsequent drawer open shows the updated value (FR-013 verification)

### Implementation for User Story 2

- [x] T015 [US2] Add `update_rolling_stock_identification(&mut self, rolling_stock_id: &RollingStockId, field: RollingStockIdentificationField, value: String) -> Result<(), DomainError>` method to `RailwayModel` in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` — locate rolling stock, validate SeriesCode non-empty, call the appropriate `apply_*` setter on the rolling stock, push `RollingStockUpdated` event (depends on T002)
- [x] T016 [US2] Create `UpdateRollingStockIdentification` use case in `src-tauri/src/catalog/application/update_rolling_stock_identification.rs` — load aggregate, locate rolling stock child, call `model.update_rolling_stock_identification(...)`, save aggregate (depends on T015)
- [x] T017 [US2] Add `RollingStockIdentificationField` enum and `UpdateRollingStockIdentificationArgs` struct with handler `update_rolling_stock_identification` to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T016)
- [x] T018 [US2] Register `update_rolling_stock_identification` in `generate!()` and `invoke_handler!()` in `src-tauri/src/lib.rs` (depends on T017)
- [x] T019 [US2] Run `cargo build --manifest-path src-tauri/Cargo.toml` to regenerate `src/lib/bindings.ts` with `updateRollingStockIdentification` and `RollingStockIdentificationField` types (depends on T018)
- [x] T020 [P] [US2] Create `RollingStockEditState.svelte.ts` class in `src/lib/features/rolling-stock-edit/RollingStockEditState.svelte.ts` tracking `activeField: string | null`, `pendingValue: string`, `isSaving: boolean`, `lastError: string | null` using `$state` runes (can be done in parallel with T015–T019 — different file)
- [x] T021 [US2] Update `src/lib/components/model-details/RollingStockCard.svelte` — add `<InPlaceEdit>` components for series_code, road_number, livery, and depot fields; wire `onSave` to `commands.updateRollingStockIdentification({ railwayModelId, rollingStockId, field, value })`; use `RollingStockEditState` for per-card state; apply the same hover affordance and error handling as the model description fields (depends on T019 + T020, reuses InPlaceEdit from T008)

**Checkpoint**: User Stories 1 AND 2 independently functional. Verify SC-008 (single identification field corrected <10s without opening drawer).

---

## Phase 5: User Story 3 — Constrained Selection for Scale, Era, and Railway Company (Priority: P3)

**Goal**: Allow collectors to click the Scale or Era badge on the Railway Model detail page, or the Railway Company value on a rolling stock card, to select a new value from a predefined picker without free-text input.

**Independent Test**: On a Railway Model detail page → click Scale badge → select different scale → verify badge updates immediately. On a rolling stock card → click Railway Company name → select different company → verify displayed value updates and persists.

**Depends on**: Phase 2 (Foundational) — `update_rolling_stock_railway_company` emits `RollingStockUpdated`

### Tests for User Story 3

> Write these tests BEFORE implementing T024–T033 and verify they fail first

- [x] T022 [P] [US3] Write `#[cfg(test)]` unit tests in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` covering: `update_scale(Scale::N)` emits `RailwayModelUpdated` with `{"scale": "N"}`; `update_epoch(Epoch::IV)` emits `{"epoch": "IV"}`; `update_rolling_stock_railway_company` emits `RollingStockUpdated` with `{"railway_company_id": "sncf"}`; invalid `railway_company_id` (not in DB) returns `DomainError::NotFound`
- [x] T023 [P] [US3] Create Vitest component test file `src/__tests__/components/BadgePicker.test.ts` covering: picker opens on trigger click; selecting an option calls `onSelect` with the selected id and closes the picker; pressing Escape closes the picker without calling `onSelect`; if `onSelect` rejects, the displayed value reverts to the original

### Implementation for User Story 3

- [x] T024 [US3] Add `update_scale(&mut self, scale: Scale)` and `update_epoch(&mut self, epoch: Epoch)` methods to `RailwayModel` in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` — each mutates the field and pushes `RailwayModelUpdated` with the minimal patch; follow the existing `update_description` pattern from quickstart.md
- [x] T025 [US3] Add `update_rolling_stock_railway_company(&mut self, rolling_stock_id: &RollingStockId, company_id: RailwayCompanyId) -> Result<(), DomainError>` method to `RailwayModel` in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` — locate rolling stock, call `apply_railway_company`, push `RollingStockUpdated` event (depends on T024 for file consistency; requires T002)
- [x] T026 [P] [US3] Create `UpdateRailwayModelClassification` use case in `src-tauri/src/catalog/application/update_railway_model_classification.rs` — load aggregate, apply `update_scale` if `scale.is_some()`, apply `update_epoch` if `epoch.is_some()`, validate at least one is provided, save aggregate (depends on T024)
- [x] T027 [P] [US3] Create `UpdateRollingStockRailwayCompany` use case in `src-tauri/src/catalog/application/update_rolling_stock_railway_company.rs` — verify `railway_company_id` exists in DB, load aggregate, locate rolling stock, call `model.update_rolling_stock_railway_company(...)`, save aggregate (depends on T025; different file from T026 — parallel)
- [x] T028 [US3] Add `UpdateRailwayModelClassificationArgs` struct and `update_railway_model_classification` handler, plus `UpdateRollingStockRailwayCompanyArgs` struct and `update_rolling_stock_railway_company` handler, to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T026 + T027)
- [x] T029 [US3] Register `update_railway_model_classification` and `update_rolling_stock_railway_company` in `generate!()` and `invoke_handler!()` in `src-tauri/src/lib.rs` (depends on T028)
- [x] T030 [US3] Run `cargo build --manifest-path src-tauri/Cargo.toml` to regenerate `src/lib/bindings.ts` with both new command types and `Scale`, `Epoch` union types (depends on T029)
- [x] T031 [P] [US3] Create `BadgePicker.svelte` reusable constrained-selection popover in `src/lib/components/BadgePicker.svelte` — props: `value: string`, `options: { id: string; label: string }[]`, `onSelect: (id: string) => Promise<void>`; implement the visual state contract from plan.md (picker panel `bg-[#0F0F0F]`, option hover amber-muted, selected option highlighted); handle ArrowDown/ArrowUp/Enter/Escape keyboard navigation; show micro-edit icon on trigger hover (can be done in parallel with T024–T030 — different file)
- [x] T032 [US3] Update `src/lib/components/RailwayModelCard.svelte` — wrap the Scale and Era badge elements with `<BadgePicker>` using Scale/Epoch enum values as options; wire `onSelect` to `commands.updateRailwayModelClassification({ railwayModelId, scale: selectedScale, epoch: null })` (and vice versa for era); revert displayed value and show toast on error (depends on T030 + T031)
- [x] T033 [US3] Update `src/lib/components/model-details/RollingStockCard.svelte` — add `<BadgePicker>` for the railway company field; load company list via `commands.getRailwayCompanies()` on mount; wire `onSelect` to `commands.updateRollingStockRailwayCompany({ railwayModelId, rollingStockId, railwayCompanyId })`; revert displayed value and show toast on error (depends on T030 + T031; same file as T021 — apply changes sequentially)

**Checkpoint**: User Stories 1, 2, AND 3 independently functional. Verify SC-002 (Scale/Era badge correction <5s; Railway Company picker <5s) and SC-004 (constrained pickers reject free-text).

---

## Phase 6: User Story 4 — Technical Specification Drawer for Rolling Stock Units (Priority: P4)

**Goal**: Allow collectors to click "Edit Specs" on any rolling stock card to open a structured 4-section side drawer (Identification, Technical, Control, Coupling) and save a full technical specification atomically.

**Independent Test**: On rolling stock listing → click "Edit Specs" on any card → fill in at least one field per section → click Save → verify values visible on card and persist after closing and reopening the drawer. Verify unsaved-changes dialog when closing with dirty form.

**Depends on**: Phase 2 (Foundational); reuses `InPlaceEdit` (US1 T008) and `BadgePicker` (US3 T031)

### Tests for User Story 4

> Write these tests BEFORE implementing T036–T042 and verify they fail first

- [x] T034 [P] [US4] Write `#[cfg(test)]` unit tests in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` covering: `update_rolling_stock_specifications` with full payload emits `RollingStockUpdated` with all 14 section fields in `changed`; empty `series_code` on drawer save returns `DomainError::Validation`; payload where all optional fields are `None` is accepted without error
- [x] T035 [P] [US4] Create Vitest component test file `src/__tests__/components/RollingStockSpecsDrawer.test.ts` covering: drawer populates all fields from mocked `getRailwayModelById` response; dirty-check triggers confirmation dialog when attempting to close with unsaved changes (FR-027); inline error is shown and drawer stays open when `updateRollingStockSpecifications` rejects with all entered values preserved (FR-028)

### Implementation for User Story 4

- [x] T036 [US4] Add `update_rolling_stock_specifications(&mut self, rolling_stock_id: &RollingStockId, spec: RollingStockSpecPatch) -> Result<(), DomainError>` method to `RailwayModel` in `src-tauri/src/catalog/domain/railway_model/railway_model.rs` — validate `series_code` non-empty, call `rs.apply_specifications(spec)` to get `changed` patch, push `RollingStockUpdated` event (depends on T002 + T003)
- [x] T037 [US4] Create `UpdateRollingStockSpecifications` use case in `src-tauri/src/catalog/application/update_rolling_stock_specifications.rs` — load aggregate, locate rolling stock, call `model.update_rolling_stock_specifications(...)`, save aggregate (depends on T036)
- [x] T038 [US4] Add `UpdateRollingStockSpecificationsArgs` struct (all fields from contract: railway_model_id, rolling_stock_id, series_code, road_number, livery, depot, flywheel_fitted, body_shell, chassis, interior_lights, lights, dcc_interface, control, coupling_socket, close_couplers, digital_shunting) and `update_rolling_stock_specifications` handler to `src-tauri/src/catalog/interface/command_handlers.rs` (depends on T037)
- [x] T039 [US4] Register `update_rolling_stock_specifications` in `generate!()` and `invoke_handler!()` in `src-tauri/src/lib.rs` (depends on T038)
- [x] T040 [US4] Run `cargo build --manifest-path src-tauri/Cargo.toml` to regenerate `src/lib/bindings.ts` with `updateRollingStockSpecifications` and `UpdateRollingStockSpecificationsArgs` types (depends on T039)
- [x] T041 [US4] Create `RollingStockSpecsDrawer.svelte` in `src/lib/features/rolling-stock-edit/components/RollingStockSpecsDrawer.svelte` — props: `open: boolean`, `railwayModelId: string`, `rollingStockId: string`, `onClose: () => void`; on open fetch via `commands.getRailwayModelById(railwayModelId)` and populate `$state` form; implement four sections (Identification: series_code/road_number/livery/depot; Technical: flywheel_fitted/body_shell/chassis/interior_lights/lights; Control: dcc_interface/control using shadcn-svelte `Select` with `bg-[#0F0F0F] border-[#1F1F1F]` overrides; Coupling: coupling_socket/close_couplers/digital_shunting); dirty-check via `$derived.by` JSON comparison; show unsaved-changes Dialog (`bg-black/90` backdrop) on dirty close attempt; wire Save to `commands.updateRollingStockSpecifications(...)`; show inline error on failure; backdrop `bg-black/80 backdrop-blur-sm` and panel `bg-[#0F0F0F] border-l border-[#1F1F1F]` per visual state contract (depends on T040; uses InPlaceEdit from T008 and BadgePicker from T031)
- [x] T042 [US4] Add "Edit Specs" button (`<Button variant="outline" size="sm" class="gap-1.5 border-[#1F1F1F] text-[#E0E0E0] hover:bg-[rgba(212,138,66,0.15)] hover:text-[#D48A42]">`) with `<PencilLine size={14} />` icon and `{m.rolling_stock_edit_specs_button()}` label to `src/lib/components/model-details/RollingStockCard.svelte`, opening `RollingStockSpecsDrawer` with the card's `railwayModelId` and `rollingStockId` props (depends on T041; same file as T021 and T033 — apply changes sequentially)

**Checkpoint**: All four User Stories fully functional. Verify SC-003 (drawer open → all 4 sections → save in <4 minutes), SC-005 (zero data loss on save failure), SC-007 (drawer works for empty, partial, and complete records).

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Code quality gates, type safety verification, and manual acceptance validation.

- [x] T043 [P] Run `pnpm format` and `pnpm lint` and fix all ESLint issues across all modified Svelte and TypeScript files
- [x] T044 [P] Run `pnpm check` and fix all TypeScript and Svelte type errors — confirm no `any` types introduced, all bindings typed correctly
- [x] T045 [P] Run `cargo fmt --manifest-path src-tauri/Cargo.toml` to format all modified Rust files
- [x] T046 [P] Run `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` and fix all warnings in new use cases, domain methods, and command handlers
- [x] T047 [P] Run `cargo test --manifest-path src-tauri/Cargo.toml` and verify all Rust unit and integration tests pass; run `pnpm test` and verify all Vitest component tests pass
- [ ] T048 Complete the full manual acceptance test checklist from `specs/024-rolling-stock-edit/quickstart.md` — verify all SC-001 through SC-008 success criteria and all FR-003/FR-012/FR-013/FR-027 acceptance scenarios pass

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1: Setup      ──────────────────────────────────────► Phase 3 (US1) can start
Phase 2: Foundational ──────────────────────────────────────► Phases 4, 5, 6 unblocked
Phase 3 (US1)        ─ InPlaceEdit.svelte ──────────────────► Reused in Phase 4 and Phase 6
Phase 4 (US2)        ─ No output reused by Phase 5 or 6
Phase 5 (US3)        ─ BadgePicker.svelte ────────────────────► Reused in Phase 6
Phase 6 (US4)        ─ Depends on Phase 2, 3, 5 outputs
Phase 7: Polish      ─ Depends on all phases complete
```

### User Story Dependencies

- **US1 (Phase 3)**: Depends only on Phase 1 (i18n). No dependency on Phase 2. Can start immediately.
- **US2 (Phase 4)**: Depends on Phase 2 (RollingStockUpdated event). Reuses `InPlaceEdit` from US1 but does not block on US1 (can stub if needed).
- **US3 (Phase 5)**: Depends on Phase 2 (for `update_rolling_stock_railway_company`). Independent of US1/US2.
- **US4 (Phase 6)**: Depends on Phase 2 + reuses `InPlaceEdit` (US1 T008) + `BadgePicker` (US3 T031).

### Critical Sequential Chains Within Each Story

```
US1: T007 → T009 → T010 → T011 → T012
US1 parallel: T008 (can start anytime before T012)
US2: T015 → T016 → T017 → T018 → T019 → T021
US2 parallel: T020 (can start anytime before T021)
US3: T024 → T025 → T026‖T027 → T028 → T029 → T030 → T032 → T033
US3 parallel: T031 (can start anytime before T032/T033)
US4: T036 → T037 → T038 → T039 → T040 → T041 → T042
```

### Parallel Opportunities Summary

| Story      | Parallelizable Pairs                                                                      |
| ---------- | ----------------------------------------------------------------------------------------- |
| Foundation | T003 ‖ T004 (after T002)                                                                  |
| US1        | T005 ‖ T006 (tests); T007 ‖ T008 (use case vs component)                                  |
| US2        | T013 ‖ T014 (tests); T020 ‖ T015–T019 (state class vs backend)                            |
| US3        | T022 ‖ T023 (tests); T026 ‖ T027 (two use cases); T031 ‖ T024–T030 (component vs backend) |
| US4        | T034 ‖ T035 (tests)                                                                       |
| Polish     | T043 ‖ T044 ‖ T045 ‖ T046 ‖ T047                                                          |

---

## Parallel Example: User Story 3

```bash
# After T024+T025 complete, launch use cases in parallel:
Task A: "Create update_railway_model_classification.rs use case"  (T026)
Task B: "Create update_rolling_stock_railway_company.rs use case" (T027)

# While the above run, launch in parallel:
Task C: "Create BadgePicker.svelte component" (T031)

# After T026+T027 complete: T028 → T029 → T030
# After T030+T031 complete: T032 then T033 (same file — sequential)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001)
2. Complete Phase 3: User Story 1 (T005–T012) — no Phase 2 needed
3. **STOP and VALIDATE**: Test US1 independently — description and details in-place edit working
4. Demo/validate SC-001 performance target

### Incremental Delivery

1. Phase 1 → Foundation ready → start US1 immediately
2. Phase 2 → Unblocks US2/US3/US4
3. US1 → Working model text editing → Demo (MVP!)
4. US2 → Working rolling stock identification editing → Demo
5. US3 → Working badge pickers for classification → Demo
6. US4 → Working technical drawer → Full feature complete
7. Phase 7 → Quality gates → Ship

### Parallel Team Strategy

With two developers:

- Developer A: Phase 2 (Foundational) → US2 → US4 (backend-heavy stories)
- Developer B: Phase 1 → US1 → US3 → Components (InPlaceEdit, BadgePicker)
- US4 waits for Developer B's components before starting the drawer frontend

---

## Notes

- [P] tasks = different files, no dependencies on incomplete tasks in the same phase
- [Story] label maps each task to its user story for traceability and independent delivery
- `cargo build` after each story's command registration is required to regenerate `src/lib/bindings.ts` — do not skip
- `RollingStockCard.svelte` is modified in US2 (T021), US3 (T033), and US4 (T042) — apply changes sequentially to avoid file conflicts
- `RailwayModelCard.svelte` is modified in US1 (T012) and US3 (T032) — apply sequentially
- `command_handlers.rs` and `lib.rs` are modified in each backend story — apply sequentially per phase
- `railway_model.rs` receives new methods in US2 (T015), US3 (T024+T025), and US4 (T036) — apply sequentially per phase
- All constrained selectors in the drawer (Control Type, DCC Interface) MUST use shadcn-svelte `Select` with charcoal overrides — never native `<select>` (see plan.md UI spec)
- All user-facing strings MUST use Paraglide message functions — never hardcode English text
