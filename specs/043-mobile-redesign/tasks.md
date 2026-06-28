# Tasks: Mobile Redesign

**Input**: Design documents from `/specs/043-mobile-redesign/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Include regression and component/integration tests because the specification explicitly requires multilingual regression validation, mobile interaction reliability, and desktop parity checks.

**Organization**: Tasks are grouped by user story so each story is independently implementable and testable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Maps to user story labels (US1, US2, US3, US4)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the planning/test scaffolding required for feature execution.

- [ ] T001 Create mobile redesign test plan notes in docs/testing/mobile-redesign-test-matrix.md
- [ ] T002 [P] Create mobile fixture helpers for viewport and safe-area simulation in src/__tests__/helpers/mobileViewport.ts
- [ ] T003 [P] Create mobile interaction helper utilities (tap target/assertions) in src/__tests__/helpers/mobileInteractions.ts
- [ ] T004 Add feature section and execution notes to specs/043-mobile-redesign/quickstart.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core responsive primitives and shared state contracts that block all user stories.

**⚠️ CRITICAL**: No user-story implementation begins before this phase completes.

- [ ] T005 Implement Tailwind safe-area utilities and touch-hover variant in src/routes/layout.css
- [ ] T006 Update mobile main-shell padding and safe-area handling in src/routes/+layout.svelte
- [ ] T007 [P] Add shared matchMedia helper for non-CSS toggles in src/lib/state/match-media.svelte.ts
- [ ] T008 Implement bounded drawer registry state (max depth 2) in src/lib/state/drawer-registry.svelte.ts
- [ ] T009 Wire drawer registry with route/back-pop synchronization in src/routes/+layout.svelte
- [ ] T010 [P] Add drawer registry unit tests (stack bound + pop order + replace behavior) in src/__tests__/state/drawer-registry.svelte.test.ts
- [ ] T011 [P] Add route/back integration test for overlay unwind behavior in src/__tests__/routes/mobile-back-stack.test.ts

**Checkpoint**: Foundational mobile primitives are complete; user stories can proceed.

---

## Phase 3: User Story 1 - Mobile Navigation And Readability (Priority: P1) 🎯 MVP

**Goal**: Deliver mobile app shell/navigation readability and safe-area correctness below 768px without changing desktop behavior.

**Independent Test**: On 375px in English and Italian, navigate primary routes and More panel; verify titles are visible, labels readable, Settings/Debug reachable in <=2 taps, and no clipping.

### Tests for User Story 1

- [ ] T012 [P] [US1] Add mobile shell safe-area regression tests in src/__tests__/routes/mobile-layout-shell.test.ts
- [ ] T013 [P] [US1] Add BottomNavigation label/truncation tests for long Italian strings in src/__tests__/lib/components/BottomNavigation.mobile.test.ts
- [ ] T014 [P] [US1] Add MoreMenu top-actions accessibility/reachability test in src/__tests__/lib/components/navigation/MoreMenu.mobile.test.ts

### Implementation for User Story 1

- [ ] T015 [US1] Add page-title context outlet support for mobile header in src/routes/+layout.svelte
- [ ] T016 [US1] Provide shared page-title context state in src/lib/state/page-title.svelte.ts
- [ ] T017 [US1] Refactor bottom navigation to keep 4 primary items plus More in src/lib/components/BottomNavigation.svelte
- [ ] T018 [US1] Move Settings/Debug as top actions in mobile More panel in src/lib/components/navigation/MoreMenu.svelte
- [ ] T019 [US1] Apply mobile-safe truncation and typography constraints for nav labels in src/lib/components/BottomNavigation.svelte
- [ ] T020 [US1] Add/update mobile navigation i18n keys in messages/en.json
- [ ] T021 [US1] Add/update mobile navigation i18n keys in messages/it.json
- [ ] T022 [US1] Add mobile title-context wiring for collection page in src/routes/collection/+page.svelte
- [ ] T023 [US1] Add mobile title-context wiring for settings page in src/routes/settings/+page.svelte

**Checkpoint**: US1 is independently functional and testable as MVP.

---

## Phase 4: User Story 2 - Mobile Collection Workflow Efficiency (Priority: P2)

**Goal**: Make collection browsing/filter/add actions touch-friendly and mobile-optimized.

**Independent Test**: Complete browse-filter-add on collection mobile view; confirm one-column card readability, reliable chip removal taps, and contextual add action visibility.

### Tests for User Story 2

- [ ] T024 [P] [US2] Add mobile collection layout tests (single-column + min width) in src/__tests__/collection/CollectionDashboard.mobile-layout.test.ts
- [ ] T025 [P] [US2] Add filter-chip touch target tests in src/__tests__/collection/CollectionDashboard.touch-target.test.ts
- [ ] T026 [P] [US2] Add VirtualGrid itemMinWidth behavior tests in src/__tests__/lib/components/VirtualGrid.mobile.test.ts

### Implementation for User Story 2

- [ ] T027 [US2] Enforce `itemMinWidth=320` and hide mobile table toggle in src/lib/features/collection/CollectionDashboard.svelte
- [ ] T028 [US2] Add contextual FAB behavior for collection mobile view in src/lib/features/collection/CollectionDashboard.svelte
- [ ] T029 [US2] Update preview card mobile layout (Option B reflow + category badge treatment) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T030 [US2] Apply 44x44 and chip-remove 36x36 touch target sizing in src/lib/features/collection/CollectionDashboard.svelte
- [ ] T031 [US2] Align collection route wrapper with new mobile spacing and safe-area constraints in src/routes/collection/+page.svelte
- [ ] T032 [US2] Add/update collection mobile UI i18n keys in messages/en.json
- [ ] T033 [US2] Add/update collection mobile UI i18n keys in messages/it.json

**Checkpoint**: US2 can be validated independently of US3/US4.

---

## Phase 5: User Story 3 - Mobile Editing Through Sheets (Priority: P3)

**Goal**: Replace mobile inline editing with bounded, consistent sheet-based editing and robust media fallback.

**Independent Test**: On mobile detail page, complete edit via sheets (parent+child max), close with back/gesture correctly, and attach media with camera fallback preserving form state.

### Tests for User Story 3

- [ ] T034 [P] [US3] Add mobile sheet depth and dismiss-order tests in src/__tests__/lib/components/drawer/DrawerShell.depth.test.ts
- [ ] T035 [P] [US3] Add mobile inline-edit suppression tests in src/__tests__/collection/InPlaceEdit.mobile.test.ts
- [ ] T036 [P] [US3] Add media capability fallback tests (camera unavailable/denied/error) in src/__tests__/lib/components/model-details/ImageUpload.mobile-capability.test.ts

### Implementation for User Story 3

- [ ] T037 [US3] Refactor drawer shell to bottom-sheet on mobile and side-panel on md+ in src/lib/components/drawer/DrawerShell.svelte
- [ ] T038 [US3] Add GPU-friendly transform/compositor and reduced-motion behavior in src/lib/components/drawer/DrawerShell.svelte
- [ ] T039 [US3] Disable inline edit behavior on mobile and route edit actions to drawer flow in src/lib/components/InPlaceEdit.svelte
- [ ] T040 [US3] Wire detail page edit actions to unified mobile sheet flow in src/routes/collection/[itemId]/+page.svelte
- [ ] T041 [US3] Add camera capability probing and fallback mode switching in src/lib/components/model-details/ImageUpload.svelte
- [ ] T042 [US3] Remove capture attribute when camera capability is unavailable and preserve form state in src/lib/components/model-details/ImageDropZone.svelte
- [ ] T043 [US3] Add/update media fallback i18n keys in messages/en.json
- [ ] T044 [US3] Add/update media fallback i18n keys in messages/it.json

**Checkpoint**: US3 edit flow works independently with bounded sheet nesting and media fallback.

---

## Phase 6: User Story 4 - Stable Mobile Experience Across Devices (Priority: P4)

**Goal**: Ensure phased rollout reliability, multilingual regression safety, and desktop parity.

**Independent Test**: Execute milestone-level checks on mobile and desktop; verify no desktop regressions, no critical 375px text collisions, and non-blocking startup placeholders.

### Tests for User Story 4

- [ ] T045 [P] [US4] Add desktop parity regression tests for updated shell/navigation/components in src/__tests__/routes/desktop-parity.mobile-redesign.test.ts
- [ ] T046 [P] [US4] Add multilingual overflow/truncation tests (en/it at 375px) in src/__tests__/routes/mobile-i18n-overflow.test.ts
- [ ] T047 [P] [US4] Add startup placeholder behavior tests during async initialization in src/__tests__/routes/mobile-startup-placeholder.test.ts

### Implementation for User Story 4

- [ ] T048 [US4] Ensure startup loading surface is non-blocking and mobile-safe in src/routes/+layout.svelte
- [ ] T049 [US4] Add milestone-based mobile regression checklist and acceptance report template in docs/testing/mobile-redesign-regression-checklist.md
- [ ] T050 [US4] Document mobile rollout and desktop parity verification steps in specs/043-mobile-redesign/quickstart.md

**Checkpoint**: All stories are independently verifiable; rollout is safe and measurable.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, docs, and full validation.

- [ ] T051 [P] Run full feature verification commands from quickstart and capture output notes in specs/043-mobile-redesign/quickstart.md
- [ ] T052 [P] Perform final touch-target audit and document exceptions in docs/testing/mobile-redesign-test-matrix.md
- [ ] T053 [P] Perform final Italian 375px visual audit notes in docs/testing/mobile-redesign-test-matrix.md
- [ ] T054 Run final code quality checks and fix findings in src/routes/+layout.svelte
- [ ] T055 Update feature changelog entry in CHANGELOG.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phases 3-6 (User Stories)**: Depend on Phase 2 completion.
- **Phase 7 (Polish)**: Depends on completion of targeted user stories.

### User Story Dependencies

- **US1 (P1)**: Starts after Foundational; no dependency on other stories.
- **US2 (P2)**: Starts after Foundational; depends on US1 shell/navigation context for complete UX but remains independently testable.
- **US3 (P3)**: Starts after Foundational; uses drawer primitives from Phase 2 and can proceed in parallel with US2 after those are complete.
- **US4 (P4)**: Starts after Foundational and story implementations; validates cross-story behavior and release safety.

### Within Each User Story

- Tests first, then implementation.
- Shared state/helpers before component integration.
- Component changes before route wiring.
- Route wiring before i18n and acceptance checks.

---

## Parallel Opportunities

- Setup: T002 and T003 parallel.
- Foundational: T007, T010, T011 parallel after T005/T006/T008.
- US1: T012-T014 parallel; T020 and T021 parallel.
- US2: T024-T026 parallel; T032 and T033 parallel.
- US3: T034-T036 parallel; T043 and T044 parallel.
- US4: T045-T047 parallel.
- Polish: T051-T053 parallel.

---

## Parallel Example: User Story 1

```bash
# Run US1 test tasks in parallel:
T012 src/__tests__/routes/mobile-layout-shell.test.ts
T013 src/__tests__/lib/components/BottomNavigation.mobile.test.ts
T014 src/__tests__/lib/components/navigation/MoreMenu.mobile.test.ts

# Run US1 localization tasks in parallel:
T020 messages/en.json
T021 messages/it.json
```

## Parallel Example: User Story 2

```bash
# Run US2 test tasks in parallel:
T024 src/__tests__/collection/CollectionDashboard.mobile-layout.test.ts
T025 src/__tests__/collection/CollectionDashboard.touch-target.test.ts
T026 src/__tests__/lib/components/VirtualGrid.mobile.test.ts

# Run US2 i18n updates in parallel:
T032 messages/en.json
T033 messages/it.json
```

## Parallel Example: User Story 3

```bash
# Run US3 behavior tests in parallel:
T034 src/__tests__/lib/components/drawer/DrawerShell.depth.test.ts
T035 src/__tests__/collection/InPlaceEdit.mobile.test.ts
T036 src/__tests__/lib/components/model-details/ImageUpload.mobile-capability.test.ts

# Run US3 i18n updates in parallel:
T043 messages/en.json
T044 messages/it.json
```

## Parallel Example: User Story 4

```bash
# Run US4 regression tests in parallel:
T045 src/__tests__/routes/desktop-parity.mobile-redesign.test.ts
T046 src/__tests__/routes/mobile-i18n-overflow.test.ts
T047 src/__tests__/routes/mobile-startup-placeholder.test.ts
```

---

## Implementation Strategy

### MVP First (US1)

1. Complete Phase 1 and Phase 2.
2. Deliver Phase 3 (US1) fully.
3. Validate US1 independent test on 375px in en/it.
4. Demo/deploy MVP increment.

### Incremental Delivery

1. Foundation complete (Phases 1-2).
2. Deliver US1, validate, and stabilize.
3. Deliver US2, validate mobile collection flow.
4. Deliver US3, validate sheet editing and camera fallback.
5. Deliver US4 validation and release hardening.
6. Complete Phase 7 polish and full verification.

### Parallel Team Strategy

1. Team completes Setup + Foundational together.
2. After Phase 2:
   - Engineer A: US2
   - Engineer B: US3
   - Engineer C: US4 regression harness
3. Merge by milestone with desktop parity checks at each gate.

---

## Notes

- `[P]` tasks are safe to execute in parallel when dependencies are met.
- Keep all user-facing text in Paraglide catalogs (`messages/en.json`, `messages/it.json`).
- Do not introduce new packages without explicit approval.
- Preserve desktop behavior for `>=768px` in every story.
