# Tasks: Rolling Stock List UX

**Input**: Design documents from `/specs/032-rolling-stock-list-ux/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Organization**: Tasks grouped by user story (US1–US4) to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on same-phase incomplete tasks)
- **[Story]**: Which user story (US1–US4 from spec.md)

---

## Phase 1: Setup (Baseline Verification)

**Purpose**: Confirm the working baseline before any changes.

- [x] T001 Verify baseline compiles cleanly: run `cargo check --manifest-path src-tauri/Cargo.toml` and `pnpm check` — fix any pre-existing issues before proceeding

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Surface `depot` on the owned rolling stock view type and regenerate the TypeScript bindings. Both US1 (depot row display) and US2 (depot inline edit) depend on this.

**⚠️ CRITICAL**: The depot-related tasks in US1 and US2 cannot begin until T004 is complete.

- [x] T002 Add `pub depot: Option<String>` field to `OwnedRollingStockView` struct in `src-tauri/src/collecting/domain/owned_rolling_stock_view.rs`
- [x] T003 Update the SQL SELECT query that builds `OwnedRollingStockView` — find the query by searching for `rolling_stocks.road_number` in `src-tauri/src/` and add `rolling_stocks.depot` to the same SELECT and the corresponding row-mapping
- [x] T004 Run `pnpm tauri dev` to rebuild the Tauri backend and regenerate `src/lib/bindings.ts` — confirm `depot: string | null` appears on the `OwnedRollingStockView` type

**Checkpoint**: `OwnedRollingStockView.depot` is in bindings.ts — user story work can begin.

---

## Phase 3: User Story 1 — Clean Empty Field Display (Priority: P1) 🎯 MVP

**Goal**: Every rolling stock field shows "—" for empty values in read-only (non-editable) mode. No italic placeholder text.

**Independent Test**: Open a Railway Model Card with rolling stock entries that have null Livery, Depot, and Company. Expand a card. Confirm: Livery row shows "—", Railway Company row shows "—", Control row shows "—", Depot row shows "—". No italic text visible anywhere in the card.

### Implementation for User Story 1

- [x] T005 [US1] Remove the `{#if editable || rollingStock.livery}` conditional guard from the Livery `<div>` in `src/lib/components/model-details/RollingStockCard.svelte` — the row must always render; static value uses `{localLivery || '—'}`
- [x] T006 [US1] Remove the `{#if editable || rollingStock.railwayCompanyName}` conditional guard from the Railway Company `<div>` in `src/lib/components/model-details/RollingStockCard.svelte` — always render; static value uses `{localRailwayCompanyName || '—'}`
- [x] T007 [US1] Remove the `{#if rollingStock.control}` conditional guard from the Control `<div>` in `src/lib/components/model-details/RollingStockCard.svelte` — always render; static value uses `{rollingStock.control ?? '—'}`
- [x] T008 [US1] Add `localDepot = $state('')` and sync it in the existing `$effect` from `rollingStock.depot ?? ''` in `src/lib/components/model-details/RollingStockCard.svelte`; then add a Depot `<div>` to the `<dl>` grid showing `{localDepot || '—'}` in static mode (editable wiring comes in US2)
- [x] T009 [US1] Update `src/lib/components/model-details/__tests__/RollingStockCard.test.ts` — add test cases asserting that Livery, Railway Company, Control, and Depot rows are rendered and show "—" when their values are null/empty

**Checkpoint**: Read-only rolling stock cards show "—" for all empty fields. No italic placeholder text.

---

## Phase 4: User Story 2 — Inline Field Editing (Priority: P1)

**Goal**: Clicking any field value (or "—") opens an inline input. Auto-saves on blur/Enter. Escape reverts. No Save/Cancel buttons visible.

**Independent Test**: Open a card in editable mode. Click the Livery "—" dash → input appears pre-filled empty. Type "Midnight Blue" → press Enter → field shows "Midnight Blue". Click Series value → change it → press Escape → value reverts. Click Depot "—" → type a depot name → click away → depot is saved. No Save/Cancel buttons visible anywhere on the card.

### Implementation for User Story 2

- [x] T010 [US2] Remove the floating Save/Cancel pill block from `src/lib/components/InPlaceEdit.svelte`: delete lines rendering the `<div class="absolute top-full ...">` pill container and its two `<button>` children, remove `let suppressBlurSave = false` declaration and both `onmousedown` handlers on the buttons, change error `<p>` margin class from `mt-7` to `mt-1`
- [x] T011 [US2] Fix `saveIdentificationField` in `src/lib/components/model-details/RollingStockCard.svelte` — the `depot` parameter is currently hard-coded to `null`; update the function signature to accept `'depot'` as a valid field and correctly pass `depot: field === 'depot' ? value || null : localDepot || null` in the `UpdateRollingStockIdentificationArgs`
- [x] T012 [US2] Add the Depot editable-mode `<InPlaceEdit>` row inside the `{#if editable}` branch in the Depot `<div>` added in T008 in `src/lib/components/model-details/RollingStockCard.svelte`, using `value={localDepot}` and `onSave={(v) => saveIdentificationField('depot', v)}`
- [x] T013 [US2] Update `src/lib/components/model-details/__tests__/RollingStockCard.test.ts` and any `InPlaceEdit` tests in `src/__tests__/` — remove assertions that check for Save/Cancel button elements; add assertions that the field value updates after a blur event

**Checkpoint**: All inline edits save on blur/Enter and revert on Escape. No Save/Cancel buttons exist in the rolling stock card UI.

---

## Phase 5: User Story 3 — Add Rolling Stock (Empty State) (Priority: P2)

**Goal**: When a Railway Model Card has zero rolling stock entries, a prominent CTA opens a Side Drawer creation form. Submitting creates the first entry.

**Independent Test**: Navigate to a model with no rolling stock. Confirm an "Add Rolling Stock" button/card is visible (not "No additional details"). Click it → Side Drawer slides in from the right with Category, Railway Company, Series Code, Road Number, Livery, Depot, Control fields. Fill in Series Code + select a Company + choose a Category → click Save → drawer closes → the new rolling stock entry appears in the list.

### Implementation for User Story 3 — Backend

- [x] T014 [P] [US3] Add `AddRollingStockToModelArgs` struct to `src-tauri/src/commands/catalogue_commands.rs` (or a co-located `args.rs` module) — must derive `Debug, Clone, serde::Serialize, serde::Deserialize, validator::Validate, specta::Type`; fields: `railway_model_id: String`, `railway_company_id: String`, `category: String`, `series_code: String` (validate length ≥ 1), `road_number: Option<String>`, `livery: Option<String>`, `depot: Option<String>`, `control: Option<String>`
- [x] T015 [US3] Create `src-tauri/src/catalog/application/add_rolling_stock_to_model.rs` — implement `AddRollingStockToModelInput` struct and `AddRollingStockToModelUseCase::execute()`: parse `railway_model_id` → load model from repo → parse `railway_company_id` → map `category` string to a `RollingStockParams` variant (use per-category defaults from data-model.md) → call `railway_model.add_rolling_stock(params)` → save aggregate → return the generated `RollingStockId`
- [x] T016 [US3] Add `#[tauri::command] #[specta::specta] pub async fn add_rolling_stock_to_model(state: tauri::State<'_, AppState>, args: AddRollingStockToModelArgs) -> Result<RollingStockId, CommandError>` to `src-tauri/src/commands/catalogue_commands.rs` — call `args.validate()`, map to `Input`, invoke use case
- [x] T017 [US3] Register `add_rolling_stock_to_model` in the Tauri specta builder (locate the existing command list in `src-tauri/src/lib.rs` or equivalent entry point and add the new command alongside the existing catalogue commands)
- [x] T018 [US3] Run `pnpm tauri dev` to regenerate `src/lib/bindings.ts` — confirm `addRollingStockToModel` and `AddRollingStockToModelArgs` appear in the generated file
- [x] T019 [US3] Add unit tests for the `AddRollingStockToModel` use case in `src-tauri/src/catalog/application/add_rolling_stock_to_model.rs` — at minimum: success path for each category, validation error when `series_code` is empty, not-found error when model ID is invalid

### Implementation for User Story 3 — Paraglide & Frontend

- [x] T020 [P] [US3] Add 6 Paraglide message keys to `messages/en.json` (and any other locale files that exist): `rolling_stock_create_drawer_title`, `rolling_stock_create_success`, `rolling_stock_create_error`, `rolling_stock_add_cta`, `rolling_stock_add_more`, `rolling_stock_field_category`; run `pnpm prepare` (or `pnpm build`) to compile message modules under `src/lib/paraglide/messages/`
- [x] T021 [US3] Create `src/lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte` — props: `open: boolean`, `railwayModelId: RailwayModelId`, `onCreated?: (id: RollingStockId) => void`, `onClose: () => void`; form fields in order: Railway Company (BadgePicker, loaded via `commands.getRailwayCompanies()`), Category (select using `RollingStockCategory` values), Series Code (text, required), Road Number (text, optional), Livery (text, optional), Depot (text, optional), Control Type (select, same `controlOptions` as `RollingStockSpecsDrawer`); on save: call `commands.addRollingStockToModel(...)`, show toast on success/failure, call `onCreated` and `onClose` on success; mirror the drawer shell styling from `RollingStockSpecsDrawer.svelte` (backdrop, Escape key, footer Save/Cancel, discard dialog if dirty)
- [x] T022 [US3] Update `src/lib/components/model-details/RollingStockList.svelte` — add props `onRollingStockAdded?: () => void` and `editable?: boolean` (already present); import `RollingStockCreateDrawer`; add `let createDrawerOpen = $state(false)`; when `editable` is true and `rollingStocks` is empty/undefined, replace the current plain dashed-border text box with an "Add Rolling Stock" CTA button (`m.rolling_stock_add_cta()`) that sets `createDrawerOpen = true`
- [x] T023 [US3] Mount `<RollingStockCreateDrawer>` at the bottom of `RollingStockList.svelte` (outside the conditional blocks) with `open={createDrawerOpen}`, `{railwayModelId}`, `onCreated={() => { onRollingStockAdded?.(); }}`, `onClose={() => { createDrawerOpen = false; }}`
- [x] T024 [US3] Update `src/lib/components/model-details/__tests__/RollingStockList.test.ts` — update empty-state tests so they assert the CTA button is present (not just the old text message) when `editable={true}`; add test asserting the old "No additional details" / plain message is shown when `editable` is false/absent

**Checkpoint**: When `editable={true}` and list is empty, the CTA button is visible. Clicking it opens the create drawer. A new entry can be created end-to-end.

---

## Phase 6: User Story 4 — Add More Rolling Stock (Priority: P2)

**Goal**: When rolling stock entries already exist, an "+ Add Rolling Stock" button is visible and opens the same drawer.

**Independent Test**: Navigate to a model with at least one rolling stock entry. Confirm the "+ Add Rolling Stock" button is visible. Click it → the same Side Drawer from US3 opens. Add a second entry → it appends to the list.

### Implementation for User Story 4

- [x] T025 [US4] Add an `{#if editable}` section below the `{#each}` list in `src/lib/components/model-details/RollingStockList.svelte` containing a secondary "+ Add Rolling Stock" button (`m.rolling_stock_add_more()`) that sets `createDrawerOpen = true` — this button appears only in the populated state
- [x] T026 [US4] Update `src/lib/components/model-details/__tests__/RollingStockList.test.ts` — add test asserting the "+ Add Rolling Stock" button is visible in the populated + editable state and absent in the non-editable state

**Checkpoint**: Both empty and populated states show their respective add CTAs. Both open the same drawer.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final lint, type, and test pass ensuring zero warnings/errors before merge.

- [x] T027 [P] Run full lint and type check: `pnpm lint && pnpm check` — fix all reported issues
- [x] T028 [P] Run Rust quality checks: `pnpm run rust:clippy && pnpm run rust:test` — fix all clippy warnings (treated as errors in CI)
- [x] T029 Run frontend tests: `pnpm test` — ensure all existing and new tests pass with zero failures
- [x] T030 Run `svelte-check` (included in `pnpm check`) and resolve any remaining TypeScript / Svelte type errors introduced by the new components

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup)
  └─► Phase 2 (Foundational: depot field + bindings)
        ├─► Phase 3 (US1: empty field display)
        ├─► Phase 4 (US2: inline editing — T010 can start from Phase 1, T011-T012 need Phase 2)
        └─► Phase 5 (US3: backend command + create drawer)
              └─► Phase 6 (US4: "Add More" button — drawer already available from US3)
                    └─► Phase 7 (Polish)
```

### User Story Dependencies

| Story           | Depends on                        | Notes                                                                             |
| --------------- | --------------------------------- | --------------------------------------------------------------------------------- |
| US1 (T005–T009) | Phase 2 complete                  | T005–T007 can start from Phase 1; T008–T009 need T004                             |
| US2 (T010–T013) | T002–T004 for depot tasks         | T010 can start after Phase 1                                                      |
| US3 (T014–T024) | Phase 2 complete                  | T014 and T020 are parallel; backend (T015–T018) and Paraglide (T020) are parallel |
| US4 (T025–T026) | T021–T023 (US3 drawer must exist) | Trivial addition once drawer is in place                                          |

### Parallel Opportunities

**Within Phase 5 (US3)**:

```
T014 AddRollingStockToModelArgs [P]     T020 Paraglide keys [P]
     ↓                                       ↓
T015 Use case                          (unblocked - different files)
     ↓
T016 Tauri command
     ↓
T017 Register command
     ↓
T018 Regenerate bindings ─────────────────────────────┐
                                                       ↓
                                             T021 RollingStockCreateDrawer
                                                       ↓
                                             T022 RollingStockList empty CTA
                                                       ↓
                                             T023 Mount drawer
                                                       ↓
                                             T024 Update tests
```

**Within Phase 7 (Polish)**: T027 and T028 can run in parallel (different tool chains).

---

## Implementation Strategy

### MVP First (US1 + US2 — Pure Frontend)

1. Complete Phase 1 (baseline)
2. Complete Phase 2 (depot binding)
3. Complete Phase 3 (US1: empty fields show "—") → **validate independently**
4. Complete Phase 4 (US2: no more Save/Cancel pill) → **validate independently**
5. **Stop and demo**: Cards are visually clean, inline editing is seamless

### Incremental Delivery

1. Phases 1–2: Foundation (depot binding)
2. Phase 3 (US1) → demo clean field display
3. Phase 4 (US2) → demo frictionless inline editing
4. Phases 5–6 (US3+US4) → demo full add workflow via drawer
5. Phase 7: Polish before merge

### Key Risk: Backend Command (T014–T018)

The new `add_rolling_stock_to_model` command is the only net-new backend code. The domain method (`RailwayModel::add_rolling_stock`) and the repository event handler (`RollingStockAdded`) already exist — only the application use case and transport layer are new. If the backend takes longer, US1 and US2 can be completed and shipped independently.

---

## Notes

- `[P]` tasks touch different files — no same-file conflicts
- `[Story]` label maps each task to an independently testable user story
- `saveIdentificationField` in `RollingStockCard.svelte` already has the `depot` parameter in `UpdateRollingStockIdentificationArgs` — it was just hard-coded to `null` (T011 corrects this)
- The `RollingStockSpecsDrawer` is **unchanged** — it continues to handle full technical spec editing after a rolling stock is created
- `pnpm tauri dev` must be run after **each** Rust type change to keep bindings in sync (T004 and T018)
- All user-facing strings must use Paraglide keys — never hardcoded strings in Svelte templates
