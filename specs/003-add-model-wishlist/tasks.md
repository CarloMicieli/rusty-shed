# Tasks: Add Railway Model to Wishlist

**Input**: Design documents from `/specs/003-add-model-wishlist/`
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅, quickstart.md ✅

**Tests**: Component tests included for core functionality.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story (US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/lib/features/wishlists/`
- **Tests**: `src/__tests__/lib/features/wishlists/`
- **Messages**: `messages/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Foundation code needed by all user stories

- [ ] T001 Add English i18n message keys to messages/en.json (40+ keys from contracts/messages.md)
- [ ] T002 [P] Add Italian i18n message keys to messages/it.json (40+ keys from contracts/messages.md)
- [ ] T003 Run `pnpm prepare` to regenerate Paraglide types
- [ ] T004 [P] Create static dropdown constants in src/lib/features/wishlists/constants.ts (CATEGORIES, SCALES, POWER_METHODS, PRIORITIES arrays)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Service layer extension that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Add `addRailwayModelToWishlist` method to src/lib/features/wishlists/WishlistState.svelte.ts
- [ ] T006 Add type imports for `AddRailwayModelToWishListArgs` in WishlistState.svelte.ts
- [ ] T007 Verify service method compiles with `pnpm check`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Add Railway Model from Wishlists Overview (Priority: P1) 🎯 MVP

**Goal**: User can open drawer, fill form, and add railway model to any wishlist

**Independent Test**: Navigate to "My Wishlists", click "Add railway model", fill form, verify item appears in selected wishlist

### Implementation for User Story 1

- [ ] T008 [P] [US1] Create form type definitions in src/lib/features/wishlists/types.ts (AddRailwayModelFormState, RollingStockFormEntry)
- [ ] T009 [P] [US1] Create RollingStockEntry.svelte component in src/lib/features/wishlists/components/RollingStockEntry.svelte
- [ ] T010 [US1] Create AddRailwayModelDrawer.svelte component in src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte
- [ ] T011 [US1] Implement drawer structure with backdrop, close handlers, accessibility attributes in AddRailwayModelDrawer.svelte
- [ ] T012 [US1] Implement form fields (wishlist dropdown, manufacturer, product code, description, category, scale, power method, epoch) in AddRailwayModelDrawer.svelte
- [ ] T013 [US1] Implement wishlist item fields (desired price, priority, notes) in AddRailwayModelDrawer.svelte
- [ ] T014 [US1] Implement form validation with $derived in AddRailwayModelDrawer.svelte
- [ ] T015 [US1] Implement form submission handler calling wishlistService.addRailwayModelToWishlist() in AddRailwayModelDrawer.svelte
- [ ] T016 [US1] Implement dropdown data loading (manufacturers, railway companies) on drawer open in AddRailwayModelDrawer.svelte
- [ ] T017 [US1] Add drawer open/close state and handlers to src/lib/features/wishlists/WishlistsDashboard.svelte
- [ ] T018 [US1] Add "Add railway model" button to WishlistsDashboard.svelte header section
- [ ] T019 [US1] Import and render AddRailwayModelDrawer in WishlistsDashboard.svelte

### Tests for User Story 1

- [ ] T020 [P] [US1] Create test file src/**tests**/lib/features/wishlists/AddRailwayModelDrawer.test.ts
- [ ] T021 [US1] Add test: form validation prevents submission when required fields empty
- [ ] T022 [US1] Add test: form reset on drawer close

**Checkpoint**: User Story 1 complete - users can add railway models to wishlists from overview

---

## Phase 4: User Story 2 - Add Railway Model from Selected Wishlist Context (Priority: P2)

**Goal**: When a wishlist is selected in sidebar, the drawer pre-populates that wishlist

**Independent Test**: Select a wishlist, click "Add railway model", verify wishlist is pre-selected in dropdown

### Implementation for User Story 2

- [ ] T023 [US2] Add `preselectedWishlistId` prop to AddRailwayModelDrawer.svelte
- [ ] T024 [US2] Implement $effect to initialize wishlistId from preselectedWishlistId when drawer opens in AddRailwayModelDrawer.svelte
- [ ] T025 [US2] Pass `activeWishlistId` as preselectedWishlistId in WishlistsDashboard.svelte
- [ ] T026 [US2] Add `onAddModel` prop to src/lib/features/wishlists/components/WishlistHeader.svelte
- [ ] T027 [US2] Add "Add railway model" button to WishlistHeader.svelte (next to "Set as Default")
- [ ] T028 [US2] Wire WishlistHeader onAddModel to drawer open handler in WishlistsDashboard.svelte

### Tests for User Story 2

- [ ] T029 [US2] Add test: wishlist is pre-selected when preselectedWishlistId is provided

**Checkpoint**: User Story 2 complete - context-aware wishlist pre-selection works

---

## Phase 5: User Story 3 - Manage Rolling Stocks in Railway Model (Priority: P3)

**Goal**: User can add/remove multiple rolling stocks during railway model creation

**Independent Test**: Add multiple rolling stocks, remove some, verify final submission includes only remaining items

### Implementation for User Story 3

- [ ] T030 [US3] Implement rolling stocks section UI in AddRailwayModelDrawer.svelte
- [ ] T031 [US3] Implement addRollingStock function in AddRailwayModelDrawer.svelte
- [ ] T032 [US3] Implement removeRollingStock function in AddRailwayModelDrawer.svelte
- [ ] T033 [US3] Wire RollingStockEntry onChange and onRemove handlers in AddRailwayModelDrawer.svelte
- [ ] T034 [US3] Update form validation to validate rolling stock entries in AddRailwayModelDrawer.svelte
- [ ] T035 [US3] Map rolling stocks to SimplifiedRollingStockArgs in form submission

### Tests for User Story 3

- [ ] T036 [P] [US3] Add test: clicking "Add rolling stock" adds new entry
- [ ] T037 [P] [US3] Add test: clicking remove button removes entry
- [ ] T038 [US3] Add test: rolling stock validation blocks submission when fields incomplete

**Checkpoint**: User Story 3 complete - full rolling stock management works

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final verification and cleanup

- [ ] T039 Run `pnpm format` to format all files
- [ ] T040 Run `pnpm lint` and fix any issues
- [ ] T041 Run `pnpm check` and fix any type errors
- [ ] T042 Run `pnpm test` and verify all tests pass
- [ ] T043 Manual verification: test complete user flow in browser
- [ ] T044 [P] Export new components from src/lib/features/wishlists/index.ts (if applicable)
- [ ] T045 Update WishlistsDashboard.svelte styling to match CollectionDashboard.svelte layout (grid, spacing, header structure per FR-012)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on T003 (Paraglide regeneration)
- **User Story 1 (Phase 3)**: Depends on Phase 2 completion
- **User Story 2 (Phase 4)**: Depends on Phase 3 (T019 - drawer must exist)
- **User Story 3 (Phase 5)**: Can start after T010 (drawer component created)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### Within Each User Story

- Type definitions before components (T008 before T010)
- Sub-components before parent components (T009 before T010)
- Core implementation before integration
- Implementation before tests

### Parallel Opportunities

**Phase 1 parallelism:**

```bash
# These can run simultaneously:
T001 & T002  # EN and IT message keys
T004         # Constants file (after T003)
```

**Phase 3 parallelism:**

```bash
# These can run simultaneously:
T008 & T009  # Types and RollingStockEntry component
T020         # Test file setup (after T010)
```

**Phase 5 parallelism:**

```bash
# These can run simultaneously:
T036 & T037  # Independent test cases
```

---

## Implementation Strategy

### MVP Scope (Recommended First Delivery)

Complete **Phase 1 + Phase 2 + Phase 3** for a working MVP:

- Users can add railway models to wishlists
- All required fields work with validation
- Estimated time: 3-4 hours

### Incremental Additions

1. **Add Phase 4** (+30 min): Wishlist pre-selection for contextual UX
2. **Add Phase 5** (+1 hour): Full rolling stock management

### Verification Commands

```bash
# After each phase:
pnpm check          # Type checking
pnpm lint           # Linting
pnpm test           # Unit tests

# Final verification:
pnpm dev            # Manual testing
```

---

## Task Summary

| Phase             | Tasks  | Parallel Tasks | Estimated Time |
| ----------------- | ------ | -------------- | -------------- |
| Setup             | 4      | 2              | 30 min         |
| Foundational      | 3      | 0              | 20 min         |
| User Story 1 (P1) | 15     | 3              | 2 hours        |
| User Story 2 (P2) | 7      | 0              | 30 min         |
| User Story 3 (P3) | 9      | 2              | 45 min         |
| Polish            | 6      | 1              | 30 min         |
| **Total**         | **44** | **8**          | **4-5 hours**  |
