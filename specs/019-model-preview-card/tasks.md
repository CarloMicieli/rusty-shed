# Implementation Tasks: Railway Model Preview Card Component

**Feature Branch**: `019-model-preview-card`
**Date**: 2026-02-12
**Status**: Ready for Implementation

---

## Overview

This document contains implementation tasks for the Railway Model Preview Card component, organized by user story to enable independent implementation and testing. Each phase represents a complete, testable increment.

**Total Tasks**: 21
**Estimated Complexity**: Medium
**MVP Scope**: Phase 3 (User Story 1 - P1 only)

---

## Implementation Strategy

### Incremental Delivery Approach

1. **MVP First**: Implement User Story 1 (P1) to deliver core value quickly
2. **Progressive Enhancement**: Add User Stories 2, 4, and 3 in priority order
3. **Polish Last**: Cross-cutting concerns and performance optimization

### Parallel Execution Opportunities

- Phase 1 tasks can run in parallel (different files)
- Within each user story phase, most implementation tasks can run in parallel after tests
- Final phase tasks are independent and can be parallelized

---

## Phase 1: Setup & Foundation

**Goal**: Set up project infrastructure for the component

**Tasks**:

- [ ] T001 [P] Add i18n message keys for component in messages/en.json
- [ ] T002 [P] Create component props type interface in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T003 [P] Set up test file structure in src/**tests**/components/RailwayModelPreviewCard.test.ts

**Parallel Execution**: All tasks can run in parallel (different concerns, no dependencies)

**Completion Criteria**: All setup files created, i18n keys defined, test infrastructure ready

---

## Phase 2: Foundational Component Structure

**Goal**: Create base component with minimal rendering

**Tasks**:

- [ ] T004 Create base component skeleton with TypeScript props in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T005 Add shadcn-svelte Card wrapper with base styling (card, gauge-frame, ring-1 ring-border/40) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T006 Implement basic component rendering test in src/**tests**/components/RailwayModelPreviewCard.test.ts

**Dependencies**: Phase 1 must complete first

**Completion Criteria**: Component renders empty card with proper styling, basic test passes

---

## Phase 3: User Story 1 (P1) - View Model Summary Information

**Goal**: Display core model information (manufacturer, product code, series, category, road number) with thumbnail

**Independent Test**: Render component with complete model data and verify all fields display correctly

**User Story**: As a model railway collector, I need to see a compact visual summary of each railway model in my collection or wishlist, so I can quickly browse and identify models without opening detailed views.

### Tasks

#### Tests

- [ ] T007 [P] [US1] Write test for displaying manufacturer and product code in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T008 [P] [US1] Write test for displaying series and category as title in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T009 [P] [US1] Write test for displaying road number in identification plate in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T010 [P] [US1] Write test for 16:9 aspect ratio thumbnail with photo in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T011 [P] [US1] Write test for long road number truncation (>25 chars) in src/**tests**/components/RailwayModelPreviewCard.test.ts

#### Implementation

- [ ] T012 [P] [US1] Implement thumbnail container with aspect-video class in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T013 [P] [US1] Implement manufacturer and product code display line in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T014 [P] [US1] Implement series and category title display in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T015 [US1] Implement road number identification plate with monospaced font in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T016 [US1] Implement road number truncation logic (>25 chars) with click/hover expansion in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T017 [US1] Add fallback display for missing road number ("# ---") in src/lib/components/RailwayModelPreviewCard.svelte

**Parallel Execution**:

- Tests (T007-T011) can all run in parallel
- After tests pass, implementation tasks (T012-T014) can run in parallel
- T015 depends on T014 (title must exist first)
- T016-T017 depend on T015 (road number plate must exist)

**Dependencies**: Phase 2 must complete first

**Completion Criteria**:

- Component displays all core information correctly
- Road numbers truncate properly when >25 characters
- Missing data shows appropriate fallbacks
- All tests pass with 60%+ coverage for this story

---

## Phase 4: User Story 2 (P2) - Identify Model Status and Characteristics

**Goal**: Add metadata badges (scale, power, era, purchase date) and status overlays (unit count, digital features)

**Independent Test**: Render cards with various metadata combinations and verify all badges display without overlap

**User Story**: As a collector, I need to see key metadata badges and status indicators at a glance, so I can quickly understand the model's specifications.

### Tasks

#### Tests

- [ ] T018 [P] [US2] Write test for scale and power method badges in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T019 [P] [US2] Write test for era and purchase date badges in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T020 [P] [US2] Write test for unit count badge (×3) in bottom-right corner in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T021 [P] [US2] Write test for digital feature icons (sound, DCC) in top-left corner in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T022 [P] [US2] Write test for multiple digital features stacking horizontally in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T023 [P] [US2] Write test for omitting badges when data is missing in src/**tests**/components/RailwayModelPreviewCard.test.ts

#### Implementation

- [ ] T024 [P] [US2] Implement metadata badge section with flexbox layout in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T025 [P] [US2] Add scale and power method Badge components in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T026 [P] [US2] Add era and purchase date Badge components with i18n in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T027 [P] [US2] Implement unit count overlay badge (absolute positioning, bottom-right) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T028 [P] [US2] Implement digital features overlay icons (absolute positioning, top-left) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T029 [US2] Add conditional rendering to omit missing metadata badges in src/lib/components/RailwayModelPreviewCard.svelte

**Parallel Execution**:

- All tests (T018-T023) can run in parallel
- Implementation tasks (T024-T028) can run in parallel after tests
- T029 is a refactoring task that touches multiple areas, runs last

**Dependencies**: Phase 3 (US1) must complete first (core display must exist)

**Completion Criteria**:

- All metadata badges display correctly
- Status overlays positioned without obscuring content
- Missing data handled gracefully (badges omitted)
- Multiple digital features stack horizontally
- Tests pass

---

## Phase 5: User Story 4 (P2) - Remove Models from Collection

**Goal**: Add delete button with confirmation dialog

**Independent Test**: Click delete button and verify confirmation dialog appears, then test both cancel and confirm actions

**User Story**: As a collector, I need to be able to delete a model directly from the card view, so I can quickly manage my inventory.

### Tasks

#### Tests

- [ ] T030 [P] [US4] Write test for delete button rendering when onDelete prop provided in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T031 [P] [US4] Write test for delete button hidden when onDelete prop omitted in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T032 [P] [US4] Write test for confirmation dialog opening on delete button click in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T033 [P] [US4] Write test for onDelete callback when user confirms in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T034 [P] [US4] Write test for dialog cancellation (no callback) in src/**tests**/components/RailwayModelPreviewCard.test.ts

#### Implementation

- [ ] T035 [P] [US4] Add delete button with Trash2 icon in card header in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T036 [P] [US4] Implement AlertDialog component for delete confirmation in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T037 [US4] Add onDelete event handler with model ID parameter in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T038 [US4] Add conditional rendering for delete button (only show if onDelete prop exists) in src/lib/components/RailwayModelPreviewCard.svelte

**Parallel Execution**:

- All tests (T030-T034) can run in parallel
- T035-T036 can run in parallel (button and dialog are separate concerns)
- T037-T038 depend on T035-T036 (wire up the pieces)

**Dependencies**: Phase 3 (US1) must complete first (core card must exist)

**Completion Criteria**:

- Delete button appears only when onDelete prop provided
- Confirmation dialog displays on click
- onDelete callback fired with correct model ID on confirm
- No callback on cancel
- Tests pass

---

## Phase 6: User Story 3 (P3) - Handle Missing Visual Data Gracefully

**Goal**: Add category-specific placeholder icons for models without photos

**Independent Test**: Render cards for multiple categories without photos and verify correct placeholder icons display

**User Story**: As a collector with models that don't have photos, I need to see a category-appropriate placeholder icon, so the card still looks complete.

### Tasks

#### Tests

- [ ] T039 [P] [US3] Write test for steam locomotive placeholder icon in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T040 [P] [US3] Write test for electric locomotive placeholder icon in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T041 [P] [US3] Write test for wagon/freight car placeholder icon in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T042 [P] [US3] Write test for generic train placeholder when category unknown in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T043 [P] [US3] Write test for photo display when photoUrl provided in src/**tests**/components/RailwayModelPreviewCard.test.ts

#### Implementation

- [ ] T044 [P] [US3] Create category-to-icon mapping with lucide-svelte icons in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T045 [P] [US3] Implement placeholder icon display logic (conditional on missing photo) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T046 [US3] Add img element for photo display with object-cover styling in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T047 [US3] Implement conditional rendering (photo vs placeholder) in src/lib/components/RailwayModelPreviewCard.svelte

**Parallel Execution**:

- All tests (T039-T043) can run in parallel
- Implementation tasks T044-T045 can run in parallel
- T046-T047 depend on T044-T045 (wiring together photo and placeholder logic)

**Dependencies**: Phase 3 (US1) must complete first (thumbnail container must exist)

**Completion Criteria**:

- Correct placeholder icons display for each category
- Generic train icon shows for unknown categories
- Photos display correctly when provided
- Smooth fallback between photo and placeholder
- Tests pass

---

## Phase 7: Polish & Cross-Cutting Concerns

**Goal**: Responsive design, accessibility, performance optimization, code quality

**Tasks**:

- [ ] T048 [P] Add responsive grid examples to component documentation in specs/019-model-preview-card/quickstart.md
- [ ] T049 [P] Add ARIA labels for delete button and thumbnail in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T050 [P] Implement hover states and transitions (scale-102, ring-primary-500) in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T051 [P] Add loading="lazy" decoding="async" to img elements in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T052 [P] Verify all user-facing strings use Paraglide i18n in src/lib/components/RailwayModelPreviewCard.svelte
- [ ] T053 Run pnpm lint and fix any linting errors
- [ ] T054 Run pnpm check and fix any TypeScript errors
- [ ] T055 [P] Write accessibility tests (ARIA attributes, semantic HTML) in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T056 [P] Write responsive design tests (Tailwind class verification) in src/**tests**/components/RailwayModelPreviewCard.test.ts
- [ ] T057 Run pnpm test:coverage and verify 60%+ coverage for component

**Parallel Execution**: All [P] tasks can run in parallel, verification tasks (T053-T054, T057) run at the end

**Dependencies**: Phases 3-6 must complete first

**Completion Criteria**:

- Component passes linting and type checking
- Accessibility standards met (ARIA labels, semantic HTML)
- Responsive design works on mobile (vertical stack)
- Performance optimized (lazy loading, smooth animations)
- Test coverage ≥60%
- All i18n strings use Paraglide

---

## Dependencies Graph

**Story Completion Order** (with parallelization):

```
Phase 1 (Setup) ──┐
                  ├──> Phase 2 (Foundation) ──┐
                  │                            │
                  │                            ├──> Phase 3 (US1 - P1) ──┬──> Phase 4 (US2 - P2) ──┐
                  │                            │                         │                         │
                  │                            │                         └──> Phase 5 (US4 - P2) ──┤
                  │                            │                                                   │
                  │                            └──> Phase 6 (US3 - P3) ────────────────────────────┤
                  │                                                                                 │
                  └─────────────────────────────────────────────────────────────────────────────────┴──> Phase 7 (Polish)
```

**Independent Stories** (can be implemented in parallel after Phase 3):

- User Story 2 (Metadata badges) - no dependency on US4 or US3
- User Story 4 (Delete button) - no dependency on US2 or US3
- User Story 3 (Placeholders) - no dependency on US2 or US4

**Critical Path**: Phase 1 → Phase 2 → Phase 3 → Phase 7

**Recommended MVP**: Implement Phase 1 → Phase 2 → Phase 3 only for quickest value delivery

---

## Parallel Execution Examples

### Phase 3 (User Story 1) - Tests in Parallel

```bash
# All test tasks can run simultaneously
vitest run --reporter=verbose src/__tests__/components/RailwayModelPreviewCard.test.ts --grep "manufacturer|product code"  # T007
vitest run --reporter=verbose src/__tests__/components/RailwayModelPreviewCard.test.ts --grep "series|category"  # T008
vitest run --reporter=verbose src/__tests__/components/RailwayModelPreviewCard.test.ts --grep "road number"  # T009
# ... etc
```

### Phase 4 (User Story 2) - Implementation in Parallel

After tests pass, these can run simultaneously:

- T024: Metadata badge section (different code area)
- T027: Unit count overlay (different code area)
- T028: Digital features overlay (different code area)

### Phase 7 (Polish) - Independent Tasks in Parallel

All [P] tasks are independent and can run simultaneously:

- T048: Documentation
- T049: ARIA labels
- T050: Hover states
- T051: Lazy loading
- T052: i18n verification
- T055: Accessibility tests
- T056: Responsive tests

---

## Testing Strategy

### Test Coverage Requirements

Target: **60%+ coverage** per constitution

**Test Categories**:

1. **Rendering Tests** (T007-T010, T018-T023, T039-T043): Verify all visual elements display correctly
2. **User Interaction Tests** (T030-T034): Verify delete button and confirmation dialog
3. **Edge Case Tests** (T011, T042): Long road numbers, missing data, unknown categories
4. **Accessibility Tests** (T055): ARIA labels, semantic HTML, keyboard navigation
5. **Responsive Tests** (T056): Tailwind class verification

### Test Execution

```bash
# Run all component tests
pnpm test src/__tests__/components/RailwayModelPreviewCard.test.ts

# Run with coverage
pnpm test:coverage src/__tests__/components/RailwayModelPreviewCard.test.ts

# Watch mode during development
pnpm test --watch src/__tests__/components/RailwayModelPreviewCard.test.ts
```

---

## Implementation Notes

### Code Quality Checklist

Per constitution requirements:

- ✅ TypeScript strict mode with proper type definitions
- ✅ All user-facing strings use Paraglide i18n
- ✅ Follows existing component patterns (StatsCard, RollingStockCard, SmartImage)
- ✅ Uses shadcn-svelte components (Card, Badge, Button, AlertDialog)
- ✅ Card styling: `card gauge-frame ring-1 ring-border/40`
- ✅ Responsive: `grid-cols-1 sm:grid-cols-2 lg:grid-cols-3`
- ✅ Icons: lucide-svelte (Train, Zap, Package, Users, Volume2, Trash2)
- ✅ Tests: Vitest + @testing-library/svelte following RollingStockCard.test.ts pattern
- ✅ Performance: <16ms render, lazy loading, smooth scrolling

### File Paths Summary

**Component**: `src/lib/components/RailwayModelPreviewCard.svelte`
**Tests**: `src/__tests__/components/RailwayModelPreviewCard.test.ts`
**i18n**: `messages/en.json`
**Documentation**: `specs/019-model-preview-card/quickstart.md`

### Key Implementation Decisions

From research.md and plan.md:

1. **No container queries**: Use standard Tailwind breakpoints (`sm:`, `lg:`)
2. **Road number truncation**: Use Svelte 5 $state for click/hover expansion, not Tooltip component
3. **Image handling**: Follow SmartImage.svelte pattern or create similar approach
4. **Category icons**: lucide-svelte with category mapping (Train, Zap, Package, Users, etc.)
5. **Delete confirmation**: shadcn-svelte AlertDialog component

---

## Next Steps

1. **Start with MVP**: Implement Phase 1 → Phase 2 → Phase 3 for quickest value
2. **Run verifications**: After each phase, run `pnpm lint`, `pnpm check`, `pnpm test`
3. **Progressive enhancement**: Add User Stories 2, 4, 3 in priority order
4. **Final polish**: Complete Phase 7 before considering feature done

**Ready to implement!** 🚀

---

**Version**: 1.0
**Last Updated**: 2026-02-12
**Total Tasks**: 57
