# Tasks: Modern Steampunk Theme System

**Input**: Design documents from `/specs/011-steampunk-theme/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Not explicitly requested in specification—tests are omitted. Add via polish phase if needed.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, migrations, and shared configuration

- [ ] T001 Create database migration in src-tauri/migrations/0007_add_theme_setting.sql
- [ ] T002 [P] Add ThemeValue enum to src-tauri/src/settings.rs with parse function
- [ ] T003 [P] Add theme field to SettingsDto and UpdateSettingsPayload in src-tauri/src/settings.rs
- [ ] T004 Update SettingsRepository SQL queries to include theme column in src-tauri/src/settings.rs
- [ ] T005 [P] Add Google Fonts preconnect and font links in src/app.html
- [ ] T006 [P] Add theme messages to messages/en.json (settings_theme_label, settings_theme_system, settings_theme_light, settings_theme_dark)
- [ ] T007 [P] Add theme messages to messages/it.json (Italian translations)
- [ ] T008 Run `pnpm tauri dev` to verify migration applies and bindings regenerate

---

## Phase 2: Foundational (Theme CSS Files)

**Purpose**: Core theme token definitions that ALL user stories depend on

**⚠️ CRITICAL**: No UI work can proceed until theme tokens are defined

- [ ] T009 Create src/lib/themes/ directory structure
- [ ] T010 [P] Create steampunk-base.css with font families, texture gradients, and variant-steampunk-\* classes in src/lib/themes/steampunk-base.css
- [ ] T011 [P] Create steampunk-light.css with Parchment & Brass color tokens in src/lib/themes/steampunk-light.css
- [ ] T012 [P] Create steampunk-dark.css with Iron & Copper color tokens in src/lib/themes/steampunk-dark.css
- [ ] T013 Update src/routes/layout.css to import steampunk themes and remove cerberus import
- [ ] T014 [P] Create themeStore.svelte.ts with state management in src/lib/stores/themeStore.svelte.ts
- [ ] T015 [P] Create steampunk-transitions.ts with custom Svelte transitions in src/lib/utils/steampunk-transitions.ts

**Checkpoint**: Theme infrastructure ready—all theme tokens defined, store created

---

## Phase 3: User Story 1 - Theme Persistence & Switching (Priority: P1) 🎯 MVP

**Goal**: User can select theme preference and it persists across app restarts

**Independent Test**: Select dark theme, restart app, verify dark theme is applied

### Implementation for User Story 1

- [ ] T016 [US1] Initialize themeStore from Tauri settings in src/routes/+layout.svelte onMount
- [ ] T017 [US1] Add OS theme change listener for system preference in src/routes/+layout.svelte
- [ ] T018 [US1] Sync resolved theme to document.body.dataset.theme in themeStore
- [ ] T019 [US1] Add default data-theme="steampunk-dark" to body element in src/app.html
- [ ] T020 [US1] Add theme selector dropdown to settings page in src/routes/my-settings/+page.svelte
- [ ] T021 [US1] Wire theme selector to themeStore.setTheme() with Tauri persistence in src/routes/my-settings/+page.svelte
- [ ] T022 [US1] Verify Paraglide messages display correctly for theme labels

**Checkpoint**: Theme persistence fully functional—MVP deliverable

---

## Phase 4: User Story 2 - Light Theme Experience (Priority: P2)

**Goal**: Light "Parchment & Brass" theme displays correctly across all pages

**Independent Test**: Enable light theme, navigate all pages, verify warm paper tones and brass accents

### Implementation for User Story 2

- [ ] T023 [P] [US2] Define complete light theme surface scale (50-950) in src/lib/themes/steampunk-light.css
- [ ] T024 [P] [US2] Define light theme primary (Burnished Gold #B8860B) scale in src/lib/themes/steampunk-light.css
- [ ] T025 [P] [US2] Define light theme secondary, tertiary, accent scales in src/lib/themes/steampunk-light.css
- [ ] T026 [P] [US2] Define light theme semantic colors (error, success, warning) in src/lib/themes/steampunk-light.css
- [ ] T027 [US2] Add parchment texture pattern for light theme in src/lib/themes/steampunk-base.css
- [ ] T028 [US2] Verify WCAG AA contrast ratios for all light theme text combinations
- [ ] T029 [US2] Update LayerChart surface colors for light theme compatibility in src/routes/layout.css

**Checkpoint**: Light theme fully styled and accessible

---

## Phase 5: User Story 3 - Dark Theme Experience (Priority: P2)

**Goal**: Dark "Iron & Copper" theme displays correctly across all pages

**Independent Test**: Enable dark theme, navigate all pages, verify cold iron surfaces and copper highlights

### Implementation for User Story 3

- [ ] T030 [P] [US3] Define complete dark theme surface scale (50-950) in src/lib/themes/steampunk-dark.css
- [ ] T031 [P] [US3] Define dark theme primary (Polished Copper #CD7F32) scale in src/lib/themes/steampunk-dark.css
- [ ] T032 [P] [US3] Define dark theme secondary, tertiary, accent (Furnace Orange #FF4500) scales in src/lib/themes/steampunk-dark.css
- [ ] T033 [P] [US3] Define dark theme semantic colors (error, success, warning) in src/lib/themes/steampunk-dark.css
- [ ] T034 [US3] Add metal grain texture pattern for dark theme in src/lib/themes/steampunk-base.css
- [ ] T035 [US3] Verify WCAG AA contrast ratios for all dark theme text combinations
- [ ] T036 [US3] Update LayerChart surface colors for dark theme compatibility in src/routes/layout.css

**Checkpoint**: Dark theme fully styled and accessible

---

## Phase 6: User Story 4 - Steampunk Component Styling (Priority: P3)

**Goal**: UI components feature rivets, metal textures, and mechanical styling

**Independent Test**: View cards, buttons, and dividers with steampunk styling elements

### Implementation for User Story 4

- [ ] T037 Create src/lib/components/steampunk/ directory
- [ ] T038 [P] [US4] Create RivetedCard.svelte with panel/plate/frame variants in src/lib/components/steampunk/RivetedCard.svelte
- [ ] T039 [P] [US4] Create ToggleValve.svelte themed toggle switch in src/lib/components/steampunk/ToggleValve.svelte
- [ ] T040 [P] [US4] Create RailDivider.svelte with train track pattern in src/lib/components/steampunk/RailDivider.svelte
- [ ] T041 [P] [US4] Create PressureGauge.svelte for visual progress indicators in src/lib/components/steampunk/PressureGauge.svelte
- [ ] T042 [US4] Create barrel export index.ts in src/lib/components/steampunk/index.ts
- [ ] T043 [US4] Define variant-steampunk-riveted CSS class in src/lib/themes/steampunk-base.css
- [ ] T044 [US4] Define variant-steampunk-brass and variant-steampunk-copper CSS classes in src/lib/themes/steampunk-base.css
- [ ] T045 [US4] Define variant-steampunk-lever button styling with press animation in src/lib/themes/steampunk-base.css
- [ ] T046 [US4] Define variant-steampunk-embossed text styling in src/lib/themes/steampunk-base.css
- [ ] T046a [P] [US4] Define variant-steampunk-parchment card styling for light theme in src/lib/themes/steampunk-base.css
- [ ] T046b [P] [US4] Define variant-steampunk-gauge circular indicator styling in src/lib/themes/steampunk-base.css
- [ ] T046c [P] [US4] Define variant-steampunk-valve toggle styling in src/lib/themes/steampunk-base.css
- [ ] T047 [US4] Add heavyDoorSlide transition for drawers in src/lib/utils/steampunk-transitions.ts
- [ ] T048 [US4] Add leverToggle transition for buttons in src/lib/utils/steampunk-transitions.ts

**Checkpoint**: Steampunk components available for use throughout app

---

## Phase 7: User Story 5 - Responsive Steampunk Design (Priority: P4)

**Goal**: Theme adapts appropriately for mobile with textures disabled and adequate touch targets

**Independent Test**: View app at mobile viewport, verify textures disabled and 44px touch targets

### Implementation for User Story 5

- [ ] T049 [US5] Add media query to disable complex textures on screens <1024px in src/lib/themes/steampunk-base.css
- [ ] T050 [US5] Add prefers-reduced-motion media query to disable animations in src/lib/themes/steampunk-base.css
- [ ] T051 [US5] Ensure minimum 44×44px touch targets for buttons in src/lib/themes/steampunk-base.css
- [ ] T052 [US5] Add responsive typography adjustments for mobile (reduced letter-spacing) in src/lib/themes/steampunk-base.css
- [ ] T053 [US5] Update RivetedCard to hide rivets on mobile via rivets="none" responsive prop
- [ ] T054 [US5] Add steampunk-focus class for visible focus states with 3:1 contrast in src/lib/themes/steampunk-base.css

**Checkpoint**: Theme fully responsive and accessible on all devices

---

## Phase 8: Integration & Polish

**Purpose**: Connect components to existing UI and final cleanup

- [ ] T055 [P] Update SidebarNavigation.svelte to use steampunk styling
- [ ] T056 [P] Update BottomNavigation.svelte with brass-pipe border treatment
- [ ] T057 Update TAG_META gradients in src/lib/config/tags.ts to use steampunk palette
- [ ] T058 Apply RivetedCard styling to ItemCard.svelte where appropriate
- [ ] T059 Add heavyDoorSlide transition to ItemDrawer.svelte
- [ ] T060 Run `pnpm format` to format all new files
- [ ] T061 Run `pnpm lint` and fix any linting issues
- [ ] T062 Run `pnpm check` for TypeScript errors
- [ ] T063 Run `pnpm rust:clippy` for Rust linting
- [ ] T064 Run `pnpm rust:fmt` for Rust formatting
- [ ] T065 Run `pnpm test` to verify no regressions
- [ ] T066 Manual verification: theme persists across app restart
- [ ] T067 Manual verification: all pages display correctly in both themes
- [ ] T068 Manual verification: contrast ratios meet WCAG AA via browser DevTools audit

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies—can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion—BLOCKS all user stories
- **User Stories (Phases 3-7)**: All depend on Foundational phase completion
  - US1 (Theme Switching) can start immediately after Phase 2
  - US2-US3 (Light/Dark themes) can run in parallel after Phase 2
  - US4 (Components) can start after Phase 2, but benefits from US2-US3 tokens
  - US5 (Responsive) depends on US4 components existing
- **Polish (Phase 8)**: Depends on US1-US5 completion

### User Story Dependencies

| Story | Can Start After | Depends On                | Can Parallel With |
| ----- | --------------- | ------------------------- | ----------------- |
| US1   | Phase 2         | themeStore, layout.css    | US2, US3          |
| US2   | Phase 2         | steampunk-light.css shell | US1, US3          |
| US3   | Phase 2         | steampunk-dark.css shell  | US1, US2          |
| US4   | Phase 2         | base.css variants         | US1, US2, US3     |
| US5   | US4             | Component files exist     | —                 |

### Parallel Opportunities

```text
After Phase 2 completion, can run in parallel:
├── US1: T016-T022 (Theme persistence)
├── US2: T023-T029 (Light theme)
├── US3: T030-T036 (Dark theme)
└── US4: T037-T048 (Components)

Within Phase 1 (parallel):
├── T002, T003, T005, T006, T007 (different files)

Within Phase 2 (parallel):
├── T010, T011, T012, T014, T015 (different files)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T008)
2. Complete Phase 2: Foundational (T009-T015)
3. Complete Phase 3: User Story 1 (T016-T022)
4. **STOP and VALIDATE**: Test theme switching and persistence
5. Deploy/demo if ready—users can switch themes!

### Incremental Delivery

| Increment | Stories Included | Delivers                              |
| --------- | ---------------- | ------------------------------------- |
| MVP       | US1              | Theme switching with basic dark/light |
| +1        | US1 + US2        | Full light theme experience           |
| +2        | US1 + US2 + US3  | Full dark theme experience            |
| +3        | US1-US4          | Steampunk components                  |
| Complete  | US1-US5          | Responsive + accessible               |

---

## Task Summary

| Phase                | Tasks          | Parallel Tasks  | Blocking         |
| -------------------- | -------------- | --------------- | ---------------- |
| 1: Setup             | T001-T008 (8)  | 5               | —                |
| 2: Foundational      | T009-T015 (7)  | 5               | All user stories |
| 3: US1 - Persistence | T016-T022 (7)  | 0               | —                |
| 4: US2 - Light Theme | T023-T029 (7)  | 4               | —                |
| 5: US3 - Dark Theme  | T030-T036 (7)  | 4               | —                |
| 6: US4 - Components  | T037-T048 (15) | 7               | —                |
| 7: US5 - Responsive  | T049-T054 (6)  | 0               | US4              |
| 8: Polish            | T055-T068 (14) | 2               | All stories      |
| **Total**            | **71 tasks**   | **27 parallel** | —                |
