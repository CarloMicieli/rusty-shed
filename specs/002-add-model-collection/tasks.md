# Tasks: Add Railway Model to Collection

**Input**: Design documents from `/specs/002-add-model-collection/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Unit tests included as requested in constitution (Test-First Emphasis principle).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/lib/` for components, `messages/` for i18n
- **Tests**: `src/__tests__/lib/` for Vitest unit tests
- All paths relative to repository root

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Paraglide message keys and state management extension

- [x] T001 Add English Paraglide message keys for form labels, validation, and UI text in messages/en.json (~40 keys from contracts/AddModelDrawer.contract.md)
- [x] T002 [P] Add Italian translations for all new message keys in messages/it.json
- [x] T003 Extend CollectionState with addRailwayModel method in src/lib/features/collection/CollectionState.svelte.ts

**Checkpoint**: Messages compile (`pnpm paraglide-js compile`), TypeScript passes (`pnpm check`)

---

## Phase 2: User Story 1 - Add Complete Railway Model (Priority: P1) 🎯 MVP

**Goal**: User can add a railway model with manufacturer, product code, description, category, scale, power method, epoch, and at least one rolling stock to their collection.

**Independent Test**: Navigate to My Collection → Click "Add railway model" → Fill form → Save → Model appears in collection with all entered data.

### Tests for User Story 1

- [x] T004 [P] [US1] Create form state initialization tests in src/**tests**/lib/features/collection/AddModelForm.test.ts
- [x] T005 [P] [US1] Create validation logic tests (required fields, rolling stock minimum) in src/**tests**/lib/features/collection/AddModelForm.test.ts
- [x] T006 [P] [US1] Create toAddRailwayModelArgs transformation tests in src/**tests**/lib/features/collection/AddModelForm.test.ts

### Implementation for User Story 1

- [x] T007 [P] [US1] Create RollingStockEntry component for dynamic rolling stock row in src/lib/features/collection/components/RollingStockEntry.svelte
- [x] T008 [US1] Create AddModelDrawer component with railway model form fields in src/lib/features/collection/components/AddModelDrawer.svelte
- [x] T009 [US1] Implement form state management with $state rune and validation with $derived in AddModelDrawer.svelte
- [x] T010 [US1] Implement reference data loading (manufacturers, railway companies) on drawer open in AddModelDrawer.svelte
- [x] T011 [US1] Implement rolling stock add/remove functionality in AddModelDrawer.svelte
- [x] T012 [US1] Implement form submission calling addRailwayModelToCollection command in AddModelDrawer.svelte
- [x] T013 [US1] Update CollectionDashboard to use new AddModelDrawer instead of ItemDrawer in src/lib/features/collection/CollectionDashboard.svelte

**Checkpoint**: User Story 1 fully functional - can add railway model with rolling stock to collection

---

## Phase 3: User Story 2 - Record Purchase Information (Priority: P2)

**Goal**: User can optionally record purchase details (seller, price, conditions, notes) when adding a model.

**Independent Test**: Add railway model → Fill purchase section with seller, price, conditions → Save → Purchase data stored with collection entry.

### Tests for User Story 2

- [ ] T014 [P] [US2] Create price parsing tests (decimal to cents conversion) in src/**tests**/lib/features/collection/AddModelForm.test.ts
- [ ] T015 [P] [US2] Create purchase section validation tests (optional fields, valid price format) in src/**tests**/lib/features/collection/AddModelForm.test.ts

### Implementation for User Story 2

- [ ] T016 [P] [US2] Create PurchaseSection component with collapsible UI in src/lib/features/collection/components/PurchaseSection.svelte
- [ ] T017 [US2] Integrate PurchaseSection into AddModelDrawer with seller dropdown loading in AddModelDrawer.svelte
- [ ] T018 [US2] Update toAddRailwayModelArgs to include purchase fields transformation in AddModelDrawer.svelte

**Checkpoint**: User Story 2 functional - purchase info section works independently

---

## Phase 4: User Story 3 - Manage Multiple Rolling Stocks (Priority: P3)

**Goal**: User can add multiple rolling stocks (3+), modify individual entries, and remove any entry dynamically.

**Independent Test**: Add railway model with 5 rolling stocks → Remove 2 → Add 1 more → Modify one entry → Save → All 4 rolling stocks saved correctly.

### Tests for User Story 3

- [ ] T019 [P] [US3] Create multi-rolling-stock management tests (add, remove, modify multiple) in src/**tests**/lib/features/collection/AddModelForm.test.ts

### Implementation for User Story 3

- [ ] T020 [US3] Enhance RollingStockEntry with per-entry validation error display in src/lib/features/collection/components/RollingStockEntry.svelte
- [ ] T021 [US3] Add visual feedback for rolling stock list (entry count, validation status) in AddModelDrawer.svelte
- [ ] T022 [US3] Implement prevent-remove-last-entry logic with user feedback in AddModelDrawer.svelte

**Checkpoint**: All 3 user stories independently functional

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T023 [P] Implement unsaved changes confirmation dialog when closing drawer in AddModelDrawer.svelte
- [ ] T024 [P] Add accessibility attributes (aria-labels, focus management, keyboard nav) in AddModelDrawer.svelte
- [ ] T025 [P] Add loading states and disabled button during submission in AddModelDrawer.svelte
- [ ] T026 Run verification scripts: pnpm format && pnpm lint && pnpm check && pnpm test
- [ ] T027 Manual testing: run full quickstart.md validation checklist

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 1 (Phase 2)**: Depends on Phase 1 completion - MVP deliverable
- **User Story 2 (Phase 3)**: Depends on Phase 2 (needs AddModelDrawer)
- **User Story 3 (Phase 4)**: Depends on Phase 2 (enhances existing components)
- **Polish (Phase 5)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Core MVP - no dependencies on other stories
- **User Story 2 (P2)**: Can start after US1 - integrates into AddModelDrawer
- **User Story 3 (P3)**: Can start after US1 - enhances RollingStockEntry

### Within Each User Story

- Tests written FIRST, verify they FAIL before implementation
- Sub-components before main component integration
- Core implementation before polish
- Story complete before moving to next priority

### Parallel Opportunities

**Phase 1** (all can run in parallel after T001):

```
T001 (en.json) → T002 (it.json) [P]
              → T003 (CollectionState)
```

**Phase 2 - User Story 1** (tests in parallel, then components):

```
T004 [P], T005 [P], T006 [P] - all tests in parallel
T007 (RollingStockEntry) [P] - can start while tests written
T008-T012 sequential (AddModelDrawer depends on T007)
T013 (integration) last
```

**Phase 3 - User Story 2** (tests in parallel):

```
T014 [P], T015 [P] - tests in parallel
T016 (PurchaseSection) [P] - can start with tests
T017-T018 sequential (integration)
```

**Phase 4 - User Story 3**:

```
T019 (tests)
T020-T022 sequential (enhancements)
```

**Phase 5** (all polish tasks in parallel):

```
T023 [P], T024 [P], T025 [P] - independent polish
T026, T027 - final verification
```

---

## Parallel Example: Phase 2 Tests

```bash
# Launch all User Story 1 tests together:
T004: "Form state initialization tests in src/__tests__/lib/features/collection/AddModelForm.test.ts"
T005: "Validation logic tests in src/__tests__/lib/features/collection/AddModelForm.test.ts"
T006: "toAddRailwayModelArgs transformation tests in src/__tests__/lib/features/collection/AddModelForm.test.ts"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: User Story 1 (T004-T013)
3. **STOP and VALIDATE**: Test User Story 1 independently
4. Deploy/demo if ready - users can add railway models to collection

### Incremental Delivery

1. Phase 1 → Foundation ready
2. Phase 2 → User Story 1 → MVP! Can add models with rolling stocks
3. Phase 3 → User Story 2 → Enhanced! Purchase info recording
4. Phase 4 → User Story 3 → Complete! Multi-rolling-stock management
5. Phase 5 → Polish → Production ready

### Single Developer Strategy

Execute phases sequentially in priority order:

1. Setup → US1 → US2 → US3 → Polish
2. Each checkpoint validates the increment works

---

## Summary

| Phase              | Tasks     | Parallelizable | Checkpoint                    |
| ------------------ | --------- | -------------- | ----------------------------- |
| Phase 1: Setup     | T001-T003 | T002           | Messages compile, TS passes   |
| Phase 2: US1 (MVP) | T004-T013 | T004-T007      | Add model to collection works |
| Phase 3: US2       | T014-T018 | T014-T016      | Purchase info section works   |
| Phase 4: US3       | T019-T022 | T019           | Multi-rolling-stock works     |
| Phase 5: Polish    | T023-T027 | T023-T025      | All tests pass, manual QA     |

**Total Tasks**: 27  
**User Story 1 (MVP)**: 13 tasks  
**User Story 2**: 5 tasks  
**User Story 3**: 4 tasks  
**Polish**: 5 tasks

**Suggested MVP Scope**: Complete through Phase 2 (T001-T013) for minimal viable feature.
