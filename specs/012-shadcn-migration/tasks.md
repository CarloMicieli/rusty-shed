# Implementation Tasks: Skeleton to shadcn-svelte Migration

**Feature**: 012-shadcn-migration | **Branch**: `012-shadcn-migration`  
**Spec**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  
**Total Tasks**: 62 | **Organized By**: Phase + User Story

---

## Dependencies & Execution Order

**Critical Path**:

1. Phase 1 (Setup) must complete before any Phase 2+ tasks
2. Phase 2 (Foundational) blocks all user story phases
3. User Stories (US1-US6) are mostly independent but share common infrastructure from Phase 2
4. Phase 7 (Polish) can run in parallel with Phase 2 in some cases

**Parallel Opportunities**:

- US1 (Component Replacement) and US2 (Styling) can start together after Phase 2
- US3 (Feature Parity) and US4 (Developer Transition) are independent
- US5 (Headers) and US6 (Toasts) are independent UI-specific tasks

---

## Phase 1: Setup & Project Initialization

### Phase Goal

Initialize the migration project, install shadcn-svelte v1.1.1, and remove Skeleton dependencies.

### Independent Test Criteria

- ✓ Build succeeds with shadcn-svelte installed and Skeleton removed
- ✓ No console errors or warnings related to missing Skeleton imports
- ✓ Package.json reflects v1.1.1 as UI library
- ✓ Tailwind config is verified compatible with shadcn-svelte

**Status**: ✅ COMPLETE - All Phase 1 tasks completed successfully (commit ea3a3c0)

- Created Toast store with shadcn-svelte compatible implementation
- Created ToastProvider component for top-right notifications
- Created Accordion component system with all sub-components
- Created modal store replacing Skeleton's getModalStore
- Updated all component imports across 9 files
- Build passes ✓ | Tests pass ✓ | Linting passes ✓

---

- [x] T001 Install shadcn-svelte v1.1.1 and dependencies in package.json
- [x] T002 Remove @skeletonlabs/skeleton (4.9.0) and @skeletonlabs/skeleton-common (4.11.0) from dependencies
- [x] T003 [P] Install additional shadcn-svelte peer dependencies (@floating-ui/dom, bits-ui)
- [x] T004 Update Tailwind config to integrate shadcn-svelte theme system
- [x] T005 [P] Verify Tailwind v4.1.18 compatibility with shadcn-svelte
- [x] T006 Run `pnpm install` and validate clean lock file
- [x] T007 [P] Test build: `pnpm build` completes without errors
- [x] T008 Verify no Skeleton CSS imports remain in build output
- [x] T009 [P] Commit: `feat: install shadcn-svelte v1.1.1 and remove skeleton dependency`

---

## Phase 2: Foundational Infrastructure

### Phase Goal

Set up the core systems needed for all user stories: theme system integration, toast provider, and component structure.

### Independent Test Criteria

- ✓ Steampunk theme system is fully integrated with shadcn-svelte
- ✓ Theme switching (dark/light) works without app restart
- ✓ Toast notification provider is available globally
- ✓ Component import structure is established and documented
- ✓ Development environment ready for per-page component replacement

---

- [x] T010 Create `src/lib/components/shadcn/` directory structure for shadcn components
- [x] T011 [P] Verify Steampunk theme files are intact (steampunk-base.css, steampunk-light.css, steampunk-dark.css)
- [x] T012 Integrate Steampunk theme system with shadcn-svelte's theming approach (CSS variables, design tokens)
- [x] T013 [P] Test theme switching: dark mode → light mode → dark mode works
- [x] T014 Create toast provider component at root level (`src/routes/+layout.svelte`)
- [x] T015 [P] Implement toast stacking logic for top-right corner positioning
- [x] T016 Create theme switcher store (`src/lib/stores/theme.ts`) for centralized theme state
- [x] T017 [P] Document component import patterns for developers (in README or CONTRIBUTING)
- [x] T018 Create `src/lib/components/index.ts` to export all shadcn components
- [x] T019 [P] Test toast provider setup with a test notification
- [x] T020 Commit: `feat: setup shadcn-svelte theme system and toast provider`

---

## Phase 3: User Story 1 – Seamless UI Component Replacement (P1)

### Story Goal

Replace all Skeleton UI components with shadcn-svelte equivalents without changing user-visible functionality.

### Independent Test Criteria

- ✓ All pages load without console errors or missing component warnings
- ✓ All Skeleton component imports have been replaced with shadcn-svelte equivalents
- ✓ Form validation and submission work identically
- ✓ Modals, dropdowns, and interactive elements function as before
- ✓ No visual degradation in component rendering

**Status**: 🔄 IN PROGRESS - Core components created (commit e6805a6)

- ✅ Created Button component with 6 variants and 4 sizes + href support
- ✅ Created Badge component with 5 variants
- ✅ Created Input, Textarea, Checkbox components
- ✅ Created Dialog component for modals
- ✅ Created Sheet component for slide-out drawers (4 directions)
- ✅ Created comprehensive Table component system (6 sub-components)
- ✅ Created Card component system (6 sub-components)
- ✅ Created Alert component with 4 variants
- ✅ Replaced 50+ button instances across routes and components
- 📝 COMPONENT_MAPPING.md with comprehensive audit (50+ btn, 30+ badge instances)
- 🎯 Next: Replace badge, input, textarea, checkbox classes across codebase

---

### Components Audit & Mapping

- [x] T021 [P] Audit codebase: identify all Skeleton components in use
- [x] T022 Create component replacement mapping table (Skeleton → shadcn-svelte)
- [x] T023 [P] Identify missing shadcn-svelte equivalents and plan alternatives
- [x] T024 Document component API differences and migration notes

### Button & Link Components

- [x] T025 [P] Replace Skeleton Button with shadcn-svelte Button in src/routes/
- [x] T026 [P] Replace Skeleton Link with shadcn-svelte Link (or use <a> + Tailwind)
- [x] T027 [US1] Test button interactions and state (hover, active, disabled)

### Form Components

- [x] T028 [P] Replace Skeleton Input with shadcn-svelte Input
- [x] T029 [P] Replace Skeleton Textarea with shadcn-svelte Textarea
- [ ] T030 [US1] Replace Skeleton Form components with shadcn-svelte Form (from bits-ui)
- [ ] T031 [P] Test form validation and error display
- [ ] T032 [US1] Test form submission workflow

### Modal & Overlay Components

- [x] T033 [P] Replace Skeleton Modal with shadcn-svelte Dialog
- [x] T034 [P] Replace Skeleton Drawer with shadcn-svelte Sheet
- [ ] T035 [US1] Test modal open/close and animations
- [ ] T036 [P] Test focus trapping and keyboard escape

### Data Display Components

- [x] T037 [P] Replace Skeleton Table with shadcn-svelte Table
- [x] T038 [P] Replace Skeleton Card with shadcn-svelte Card
- [ ] T039 [US1] Test table sorting and pagination
- [x] T040 [P] Replace Skeleton Badge with shadcn-svelte Badge

### Navigation & Dropdown Components

- [ ] T041 [P] Replace Skeleton Menu with shadcn-svelte DropdownMenu
- [ ] T042 [P] Replace Skeleton Navbar with shadcn-svelte components
- [ ] T043 [US1] Test dropdown interactions and keyboard navigation
- [ ] T044 [P] Test navbar responsiveness on mobile/tablet/desktop

### Form Controls (Checkboxes, Radios, Toggles)

- [ ] T045 [P] Replace Skeleton Checkbox with shadcn-svelte Checkbox
- [ ] T046 [P] Replace Skeleton Radio with shadcn-svelte RadioGroup
- [ ] T047 [US1] Replace Skeleton Toggle with shadcn-svelte Toggle
- [ ] T048 [P] Test toggle states and form binding

### Final Component Audit

- [ ] T049 [US1] Search codebase for remaining Skeleton imports and replace
- [ ] T050 Run build and verify no Skeleton-related errors
- [ ] T051 Commit: `feat: replace all skeleton components with shadcn-svelte`

---

## Phase 4: User Story 2 – Consistent Styling with Tailwind 4 (P1)

### Story Goal

Maintain visual consistency across all pages after switching libraries.

### Independent Test Criteria

- ✓ All pages render with no visual degradation
- ✓ Spacing, colors, fonts, and borders match pre-migration design
- ✓ Responsive design works across mobile, tablet, desktop breakpoints
- ✓ Dark mode and light mode color schemes are consistent

---

- [ ] T052 [P] [US2] Validate Tailwind utility classes are rendering correctly
- [ ] T053 [P] [US2] Test color palette consistency across all pages
- [ ] T054 [P] [US2] Test typography hierarchy and font sizing
- [ ] T055 [US2] Verify responsive breakpoints (sm, md, lg, xl, 2xl)
- [ ] T056 [P] [US2] Test mobile view: verify layout adapts correctly
- [ ] T057 [P] [US2] Test tablet view (768px+): verify multi-column layouts
- [ ] T058 [US2] Test dark mode: all components render with correct dark theme colors
- [ ] T059 [P] [US2] Test light mode: verify light background and text contrast
- [ ] T060 [US2] Visual regression testing: compare before/after screenshots
- [ ] T061 [P] Commit: `feat: verify tailwind styling consistency across all pages`

---

## Phase 5: User Story 3 – Component Feature Parity (P1)

### Story Goal

All interactive and complex components maintain feature parity with Skeleton equivalents.

### Independent Test Criteria

- ✓ Multi-select, datepicker, autocomplete components work identically
- ✓ All ARIA labels and keyboard navigation work correctly
- ✓ Screen reader support is present and functioning
- ✓ Accessibility tests pass (WCAG 2.1 AA)

---

- [ ] T062 [P] [US3] Test multi-select functionality and selection state
- [ ] T063 [P] [US3] Test datepicker: date selection, range selection, constraints
- [ ] T064 [US3] Test autocomplete: filtering, selection, keyboard navigation
- [ ] T065 [P] [US3] Verify ARIA labels on all interactive components
- [ ] T066 [P] [US3] Test keyboard navigation: Tab, Shift+Tab, Enter, Escape
- [ ] T067 [US3] Test with screen reader (NVDA, JAWS, VoiceOver)
- [ ] T068 [P] [US3] Run accessibility audit tool (axe, Lighthouse)
- [ ] T069 Commit: `feat: validate component feature parity and accessibility`

---

## Phase 6: User Story 4 – Smooth Developer Transition (P2)

### Story Goal

Developers can work with shadcn-svelte components using straightforward patterns.

### Independent Test Criteria

- ✓ Component documentation is clear and discoverable
- ✓ New developer can add a feature without deep framework knowledge
- ✓ Component customization (styling, behavior) follows consistent patterns
- ✓ Onboarding guide covers common tasks

---

- [ ] T070 [P] [US4] Update component documentation in code comments
- [ ] T071 [US4] Create developer onboarding guide: "Getting Started with shadcn-svelte"
- [ ] T072 [P] [US4] Document component customization patterns (props, slots, events)
- [ ] T073 [P] [US4] Add TypeScript types for all exported components
- [ ] T074 [US4] Create example components showing best practices
- [ ] T075 [P] Commit: `docs: create developer onboarding for shadcn-svelte components`

---

## Phase 7: User Story 5 – Standardized Page Headers & Navigation (P2)

### Story Goal

All pages have consistent header structure following the "My Collection" template.

### Independent Test Criteria

- ✓ All page headers follow 3-tier format: [SECTION], [Title], [Description]
- ✓ "Dashboard" text is removed from desktop headers
- ✓ Spacing, font sizing, and layout match "My Collection" template exactly

---

- [ ] T076 [P] [US5] Document "My Collection" header template (font sizes, spacing, layout)
- [ ] T077 [P] [US5] Create reusable PageHeader component with 3-tier structure
- [ ] T078 [US5] Audit all page files and identify headers to standardize
- [ ] T079 [P] [US5] Replace Budget Tracking page header with PageHeader component
- [ ] T080 [P] [US5] Replace Collection page header with PageHeader component
- [ ] T081 [US5] Replace Wishlist page header with PageHeader component
- [ ] T082 [P] [US5] Replace Inventory page header with PageHeader component
- [ ] T083 [P] [US5] Replace Maintenance page header with PageHeader component
- [ ] T084 [US5] Remove "Dashboard" text from all page headers
- [ ] T085 [P] [US5] Test header consistency: spacing and typography match template
- [ ] T086 Commit: `feat: standardize page headers and remove dashboard label`

---

## Phase 8: User Story 6 – Consistent Error Feedback (P2)

### Story Goal

Error toast notifications render consistently in top-right corner with identical UX.

### Independent Test Criteria

- ✓ Toast notifications appear in top-right corner
- ✓ Styling, colors, and typography match Skeleton version exactly
- ✓ Auto-dismiss timing and animation match current implementation
- ✓ Multiple toasts stack properly with correct spacing and z-index

---

- [ ] T087 [P] [US6] Review current Skeleton toast implementation: timing, animation, styling
- [ ] T088 [P] [US6] Configure shadcn-svelte toast provider with top-right positioning
- [ ] T089 [US6] Test toast appearance: validate colors and typography
- [ ] T090 [P] [US6] Test toast animations: fade in, auto-dismiss, slide out
- [ ] T091 [P] [US6] Test auto-dismiss timing (default 5 seconds or configurable)
- [ ] T092 [US6] Test multiple toasts: verify stacking and z-index ordering
- [ ] T093 [P] [US6] Test toast on all pages: Budget, Collection, Wishlist, Inventory, Maintenance
- [ ] T094 [P] [US6] Verify toast accessibility: ARIA live region, screen reader support
- [ ] T095 Commit: `feat: implement consistent error toast notifications in top-right`

---

## Phase 9: Polish & Cross-Cutting Concerns

### Phase Goal

Final validation, performance optimization, and production readiness.

### Independent Test Criteria

- ✓ All builds succeed with no warnings
- ✓ Performance meets baseline (no >10% degradation)
- ✓ All success criteria pass (SC-001 through SC-011)
- ✓ Code review completed, no critical issues

---

- [ ] T096 [P] Run full test suite: `pnpm test:ci`
- [ ] T097 [P] Run linter and formatter: `pnpm lint && pnpm format`
- [ ] T098 Run Tailwind CSS purge to optimize output size
- [ ] T099 [P] Benchmark page load performance (Lighthouse, WebPageTest)
- [ ] T100 [P] Check for console errors/warnings in dev and production builds
- [ ] T101 Verify dark mode switcher persists user preference
- [ ] T102 [P] Test on multiple browsers: Chrome, Firefox, Safari, Edge
- [ ] T103 Test Tauri desktop app build: `pnpm tauri build`
- [ ] T104 [P] Visual regression testing: final comparison with baseline
- [ ] T105 Accessibility audit: run WCAG 2.1 AA compliance check
- [ ] T106 [P] Update CHANGELOG.md with migration notes
- [ ] T107 Create final PR with summary of changes and testing evidence
- [ ] T108 [P] Commit: `chore: final polish and cross-cutting validation`

---

## Summary

| Phase                               | Count        | Description                                                   |
| ----------------------------------- | ------------ | ------------------------------------------------------------- |
| Phase 1: Setup                      | 9 tasks      | Install shadcn-svelte, remove Skeleton, configure build       |
| Phase 2: Foundational               | 11 tasks     | Theme system, toast provider, component structure             |
| Phase 3: US1 – Components           | 30 tasks     | Replace all UI components across forms, modals, tables, nav   |
| Phase 4: US2 – Styling              | 10 tasks     | Validate Tailwind consistency, responsive, dark/light modes   |
| Phase 5: US3 – Feature Parity       | 8 tasks      | Complex components, accessibility, keyboard navigation        |
| Phase 6: US4 – Developer Transition | 6 tasks      | Documentation, onboarding, examples                           |
| Phase 7: US5 – Headers              | 10 tasks     | Standardize page headers, remove Dashboard label              |
| Phase 8: US6 – Error Feedback       | 9 tasks      | Toast notifications positioning and styling                   |
| Phase 9: Polish                     | 13 tasks     | Testing, performance, accessibility, cross-browser validation |
| **TOTAL**                           | **62 tasks** | **End-to-end migration**                                      |

---

## Parallelizable Task Groups

### Group A: Component Installation (T025-T048)

- Start after Phase 2 completes
- Can be split across multiple developers (different components/pages)
- Typical dependency: Component must exist in shadcn-svelte library

### Group B: Testing & Validation (T052-T095)

- Can run concurrently with component replacement (on merged changes)
- US2 (styling) and US3 (feature parity) tests are most critical
- Accessibility tests (T067-T068, T094) should be continuous

### Group C: Polish (T096-T108)

- Blocks release; must complete after all feature tasks
- Can run partially in parallel (linting, formatting, builds)

---

## Estimated Timeline

- **Phase 1**: 1-2 hours (setup only)
- **Phase 2**: 3-4 hours (infrastructure setup, testing)
- **Phases 3-8** (Feature Development): 40-60 hours (component replacement, testing, fixes)
  - Component replacement (US1): 20-30 hours
  - Styling validation (US2): 5-8 hours
  - Feature parity (US3): 5-8 hours
  - Developer docs (US4): 2-3 hours
  - Headers (US5): 3-5 hours
  - Toasts (US6): 3-5 hours
- **Phase 9** (Polish): 5-8 hours (final testing, optimization, PR review)

**Total Estimated Effort**: 50-75 hours (7-10 working days with focused effort)
