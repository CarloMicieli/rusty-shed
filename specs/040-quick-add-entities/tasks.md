# Tasks: On-the-Fly Entity Quick-Add

**Input**: Design documents from `/specs/040-quick-add-entities/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/ipc-commands.md, quickstart.md

**Tests**: Included because testing is explicitly required in spec.md User Scenarios and plan.md verification gates.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story label (`[US1]`, `[US2]`, `[US3]`, `[US4]`) for story-phase tasks only
- Every task includes explicit file path(s)

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish feature scaffolding and task/test structure before core implementation.

- [X] T001 Create quick-add feature scaffolding in `src/lib/features/quick-add/types.ts`, `src/lib/features/quick-add/QuickAddEntityForm.svelte`, and `src/lib/components/drawer/QuickAddShell.svelte`
- [X] T002 [P] Create quick-add validation scaffolding in `src/lib/schemas/quick-add-form.ts`
- [X] T003 [P] Create test scaffolding files `src/__tests__/quick-add/QuickAddEntityForm.test.ts`, `src/__tests__/acquisition/AcquisitionDrawer.test.ts`, `src/__tests__/collection/AddCollectionItemDrawer.test.ts`, and `src/__tests__/wishlists/AddWishlistItemDrawer.test.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend/frontend infrastructure that blocks all user stories.

**⚠️ CRITICAL**: No user story implementation starts before this phase completes.

- [X] T004 Add case-insensitive uniqueness migration with `CREATE UNIQUE INDEX IF NOT EXISTS` in `src-tauri/migrations/0007_quick_add_case_insensitive_indexes.sql`
- [X] T005 [P] Implement `CreateManufacturerArgs` and `create_manufacturer` command in `src-tauri/src/catalog/interface/manufacturers.rs`
- [X] T006 Register `create_manufacturer` command in `src-tauri/src/lib.rs`
- [X] T007 [P] Add manufacturer command tests (validation/conflict/success) in `src-tauri/src/catalog/interface/manufacturers.rs`
- [X] T008 Regenerate IPC bindings after Rust changes in `src/lib/bindings.ts` using `pnpm specta:generate`
- [X] T009 Implement shared quick-add form schema in `src/lib/schemas/quick-add-form.ts`
- [X] T010 [P] Implement stacked drawer shell with scrim and modal semantics in `src/lib/components/drawer/QuickAddShell.svelte`
- [X] T011 [P] Add `dimmed` support to parent shell in `src/lib/components/drawer/DrawerShell.svelte`
- [X] T012 Implement shared create-only quick-add form behavior in `src/lib/features/quick-add/QuickAddEntityForm.svelte`
- [X] T013 Add baseline quick-add translation keys in `messages/en.json` and `messages/it.json`

**Checkpoint**: Foundation complete. User stories can now be implemented and tested independently.

---

## Phase 3: User Story 1 - Add Manufacturer During Acquisition (Priority: P1) 🎯 MVP

**Goal**: Let users quick-add a manufacturer from Acquisition and auto-select it without losing entered acquisition data.

**Independent Test**: Open Acquisition drawer, quick-add a manufacturer via `+`, verify auto-selection and full parent form state persistence.

### Tests for User Story 1

- [ ] T014 [P] [US1] Add manufacturer quick-add success flow test in `src/__tests__/acquisition/AcquisitionDrawer.test.ts`
- [ ] T015 [P] [US1] Add manufacturer cancel/data-persistence test in `src/__tests__/acquisition/AcquisitionDrawer.test.ts`

### Implementation for User Story 1

- [X] T016 [US1] Add manufacturer quick-add trigger UI in `src/lib/features/acquisition/components/AcquisitionItemCard.svelte`
- [X] T017 [US1] Implement manufacturer quick-add handshake (append, select, close, toast) in `src/lib/features/acquisition/AcquisitionDrawer.svelte`
- [X] T018 [US1] Add manufacturer success/error localized messages in `messages/en.json` and `messages/it.json`
- [X] T019 [US1] Wire manufacturer duplicate-name source list and drawer open guard in `src/lib/features/acquisition/AcquisitionDrawer.svelte`

**Checkpoint**: US1 is independently functional and testable.

---

## Phase 4: User Story 2 - Add Seller/Buyer During Acquisition (Priority: P1)

**Goal**: Let users quick-add seller or buyer from Acquisition using seller-domain persistence and contextual UI labels.

**Independent Test**: Open Acquisition drawer, quick-add from Seller and Buyer fields, verify each is persisted and selected correctly.

### Tests for User Story 2

- [ ] T020 [P] [US2] Add seller quick-add flow test in `src/__tests__/acquisition/AcquisitionDrawer.test.ts`
- [ ] T021 [P] [US2] Add buyer quick-add flow test (seller-backed) in `src/__tests__/acquisition/AcquisitionDrawer.test.ts`

### Implementation for User Story 2

- [X] T022 [US2] Add seller/buyer quick-add trigger callbacks in `src/lib/features/acquisition/components/AcquisitionBatchFields.svelte`
- [X] T023 [US2] Implement seller/buyer quick-add orchestration and auto-selection in `src/lib/features/acquisition/AcquisitionDrawer.svelte`
- [X] T024 [US2] Implement seller quick-add payload defaults (`sellerType`) and error handling in `src/lib/features/quick-add/QuickAddEntityForm.svelte`
- [X] T025 [US2] Add seller/buyer localized strings and toasts in `messages/en.json` and `messages/it.json`

**Checkpoint**: US2 is independently functional and testable.

---

## Phase 5: User Story 3 - Add Manufacturer from Collection and Wishlist (Priority: P2)

**Goal**: Reuse the same quick-add manufacturer flow in Collection and Wishlist add forms.

**Independent Test**: Open Collection and Wishlist add drawers, quick-add manufacturer, verify auto-selection and preserved parent form state.

### Tests for User Story 3

- [X] T026 [P] [US3] Add collection manufacturer quick-add test in `src/__tests__/collection/AddCollectionItemDrawer.test.ts`
- [X] T027 [P] [US3] Add wishlist manufacturer quick-add test in `src/__tests__/wishlists/AddWishlistItemDrawer.test.ts`

### Implementation for User Story 3

- [X] T028 [US3] Wire manufacturer quick-add trigger and selection flow in `src/lib/features/collection/components/AddCollectionItemDrawer.svelte`
- [X] T029 [US3] Wire manufacturer quick-add trigger and selection flow in `src/lib/features/wishlists/AddWishlistItemDrawer.svelte`
- [X] T030 [US3] Enforce single active quick-add drawer per parent form session in `src/lib/features/collection/components/AddCollectionItemDrawer.svelte` and `src/lib/features/wishlists/AddWishlistItemDrawer.svelte`

**Checkpoint**: US3 is independently functional and testable.

---

## Phase 6: User Story 4 - Mobile Quick-Add Bottom-Sheet (Priority: P3)

**Goal**: Provide mobile bottom-sheet behavior with swipe-to-dismiss and keyboard-safe save action.

**Independent Test**: In mobile viewport, open quick-add, verify bottom-sheet transitions, swipe-to-dismiss behavior, and save visibility above keyboard.

### Tests for User Story 4

- [ ] T031 [P] [US4] Add mobile bottom-sheet render/animation test in `src/__tests__/quick-add/QuickAddEntityForm.test.ts`
- [ ] T032 [P] [US4] Add mobile keyboard-safe save visibility test in `src/__tests__/quick-add/QuickAddEntityForm.test.ts`
- [ ] T033 [P] [US4] Add dirty-form scrim dismiss confirmation test in `src/__tests__/quick-add/QuickAddEntityForm.test.ts`

### Implementation for User Story 4

- [X] T034 [US4] Implement mobile bottom-sheet and swipe-to-dismiss interactions in `src/lib/components/drawer/QuickAddShell.svelte`
- [X] T035 [US4] Implement keyboard-aware safe-area/visualViewport handling in `src/lib/components/drawer/QuickAddShell.svelte`
- [X] T036 [US4] Implement dirty-dismiss confirmation flow in `src/lib/features/quick-add/QuickAddEntityForm.svelte`

**Checkpoint**: US4 is independently functional and testable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, consistency checks, and documentation updates across all stories.

- [ ] T037 [P] Run frontend verification (`pnpm svelte-check`, `pnpm lint`, `pnpm test`) and resolve issues in `src/lib/features/quick-add/QuickAddEntityForm.svelte`, `src/lib/components/drawer/QuickAddShell.svelte`, and related test files under `src/__tests__/`
- [ ] T038 [P] Run backend verification (`cargo test`, `cargo clippy -- -D warnings`) and resolve issues in `src-tauri/src/catalog/interface/manufacturers.rs` and `src-tauri/src/lib.rs`
- [X] T039 [P] Regenerate and verify typed command bindings in `src/lib/bindings.ts` using `pnpm specta:generate`
- [ ] T040 Update implementation/runbook notes and validation outcomes in `specs/040-quick-add-entities/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2.
- **Phase 4 (US2)**: Depends on Phase 2; can run in parallel with US1 after foundational completion.
- **Phase 5 (US3)**: Depends on Phase 2; can run in parallel with US1/US2 after foundational completion.
- **Phase 6 (US4)**: Depends on Phase 2 and shared quick-add shell/form from foundational tasks.
- **Phase 7 (Polish)**: Depends on completion of all targeted user stories.

### User Story Dependencies

- **US1 (P1)**: Independent after foundational phase.
- **US2 (P1)**: Independent after foundational phase.
- **US3 (P2)**: Reuses shared quick-add components; independent after foundational phase.
- **US4 (P3)**: Builds on shared quick-add shell; independent validation in mobile viewport.

### Within Each User Story

- Tests first, then implementation.
- Shared schema/form/shell from foundational phase must not be bypassed.
- Handshake order on success is fixed: append local option → select new id → close quick-add → show toast.

---

## Parallel Opportunities

- **Setup**: T002 and T003 can run in parallel after T001.
- **Foundational**: T005, T007, T010, T011 can run in parallel once T004 starts and file ownership does not conflict.
- **US1**: T014 and T015 in parallel.
- **US2**: T020 and T021 in parallel.
- **US3**: T026 and T027 in parallel; T028 and T029 can run in parallel by different contributors.
- **US4**: T031, T032, and T033 in parallel.
- **Polish**: T037, T038, and T039 in parallel near feature completion.

## Parallel Example: User Story 1

```bash
# Tests in parallel
Task T014: src/__tests__/acquisition/AcquisitionDrawer.test.ts
Task T015: src/__tests__/acquisition/AcquisitionDrawer.test.ts

# Then implementation sequence
Task T016 -> Task T017 -> Task T018 -> Task T019
```

## Parallel Example: User Story 2

```bash
# Tests in parallel
Task T020: src/__tests__/acquisition/AcquisitionDrawer.test.ts
Task T021: src/__tests__/acquisition/AcquisitionDrawer.test.ts

# Implementation can split by file ownership
Task T022: src/lib/features/acquisition/components/AcquisitionBatchFields.svelte
Task T024: src/lib/features/quick-add/QuickAddEntityForm.svelte
```

## Parallel Example: User Story 3

```bash
# Tests in parallel
Task T026: src/__tests__/collection/AddCollectionItemDrawer.test.ts
Task T027: src/__tests__/wishlists/AddWishlistItemDrawer.test.ts

# Implementation in parallel
Task T028: src/lib/features/collection/components/AddCollectionItemDrawer.svelte
Task T029: src/lib/features/wishlists/AddWishlistItemDrawer.svelte
```

## Parallel Example: User Story 4

```bash
# Mobile-focused tests in parallel
Task T031: src/__tests__/quick-add/QuickAddEntityForm.test.ts
Task T032: src/__tests__/quick-add/QuickAddEntityForm.test.ts
Task T033: src/__tests__/quick-add/QuickAddEntityForm.test.ts

# Shell implementation
Task T034 -> Task T035 -> Task T036
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2.
2. Deliver US1 (Phase 3) end-to-end.
3. Validate US1 independently before expanding scope.

### Incremental Delivery

1. Foundation ready (Phases 1-2).
2. Deliver US1 and US2 (highest priority acquisition flow).
3. Deliver US3 (collection/wishlist reuse).
4. Deliver US4 (mobile refinement).
5. Run full polish/verification phase.

### Suggested MVP Scope

- **MVP**: US1 only (manufacturer quick-add inside acquisition with zero data loss and auto-selection).
- **Next increment**: US2 (seller/buyer in acquisition).

---

## Notes

- All tasks follow strict checklist format: checkbox, task ID, optional `[P]`, required story label in story phases, and explicit file path.
- No task depends on undefined external feature work; this task list executes Feature 040 end-to-end.
