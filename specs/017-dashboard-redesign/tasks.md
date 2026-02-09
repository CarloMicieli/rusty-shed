---
description: 'Task breakdown for Dashboard Collector\'s Overview Redesign'
---

# Tasks: Dashboard Collector's Overview Redesign

**Input**: Design documents from `/specs/017-dashboard-redesign/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

This is a Tauri 2 + SvelteKit project:

- Backend: `src-tauri/src/`
- Frontend: `src/lib/`, `src/routes/`
- Messages: `messages/en.json`, `messages/it.json`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and i18n message keys

- [x] T001 [P] Add English i18n message keys for purchase groups in messages/en.json (11 keys: dashboard_purchase_group_title, dashboard_purchase_on, dashboard_seller_from, dashboard_seller_unknown, dashboard_purchase_notes, dashboard_more_items, dashboard_condition_new, dashboard_condition_preowned, dashboard_condition_unknown, dashboard_empty_purchases, dashboard_add_first_purchase)
- [x] T002 [P] Add Italian i18n message keys for purchase groups in messages/it.json (translations for all 11 keys)
- [x] T003 Regenerate Paraglide types by running pnpm prepare

**Checkpoint**: i18n messages ready for use in components

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core backend domain entities that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 [P] Create PurchaseCondition enum in src-tauri/src/dashboard/domain/purchase_condition.rs with variants (New, PreOwned, Unknown) and From<Option<String>> implementation
- [x] T005 [P] Create ModelCard entity in src-tauri/src/dashboard/domain/model_card.rs with fields (id, thumbnail_path, manufacturer, product_code, condition, description)
- [x] T006 Create PurchaseGroup entity in src-tauri/src/dashboard/domain/purchase_group.rs with fields (id, purchase_date, seller_name, notes, model_cards, total_count)
- [x] T007 Extend DashboardSummary entity in src-tauri/src/dashboard/domain/dashboard_summary.rs by adding purchase_groups field Vec<PurchaseGroup>
- [x] T008 Export new domain types in src-tauri/src/dashboard/domain/mod.rs (PurchaseCondition, ModelCard, PurchaseGroup)
- [x] T009 Verify domain layer compiles with cargo check in src-tauri/

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - View Recent Acquisitions Grouped by Purchase Event (Priority: P1) 🎯 MVP

**Goal**: Display 2-3 most recent purchase groups with date/seller/notes metadata and up to 3 model cards per group

**Independent Test**: Add 2-3 models from different purchase events (different dates/sellers) and verify they appear grouped chronologically with purchase metadata on dashboard

### Backend Implementation for User Story 1

- [x] T010 [P] [US1] Create PurchaseGroupRow entity in src-tauri/src/dashboard/infrastructure/entities.rs with fields (purchase_date, seller_id, seller_name, notes, model_count)
- [x] T011 [P] [US1] Create ModelCardRow entity in src-tauri/src/dashboard/infrastructure/entities.rs with fields (model_id, manufacturer_id, manufacturer_name, product_code, description, image_path, purchase_condition, purchase_date, seller_id)
- [x] T012 [US1] Implement TryFrom<(PurchaseGroupRow, Vec<ModelCardRow>)> for PurchaseGroup in src-tauri/src/dashboard/infrastructure/entities.rs with purchase group ID generation and LIMIT 3 model cards
- [x] T013 [US1] Implement TryFrom<ModelCardRow> for ModelCard in src-tauri/src/dashboard/infrastructure/entities.rs with RailwayModelId parsing and PurchaseCondition conversion
- [x] T014 [US1] Add fetch_purchase_groups method to DashboardRepository in src-tauri/src/dashboard/infrastructure/dashboard_repository.rs with GROUP BY (purchase_date, seller_id), ORDER BY purchase_date DESC, LIMIT 3 groups
- [x] T015 [US1] Add nested query in fetch_purchase_groups to fetch up to 3 model cards per group in src-tauri/src/dashboard/infrastructure/dashboard_repository.rs with JOIN across purchase_infos, collection_items, railway_models, manufacturers tables
- [x] T016 [US1] Update get_dashboard_summary method in src-tauri/src/dashboard/infrastructure/dashboard_repository.rs to call fetch_purchase_groups and include in DashboardSummary
- [x] T017 [US1] Run cargo clippy and cargo fmt on backend code
- [x] T018 [US1] Regenerate TypeScript types by running pnpm run tauri:gen-types to export PurchaseGroup, ModelCard, PurchaseCondition to src/lib/bindings.ts

### Frontend Implementation for User Story 1

- [x] T019 [P] [US1] Create PurchaseGroupCard component in src/lib/features/dashboard/components/PurchaseGroupCard.svelte with purchase header (date with 📅, seller with 🏪, notes in italic) and model cards grid (grid-cols-1 md:grid-cols-3 gap-4) using industrial-luxe styling (border-white/10, bg-black/20, text-zinc-400)
- [x] T020 [P] [US1] Add "+N more models..." indicator logic in PurchaseGroupCard component when group.totalCount > group.modelCards.length
- [x] T021 [P] [US1] Implement date formatting in PurchaseGroupCard using date-fns format(new Date(isoDate), 'MMMM d, yyyy')
- [x] T022 [P] [US1] Add seller name fallback to "Unknown source" message in PurchaseGroupCard when sellerName is null
- [x] T023 [US1] Export PurchaseGroupCard in src/lib/features/dashboard/index.ts
- [x] T024 [US1] Update dashboard page in src/routes/my-dashboard/+page.svelte to replace "Recently Added" section with purchase groups section using PurchaseGroupCard component
- [x] T025 [US1] Add empty state handling in dashboard page when purchaseGroups.length === 0 with "No recent acquisitions" message and "Add your first model" button
- [x] T026 [US1] Add loading skeleton in dashboard page with 2 skeleton cards while dashboard.isLoading
- [x] T027 [US1] Run pnpm lint and pnpm check on frontend code

**Checkpoint**: At this point, User Story 1 should be fully functional - dashboard displays recent purchase groups with metadata

---

## Phase 4: User Story 2 - Quick Visual Recognition of Individual Models (Priority: P1) 🎯 MVP

**Goal**: Display each model with prominent thumbnail, manufacturer, product code, condition badge, and truncated description in horizontal card layout

**Independent Test**: View any single model card and verify it displays 16:9 aspect ratio thumbnail (or placeholder), manufacturer name in orange-400, product code, condition badge (top-right corner), and description (truncated to 100 characters if needed)

### Frontend Implementation for User Story 2

- [x] T028 [P] [US2] Create ModelCard component in src/lib/features/dashboard/components/ModelCard.svelte as horizontal button with flex gap-3 layout, 16:9 aspect ratio thumbnail (w-40 aspect-video), manufacturer in text-orange-400, product code, and description
- [x] T029 [P] [US2] Add thumbnail image handling in ModelCard with convertFileSrc for valid paths or TrainFront icon placeholder for null paths using lucide-svelte
- [x] T030 [P] [US2] Add condition badge rendering in ModelCard with absolute positioning (top-2 right-2), variant mapping (New→default, PreOwned→secondary, Unknown→outline), using shadcn-svelte Badge component
- [x] T031 [P] [US2] Implement description truncation in ModelCard to show ~100 characters with graceful line-clamp-2 CSS
- [x] T032 [P] [US2] Apply industrial-luxe styling to ModelCard (bg-zinc-900/50, hover:bg-zinc-800/70, border-zinc-800, text-zinc-300, transition-colors duration-200)
- [x] T033 [US2] Export ModelCard in src/lib/features/dashboard/index.ts
- [x] T034 [US2] Import and use ModelCard in PurchaseGroupCard component grid section in src/lib/features/dashboard/components/PurchaseGroupCard.svelte
- [x] T035 [US2] Verify all model cards maintain consistent height with flex layout (not grid) in PurchaseGroupCard
- [x] T036 [US2] Run pnpm lint and pnpm check on frontend code

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently - dashboard displays purchase groups with visually rich horizontal model cards

---

## Phase 5: User Story 3 - Navigate to Full Model Details (Priority: P2)

**Goal**: Enable click navigation from any model card to its full specification page

**Independent Test**: Click any model card on dashboard and verify navigation to /railway-model/[id] page with full model details

### Frontend Implementation for User Story 3

- [x] T037 [P] [US3] Add onclick navigation handler to ModelCard button in src/lib/features/dashboard/components/ModelCard.svelte using goto(resolve('/railway-model/' + model.id))
- [x] T038 [P] [US3] Import goto from $app/navigation and resolve from $lib/utils/paths in ModelCard component
- [x] T039 [P] [US3] Add cursor-pointer class and hover state to ModelCard button for visual affordance
- [ ] T040 [US3] Implement scroll position preservation in src/routes/my-dashboard/+page.svelte using sessionStorage.setItem('dashboard-scroll', window.scrollY.toString()) in afterNavigate hook
- [ ] T041 [US3] Add scroll position restoration in dashboard page using onMount to read sessionStorage.getItem('dashboard-scroll') and call window.scrollTo(0, scrollY)
- [ ] T042 [US3] Test navigation flow: dashboard → model details → browser back → dashboard (scroll position preserved)
- [x] T043 [US3] Run pnpm lint and pnpm check on frontend code

**Checkpoint**: All user stories (US1, US2, US3) should now work together - full navigation flow with scroll preservation

---

## Phase 6: User Story 4 - Access Complete Collection Inventory (Priority: P3)

**Goal**: Provide clear navigation from dashboard to full collection table view

**Independent Test**: Click "View All" link on dashboard and verify navigation to /my-collection page with full collection table

### Frontend Implementation for User Story 4

- [x] T044 [P] [US4] Add "View All" link in dashboard page header in src/routes/my-dashboard/+page.svelte with href={resolve('/my-collection')} and styling (text-orange-400, hover:underline)
- [x] T045 [P] [US4] Verify navigation to collection page works and displays full collection table
- [ ] T046 [P] [US4] Add breadcrumb or back link on collection page header to return to dashboard (if not already present)
- [ ] T047 [US4] Test navigation flow: dashboard → "View All" → collection page → back to dashboard
- [x] T048 [US4] Run pnpm lint and pnpm check on frontend code

**Checkpoint**: All user stories should now be independently functional with complete navigation between dashboard and collection views

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories, testing, and performance validation

### Backend Testing

- [ ] T049 [P] Create test helper setup_test_pool in src-tauri/src/dashboard/infrastructure/dashboard_repository_test.rs for SQLite test database
- [ ] T050 [P] Create test helper seed_4_purchase_groups in test file to insert 4 purchase events with different dates/sellers
- [ ] T051 [P] Create test helper seed_purchase_with_5_models in test file to insert 1 purchase with 5 models
- [ ] T052 [P] Write test_fetch_purchase_groups_limits_to_3 to verify LIMIT 3 groups returned in src-tauri/src/dashboard/infrastructure/dashboard_repository_test.rs
- [ ] T053 [P] Write test_model_cards_limited_to_3_per_group to verify max 3 cards per group and total_count=5 in test file
- [ ] T054 [P] Write test_purchase_groups_sorted_chronologically to verify DESC order by purchase_date in test file
- [ ] T055 [P] Write test_model_card_row_conversion to verify TryFrom<ModelCardRow> for ModelCard in src-tauri/src/dashboard/infrastructure/entities_test.rs
- [ ] T056 Run cargo test in src-tauri/ and verify all tests pass

### Frontend Testing

- [ ] T057 [P] Create ModelCard component test in src/lib/features/dashboard/components/**tests**/ModelCard.test.ts with Vitest and @testing-library/svelte
- [ ] T058 [P] Write test case "renders model information correctly" to verify manufacturer, product code, condition badge display
- [ ] T059 [P] Write test case "displays placeholder when thumbnail is null" to verify TrainFront icon shown
- [ ] T060 [P] Write test case "navigates to model details on click" to verify onclick handler called with correct ID
- [ ] T061 [P] Create PurchaseGroupCard component test in src/lib/features/dashboard/components/**tests**/PurchaseGroupCard.test.ts
- [ ] T062 [P] Write test case "renders purchase metadata correctly" to verify date, seller, notes display
- [ ] T063 [P] Write test case "displays +N more indicator when totalCount > 3" to verify counter logic
- [ ] T064 [P] Write test case "does not show +N more when totalCount <= 3" to verify conditional rendering
- [ ] T065 Run pnpm test and verify all frontend tests pass

### Responsive Design Validation

- [ ] T066 [P] Test dashboard layout at 320px viewport (mobile) and verify vertical stacking of cards
- [ ] T067 [P] Test dashboard layout at 768px viewport (tablet) and verify grid-cols-1 md:grid-cols-3 breakpoint
- [ ] T068 [P] Test dashboard layout at 1920px viewport (desktop) and verify 3-column grid for model cards
- [ ] T069 Verify horizontal ModelCard layout (flex gap-3) maintains consistent height across all breakpoints

### Performance Validation

- [ ] T070 Run cargo clippy -- -D warnings in src-tauri/ and fix any warnings
- [ ] T071 Run cargo fmt in src-tauri/ and verify formatting
- [ ] T072 Run pnpm lint and fix any frontend linting errors
- [ ] T073 Run pnpm format and verify Prettier formatting
- [ ] T074 Run pnpm check (svelte-check) and fix any TypeScript errors
- [ ] T075 Open DevTools Network tab and verify dashboard load time < 2 seconds with 30 models
- [ ] T076 Verify purchase grouping query executes in < 200ms using query profiling
- [ ] T077 Verify total query count <= 10 queries per dashboard load

### User Acceptance Testing

- [ ] T078 Manual test: Add 3 models from different purchase dates/sellers and verify correct grouping
- [ ] T079 Manual test: Add 5 models in single purchase and verify "+2 more models..." indicator
- [ ] T080 Manual test: Click model card and verify navigation to model details page
- [ ] T081 Manual test: Navigate back from model details and verify scroll position preserved
- [ ] T082 Manual test: Verify empty state displays "No recent acquisitions" when no models exist
- [ ] T083 Manual test: Test responsive layout on mobile device (320px-768px)
- [ ] T084 Manual test: Verify all i18n strings display correctly in both EN and IT locales

### Documentation

- [ ] T085 [P] Take screenshots of dashboard with purchase groups for specs/017-dashboard-redesign/ directory
- [ ] T086 [P] Update CHANGELOG.md with "Added: Dashboard Collector's Overview with purchase-grouped model cards"
- [ ] T087 [P] Update docs/FEATURE_IMPLEMENTATION.md with dashboard redesign implementation notes
- [ ] T088 Add migration notes to plan.md about deprecated recent_items field in DashboardSummary

---

## Task Summary

- **Total Tasks**: 88
- **Phase 1 (Setup)**: 3 tasks
- **Phase 2 (Foundational)**: 6 tasks
- **Phase 3 (User Story 1)**: 18 tasks (Backend: 9, Frontend: 9)
- **Phase 4 (User Story 2)**: 9 tasks (Frontend: 9)
- **Phase 5 (User Story 3)**: 7 tasks (Frontend: 7)
- **Phase 6 (User Story 4)**: 5 tasks (Frontend: 5)
- **Phase 7 (Polish)**: 40 tasks (Testing: 21, Validation: 9, UAT: 7, Docs: 4)

## Parallel Execution Opportunities

### Setup Phase (All Parallel)

- T001 and T002 can run in parallel (different files)
- T003 runs after both complete

### Foundational Phase

- T004 and T005 can run in parallel (independent entities)
- T006 runs after T005 (depends on ModelCard)
- T007 runs after T006 (depends on PurchaseGroup)

### User Story 1 (Backend)

- T010 and T011 can run in parallel (different row types)
- T012 and T013 can run in parallel after T010-T011 (different conversions)
- T014-T016 are sequential (repository method implementation)
- T017 and T018 can run in parallel after backend complete

### User Story 1 (Frontend)

- T019-T022 can all run in parallel (independent PurchaseGroupCard implementation)
- T024-T026 run after T023 (need exported component)

### User Story 2

- T028-T032 can all run in parallel (independent ModelCard implementation)
- T034-T035 run after T033 (need exported component)

### User Story 3 & 4

- T037-T039 can run in parallel (ModelCard navigation)
- T044-T046 can run in parallel (View All link)

### Polish Phase

- All test creation tasks (T049-T051, T057-T064) can run in parallel
- All test execution tasks (T052-T056, T058-T065) run after respective test creation
- All responsive validation (T066-T069) can run in parallel
- All performance validation (T070-T077) can run in parallel
- All UAT tasks (T078-T084) can run in parallel
- All documentation tasks (T085-T088) can run in parallel

## Dependencies Between User Stories

- **US2 depends on US1**: ModelCard component must be integrated into PurchaseGroupCard
- **US3 depends on US2**: Navigation requires ModelCard to be implemented
- **US4 is independent**: View All link can be implemented separately

## Implementation Strategy

### MVP Scope (Recommended First Release)

Implement **User Story 1 ONLY** for initial release:

- Displays purchase groups with basic metadata
- Shows model information without visual polish
- Provides core collector's overview value
- Estimated: 4 hours (T001-T027)

### Full Release (All User Stories)

Complete all phases for full feature:

- Visual model cards with thumbnails (US2)
- Navigation and scroll preservation (US3)
- Collection view access (US4)
- Complete testing and polish
- Estimated: 11 hours total

## Time Estimates by Phase

| Phase                     | Duration  | Cumulative   |
| ------------------------- | --------- | ------------ |
| Phase 1: Setup            | 30 min    | 30 min       |
| Phase 2: Foundational     | 1 hour    | 1.5 hours    |
| Phase 3: User Story 1     | 3.5 hours | 5 hours      |
| Phase 4: User Story 2     | 2 hours   | 7 hours      |
| Phase 5: User Story 3     | 1 hour    | 8 hours      |
| Phase 6: User Story 4     | 30 min    | 8.5 hours    |
| Phase 7: Polish & Testing | 2.5 hours | **11 hours** |

**Total Estimated Time**: 11 hours (1.5 developer days)

---

## Common Pitfalls & Solutions

### Backend

- **Pitfall**: Purchase groups empty due to NULL seller_id grouping
  - **Solution**: Use `(pi.seller_id = $2 OR (pi.seller_id IS NULL AND $2 IS NULL))` in WHERE clause

- **Pitfall**: ModelCard conversion fails with RailwayModelId parse error
  - **Solution**: Ensure model_id column uses correct format `trn:railway-model:{manufacturer}:{product_code}`

### Frontend

- **Pitfall**: Images don't load in Tauri
  - **Solution**: Always use `convertFileSrc()` from `@tauri-apps/api/core` for file paths

- **Pitfall**: Grid layout breaks on mobile
  - **Solution**: Use `grid-cols-1 md:grid-cols-3` (not `sm:grid-cols-3`) for correct breakpoint

- **Pitfall**: Scroll position not preserved
  - **Solution**: Use `sessionStorage` (not `localStorage`) and clear after restoration

### Testing

- **Pitfall**: Integration tests fail due to missing test database
  - **Solution**: Create in-memory SQLite with `:memory:` connection string

- **Pitfall**: Component tests fail with "module not found" for bindings
  - **Solution**: Run `pnpm run tauri:gen-types` before test execution

---

## Definition of Done

Each user story is considered complete when:

✅ All tasks for that story are checked off
✅ Backend tests pass for that story's domain logic
✅ Frontend tests pass for that story's components
✅ Manual testing confirms acceptance scenarios
✅ Code is formatted (cargo fmt, pnpm format)
✅ Code passes linting (cargo clippy, pnpm lint)
✅ TypeScript types are regenerated (specta)
✅ i18n strings work in both EN and IT locales
✅ Responsive design works (320px-2560px)
✅ Performance targets met (<2s load, <200ms queries)

---

## Next Steps

1. **Start with Phase 1**: Add i18n message keys (T001-T003)
2. **Build Foundation**: Implement domain entities (T004-T009)
3. **Implement MVP**: Complete User Story 1 (T010-T027) for first working version
4. **Add Visual Polish**: Implement User Story 2 (T028-T036) for better UX
5. **Enable Navigation**: Implement User Story 3 (T037-T043) for interaction
6. **Complete Feature**: Implement User Story 4 (T044-T048) and Polish (T049-T088)

Ready to begin implementation! 🚀
