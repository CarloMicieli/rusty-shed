# Tasks: Dashboard Redesign

**Input**: Design documents from `/specs/009-dashboard-redesign/`  
**Prerequisites**: plan.md ✅, spec.md ✅

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Add required translation keys and create shared components

- [ ] T001 [P] Add new translation keys for dashboard redesign to messages/en.json
- [ ] T002 [P] Add new translation keys for dashboard redesign to messages/it.json
- [ ] T003 [P] Create PageHeader component in src/lib/components/PageHeader.svelte
- [ ] T004 [P] Create StatusBadge component in src/lib/components/StatusBadge.svelte

**Checkpoint**: Shared components ready for use in user stories

---

## Phase 2: User Story 1 - View Dashboard with Clear Page Identity (Priority: P1) 🎯 MVP

**Goal**: Add integrated page title with contextual breadcrumb matching My Collection styling

**Independent Test**: Navigate to Dashboard and verify title "Dashboard" appears on page with "DASHBOARD / OVERVIEW" subtitle

### Implementation for User Story 1

- [ ] T005 [US1] Import PageHeader component and add to dashboard layout in src/routes/my-dashboard/+page.svelte
- [ ] T006 [US1] Add page title section with h1 heading "Dashboard" and uppercase subtitle "DASHBOARD / OVERVIEW" in src/routes/my-dashboard/+page.svelte
- [ ] T007 [US1] Add brief description text below title matching Collection view pattern in src/routes/my-dashboard/+page.svelte

**Checkpoint**: Dashboard has consistent page identity with My Collection view

---

## Phase 3: User Story 2 - Access Quick Actions from Command Center (Priority: P1)

**Goal**: Provide three distinct quick action buttons in labeled Command Center area

**Independent Test**: Verify three buttons visible (Add to Collection, Add to Wishlist, Log Maintenance) and each navigates correctly

### Implementation for User Story 2

- [ ] T008 [US2] Add "Log Maintenance" action to actions array in src/routes/my-dashboard/+page.svelte
- [ ] T009 [US2] Rename "Quick Actions" section header to "Command Center" in src/routes/my-dashboard/+page.svelte
- [ ] T010 [US2] Update QuickActionButtons component styling for distinct button appearance in src/lib/components/QuickActionButtons.svelte
- [ ] T011 [US2] Implement Log Maintenance click handler to show "coming soon" toast (maintenance feature not yet implemented) in src/routes/my-dashboard/+page.svelte

**Checkpoint**: Command Center displays three functional action buttons

---

## Phase 4: User Story 3 - Browse Recently Added Models in Visual Gallery (Priority: P2)

**Goal**: Display recently added models as large visual cards with prominent imagery

**Independent Test**: Add a model with image and verify it displays as large visual card with clickable navigation

### Implementation for User Story 3

- [ ] T012 [US3] Enhance RecentItemCard to display larger images with overlay title in src/lib/components/RecentItemCard.svelte
- [ ] T013 [US3] Add click handler to RecentItemCard for navigation based on source field (Collection → /my-collection/{id}, Wishlist → /my-wishlists/{id}) in src/lib/components/RecentItemCard.svelte
- [ ] T014 [US3] Update gallery grid layout for larger card display in src/routes/my-dashboard/+page.svelte
- [ ] T015 [US3] Improve empty state message with prompt to add models in src/routes/my-dashboard/+page.svelte

**Checkpoint**: Recently Added section displays visual gallery with clickable cards

---

## Phase 5: User Story 4 - Monitor Depot Work-in-Progress (Priority: P2)

**Goal**: Show depot items with color-coded status badges for service state

**Independent Test**: Verify depot items display "In Service" or "Under Repair" badges with appropriate colors

### Implementation for User Story 4

- [ ] T016 [US4] Integrate StatusBadge component into DepotListCard in src/lib/components/DepotListCard.svelte
- [ ] T017 [US4] Integrate StatusBadge component into DepotTable in src/lib/components/DepotTable.svelte
- [ ] T018 [US4] Update DepotView section header styling to emphasize utility focus in src/lib/components/DepotView.svelte
- [ ] T019 [US4] Add default "In Service" status to all depot items (DashboardDepotEntry lacks status field; placeholder for MVP)

**Checkpoint**: Depot displays items with color-coded status badges

---

## Phase 6: User Story 5 - View Statistics in Widget Cards (Priority: P3)

**Goal**: Display statistics in distinct info-card widgets separated from charts

**Independent Test**: Verify stats appear in individual card containers with visual separation from charts below

### Implementation for User Story 5

- [ ] T020 [US5] Update StatsCard component with enhanced widget styling in src/lib/components/StatsCard.svelte
- [ ] T021 [US5] Add visual separator between statistics grid and charts section in src/routes/my-dashboard/+page.svelte
- [ ] T022 [US5] Add alert indicator styling for maintenance due card in src/lib/components/StatsCard.svelte
- [ ] T023 [US5] Ensure skeleton loading states match new widget styling in src/routes/my-dashboard/+page.svelte

**Checkpoint**: Statistics display as distinct widget cards with chart separation

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final improvements and validation

- [ ] T024 [P] Run `pnpm format` to format all modified files
- [ ] T025 [P] Run `pnpm lint` to verify no linting errors
- [ ] T026 [P] Run `pnpm check` to verify TypeScript types
- [ ] T027 Verify mobile responsiveness of all dashboard sections
- [ ] T028 Test all quick action button navigations work correctly
- [ ] T029 Verify visual consistency with My Collection page styling

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **User Story 1 (Phase 2)**: Depends on T003 (PageHeader component)
- **User Story 2 (Phase 3)**: Depends on T001/T002 (translation keys)
- **User Story 3 (Phase 4)**: No dependencies on other stories
- **User Story 4 (Phase 5)**: Depends on T004 (StatusBadge component)
- **User Story 5 (Phase 6)**: No dependencies on other stories
- **Polish (Phase 7)**: Depends on all user stories complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Setup - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Setup - No dependencies on other stories
- **User Story 3 (P2)**: Can start after Setup - No dependencies on other stories
- **User Story 4 (P2)**: Can start after Setup (needs StatusBadge) - No dependencies on other stories
- **User Story 5 (P3)**: Can start after Setup - No dependencies on other stories

### Parallel Opportunities

- T001, T002, T003, T004 can all run in parallel (different files)
- After Setup, User Stories 1-5 can be worked on in parallel
- Within each story, tasks are sequential (same file dependencies)

---

## Parallel Example: Setup Phase

```bash
# Launch all setup tasks together:
Task T001: "Add translation keys to messages/en.json"
Task T002: "Add translation keys to messages/it.json"
Task T003: "Create PageHeader component"
Task T004: "Create StatusBadge component"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 2 Only)

1. Complete Phase 1: Setup (all [P] tasks in parallel)
2. Complete Phase 2: User Story 1 (Page Identity)
3. Complete Phase 3: User Story 2 (Command Center)
4. **STOP and VALIDATE**: Test page identity and quick actions
5. Deploy/demo if ready - users have core improvements

### Incremental Delivery

1. Setup → Page Identity + Command Center → MVP ready
2. Add User Story 3 (Gallery) → Visual enhancement
3. Add User Story 4 (Depot Status) → Workflow improvement
4. Add User Story 5 (Widget Stats) → Polish
5. Each story adds value without breaking previous

---

## Notes

- All strings must use Paraglide-JS (import from `$lib/paraglide/messages.js`)
- Use Svelte 5 Runes syntax (`$state`, `$derived`, `$props`)
- Follow existing component patterns from Collection view
- No backend changes required - frontend only
- Commit after each task or logical group
