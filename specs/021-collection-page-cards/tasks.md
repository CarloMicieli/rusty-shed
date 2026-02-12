---
description: 'Implementation tasks for Collection Page Card Integration'
---

# Tasks: Collection Page Card Integration

**Input**: Design documents from `/specs/021-collection-page-cards/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Test tasks included as this is a critical UI integration feature requiring comprehensive testing.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/lib/` for components and utilities
- **Tests**: `tests/unit/` for unit tests
- **Feature**: `src/lib/features/collection/` for collection-specific code

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create utility module structure and prepare for implementation

- [x] T001 Create directory structure for card mapper utility at src/lib/features/collection/utils/
- [x] T002 [P] Create test directory structure at src/**tests**/lib/features/collection/
- [x] T003 [P] Review and understand existing RailwayModelPreviewCard component in src/lib/components/RailwayModelPreviewCard.svelte
- [x] T004 [P] Review and understand existing CollectionDashboard component in src/lib/features/collection/CollectionDashboard.svelte

**Checkpoint**: Directory structure ready, existing components understood

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core data transformation utilities that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Create cardDataMapper.ts module skeleton in src/lib/features/collection/utils/cardDataMapper.ts with function exports
- [x] T006 Implement extractPurchaseDate() helper function in src/lib/features/collection/utils/cardDataMapper.ts
- [x] T007 Implement mapCategory() function with category mapping logic in src/lib/features/collection/utils/cardDataMapper.ts
- [x] T008 Implement extractDigitalFeatures() function with Sound/DCC detection in src/lib/features/collection/utils/cardDataMapper.ts
- [x] T009 Implement collectionItemToCardData() main transformation function in src/lib/features/collection/utils/cardDataMapper.ts
- [x] T010 Create comprehensive unit test file at src/**tests**/lib/features/collection/cardDataMapper.test.ts
- [x] T011 [P] Write unit tests for extractPurchaseDate() covering all PurchaseInfo union types
- [x] T012 [P] Write unit tests for mapCategory() covering all Category enum values
- [x] T013 [P] Write unit tests for extractDigitalFeatures() covering Sound/DCC detection and edge cases
- [x] T014 [P] Write unit tests for collectionItemToCardData() covering all field mappings and null handling
- [x] T015 Run pnpm test to verify all cardDataMapper tests pass (30/30 passing)
- [x] T016 Run pnpm lint and pnpm check to verify code quality and type safety

**Checkpoint**: Foundation ready - cardDataMapper utility fully implemented and tested

---

## Phase 3: User Story 1 - View Collection with New Preview Cards (Priority: P1) 🎯 MVP

**Goal**: Users can browse their railway model collection displayed with the new RailwayModelPreviewCard components, providing richer visual presentation with thumbnails, metadata badges, and digital feature indicators.

**Independent Test**: Open the collection page and verify that models display with thumbnails (or category placeholders), metadata badges (scale, era), digital feature overlays (sound, DCC icons), and unit count indicators. All existing functionality (filtering, search, delete, navigation) continues to work.

### Implementation for User Story 1

- [x] T017 [US1] Import RailwayModelPreviewCard component in src/lib/features/collection/CollectionDashboard.svelte
- [x] T018 [US1] Import collectionItemToCardData from cardDataMapper utility in src/lib/features/collection/CollectionDashboard.svelte
- [x] T019 [US1] Replace ItemCard component with RailwayModelPreviewCard in grid rendering section of src/lib/features/collection/CollectionDashboard.svelte
- [x] T020 [US1] Transform CollectionItemView data using collectionItemToCardData() before passing to RailwayModelPreviewCard
- [x] T021 [US1] Update onDelete callback to pass item.id correctly to RailwayModelPreviewCard
- [x] T022 [US1] Update onclick callback to preserve existing navigation behavior (goto /models/[id])
- [x] T023 [US1] Verify existing click handler (handleCardClick) works with new component
- [x] T024 [US1] Verify existing delete handler (ui.requestDelete) works with new component
- [x] T025 [US1] Run pnpm lint to verify no linting errors in modified CollectionDashboard component
- [x] T026 [US1] Run pnpm check to verify TypeScript types are correct for new component integration
- [x] T027 [US1] Run pnpm test to ensure existing tests still pass (no regressions - 280/280 passing)

### Manual Testing for User Story 1

- [ ] T028 [US1] Test collection page loads and displays models with RailwayModelPreviewCard components
- [ ] T029 [US1] Verify thumbnails display for models with images, category placeholders for models without images
- [ ] T030 [US1] Verify metadata badges (scale, era, purchase date) display correctly on cards
- [ ] T031 [US1] Verify digital feature overlays (Sound speaker icon, DCC bolt icon) appear on models with those features
- [ ] T032 [US1] Verify unit count badge appears on multi-unit models (e.g., "×3")
- [ ] T033 [US1] Verify road numbers display and truncate correctly with expand/collapse toggle for long numbers
- [ ] T034 [US1] Test clicking a preview card navigates to /models/[id] detail page
- [ ] T035 [US1] Test delete button opens confirmation dialog (DeleteModal)
- [ ] T036 [US1] Test hover effects work smoothly on preview cards
- [ ] T037 [US1] Test responsive layout on mobile, tablet, and desktop viewports
- [ ] T038 [US1] Verify filter functionality still works with new cards (filter by scale, tags)
- [ ] T039 [US1] Verify search functionality still works with new cards
- [ ] T040 [US1] Verify add model functionality still works (AddModelDrawer)
- [ ] T041 [US1] Verify empty state displays when collection is empty
- [ ] T042 [US1] Verify no-results state displays when filters return no matches
- [ ] T043 [US1] Verify loading skeletons display during initial data fetch

**Checkpoint**: User Story 1 complete - Collection page displays new preview cards with all functionality working

---

## Phase 4: User Story 2 - View Detailed Model Information (Priority: P2)

**Goal**: Users can view comprehensive model details using the RailwayModelCard component when clicking a preview card, providing full specifications, rolling stock details, and image upload capabilities.

**Independent Test**: Click a model preview card in the collection, verify navigation to /models/[id] route, and confirm RailwayModelCard displays with full specifications, tabbed rolling stock information, and image upload functionality (for owned models).

### Implementation for User Story 2

- [x] T044 [US2] Verify existing /models/[...modelId]/+page.svelte route still works with new preview cards
- [x] T045 [US2] Test navigation from RailwayModelPreviewCard to model detail page (/models/[id])
- [x] T046 [US2] Verify RailwayModelCard displays full specifications (era, power method, category, description)
- [x] T047 [US2] Verify tabbed navigation works for multi-unit models (Details tab, Rolling Stock tab)
- [x] T048 [US2] Verify single-unit models display unified specifications without tabs
- [x] T049 [US2] Test image upload functionality (drag-and-drop and file browser) for models in collection
- [x] T050 [US2] Test image upload success updates the model image in both detail view and collection grid
- [x] T051 [US2] Verify rolling stock expandable rows work correctly in detail view
- [x] T052 [US2] Test back button navigation returns to collection page preserving scroll position

### Manual Testing for User Story 2

- [ ] T053 [US2] Test clicking multiple different model cards navigates to correct detail pages
- [ ] T054 [US2] Verify all model specifications display correctly in detail view
- [ ] T055 [US2] Test image upload with valid image file (JPEG, PNG, WebP)
- [ ] T056 [US2] Test image upload error handling (invalid format, file too large)
- [ ] T057 [US2] Test drag-and-drop image upload functionality
- [ ] T058 [US2] Verify image appears in detail view immediately after successful upload
- [ ] T059 [US2] Verify image appears in collection grid after returning from detail view
- [ ] T060 [US2] Test rolling stock tabs for multi-unit models
- [ ] T061 [US2] Test rolling stock expand/collapse for detailed specifications
- [ ] T062 [US2] Verify accessibility (keyboard navigation, screen reader compatibility)

**Checkpoint**: User Story 2 complete - Detail view integration verified with full functionality

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Code cleanup, documentation, and final quality checks

- [x] T063 [P] Deprecate ItemCard component with code comment marking it as deprecated in src/lib/features/collection/components/ItemCard.svelte
- [ ] T064 [P] Update CLAUDE.md if needed to document new cardDataMapper utility location
- [x] T065 [P] Add JSDoc comments to all exported functions in src/lib/features/collection/utils/cardDataMapper.ts
- [x] T066 Run pnpm format to ensure all code is properly formatted
- [x] T067 Run pnpm lint to ensure no linting errors remain (0 errors in feature files)
- [x] T068 Run pnpm check to verify all TypeScript types are correct (0 errors in feature files)
- [x] T069 Run pnpm test to ensure all tests pass (280/280 passing)
- [ ] T070 Run pnpm test:coverage and verify cardDataMapper utility has ≥90% coverage
- [ ] T071 Perform visual regression testing by comparing collection page screenshots before/after integration
- [ ] T072 Test performance with large collection (1000+ models) to ensure smooth rendering
- [ ] T073 Verify no console errors or warnings appear in browser dev tools
- [ ] T074 Test keyboard navigation and accessibility features (tab order, focus states, ARIA labels)
- [ ] T075 Run quickstart.md validation steps to ensure developer guide is accurate
- [ ] T076 Commit all changes with conventional commit message following project conventions
- [ ] T077 Prepare pull request description with screenshots and testing evidence

**Checkpoint**: All quality gates passed - Feature ready for code review and merge

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User Story 1 can start after Foundational phase
  - User Story 2 depends on User Story 1 completion (navigation flow)
- **Polish (Phase 5)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Depends on Foundational (Phase 2) - Implements core grid view integration
- **User Story 2 (P2)**: Depends on User Story 1 - Verifies navigation and detail view integration

### Within Each Phase

**Phase 2 (Foundational)**:

- T005 must complete before T006-T009 (module skeleton first)
- T006-T009 can run in parallel (different functions)
- T010 must complete before T011-T014 (test file first)
- T011-T014 can run in parallel (different test suites)
- T015-T016 must run after all tests written (verification)

**Phase 3 (User Story 1)**:

- T017-T018 can run in parallel (different imports)
- T019-T024 must run sequentially (component integration)
- T025-T027 must run after implementation (quality checks)
- T028-T043 manual tests can run in parallel (different test scenarios)

**Phase 4 (User Story 2)**:

- T044-T052 must run after User Story 1 (navigation depends on preview cards)
- T053-T062 manual tests can run in parallel (different test scenarios)

**Phase 5 (Polish)**:

- T063-T065 can run in parallel (different files)
- T066-T070 quality gates must run sequentially (dependencies on code changes)
- T071-T075 validation tasks can run in parallel (different validation aspects)
- T076-T077 must run last (final commit and PR)

### Parallel Opportunities

**Setup Phase (Phase 1)**:

- T002, T003, T004 can all run in parallel

**Foundational Phase (Phase 2)**:

- T006, T007, T008, T009 can run in parallel (different helper functions)
- T011, T012, T013, T014 can run in parallel (different test suites)

**User Story 1 Phase (Phase 3)**:

- T017, T018 can run in parallel (different imports)
- T028-T043 can all run in parallel (independent manual tests)

**User Story 2 Phase (Phase 4)**:

- T053-T062 can all run in parallel (independent manual tests)

**Polish Phase (Phase 5)**:

- T063, T064, T065 can run in parallel (different files)
- T071, T072, T073, T074 can run in parallel (different validation aspects)

---

## Parallel Example: Foundational Phase

```bash
# Launch all helper function implementations together:
Task: "Implement extractPurchaseDate() helper function in src/lib/features/collection/utils/cardDataMapper.ts"
Task: "Implement mapCategory() function in src/lib/features/collection/utils/cardDataMapper.ts"
Task: "Implement extractDigitalFeatures() function in src/lib/features/collection/utils/cardDataMapper.ts"

# Launch all test suites together after implementation:
Task: "Write unit tests for extractPurchaseDate() in tests/unit/features/collection/cardDataMapper.test.ts"
Task: "Write unit tests for mapCategory() in tests/unit/features/collection/cardDataMapper.test.ts"
Task: "Write unit tests for extractDigitalFeatures() in tests/unit/features/collection/cardDataMapper.test.ts"
```

## Parallel Example: User Story 1 Manual Tests

```bash
# Launch all manual testing scenarios together:
Task: "Test collection page loads and displays models"
Task: "Verify thumbnails display correctly"
Task: "Verify metadata badges display"
Task: "Verify digital feature overlays appear"
Task: "Verify unit count badge appears"
Task: "Test responsive layout on all viewports"
Task: "Test filter functionality"
Task: "Test search functionality"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (4 tasks)
2. Complete Phase 2: Foundational (12 tasks) - CRITICAL
3. Complete Phase 3: User Story 1 (27 tasks)
4. **STOP and VALIDATE**: Test User Story 1 independently - collection page should work with new preview cards
5. Optional: Deploy/demo User Story 1 before proceeding

**MVP Deliverable**: Collection page displays railway models with RailwayModelPreviewCard components showing thumbnails, metadata badges, digital features, and all existing functionality (filter, search, delete, navigation) working correctly.

### Incremental Delivery

1. **Foundation** (Phase 1 + 2): Setup + cardDataMapper utility → Foundation ready
2. **MVP** (Phase 3): Add User Story 1 → Test independently → Collection grid integration complete ✅
3. **Enhancement** (Phase 4): Add User Story 2 → Test independently → Detail view integration verified ✅
4. **Polish** (Phase 5): Code cleanup and quality checks → Feature production-ready ✅

Each phase adds value without breaking previous work. User Story 1 alone delivers significant UX improvement.

### Sequential Implementation (Recommended)

For single developer:

1. Complete Phase 1 (Setup) - ~30 minutes
2. Complete Phase 2 (Foundational) - ~4-6 hours (includes testing)
3. Complete Phase 3 (User Story 1) - ~4-6 hours (includes manual testing)
4. Complete Phase 4 (User Story 2) - ~2-3 hours (verification and testing)
5. Complete Phase 5 (Polish) - ~1-2 hours (cleanup and final checks)

**Total Estimated Time**: 12-18 hours for complete feature

### Parallel Team Strategy

With multiple developers (if applicable):

1. Team completes Phase 1 together (~30 minutes)
2. Team completes Phase 2 together (~4-6 hours) - CRITICAL BLOCKING PHASE
3. Once Foundational is done:
   - Developer A: User Story 1 implementation (T017-T024)
   - Developer B: User Story 1 testing (T028-T043)
4. Developer A completes User Story 2 while Developer B performs regression testing
5. Team completes Phase 5 together (polish and final quality checks)

---

## Task Summary

**Total Tasks**: 77

- **Setup (Phase 1)**: 4 tasks
- **Foundational (Phase 2)**: 12 tasks (BLOCKING)
- **User Story 1 (Phase 3)**: 27 tasks (MVP)
- **User Story 2 (Phase 4)**: 19 tasks (Enhancement)
- **Polish (Phase 5)**: 15 tasks (Quality & Cleanup)

**Parallel Opportunities**: 20 tasks marked [P] can run in parallel
**Critical Path**: Phase 2 → Phase 3 → Phase 4 → Phase 5

**MVP Scope**: Phase 1 + Phase 2 + Phase 3 = 43 tasks (56% of total)
**Estimated MVP Time**: 9-13 hours

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- All tests must pass before proceeding to next phase
- Commit after each logical group of tasks (end of each phase recommended)
- Stop at any checkpoint to validate progress independently
- Run quality gates (lint, check, test) frequently during implementation
- Follow project conventions from CLAUDE.md (commit messages, code style, etc.)
- Existing navigation pattern (goto /models/[id]) is preserved - no breaking changes
- ItemCard component can be deprecated but not deleted (may be used elsewhere)
- Focus on User Story 1 first - it delivers 80% of the value
