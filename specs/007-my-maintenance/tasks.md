# Tasks: My Maintenance Page

**Input**: Design documents from `/specs/007-my-maintenance/`
**Prerequisites**: plan.md (required), spec.md (required for user stories)

**Tests**: Not explicitly requested - implementation-only tasks.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/` (Rust - already implemented)
- **Frontend**: `src/` (Svelte 5, TypeScript)
- **Messages**: `messages/` (i18n JSON files)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and feature structure

- [x] T001 Create maintenance feature directory structure at `src/lib/features/maintenance/`
- [x] T002 [P] Create feature barrel export at `src/lib/features/maintenance/index.ts`
- [x] T003 [P] Create route directory at `src/routes/my-maintenance/`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core services that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T003a [Backend] Update `list_due_card_views` query in `src-tauri/src/maintenance/infrastructure/sqlite_repository.rs` to JOIN through owned_rolling_stocks → rolling_stocks → railway_models → manufacturers and populate manufacturer_name, product_code, series, and road_number fields ✅ DONE
- [x] T003b [Backend] Regenerate TypeScript bindings by running `pnpm tauri:codegen` to update `MaintenanceCardView` type in frontend ✅ DONE
- [x] T004 Create MaintenanceService class at `src/lib/features/maintenance/services/MaintenanceService.ts` wrapping Tauri commands (`getMaintenanceDashboard`, `addMaintenanceCard`, `addMaintenanceEvent`)
- [x] T005 Create MaintenanceState class at `src/lib/features/maintenance/MaintenanceState.svelte.ts` with reactive state for loading, error, and maintenance cards
- [x] T006 [P] Add i18n message keys for maintenance feature to `messages/en.json` (page title, labels, buttons, urgency indicators, empty states)
- [x] T007 [P] Add i18n message keys for maintenance feature to `messages/it.json` (Italian translations)
- [x] T008 [P] Create types file at `src/lib/features/maintenance/types.ts` for UI-specific types (urgency levels, form state)

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View Urgent Maintenance Overview (Priority: P1) 🎯 MVP

**Goal**: Display prioritized list of rolling stock requiring maintenance with visual urgency indicators

**Independent Test**: Navigate to `/my-maintenance` and see top 10 maintenance cards sorted by due date with urgency highlighting

### Implementation for User Story 1

- [x] T009 Create MaintenanceCardItem component at `src/lib/features/maintenance/components/MaintenanceCardItem.svelte` showing manufacturer, product code, series code, due date, and urgency styling
- [x] T010 Create getUrgencyLevel utility function at `src/lib/features/maintenance/utils/urgency.ts` to calculate urgency (overdue, warning, normal) from due date
- [x] T011 [P] [US1] Create MaintenanceCardList component at `src/lib/features/maintenance/components/MaintenanceCardList.svelte` rendering up to 10 cards sorted by due date
- [x] T012 [US1] Create page component at `src/routes/my-maintenance/+page.svelte` with quick action buttons layout and MaintenanceCardList integration
- [x] T013 [US1] Add empty state component at `src/lib/features/maintenance/components/EmptyMaintenanceState.svelte` for when no maintenance cards exist
- [x] T014 [US1] Integrate MaintenanceState context provider in page and load data on mount
- [x] T015 [US1] Add navigation link to My Maintenance page in `src/lib/components/SidebarNavigation.svelte`
- [ ] T016 [P] [US1] Add navigation link to My Maintenance page in `src/lib/components/BottomNavigation.svelte`

**Checkpoint**: At this point, User Story 1 should be fully functional - users can view their maintenance overview

---

## Phase 4: User Story 2 - Create Maintenance Card (Priority: P2)

**Goal**: Allow users to create maintenance cards for rolling stock items

**Independent Test**: Click "Add Maintenance Card" button, select rolling stock, submit form, see new card in list

### Implementation for User Story 2

- [x] T017 [US2] Create RollingStockSelector component at `src/lib/features/maintenance/components/RollingStockSelector.svelte` for selecting owned rolling stock without existing maintenance cards
- [x] T018 [US2] Create AddMaintenanceCardModal component at `src/lib/features/maintenance/components/AddMaintenanceCardModal.svelte` with rolling stock selection and form submission
- [x] T019 [US2] Add `createMaintenanceCard` method to MaintenanceState for handling form submission and state update
- [x] T020 [US2] Wire "Add Maintenance Card" quick action button in page to open AddMaintenanceCardModal
- [x] T021 [US2] Add success/error toast notifications for maintenance card creation

**Checkpoint**: At this point, User Stories 1 AND 2 are functional - users can view and create maintenance cards

---

## Phase 5: User Story 3 - Add Maintenance Event (Priority: P3)

**Goal**: Allow users to log maintenance events and update due dates

**Independent Test**: Click "Add Maintenance Event" button, select maintenance card, enter details, submit, see updated due date

### Implementation for User Story 3

- [x] T022 [US3] Create MaintenanceCardSelector component at `src/lib/features/maintenance/components/MaintenanceCardSelector.svelte` for selecting existing maintenance cards
- [x] T023 [US3] Create AddMaintenanceEventModal component at `src/lib/features/maintenance/components/AddMaintenanceEventModal.svelte` with card selection, date picker, maintenance type, and notes
- [x] T024 [US3] Add `addMaintenanceEvent` method to MaintenanceState for handling event submission and refreshing card list
- [x] T025 [US3] Wire "Add Maintenance Event" quick action button in page to open AddMaintenanceEventModal
- [x] T026 [US3] Add success/error toast notifications for maintenance event logging

**Checkpoint**: All user stories are now independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T027 Add loading skeleton states to MaintenanceCardList component
- [ ] T028 [P] Add error boundary and retry functionality to page component
- [ ] T029 [P] Update `src/lib/features/maintenance/README.md` with feature documentation
- [x] T030 Run `pnpm format` and `pnpm lint` to ensure code style compliance
- [x] T031 Run `pnpm check` to verify TypeScript types
- [x] T032 Run `pnpm test` to verify no regressions

---

## Dependencies

```text
Phase 1 (Setup)
    │
    ▼
Phase 2 (Foundation) ─────────────────────────────┐
    │                                              │
    ├─────────────────┬─────────────────┐          │
    ▼                 ▼                 ▼          │
Phase 3 (US1)    Phase 4 (US2)    Phase 5 (US3)   │
    │                 │                 │          │
    └─────────────────┴─────────────────┘          │
                      │                            │
                      ▼                            │
              Phase 6 (Polish) ◄───────────────────┘
```

### Task Dependencies Detail

| Task      | Depends On       | Notes                                 |
| --------- | ---------------- | ------------------------------------- |
| T004-T008 | T001-T003        | Foundation requires setup complete    |
| T009-T016 | T004, T005, T006 | US1 requires service, state, and i18n |
| T017-T021 | T012, T019       | US2 requires page and state method    |
| T022-T026 | T012, T024       | US3 requires page and state method    |
| T027-T032 | All US phases    | Polish requires all features complete |

## Parallel Execution Opportunities

### Phase 1 (all parallel after T001)

- T002, T003 can run in parallel

### Phase 2 (parallel group)

- T006, T007, T008 can run in parallel (after T004, T005)

### Phase 3 (partial parallel)

- T011, T016 can run in parallel with other US1 tasks
- T015, T016 can run in parallel (different files)

### Phase 6 (partial parallel)

- T028, T029 can run in parallel

## Implementation Strategy

1. **MVP (Phase 3)**: Deliver User Story 1 first - read-only maintenance overview delivers immediate value
2. **Increment 1 (Phase 4)**: Add maintenance card creation - enables tracking new items
3. **Increment 2 (Phase 5)**: Add maintenance event logging - completes the maintenance workflow
4. **Polish (Phase 6)**: Loading states, error handling, documentation

## Summary

| Metric                 | Count                          |
| ---------------------- | ------------------------------ |
| Total Tasks            | 32                             |
| Phase 1 (Setup)        | 3                              |
| Phase 2 (Foundation)   | 5                              |
| Phase 3 (US1 - MVP)    | 8                              |
| Phase 4 (US2)          | 5                              |
| Phase 5 (US3)          | 5                              |
| Phase 6 (Polish)       | 6                              |
| Parallel Opportunities | 10 tasks marked [P]            |
| Suggested MVP Scope    | Phases 1-3 (User Story 1 only) |
