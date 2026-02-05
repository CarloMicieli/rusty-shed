# Tasks: Responsive Navigation System

**Feature**: Responsive Navigation System  
**Branch**: `013-responsive-navigation`  
**Prerequisites**: [plan.md](plan.md), [spec.md](spec.md), [research.md](research.md), [data-model.md](data-model.md), [contracts/](contracts/)

**Tests**: Component tests included per constitution testing requirements

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `- [ ] [ID] [P?] [Story?] Description with file path`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4, US5)
- **No Story Label**: Setup and foundational tasks

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create type definitions, configuration, and translation keys

- [ ] T001 [P] Create navigation types file in src/lib/components/navigation/types.ts
- [ ] T002 [P] Create navigation configuration file in src/lib/components/navigation/config.ts
- [ ] T003 [P] Add new translation keys to messages/en.json (app_home, app_finance, app_digital_dcc, app_railway_tracks, app_more, app_more_aria)
- [ ] T004 [P] Add new translation keys to messages/it.json with Italian translations

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core utilities and helpers that all components depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 [P] Implement isActive helper function in src/lib/components/navigation/utils.ts
- [ ] T006 [P] Verify all required lucide-svelte icons are available (LayoutDashboard, TrainFront, Wallet, Heart, Wrench, Warehouse, Cpu, TrainTrack, Ellipsis) - TrainTrack confirmed in v0.562.0
- [ ] T007 Run pnpm prepare to compile Paraglide message updates

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Desktop Navigation Access (Priority: P1) 🎯 MVP

**Goal**: Desktop users can access all 9 features through a persistent sidebar

**Independent Test**: Open app on desktop viewport (≥768px), verify all 9 navigation items are visible and clickable with correct icons and labels

### Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T008 [P] [US1] Create SidebarNavigation component test in src/**tests**/components/navigation/SidebarNavigation.test.ts
- [ ] T009 [P] [US1] Write test: "renders all 9 navigation items on desktop"
- [ ] T010 [P] [US1] Write test: "applies active state to current route"
- [ ] T011 [P] [US1] Write test: "navigates to correct route on click"
- [ ] T012 [P] [US1] Write test: "uses Paraglide translations for labels"

### Implementation for User Story 1

- [ ] T013 [US1] Update SidebarNavigation.svelte to use NAVIGATION_ITEMS config from src/lib/components/navigation/config.ts
- [ ] T014 [US1] Update desktop sidebar to display all 9 features with correct icons in src/lib/components/SidebarNavigation.svelte
- [ ] T015 [US1] Implement active state detection using $page.url.pathname in src/lib/components/SidebarNavigation.svelte
- [ ] T016 [US1] Apply responsive class "hidden md:flex" to show sidebar only on desktop in src/lib/components/SidebarNavigation.svelte
- [ ] T017 [US1] Update labels to use new Paraglide message functions (m.app_home(), m.app_finance(), etc.) in src/lib/components/SidebarNavigation.svelte
- [ ] T018 [US1] Verify keyboard navigation (Tab, Enter) works in desktop sidebar
- [ ] T019 [US1] Run component tests and verify all US1 tests pass

**Checkpoint**: Desktop sidebar should display all 9 features, highlight active route, and be keyboard navigable

---

## Phase 4: User Story 2 - Mobile Primary Navigation (Priority: P1) 🎯 MVP

**Goal**: Mobile users can access 4 primary features through a bottom navigation bar with 5 slots

**Independent Test**: Open app on mobile viewport (<768px), verify 5-slot bottom bar is visible with Home, Collection, Finance, Wishlists, and More button

### Tests for User Story 2

- [ ] T020 [P] [US2] Create BottomNavigation component test in src/**tests**/components/navigation/BottomNavigation.test.ts
- [ ] T021 [P] [US2] Write test: "renders 5 slots on mobile (4 primary + More button)"
- [ ] T022 [P] [US2] Write test: "applies active state to current primary feature"
- [ ] T023 [P] [US2] Write test: "navigates to correct route on tap"
- [ ] T024 [P] [US2] Write test: "hides bottom bar on desktop viewport"
- [ ] T025 [P] [US2] Write test: "More button appears as 5th slot"

### Implementation for User Story 2

- [ ] T026 [US2] Update BottomNavigation.svelte to use PRIMARY_ITEMS config from src/lib/components/navigation/config.ts
- [ ] T027 [US2] Implement 5-slot bottom bar layout (4 primary items + More button) in src/lib/components/BottomNavigation.svelte
- [ ] T028 [US2] Implement active state detection for primary features in src/lib/components/BottomNavigation.svelte
- [ ] T029 [US2] Add More button with Ellipsis icon as 5th slot in src/lib/components/BottomNavigation.svelte
- [ ] T030 [US2] Apply responsive class "md:hidden" to show bottom bar only on mobile in src/lib/components/BottomNavigation.svelte
- [ ] T031 [US2] Update labels to use new Paraglide message functions in src/lib/components/BottomNavigation.svelte
- [ ] T032 [US2] Verify tap targets are ≥44px (h-16 = 64px) for accessibility
- [ ] T033 [US2] Run component tests and verify all US2 tests pass

**Checkpoint**: Mobile bottom bar should display 4 primary features + More button, highlight active route

---

## Phase 5: User Story 3 - Mobile Secondary Feature Access (Priority: P2)

**Goal**: Mobile users can access 4 secondary features through the More menu (bottom sheet)

**Independent Test**: Tap More button on mobile, verify bottom sheet opens with Maintenance, Depot, Digital DCC, and Railway Tracks

### Tests for User Story 3

- [ ] T034 [P] [US3] Create MoreMenu component test in src/**tests**/components/navigation/MoreMenu.test.ts
- [ ] T035 [P] [US3] Write test: "opens bottom sheet when More button is tapped"
- [ ] T036 [P] [US3] Write test: "displays 4 secondary features in sheet"
- [ ] T037 [P] [US3] Write test: "closes sheet and navigates when secondary feature is tapped"
- [ ] T038 [P] [US3] Write test: "closes sheet when backdrop is tapped"
- [ ] T039 [P] [US3] Write test: "closes sheet when ESC key is pressed"
- [ ] T040 [P] [US3] Write test: "More button shows active state when on secondary feature route"

### Implementation for User Story 3

- [ ] T041 [US3] Create MoreMenu component in src/lib/components/navigation/MoreMenu.svelte
- [ ] T042 [US3] Implement bottom sheet using shadcn-svelte Sheet component in src/lib/components/navigation/MoreMenu.svelte
- [ ] T043 [US3] Display SECONDARY_ITEMS in bottom sheet menu in src/lib/components/navigation/MoreMenu.svelte
- [ ] T044 [US3] Implement active state detection for secondary features in src/lib/components/navigation/MoreMenu.svelte
- [ ] T045 [US3] Add moreMenuOpen state ($state rune) in src/lib/components/BottomNavigation.svelte
- [ ] T046 [US3] Implement toggleMoreMenu and closeMoreMenu handlers in src/lib/components/BottomNavigation.svelte
- [ ] T047 [US3] Connect More button to open/close MoreMenu component in src/lib/components/BottomNavigation.svelte
- [ ] T048 [US3] Implement More button active state logic (active if any secondary feature route matches) in src/lib/components/BottomNavigation.svelte
- [ ] T049 [US3] Implement auto-close on navigation in src/lib/components/navigation/MoreMenu.svelte
- [ ] T050 [US3] Verify sheet opens/closes with keyboard (Space/Enter on More button, ESC to close)
- [ ] T051 [US3] Run component tests and verify all US3 tests pass

**Checkpoint**: More menu should open on tap, display 4 secondary features, close on selection or backdrop tap, and More button shows active state correctly

---

## Phase 6: User Story 4 - Consistent Feature Identity (Priority: P2)

**Goal**: Consistent naming, iconography, and visual identity across desktop and mobile

**Independent Test**: Compare feature labels and icons across desktop sidebar and mobile views, verify they match exactly

### Tests for User Story 4

- [ ] T052 [P] [US4] Write test: "desktop and mobile use identical icon components for each feature"
- [ ] T053 [P] [US4] Write test: "desktop and mobile use identical Paraglide message functions for labels"
- [ ] T054 [P] [US4] Write test: "all 9 features use correct updated names (Home, Finance, Digital (DCC) with parentheses, Railway Tracks, etc.)"
- [ ] T055 [P] [US4] Write test: "icon mappings match specification (LayoutDashboard for Home, TrainFront for Collection, etc.)"

### Implementation for User Story 4

- [ ] T056 [P] [US4] Verify NAVIGATION_ITEMS config uses correct icons per spec in src/lib/components/navigation/config.ts
- [ ] T057 [P] [US4] Verify all components reference shared config (no hardcoded navigation data)
- [ ] T058 [P] [US4] Verify icon sizes are consistent (size={20} across all contexts)
- [ ] T059 [US4] Remove deprecated translation keys from messages/en.json (app_dashboard, budget_title, app_digital_roster, app_tracks with old labels)
- [ ] T060 [US4] Remove deprecated translation keys from messages/it.json
- [ ] T061 [US4] Run component tests and verify all US4 tests pass

**Checkpoint**: All features use identical icons and labels across desktop and mobile, deprecated keys removed

---

## Phase 7: User Story 5 - Localized Navigation (Priority: P3)

**Goal**: Navigation labels appear in user's preferred language

**Independent Test**: Switch language preference, verify all navigation labels update to selected language

### Tests for User Story 5

- [ ] T062 [P] [US5] Write test: "navigation labels update when locale changes"
- [ ] T063 [P] [US5] Write test: "wrapping navigation in {#key locale} block triggers re-render on language change"
- [ ] T064 [P] [US5] Write test: "all navigation labels use Paraglide message functions (no hardcoded strings)"

### Implementation for User Story 5

- [ ] T065 [US5] Wrap SidebarNavigation content in {#key locale} block in src/lib/components/SidebarNavigation.svelte
- [ ] T066 [US5] Wrap BottomNavigation content in {#key locale} block in src/lib/components/BottomNavigation.svelte
- [ ] T067 [US5] Verify MoreMenu uses Paraglide message functions for secondary feature labels in src/lib/components/navigation/MoreMenu.svelte
- [ ] T068 [US5] Test language switching manually (English ↔ Italian)
- [ ] T069 [US5] Verify aria-label for More button uses m.app_more_aria() in src/lib/components/BottomNavigation.svelte
- [ ] T070 [US5] Run component tests and verify all US5 tests pass

**Checkpoint**: All navigation labels update reactively when language changes

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T071 [P] Run pnpm lint and fix any linting errors
- [ ] T072 [P] Run pnpm format to format all modified files
- [ ] T073 [P] Verify responsive transition is smooth at 768px breakpoint (no flicker)
- [ ] T074 [P] Test on mobile devices (iOS Safari, Android Chrome) for touch interactions
- [ ] T075 [P] Verify color contrast meets WCAG AA (active state bg-primary vs text-primary-foreground)
- [ ] T076 [P] Performance profiling: verify navigation transitions <300ms, bottom sheet <200ms
- [ ] T077 [P] Test edge case: rapidly switching between mobile and desktop viewports
- [ ] T078 [P] Test edge case: extremely narrow mobile viewport (320px)
- [ ] T079 [P] Test edge case: bookmarking secondary feature and opening on mobile
- [ ] T080 [P] Test edge case: More menu open during viewport resize to desktop (verify auto-close per FR-015)
- [ ] T081 Run full test suite: pnpm test
- [ ] T082 Verify test coverage meets targets (component tests ≥80%)
- [ ] T083 Run quickstart.md validation: follow guide to verify it's accurate
- [ ] T084 [P] Update CHANGELOG.md with feature changes
- [ ] T085 Create PR with all changes and link to spec

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phases 3-7)**: All depend on Foundational phase completion
  - User Story 1 (P1): Can start after Phase 2 - No dependencies on other stories
  - User Story 2 (P1): Can start after Phase 2 - No dependencies on other stories (MVP with US1)
  - User Story 3 (P2): Can start after Phase 2 - Integrates with US2 but independently testable
  - User Story 4 (P2): Can start after Phase 2 - Validates US1, US2, US3 consistency
  - User Story 5 (P3): Can start after Phase 2 - Validates localization across all stories
- **Polish (Phase 8)**: Depends on all user stories being complete

### User Story Dependencies

- **US1 + US2**: Both P1 priority, can develop in parallel, together form MVP
- **US3**: Depends on US2 (adds More menu to bottom bar), can start after US2 foundation
- **US4**: Cross-cutting validation, should run after US1-US3 implementation
- **US5**: Cross-cutting localization, should run after US1-US3 implementation

### Within Each User Story

1. Write tests FIRST (they should FAIL)
2. Implement components
3. Run tests (they should PASS)
4. Verify manual test scenarios

### Parallel Opportunities

#### Phase 1: All Setup tasks can run in parallel

```bash
T001: Create types.ts
T002: Create config.ts
T003: Add English translation keys
T004: Add Italian translation keys
```

#### Phase 2: All Foundational tasks can run in parallel

```bash
T005: Implement utils.ts
T006: Verify icons available
```

#### Phase 3 (US1): Tests can run in parallel

```bash
T008-T012: All test writing can happen in parallel (different test cases)
```

#### Phase 4 (US2): Tests can run in parallel

```bash
T020-T025: All test writing can happen in parallel
```

#### Phase 5 (US3): Tests can run in parallel

```bash
T034-T040: All test writing can happen in parallel
```

#### Phase 6 (US4): Tests can run in parallel

```bash
T052-T055: All test writing can happen in parallel
T056-T058: Verification tasks can run in parallel
T059-T060: Deprecation cleanup can run in parallel
```

#### Phase 7 (US5): Tests can run in parallel

```bash
T062-T064: All test writing can happen in parallel
T065-T066: Both wrapping tasks can run in parallel
```

#### Phase 8: Most polish tasks can run in parallel

```bash
T071-T080: Linting, formatting, testing, edge cases
```

#### Across User Stories (if team capacity allows):

Once Phase 2 completes:

- Developer A: User Story 1 (T008-T019)
- Developer B: User Story 2 (T020-T033)
- Then collaborate on User Story 3 (integrates US2)
- Then validate with User Stories 4 & 5

---

## Parallel Example: Phase 1 (Setup)

```bash
# All setup tasks can run simultaneously:
Terminal 1: Create src/lib/components/navigation/types.ts
Terminal 2: Create src/lib/components/navigation/config.ts
Terminal 3: Update messages/en.json
Terminal 4: Update messages/it.json
# All complete in parallel → Phase 2 ready
```

---

## Parallel Example: User Story 1 (Tests)

```bash
# All US1 tests can be written in parallel:
Terminal 1: Write test "renders all 9 navigation items on desktop"
Terminal 2: Write test "applies active state to current route"
Terminal 3: Write test "navigates to correct route on click"
Terminal 4: Write test "uses Paraglide translations for labels"
# All fail initially → Implementation begins
```

---

## Implementation Strategy

### MVP First (User Stories 1 & 2 Only)

**Minimum Viable Product**: Desktop sidebar + Mobile bottom bar (4 primary features)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (Desktop Navigation)
4. Complete Phase 4: User Story 2 (Mobile Primary Navigation)
5. **STOP and VALIDATE**: Test both desktop and mobile views independently
6. Deploy/demo if ready

**Result**: Users can access 4 primary features on mobile, all 9 on desktop. More menu not yet implemented.

### Incremental Delivery

1. **Foundation** (Phases 1-2) → Types, config, translations ready
2. **MVP** (Phases 3-4) → Desktop + Mobile primary navigation → Deploy/Demo
3. **More Menu** (Phase 5) → Add secondary feature access on mobile → Deploy/Demo
4. **Consistency** (Phase 6) → Validate icons/labels match → Deploy/Demo
5. **Localization** (Phase 7) → Verify translations work → Deploy/Demo
6. **Polish** (Phase 8) → Final quality pass → Production ready

### Parallel Team Strategy

With 2 developers:

1. **Together**: Complete Phase 1 (Setup) and Phase 2 (Foundational)
2. **Split**:
   - Developer A: User Story 1 (Desktop)
   - Developer B: User Story 2 (Mobile bottom bar)
3. **Collaborate**: User Story 3 (More menu - extends US2)
4. **Validate**: User Stories 4 & 5 (cross-cutting)
5. **Polish**: Phase 8 together

---

## Testing Summary

### Component Tests (Per Constitution)

- **SidebarNavigation**: 5 tests (T009-T012 + edge cases)
- **BottomNavigation**: 6 tests (T021-T025 + More button)
- **MoreMenu**: 7 tests (T035-T040 + accessibility)
- **Consistency**: 4 tests (T052-T055)
- **Localization**: 3 tests (T062-T064)

**Total**: ~25 component tests covering navigation behavior, responsive design, and accessibility

### Manual Testing Checklist

From quickstart.md and spec edge cases:

- [ ] Desktop sidebar shows all 9 features
- [ ] Mobile bottom bar shows 4 primary features + More
- [ ] More menu opens with 4 secondary features
- [ ] Active state highlights correctly on all viewports
- [ ] Navigation works with keyboard (Tab, Enter)
- [ ] Tap targets are ≥44px on mobile
- [ ] Localization updates when language changes
- [ ] Responsive transition is smooth at 768px breakpoint
- [ ] Rapid viewport switching doesn't cause flicker
- [ ] Narrow viewports (320px) render correctly
- [ ] Bookmarked secondary features open correctly on mobile
- [ ] More menu closes when resizing to desktop

---

## Notes

- **[P] tasks**: Different files, no dependencies, can run in parallel
- **[Story] label**: Maps task to specific user story for traceability (US1-US5)
- **No Story label**: Setup and foundational tasks
- Each user story is independently completable and testable
- MVP = User Stories 1 & 2 (desktop + mobile primary navigation)
- Tests follow TDD: Write first (fail), implement (pass)
- Commit after each logical group of tasks
- Use checkpoints to validate stories independently
- Avoid vague tasks or cross-file conflicts

---

## Total Task Count

- **Setup**: 4 tasks
- **Foundational**: 3 tasks
- **User Story 1**: 12 tasks (5 tests + 7 implementation)
- **User Story 2**: 14 tasks (6 tests + 8 implementation)
- **User Story 3**: 18 tasks (7 tests + 11 implementation)
- **User Story 4**: 10 tasks (4 tests + 6 implementation)
- **User Story 5**: 9 tasks (3 tests + 6 implementation)
- **Polish**: 15 tasks

**Grand Total**: 85 tasks

**Estimated Time**: 12-16 hours for 1 developer, 8-10 hours for 2 developers working in parallel
