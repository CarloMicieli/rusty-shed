# Tasks: Depot Page Redesign

**Input**: Design documents from `/specs/020-depot-redesign/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Tests are NOT explicitly requested in the specification. Manual testing checklist provided in quickstart.md.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Frontend**: `src/` (SvelteKit application)
- **Backend**: `src-tauri/src/` (Rust/Tauri backend)
- **Tests**: `src/__tests__/` (frontend), `src-tauri/src/` (backend unit tests)
- **Messages**: `messages/` (Paraglide i18n)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: No new project setup needed - existing Tauri application

**Note**: This project already exists. No setup tasks required.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure changes that MUST be complete before ANY user story can be fully implemented

**⚠️ CRITICAL**: No user story UI work can begin until this phase is complete

### Backend: Add Epoch Field to Depot View

- [x] T001 [P] Add `pub epoch: Option<Epoch>` field to DepotRollingStockView in src-tauri/src/collecting/domain/depot_view.rs
- [x] T002 [P] Add import for Epoch type in src-tauri/src/collecting/domain/depot_view.rs
- [x] T003 Update mapper to include epoch in src-tauri/src/collecting/infrastructure/mappers.rs (add `epoch: Some(collection_item.railway_model.epoch.clone())`)
- [x] T004 Rebuild backend to regenerate TypeScript bindings with `cd src-tauri && cargo build`
- [x] T005 Verify TypeScript bindings include epoch field in generated types

### Frontend: Add i18n Messages

- [x] T006 [P] Add `depot_railcars_and_emu_title` message to messages/en.json
- [x] T007 [P] Add `depot_passenger_cars_title` message to messages/en.json
- [x] T008 [P] Add `depot_freight_cars_title` message to messages/en.json
- [x] T009 [P] Add `depot_empty_railcars_and_emu` message to messages/en.json
- [x] T010 [P] Add `depot_empty_passenger_cars` message to messages/en.json
- [x] T011 [X] Add `depot_empty_freight_cars` message to messages/en.json
- [x] T012 [P] Add `depot_era` message to messages/en.json (for Era column header)
- [x] T013 Rebuild Paraglide messages with `pnpm run paraglide:compile`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Quick Search for Specific Model (Priority: P1) 🎯 MVP

**Goal**: Users can quickly find specific rolling stock by typing road number, manufacturer, or series with sub-200ms response time

**Independent Test**: Type "103" or "Roco" in search box → matching items appear across all categories within 200ms → clear search → all items reappear

### Implementation for User Story 1

**Note**: Search functionality already exists with 150ms debounce. This story verifies search works with new 4-category structure.

- [x] T014 [US1] Update filteredLocomotives derived in src/lib/features/depot/DepotState.svelte.ts (verify existing filter logic)
- [x] T015 [US1] Add filteredRailcarsEmuDmu derived (rename from filteredTrains) in src/lib/features/depot/DepotState.svelte.ts
- [x] T016 [US1] Add filteredPassengerCars derived in src/lib/features/depot/DepotState.svelte.ts
- [x] T017 [US1] Add filteredFreightCars derived in src/lib/features/depot/DepotState.svelte.ts
- [x] T018 [US1] Update totalFiltered calculation to sum all 4 categories in src/lib/features/depot/DepotState.svelte.ts
- [x] T019 [US1] Update return object to export new filtered category getters in src/lib/features/depot/DepotState.svelte.ts
- [x] T020 [US1] Update derived state variables in src/routes/my-depot/+page.svelte (add filteredRailcarsEmuDmu, filteredPassengerCars, filteredFreightCars)

**Manual Test for User Story 1**:

1. Open `/my-depot` route
2. Type partial road number (e.g., "103") → verify items filter across categories
3. Type manufacturer (e.g., "Roco") → verify items filter correctly
4. Clear search → verify all items reappear
5. Measure response time (should be under 200ms)

**Checkpoint**: Search functionality works with 4-category structure and meets performance target

---

## Phase 4: User Story 2 - Browse by Rolling Stock Type (Priority: P2)

**Goal**: Users can view rolling stock organized into 4 distinct categories (Locomotives, Railcars & EMU/DMU, Passenger Cars, Freight Cars) with collapsible sections and count badges

**Independent Test**: Open depot page → see 4 accordion sections with correct counts → expand/collapse each independently → verify sticky headers when scrolling

### Implementation for User Story 2

#### Backend: Category Logic (No changes needed - using existing enum)

**Note**: RollingStockCategory enum already supports new 4-category structure. No backend changes needed.

#### Frontend: Category Reorganization

- [x] T021 [P] [US2] Update locomotives derived getter in src/lib/features/depot/DepotState.svelte.ts (verify existing logic)
- [x] T022 [P] [US2] Add railcarsEmuDmu derived getter (rename from trains) in src/lib/features/depot/DepotState.svelte.ts
- [x] T023 [P] [US2] Add passengerCars derived getter (split from cars) in src/lib/features/depot/DepotState.svelte.ts
- [x] T024 [P] [US2] Add freightCars derived getter (split from cars) in src/lib/features/depot/DepotState.svelte.ts
- [x] T025 [US2] Update return object to export all 4 category getters in src/lib/features/depot/DepotState.svelte.ts

#### Frontend: Accordion UI Implementation

- [x] T026 [US2] Import Accordion component from shadcn-svelte in src/routes/my-depot/+page.svelte
- [x] T027 [US2] Import Users icon for passenger cars in src/routes/my-depot/+page.svelte
- [x] T028 [US2] Remove grid/table toggle buttons from PageHeader actions in src/routes/my-depot/+page.svelte
- [x] T029 [US2] Remove viewMode state and handleViewModeChange function in src/routes/my-depot/+page.svelte
- [x] T030 [US2] Replace grid/table conditional rendering with Accordion.Root component in src/routes/my-depot/+page.svelte
- [x] T031 [P] [US2] Create Locomotives accordion item with TrainFront icon and count badge in src/routes/my-depot/+page.svelte
- [x] T032 [P] [US2] Create Railcars & EMU/DMU accordion item with TramFront icon and count badge in src/routes/my-depot/+page.svelte
- [x] T033 [P] [US2] Create Passenger Cars accordion item with Users icon and count badge in src/routes/my-depot/+page.svelte
- [x] T034 [P] [US2] Create Freight Cars accordion item with Box icon and count badge in src/routes/my-depot/+page.svelte
- [x] T035 [US2] Add sticky header styling to all Accordion.Trigger components (sticky top-[var(--header-offset)] z-10 bg-surface-900/95 backdrop-blur-sm) in src/routes/my-depot/+page.svelte
- [x] T036 [US2] Update each Accordion.Content to render DepotTable instead of DepotSection in src/routes/my-depot/+page.svelte
- [x] T037 [US2] Add conditional rendering to hide empty categories (if filteredItems.length > 0) in src/routes/my-depot/+page.svelte

**Manual Test for User Story 2**:

1. Open `/my-depot` route
2. Verify 4 accordion sections appear (if items exist in each category)
3. Verify count badges show correct numbers
4. Expand/collapse each section independently
5. Scroll within expanded section → verify header stays visible (sticky)
6. Search to filter items → verify empty sections hide
7. Verify only owned items appear (no soft-deleted items)
8. Verify duplicates all appear (not filtered out)

**Checkpoint**: 4-category organization with accordion UI complete and independently testable

---

## Phase 5: User Story 3 - View Technical Details at a Glance (Priority: P3)

**Goal**: Users can view key technical details (Series, Road Number, Manufacturer, Product Code, DCC Address, Era, Livery) in a scannable table format

**Independent Test**: Expand any category → verify table shows all 7 required columns → verify Era column displays epoch values → verify visual hierarchy (primary vs secondary info)

### Implementation for User Story 3

#### Frontend: Add Era Column to Table

- [x] T038 [P] [US3] Add Era column header after Product Code in src/lib/features/depot/components/DepotTable.svelte
- [x] T039 [P] [US3] Add Era column data cell displaying item.epoch with null fallback in src/lib/features/depot/components/DepotTable.svelte
- [x] T040 [US3] Apply muted foreground styling to Era column (text-muted-foreground) in src/lib/features/depot/components/DepotTable.svelte

#### Frontend: Table Visual Hierarchy

- [x] T041 [P] [US3] Verify primary information (Series, Road Number) uses default text styling in src/lib/features/depot/components/DepotTable.svelte
- [x] T042 [P] [US3] Verify secondary information (Product Code, Era) uses muted styling in src/lib/features/depot/components/DepotTable.svelte
- [x] T043 [US3] Test table readability with 50+ items per category

**Manual Test for User Story 3**:

1. Expand each category section
2. Verify table displays all 7 columns: Series, Road Number, Manufacturer, Product Code, DCC Address, Era, Livery
3. Verify Era column shows epoch values (e.g., "IV", "III/IV") or "-" for null
4. Verify visual hierarchy: Series/Road Number prominent, Product Code/Era muted
5. Scroll through 50+ items → verify table remains scannable
6. Verify DCC Address column shows values for DCC-equipped models, blank for others

**Checkpoint**: Technical details table complete with Era column and visual hierarchy

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Cleanup, verification, and final polish

### Code Cleanup

- [X] T044 [P] Delete LocomotiveCard.svelte component (grid view removed) from src/lib/features/depot/components/
- [X] T045 [P] Delete TrainCard.svelte component (grid view removed) from src/lib/features/depot/components/
- [X] T046 [P] Delete CarCard.svelte component (grid view removed) from src/lib/features/depot/components/
- [X] T047 Review and update/remove DepotSection.svelte if only used for grid view in src/lib/features/depot/components/

### Verification & Quality Checks

- [X] T048 Run frontend linter with `pnpm lint` → verify no errors
- [X] T049 Run TypeScript type checking with `pnpm check` → verify no errors
- [X] T050 Run frontend formatter with `pnpm format`
- [X] T051 Run backend formatter with `cargo fmt` in src-tauri/
- [X] T052 Run backend linter with `cargo clippy` in src-tauri/ → verify no warnings
- [X] T053 Run backend tests with `cargo test` in src-tauri/ → verify all pass
- [ ] T054 Run manual testing checklist from quickstart.md

### Performance Validation

- [ ] T055 Test with 100+ rolling stock items → verify search debounce works (150ms)
- [ ] T056 Test with 500+ rolling stock items → verify UI remains responsive
- [ ] T057 Verify accordion animations are smooth when expanding/collapsing
- [ ] T058 Verify sticky headers perform well during scrolling

### Documentation Updates

- [ ] T059 Update CLAUDE.md if any new patterns or conventions emerged
- [ ] T060 Update memory files if any lessons learned

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: N/A - existing project, no setup needed
- **Phase 2 (Foundational)**: Can start immediately - BLOCKS all user stories until complete
- **Phase 3 (User Story 1 - P1)**: Depends on Phase 2 completion
- **Phase 4 (User Story 2 - P2)**: Depends on Phase 2 completion (can run parallel to US1 if staffed)
- **Phase 5 (User Story 3 - P3)**: Depends on Phase 2 completion (can run parallel to US1/US2 if staffed)
- **Phase 6 (Polish)**: Depends on all user stories being complete

### User Story Dependencies

- **User Story 1 (Search)**: Independent - only depends on Foundational phase
- **User Story 2 (Category Organization)**: Independent - only depends on Foundational phase
- **User Story 3 (Technical Details Table)**: Independent - only depends on Foundational phase

**Note**: All 3 user stories are truly independent and can be implemented in parallel after Foundational phase completes.

### Within Each User Story

**User Story 1** (Quick Search):

- DepotState updates must complete before page component updates
- All filtered getters can be added in parallel (T014-T017 marked [P])

**User Story 2** (Browse by Category):

- Category getters can be implemented in parallel (T021-T024 marked [P])
- Accordion items can be created in parallel (T031-T034 marked [P])
- Sticky styling and content updates are sequential

**User Story 3** (Technical Details):

- Era column header and data can be added in parallel (T038-T039 marked [P])
- Visual hierarchy verification can run in parallel (T041-T042 marked [P])

### Parallel Opportunities

**Foundational Phase**:

- All backend tasks (T001-T003) can run in parallel
- All i18n message additions (T006-T012) can run in parallel

**User Story 1**:

```bash
# Parallel: All filtered derivations
Task T014: filteredLocomotives
Task T015: filteredRailcarsEmuDmu
Task T016: filteredPassengerCars
Task T017: filteredFreightCars
```

**User Story 2**:

```bash
# Parallel: All category getters
Task T021: locomotives getter
Task T022: railcarsEmuDmu getter
Task T023: passengerCars getter
Task T024: freightCars getter

# Parallel: All accordion items
Task T031: Locomotives accordion
Task T032: Railcars & EMU accordion
Task T033: Passenger Cars accordion
Task T034: Freight Cars accordion
```

**User Story 3**:

```bash
# Parallel: Era column implementation
Task T038: Era header
Task T039: Era data cell
Task T040: Era styling

# Parallel: Visual hierarchy verification
Task T041: Primary info styling
Task T042: Secondary info styling
```

**Polish Phase**:

```bash
# Parallel: Component deletions
Task T044: Delete LocomotiveCard
Task T045: Delete TrainCard
Task T046: Delete CarCard
```

---

## Parallel Example: Foundational Phase

```bash
# Backend epoch field (can all run together):
- T001: Add epoch field to DepotRollingStockView
- T002: Add Epoch import
- (Then T003: Update mapper - depends on T001/T002)

# i18n messages (all independent):
- T006: Add depot_railcars_and_emu_title
- T007: Add depot_passenger_cars_title
- T008: Add depot_freight_cars_title
- T009: Add depot_empty_railcars_and_emu
- T010: Add depot_empty_passenger_cars
- T011: Add depot_empty_freight_cars
- T012: Add depot_era
```

---

## Parallel Example: User Story 2

```bash
# Category getters (all touch same file but different sections):
- T021: locomotives getter
- T022: railcarsEmuDmu getter
- T023: passengerCars getter
- T024: freightCars getter

# Accordion items (all touch same file but different sections):
- T031: Locomotives accordion with icon/badge
- T032: Railcars & EMU accordion with icon/badge
- T033: Passenger Cars accordion with icon/badge
- T034: Freight Cars accordion with icon/badge
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 2: Foundational (T001-T013) → **CRITICAL CHECKPOINT**
2. Complete Phase 3: User Story 1 (T014-T020) → Search works with 4 categories
3. **STOP and VALIDATE**: Test search functionality independently
4. Manual test: Search by road number, manufacturer, series
5. Verify 200ms response time
6. If satisfied, this is a minimal viable increment

### Incremental Delivery

1. **Foundation** → Epoch field added, i18n messages ready
2. **+ User Story 1** → Search works with new structure (MVP!)
   - Test: Search filtering across 4 categories
   - Deploy/Demo: "Search is faster and works across reorganized categories"

3. **+ User Story 2** → Category organization with accordion
   - Test: Expand/collapse, count badges, sticky headers
   - Deploy/Demo: "Browse by category with collapsible sections"

4. **+ User Story 3** → Technical details table
   - Test: All 7 columns visible, Era column populated
   - Deploy/Demo: "Complete depot view with era information"

5. **+ Polish** → Cleanup and optimization
   - Full feature complete

### Parallel Team Strategy

With multiple developers:

1. **Together**: Complete Foundational phase (T001-T013)
2. **Once Foundational is done**:
   - Developer A: User Story 1 (T014-T020) - Search
   - Developer B: User Story 2 (T021-T037) - Categories & Accordion
   - Developer C: User Story 3 (T038-T043) - Table & Era column
3. **Integration**: Merge all 3 stories → Full feature complete
4. **Together**: Polish phase (T044-T060)

### Sequential Solo Strategy

Working alone, prioritize by user value:

1. Foundational (T001-T013) - ~1-2 hours
2. User Story 1 (T014-T020) - ~30 minutes (search already works)
3. User Story 2 (T021-T037) - ~2-3 hours (main UI changes)
4. User Story 3 (T038-T043) - ~30 minutes (just add column)
5. Polish (T044-T060) - ~1 hour

**Total estimated time**: 5-7 hours for full implementation

---

## Task Statistics

- **Total Tasks**: 60
- **Foundational**: 13 tasks (blocks all user stories)
- **User Story 1 (P1)**: 7 tasks (search functionality)
- **User Story 2 (P2)**: 17 tasks (category organization)
- **User Story 3 (P3)**: 6 tasks (technical details table)
- **Polish**: 17 tasks (cleanup and verification)

**Parallel Tasks Identified**: 21 tasks marked with [P]

**Independent Test Criteria**:

- US1: Search works across 4 categories with <200ms response
- US2: 4 accordion sections with counts, sticky headers, expand/collapse
- US3: Table shows 7 columns including Era, visual hierarchy clear

**Suggested MVP Scope**: Foundational + User Story 1 (20 tasks, ~2-3 hours)

---

## Notes

- **[P] tasks** = Different files or independent sections, can run in parallel
- **[Story] label** = Maps task to specific user story for traceability (US1, US2, US3)
- **Each user story is independently testable** per specification requirements
- **No new tests requested** in spec - manual testing checklist in quickstart.md
- **Commit strategy**: Commit after each task or logical group (e.g., all i18n messages)
- **Checkpoints**: Stop after each user story phase to validate independently
- **Avoid**: Implementing all 3 stories at once without testing each independently

**Format Validation**: ✅ All 60 tasks follow checklist format: `- [ ] [ID] [P?] [Story?] Description with file path`
