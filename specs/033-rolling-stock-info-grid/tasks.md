# Tasks: Rolling Stock Information Grid

**Input**: Design documents from `/specs/033-rolling-stock-info-grid/`
**Prerequisites**: plan.md ✅ · spec.md ✅ · research.md ✅ · data-model.md ✅ · contracts/tauri-commands.md ✅ · quickstart.md ✅

**Scope**: Frontend-only. No Rust backend changes required. All persistence uses existing Tauri commands.

**Tests**: Unit tests included for the new `InPlaceBooleanEdit` primitive (constitution Test-First Emphasis requirement).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story label (US1–US5)
- All paths relative to repo root

---

## Phase 1: Setup

**Purpose**: Audit existing code before making changes.

- [X] T001 Read and audit `src/lib/components/model-details/RollingStockCard.svelte` to document existing local state variables, save handlers, and current layout structure

**Checkpoint**: Existing component understood — ready to add new fields and restructure.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add Paraglide message keys required by all five user stories for field labels.

**⚠️ CRITICAL**: No user story work can begin until message keys exist — Paraglide compile will fail otherwise.

- [X] T002 Add 5 new keys to `messages/en.json`: `rolling_stock_field_interior_lights`, `rolling_stock_field_lights`, `rolling_stock_field_control_type`, `rolling_stock_field_length`, `rolling_stock_field_series`

**Checkpoint**: `pnpm run paraglide:compile` passes — all label keys resolvable.

---

## Phase 3: User Story 1 — Complete Field Grid Display (Priority: P1) 🎯 MVP

**Goal**: Replace the unstructured `<dl>` with a fixed 5-row × 3-column CSS grid showing all 15 rolling stock attributes as labelled read-only cells.

**Independent Test**: Open any rolling stock card. Verify all 15 fields appear in the correct grid positions with labels and static values (no editing interaction needed). Empty fields show an italicised placeholder rather than collapsing.

### Implementation for User Story 1

- [X] T003 [US1] Add local `$state` variables for 7 new fields (`localFlywheelFitted`, `localBodyShell`, `localChassis`, `localInteriorLights`, `localLights`, `localCouplingSocket`, `localCloseCouplers`, `localDigitalShunting`) in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T004 [US1] Extend the existing `$effect` / prop-sync block to extract all 7 new field values from `OwnedRollingStockView.technical_specifications` using the extraction mapping from `specs/033-rolling-stock-info-grid/data-model.md` in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T005 [US1] Replace the existing `<dl>` element with `<div class="grid grid-cols-3 gap-x-4 gap-y-3">` containing 15 labelled field cells (label above, value/placeholder below) following the Field Registry in `data-model.md` in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T006 [US1] Update the card header section to show Series Code and Road Number left-aligned and the Railway Company badge right-aligned in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T007 [US1] Render the 5 boolean fields (Flywheel Fitted, Interior Lights, Lights, Close Couplers, Digital Shunting) as static read-only chips: green "Yes" for YES, muted "No" for NO, italic "—" for null in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T008 [US1] Render the 3 enum-material fields (Body Shell, Chassis) and Coupling Socket as static text values, falling back to an italic "—" placeholder when null in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T009 [US1] Render italicised "Add [Field]" placeholder text in empty text/numeric cells (Depot, Livery, Road Number, Length) and add an empty spacer `<div>` for Row 4, Col 3 in `src/lib/components/model-details/RollingStockCard.svelte`

**Checkpoint**: User Story 1 fully functional — all 15 fields visible in correct grid positions; empty fields show placeholders; header layout correct.

---

## Phase 4: User Story 2 — Inline Editing for Text and Numeric Fields (Priority: P1)

**Goal**: Make Series Code, Road Number, Depot, Livery, and Length grid cells click-to-edit using existing `InPlaceEdit` primitive with automatic persistence.

**Independent Test**: Click the Depot placeholder. Verify an input appears. Type a value, click away, verify it is persisted and the grid cell shows the new value. Click Length, change value, press Enter, verify saved.

### Implementation for User Story 2

- [X] T010 [US2] Wire `InPlaceEdit` (text variant) for Series Code, Road Number, Depot, and Livery grid cells with `onSave` calling `commands.updateRollingStockIdentification` in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T011 [US2] Wire `InPlaceEdit` (number variant) for the Length grid cell with `onSave` calling `commands.updateRollingStockDcc` (passing `lengthMillimeters` and `lengthInches`) in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T012 [US2] Confirm saving state indicator (spinner or colour pulse) and Escape-to-revert behaviour are provided by the existing `InPlaceEdit` component — document any gap and patch if needed in `src/lib/components/InPlaceEdit.svelte`
- [X] T013 [US2] Add inline error display and local value revert on `Result.status === 'error'` for all text/numeric `onSave` handlers in `src/lib/components/model-details/RollingStockCard.svelte`

**Checkpoint**: User Story 2 fully functional — clicking any text/numeric field activates an input; blur/Enter saves; Escape reverts; errors shown inline.

---

## Phase 5: User Story 3 — Inline Editing for Enumerated Fields (Priority: P1)

**Goal**: Body Shell, Chassis, and Coupling Socket grid cells open a searchable dropdown when clicked and persist the selection immediately via `updateRollingStockSpecifications`.

**Independent Test**: Click the Body Shell field. Verify a dropdown lists `—`, `Plastic`, `Metal Die-Cast`. Select a different option. Verify the change is persisted without a Save button.

### Implementation for User Story 3

- [X] T014 [P] [US3] Define option set constants `BODY_SHELL_OPTIONS`, `CHASSIS_OPTIONS`, `COUPLING_SOCKET_OPTIONS` (as `const` arrays matching `data-model.md`) and the `featureFlagToBool` conversion helper in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T015 [US3] Implement `saveSpecField` async handler that builds the complete `UpdateRollingStockSpecificationsArgs` from current local state and calls `commands.updateRollingStockSpecifications` (call pattern from `contracts/tauri-commands.md`) in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T016 [US3] Wire `InPlaceSelectEdit` for Body Shell and Chassis grid cells using `BODY_SHELL_OPTIONS` / `CHASSIS_OPTIONS` and `onSave` → `saveSpecField` in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T017 [US3] Wire `InPlaceSelectEdit` for Coupling Socket grid cell using `COUPLING_SOCKET_OPTIONS` and `onSave` → `saveSpecField` in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T018 [US3] Add inline error indicator and local enum state revert on `saveSpecField` failure for Body Shell, Chassis, and Coupling Socket in `src/lib/components/model-details/RollingStockCard.svelte`

**Checkpoint**: User Story 3 fully functional — clicking any enum field opens a searchable dropdown; selecting an option persists immediately; Escape closes without saving; errors shown inline.

---

## Phase 6: User Story 4 — Inline Toggle for Boolean Fields (Priority: P2)

**Goal**: Create the `InPlaceBooleanEdit` primitive and wire it to all 5 boolean fields (Flywheel Fitted, Interior Lights, Lights, Close Couplers, Digital Shunting). A single click toggles state and persists immediately.

**Independent Test**: Find a card where Flywheel Fitted is unchecked. Click the toggle. Verify it changes to checked and the change is persisted. Click again — verify it reverts to unchecked and persists.

### Tests for User Story 4 (TDD — write before implementing the component)

- [X] T019 [P] [US4] Write Vitest unit tests for `InPlaceBooleanEdit` covering: null renders "—" chip; YES renders green "Yes" chip; NO renders muted "No" chip; clicking "Yes" button calls `onSave('YES')`; clicking "—" button calls `onSave(null)`; Escape key cancels without calling `onSave`; rejected `onSave` shows error and reverts at `src/lib/components/__tests__/InPlaceBooleanEdit.test.ts`

### Implementation for User Story 4

- [X] T020 [US4] Create `src/lib/components/InPlaceBooleanEdit.svelte` implementing the component contract from `data-model.md`: props `value: 'YES' | 'NO' | null`, `placeholder?: string`, `onSave: (v: 'YES' | 'NO' | null) => Promise<void>`; view/edit/saving/error state machine; 3-button inline picker (`—` / `Yes` / `No`); Escape cancels; buttons disabled during save
- [X] T021 [US4] Wire `InPlaceBooleanEdit` for Flywheel Fitted, Interior Lights, and Lights grid cells with `onSave` → `saveSpecField` (these fields pass as `string | null` directly) in `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T022 [US4] Wire `InPlaceBooleanEdit` for Close Couplers and Digital Shunting grid cells with `onSave` → `saveSpecField` (convert via `featureFlagToBool` since args type is `boolean | null`) in `src/lib/components/model-details/RollingStockCard.svelte`

**Checkpoint**: User Story 4 fully functional — all 5 boolean fields toggle on click; persist immediately; toggle is disabled during save; error reverts toggle; all unit tests pass.

---

## Phase 7: User Story 5 — Consistent Grid Across Single and Multiple Entry Views (Priority: P2)

**Goal**: The grid layout, field order, labels, and editing interactions are identical for each rolling stock entry regardless of how many entries appear on the card. Only one field is in edit mode at any time across all entries.

**Independent Test**: Open a card with one entry — note grid structure. Open a card with three entries — verify identical grid per entry. Edit a field on the second entry — verify only that field is in edit mode; the first and third entries remain read-only.

### Implementation for User Story 5

- [X] T023 [US5] Audit the parent component that renders multiple `RollingStockCard` instances to identify where shared `activeEditId` state should live in `src/lib/components/model-details/` (read relevant parent `.svelte` files)
- [X] T024 [US5] Implement a shared `activeEditId` reactive variable (Svelte 5 `$state`) in the parent component that owns all rolling stock entries on a card, and pass it down as props to each `RollingStockCard` instance in the relevant parent component under `src/lib/components/model-details/`
- [X] T025 [US5] Update `RollingStockCard.svelte` to accept `activeEditId` and `setActiveEditId` props; wire InPlace\* `onActivate`/`onDeactivate` callbacks (or equivalent) to enforce the single-active-field constraint across all entries in `src/lib/components/model-details/RollingStockCard.svelte`

**Checkpoint**: User Story 5 fully functional — grid layout identical per entry; editing one field on one entry locks all other fields across all entries to read-only display.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, formatting, and manual acceptance test.

- [X] T026 [P] Run `pnpm check` (svelte-check + tsc) and fix all TypeScript/Svelte type errors in modified files
- [X] T027 [P] Run `pnpm lint` and fix all ESLint/Prettier warnings in `src/lib/components/InPlaceBooleanEdit.svelte` and `src/lib/components/model-details/RollingStockCard.svelte`
- [X] T028 Run `pnpm test` and confirm all Vitest unit tests pass (including `InPlaceBooleanEdit.test.ts`)
- [X] T029 Validate manual acceptance checklist from `quickstart.md`: grid visible · empty placeholder · text edit · enum edit · boolean edit · Escape cancels · save indicator · error handling · multi-entry consistency

**Checkpoint**: Zero linting errors, zero type errors, all tests green, manual checklist passed — feature complete.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user stories**
- **US1 (Phase 3)**: Depends on Phase 2 — **BLOCKS US2, US3, US4, US5** (grid must exist before wiring edit interactions)
- **US2 (Phase 4)**: Depends on US1 (grid cells must exist)
- **US3 (Phase 5)**: Depends on US1; T015 (`saveSpecField`) should complete before T016/T017/T018
- **US4 (Phase 6)**: Depends on US1; T015 from US3 must be complete (shares `saveSpecField`); T019 (tests) written before T020 (implementation)
- **US5 (Phase 7)**: Depends on US1 — can proceed in parallel with US2/US3/US4 after US1 is done
- **Polish (Phase 8)**: Depends on all desired stories being complete

### User Story Dependencies

```
Phase 1 (Audit)
    └─▶ Phase 2 (Message Keys)
            └─▶ Phase 3 / US1 (Grid Display)  🎯 MVP gate
                    ├─▶ Phase 4 / US2 (Text/Numeric Edit)
                    ├─▶ Phase 5 / US3 (Enum Edit)
                    ├─▶ Phase 6 / US4 (Boolean Toggle)
                    └─▶ Phase 7 / US5 (Multi-Entry Consistency)
                                └─▶ Phase 8 (Polish)
```

### Parallel Opportunities Within Phases

- **Phase 3**: T005/T006 can proceed in parallel (layout vs header); T007/T008/T009 can proceed in parallel (different field groups)
- **Phase 4**: T010/T011 can proceed in parallel (different field types)
- **Phase 5**: T014 (constants) can run in parallel with T015 (handler) before T016/T017
- **Phase 6**: T019 (tests) runs first; T020 then T021/T022 in parallel
- **Phase 7**: T023 (audit) before T024/T025; US5 as a whole parallels US2/US3/US4
- **Phase 8**: T026/T027 in parallel; T028 after both; T029 last

---

## Parallel Example: User Story 1

```bash
# After T002 (message keys) completes:
Task A: T003 — Add local $state variables            (RollingStockCard.svelte)
Task B: T004 — Extract values from prop in $effect   (RollingStockCard.svelte)

# After T003 + T004:
Task C: T005 — Replace <dl> with CSS grid layout     (RollingStockCard.svelte)
Task D: T006 — Update header layout                  (RollingStockCard.svelte)

# After T005:
Task E: T007 — Render boolean static chips           (RollingStockCard.svelte)
Task F: T008 — Render enum static text values        (RollingStockCard.svelte)
Task G: T009 — Add italic placeholders               (RollingStockCard.svelte)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only — Grid Display)

1. Complete Phase 1: Audit
2. Complete Phase 2: Message keys
3. Complete Phase 3: US1 — grid display (all 15 fields visible, read-only)
4. **STOP and VALIDATE**: All fields visible in correct positions, placeholders work
5. Demo to user — collector can immediately see all attributes without the specs drawer

### Incremental Delivery

1. Phase 1 + 2 → Message keys in place
2. Phase 3 (US1) → Static 5×3 grid — **demo-ready MVP**
3. Phase 4 (US2) → Text/numeric fields become editable
4. Phase 5 (US3) → Enum fields become editable (Body Shell, Chassis, Coupling Socket)
5. Phase 6 (US4) → Boolean fields become toggleable (creates `InPlaceBooleanEdit`)
6. Phase 7 (US5) → Multi-entry consistency enforced
7. Phase 8 → Polish and final validation

### Suggested Single-Developer Sequence

```
T001 → T002 → T003 → T004 → T005 → T006 → T007 → T008 → T009
     → T010 → T011 → T012 → T013
     → T014 → T015 → T016 → T017 → T018
     → T019 → T020 → T021 → T022
     → T023 → T024 → T025
     → T026 → T027 → T028 → T029
```

---

## Notes

- `[P]` tasks touch different files or have no shared state — safe to run in parallel
- `InPlaceBooleanEdit` test (T019) must be written before the component (T020) — TDD per constitution
- `featureFlagToBool` helper (T014/T015) is shared by US3 and US4 — implement once in US3 phase
- No Rust changes required; do not modify any file in `src-tauri/`
- All new UI strings must use Paraglide keys — never hardcode visible strings
- Commit after each phase checkpoint using Conventional Commits format
