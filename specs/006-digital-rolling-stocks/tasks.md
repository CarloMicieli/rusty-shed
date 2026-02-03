# Tasks: Digital Rolling Stock Management

**Feature Branch**: `006-digital-rolling-stocks`  
**Input**: Design documents from `/specs/006-digital-rolling-stocks/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Project structure and shared configuration

- [x] T001 Add Paraglide messages for digital roster feature in `messages/en.json`
- [x] T002 [P] Add Paraglide messages for digital roster feature in `messages/it.json`
- [x] T003 Run `pnpm prepare` to regenerate Paraglide files

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Backend infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Backend View Extensions

- [x] T004 Extend `DigitalRollingStockView` with catalog fields (category, railway_company_name, scale, power_method, road_number, series_code, description) in `src-tauri/src/dcc_inventory/application/views.rs`
- [x] T005 Add `DigitalSummary` view struct in `src-tauri/src/dcc_inventory/application/views.rs`
- [x] T006 [P] Add `CheckDuplicateAddressResult` view struct in `src-tauri/src/dcc_inventory/application/views.rs`
- [x] T007 [P] Add `InstallableRollingStockView` view struct in `src-tauri/src/dcc_inventory/application/views.rs`

### Repository Extensions

- [x] T008 Add `get_digital_summary` method to `DigitalRollingStockRepository` trait in `src-tauri/src/dcc_inventory/domain/repositories.rs`
- [x] T009 [P] Add `check_address_exists` method to `DigitalRollingStockRepository` trait in `src-tauri/src/dcc_inventory/domain/repositories.rs`
- [x] T010 [P] Add `find_installable_rolling_stocks` method to `DigitalRollingStockRepository` trait in `src-tauri/src/dcc_inventory/domain/repositories.rs`
- [x] T011 Implement `get_digital_summary` in `src-tauri/src/dcc_inventory/infrastructure/sqlite_digital_rolling_stock_repository.rs`
- [x] T012 [P] Implement `check_address_exists` in `src-tauri/src/dcc_inventory/infrastructure/sqlite_digital_rolling_stock_repository.rs`
- [x] T013 [P] Implement `find_installable_rolling_stocks` in `src-tauri/src/dcc_inventory/infrastructure/sqlite_digital_rolling_stock_repository.rs`
- [x] T014 Extend `find_all_digital_rolling_stocks` query to JOIN catalog tables and exclude Function decoders in `src-tauri/src/dcc_inventory/infrastructure/sqlite_digital_rolling_stock_repository.rs`

### Unit Tests (Constitution: Test-First Emphasis)

- [x] T014a [P] Write unit test for `GetDigitalSummaryUseCase` verifying percentage calculation excludes dummies and counts factory-fitted
- [x] T014b [P] Write unit test for `CheckDuplicateAddressUseCase` verifying duplicate detection and exclude-self logic
- [x] T014c [P] Write unit test for `GetInstallableRollingStocksUseCase` verifying dummy exclusion and hasDecoder flag
- [x] T014d [P] Write integration test for enriched `find_all_digital_rolling_stocks` query in `src-tauri/src/dcc_inventory/infrastructure/sqlite_digital_rolling_stock_repository.rs`

### Use Cases

- [x] T015 Create `GetDigitalSummaryUseCase` in `src-tauri/src/dcc_inventory/application/get_digital_summary.rs`
- [x] T016 [P] Create `GetDecodersUseCase` in `src-tauri/src/dcc_inventory/application/get_decoders.rs`
- [x] T017 [P] Create `CheckDuplicateAddressUseCase` in `src-tauri/src/dcc_inventory/application/check_duplicate_address.rs`
- [x] T018 [P] Create `GetInstallableRollingStocksUseCase` in `src-tauri/src/dcc_inventory/application/get_installable_rolling_stocks.rs`
- [x] T019 Export new use cases in `src-tauri/src/dcc_inventory/application/mod.rs`

### Tauri Commands

- [x] T020 Add `CheckDccAddressDuplicateArgs` in `src-tauri/src/dcc_inventory/interface/command_args.rs`
- [x] T021 Add `get_digital_summary` command handler in `src-tauri/src/dcc_inventory/interface/command_handlers.rs`
- [x] T022 [P] Add `get_decoders` command handler in `src-tauri/src/dcc_inventory/interface/command_handlers.rs`
- [x] T023 [P] Add `check_dcc_address_duplicate` command handler in `src-tauri/src/dcc_inventory/interface/command_handlers.rs`
- [x] T024 [P] Add `get_installable_rolling_stocks` command handler in `src-tauri/src/dcc_inventory/interface/command_handlers.rs`
- [x] T025 Register new commands in `src-tauri/src/lib.rs` generate_handler macro
- [x] T026 Run `pnpm tauri dev` to regenerate TypeScript bindings

### Backend Verification

- [x] T027 Run `pnpm rust:check` to verify compilation
- [x] T028 Run `pnpm rust:clippy` to verify no warnings
- [x] T029 Run `pnpm rust:test` to verify existing tests pass

**Checkpoint**: Backend foundation ready. All new Tauri commands available in bindings.ts.

---

## Phase 3: User Story 1 & 2 - View Summary & Browse Roster (Priority: P1) 🎯 MVP

**Goal**: Display the digital roster page with summary statistics and browsable list

**Independent Test**: Navigate to `/my-digital-roster`, verify summary percentage displays correctly, verify list shows digital rolling stocks with correct columns

### Frontend Feature Module Setup

- [x] T030 [US1] Create feature directory structure `src/lib/features/digital-roster/components/`
- [x] T031 [US1] Create `DigitalRosterState.svelte.ts` context provider with Svelte 5 runes in `src/lib/features/digital-roster/`
- [x] T032 [US1] Create `DigitalRosterController.svelte.ts` controller class in `src/lib/features/digital-roster/`
- [x] T033 [US1] Create `index.ts` public exports in `src/lib/features/digital-roster/`

### Summary Component (US1)

- [x] T034 [US1] Create `DigitalSummary.svelte` component in `src/lib/features/digital-roster/components/`
- [x] T035 [US1] Implement summary loading from `getDigitalSummary` command
- [x] T036 [US1] Add empty state handling when no rolling stocks exist

### Roster Table Component (US2)

- [x] T037 [US2] Create `DigitalRosterTable.svelte` component in `src/lib/features/digital-roster/components/`
- [x] T038 [US2] Implement table with columns: DCC Address, Road Number, Category, Railway, Scale, Power Method
- [x] T039 [US2] Implement data loading from `getDigitalRollingStocks` command
- [x] T040 [US2] Add empty state handling when no digital rolling stocks exist
- [x] T041 [US2] Sort table by DCC address by default

### Route & Navigation

- [x] T042 [US1] Create route directory `src/routes/my-digital-roster/`
- [x] T043 [US1] Create `+page.svelte` in `src/routes/my-digital-roster/`
- [x] T044 [US1] Create `+page.server.ts` SSR stub in `src/routes/my-digital-roster/`
- [x] T045 [US1] Compose Summary and Table components in page
- [x] T046 [US1] Add navigation item to `src/lib/components/SidebarNavigation.svelte`
- [x] T047 [P] [US1] Add navigation item to `src/lib/components/BottomNavigation.svelte`

### P1 Verification

- [x] T048 Run `pnpm lint` to verify no linting errors
- [x] T049 Run `pnpm check` to verify TypeScript compilation
- [x] T050 Manual test: Navigate to page, verify summary and list display correctly

**Checkpoint**: MVP complete. Users can view digital roster summary and browse by DCC address.

---

## Phase 4: User Story 3 - Filter Digital Rolling Stock (Priority: P2)

**Goal**: Enable filtering the roster by DCC address or road number/description

**Independent Test**: Enter search text, verify list filters correctly, clear filter restores full list

### Implementation

- [x] T051 [US3] Add filter input field to `DigitalRosterTable.svelte`
- [x] T052 [US3] Implement client-side filtering logic (DCC address or road number match)
- [x] T053 [US3] Add debounced filter input with Svelte 5 reactivity
- [x] T054 [US3] Show empty state when filter matches no results
- [x] T055 [US3] Add clear filter button/functionality

**Checkpoint**: Users can search and filter their digital roster.

---

## Phase 5: User Story 4 - Change DCC Address (Priority: P2)

**Goal**: Allow users to change the DCC address of a digital rolling stock with duplicate warnings

**Independent Test**: Select a rolling stock, change its address, verify warning appears for duplicates, verify change persists

### Components

- [x] T056 [US4] Create `DccAddressEditor.svelte` modal/inline editor component in `src/lib/features/digital-roster/components/`
- [x] T057 [US4] Implement address input with validation (1-9999 range)
- [x] T058 [US4] Call `checkDccAddressDuplicate` on address change to show warning
- [x] T059 [US4] Display duplicate warning message (soft warning, allow save)
- [x] T060 [US4] Call `changeDccAddress` command on save
- [x] T061 [US4] Refresh roster table after successful address change

### Integration

- [x] T062 [US4] Add "Edit" action button to roster table rows
- [x] T063 [US4] Wire edit button to open DccAddressEditor

**Checkpoint**: Users can change DCC addresses with duplicate detection.

---

## Phase 6: User Story 5 - Install Decoder (Priority: P3)

**Goal**: Allow users to install a decoder on a rolling stock via right-sliding drawer

**Independent Test**: Click "Install Decoder", fill form, submit, verify new entry appears in roster

### Drawer Component

- [x] T064 [US5] Create `DecoderInstallDrawer.svelte` component in `src/lib/features/digital-roster/components/`
- [x] T065 [US5] Implement right-sliding drawer using Skeleton UI Drawer
- [x] T066 [US5] Add rolling stock dropdown (from `getInstallableRollingStocks`)
- [x] T067 [US5] Add decoder dropdown (from `getDecoders`)
- [x] T068 [US5] Add installation date picker with today as default
- [x] T069 [US5] Add DCC address input with validation
- [x] T070 [US5] Check for duplicate address and show warning
- [x] T071 [US5] Call `newDigitalRollingStock` command on submit
- [x] T072 [US5] Close drawer and refresh roster on success

### Integration

- [x] T073 [US5] Add "Install Decoder" button to page header
- [x] T074 [US5] Wire button to open DecoderInstallDrawer

**Checkpoint**: Users can install decoders on their rolling stocks.

---

## Phase 7: User Story 6 - Replace Existing Decoder (Priority: P3)

**Goal**: Prompt for confirmation when installing a decoder on a rolling stock that already has one

**Independent Test**: Select a rolling stock with existing decoder, attempt install, verify confirmation dialog appears

### Implementation

- [x] T075 [US6] Detect if selected rolling stock has existing decoder (from `InstallableRollingStockView.hasDecoder`)
- [x] T076 [US6] Show confirmation dialog when replacing existing decoder
- [x] T077 [US6] Call `changeDecoder` command for replacement (instead of new)
- [x] T078 [US6] Handle cancel action - return to form without changes

**Checkpoint**: Users can replace existing decoders with confirmation.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup and verification

- [x] T079 [P] Ensure all UI strings use Paraglide (no hardcoded text)
- [x] T080 [P] Add loading states to all async operations
- [x] T081 [P] Add error handling with toast notifications
- [x] T082 Run full verification: `pnpm rust:check && pnpm rust:clippy && pnpm rust:test`
- [x] T083 Run full verification: `pnpm lint && pnpm check && pnpm test`
- [ ] T084 Run quickstart.md validation checklist
- [ ] T085 Manual E2E test of all user stories

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup) ─────────────────────────────────────────────────────────┐
                                                                          │
Phase 2 (Foundational) ──────────────────────────────────────────────────┤
    │                                                                     │
    ├─► Phase 3 (US1 + US2: Summary & Browse) ─► MVP Milestone           │
    │        │                                                            │
    │        ├─► Phase 4 (US3: Filter) ──────────────────────────────────┤
    │        │                                                            │
    │        ├─► Phase 5 (US4: Change Address) ──────────────────────────┤
    │        │                                                            │
    │        ├─► Phase 6 (US5: Install Decoder) ─────────────────────────┤
    │        │        │                                                   │
    │        │        └─► Phase 7 (US6: Replace Decoder) ────────────────┤
    │        │                                                            │
    └────────┴─────────────────────────────────────────────────────────► Phase 8 (Polish)
```

### User Story Dependencies

| Story                | Priority | Depends On | Can Parallelize With |
| -------------------- | -------- | ---------- | -------------------- |
| US1 (Summary)        | P1       | Phase 2    | US2                  |
| US2 (Browse)         | P1       | Phase 2    | US1                  |
| US3 (Filter)         | P2       | US2        | US4                  |
| US4 (Change Address) | P2       | US2        | US3                  |
| US5 (Install)        | P3       | Phase 2    | -                    |
| US6 (Replace)        | P3       | US5        | -                    |

### Within Each Phase

- Tasks marked `[P]` can run in parallel
- Sequential tasks follow numeric order
- Verification tasks must run after all implementation tasks

### Parallel Opportunities

**Phase 2 (Backend)**:

```bash
# After T004-T007 complete, these can run in parallel:
T008 & T009 & T010  # Repository trait methods
T011 & T012 & T013  # Repository implementations
T015 & T016 & T017 & T018  # Use cases
T021 & T022 & T023 & T024  # Command handlers
```

**Phase 3 (Frontend MVP)**:

```bash
# After T030-T033 complete:
T034-T036 (Summary) & T037-T041 (Table)  # Can build in parallel
T046 & T047  # Navigation items in parallel
```

---

## Summary

| Phase   | Tasks     | Purpose                          |
| ------- | --------- | -------------------------------- |
| Phase 1 | T001-T003 | Setup (Paraglide messages)       |
| Phase 2 | T004-T029 | Backend foundation (30 tasks)    |
| Phase 3 | T030-T050 | MVP: Summary + Browse (21 tasks) |
| Phase 4 | T051-T055 | Filter (5 tasks)                 |
| Phase 5 | T056-T063 | Change Address (8 tasks)         |
| Phase 6 | T064-T074 | Install Decoder (11 tasks)       |
| Phase 7 | T075-T078 | Replace Decoder (4 tasks)        |
| Phase 8 | T079-T085 | Polish (7 tasks)                 |

**Total**: 89 tasks  
**MVP Scope**: Phases 1-3 (50 tasks) → Users can view summary and browse roster  
**Full Feature**: All phases (85 tasks) → Complete digital roster management
