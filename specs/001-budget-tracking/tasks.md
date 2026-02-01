# Tasks: Budget Tracking

**Input**: Design documents from `/specs/001-budget-tracking/`  
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Tests**: Not explicitly requested in the feature specification. Implementation tasks only.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Backend**: `src-tauri/src/` (Rust/Tauri)
- **Frontend**: `src/` (Svelte/TypeScript)
- **Migrations**: `src-tauri/migrations/`

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Create module structure and database schema

- [ ] T001 Create database migration file at src-tauri/migrations/0007_create_budget_schema.sql with budget_config and extra_budgets tables
- [ ] T002 Create budget module structure with mod.rs files at src-tauri/src/budget/{mod.rs,domain/mod.rs,application/mod.rs,infrastructure/mod.rs,interface/mod.rs}
- [ ] T003 Register budget module in src-tauri/src/lib.rs
- [ ] T004 [P] Create frontend feature directory structure at src/lib/features/budget/{index.ts,services/,components/}
- [ ] T005 [P] Add budget-related Paraglide message keys to messages/en.json and messages/it.json

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain types and infrastructure that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Domain Layer (src-tauri/src/budget/domain/)

- [ ] T006 Create BudgetMode enum in src-tauri/src/budget/domain/budget_mode.rs with Yearly/Monthly variants and sqlx::Type derive
- [ ] T007 Create BudgetConfigId newtype in src-tauri/src/budget/domain/budget_config_id.rs
- [ ] T008 Create ExtraBudgetId newtype in src-tauri/src/budget/domain/extra_budget_id.rs
- [ ] T009 [P] Create BudgetEvent enum in src-tauri/src/budget/domain/budget_events.rs with BudgetConfigured, ExtraBudgetAdded, ExtraBudgetRemoved, AnnualResetPerformed variants
- [ ] T010 Create BudgetConfiguration aggregate in src-tauri/src/budget/domain/budget_configuration.rs with monthly_amount(), yearly_amount() methods and pending_events
- [ ] T011 [P] Create ExtraBudgetEntry value object in src-tauri/src/budget/domain/extra_budget_entry.rs
- [ ] T012 Create BudgetRepository trait in src-tauri/src/budget/domain/repository.rs

### Infrastructure Layer (src-tauri/src/budget/infrastructure/)

- [ ] T013 Create BudgetConfigRow and ExtraBudgetRow structs in src-tauri/src/budget/infrastructure/entities.rs
- [ ] T014 Create row_to_budget_config and row_to_extra_budget mappers in src-tauri/src/budget/infrastructure/mappers.rs
- [ ] T015 Implement SQL queries for budget_config CRUD in src-tauri/src/budget/infrastructure/database.rs
- [ ] T016 Implement SQL queries for extra_budgets CRUD in src-tauri/src/budget/infrastructure/database.rs
- [ ] T017 Implement BudgetRepository trait for SqliteUnitOfWork in src-tauri/src/budget/infrastructure/repositories.rs

### Interface Layer (src-tauri/src/budget/interface/)

- [ ] T018 Create transport DTOs (SetBudgetConfigArgs, AddExtraBudgetArgs, etc.) in src-tauri/src/budget/interface/command_args.rs with validator::Validate derives

**Checkpoint**: Foundation ready - Run `cargo check` and `cargo clippy` to verify compilation

---

## Phase 3: User Story 1 - Set Monthly/Yearly Budget (Priority: P1) 🎯 MVP

**Goal**: Users can configure their hobby budget as yearly or monthly amount

**Independent Test**: Open budget configuration, enter $1200 yearly, verify it saves and shows $100/month on dashboard

### Backend Implementation

- [ ] T019 [US1] Implement SetBudgetUseCase in src-tauri/src/budget/application/set_budget.rs with input validation and domain event emission
- [ ] T020 [US1] Create get_budget_config Tauri command handler in src-tauri/src/budget/interface/command_handlers.rs
- [ ] T021 [US1] Create set_budget_config Tauri command handler in src-tauri/src/budget/interface/command_handlers.rs
- [ ] T022 [US1] Register budget commands in Tauri command builder (src-tauri/src/lib.rs or main.rs)

### Frontend Implementation

- [ ] T023 [US1] Create budget.service.ts in src/lib/features/budget/services/ with getBudgetConfig() and setBudgetConfig() invoke wrappers
- [ ] T024 [US1] Create BudgetState.svelte.ts controller in src/lib/features/budget/ with $state for config and load/save methods
- [ ] T025 [US1] Create BudgetConfigForm.svelte component in src/lib/features/budget/components/ with yearly/monthly toggle and amount input
- [ ] T026 [US1] Create budget management route at src/routes/my-budget/+page.svelte with BudgetConfigForm
- [ ] T027 [US1] Add navigation link to my-budget in app navigation

**Checkpoint**: User Story 1 complete - Can set and persist budget configuration

---

## Phase 4: User Story 2 - Track Monthly Spending with Roll-over (Priority: P1) 🎯 MVP

**Goal**: Calculate and display monthly budget with rollover from previous months

**Independent Test**: Set $100/month budget, record $80 in purchases, verify next month shows $120 available

### Backend Implementation

- [ ] T028 [US2] Create MonthlyBudgetRecord read model in src-tauri/src/budget/domain/monthly_budget_record.rs with available(), remaining(), remaining_percentage() methods
- [ ] T029 [US2] Create MonthStatus enum (Projected/InProgress/Completed) in src-tauri/src/budget/domain/month_status.rs
- [ ] T030 [US2] Implement calculate_rollover_chain function in src-tauri/src/budget/application/budget_query.rs that derives rollover from spending data
- [ ] T031 [US2] Implement SQL query to aggregate monthly spending from collection_items.purchase_info in src-tauri/src/budget/infrastructure/database.rs
- [ ] T032 [US2] Create get_monthly_budget_records Tauri command in src-tauri/src/budget/interface/command_handlers.rs
- [ ] T033 [US2] Implement annual reset check in budget query (compare last_reset_year with current year)

### Frontend Implementation

- [ ] T034 [US2] Add getMonthlyBudgetRecords() to src/lib/features/budget/services/budget.service.ts
- [ ] T035 [US2] Add monthlyRecords state and derived calculations to BudgetState.svelte.ts
- [ ] T036 [US2] Create BudgetTable.svelte component in src/lib/features/budget/components/ showing 12-month breakdown with rollover columns

**Checkpoint**: User Story 2 complete - Rollover calculations work correctly

---

## Phase 5: User Story 3 - View Budget Status on Dashboard (Priority: P1) 🎯 MVP

**Goal**: Display budget widgets (donut, bar chart, heatmap) on dashboard

**Independent Test**: Configure budget, make purchases, verify dashboard shows remaining %, monthly bars, and activity heatmap

### Backend Implementation

- [ ] T037 [US3] Create BudgetDashboardSummary, MonthlySpendingPoint, QuarterlyActivityPoint DTOs in src-tauri/src/budget/domain/
- [ ] T038 [US3] Create SpendingLevel enum (None/Low/Medium/High) in src-tauri/src/budget/domain/spending_level.rs
- [ ] T039 [US3] Implement get_budget_dashboard query in src-tauri/src/budget/application/budget_query.rs combining donut/bar/heatmap data
- [ ] T040 [US3] Create get_budget_dashboard Tauri command in src-tauri/src/budget/interface/command_handlers.rs

### Frontend Implementation

- [ ] T041 [US3] Add getBudgetDashboard() to src/lib/features/budget/services/budget.service.ts
- [ ] T042 [US3] Add dashboard state to BudgetState.svelte.ts with $derived for chart data
- [ ] T043 [US3] Create BudgetDonutChart.svelte in src/lib/features/budget/components/ with green→yellow→red gradient based on remaining %
- [ ] T044 [US3] Create YearlySpendingChart.svelte in src/lib/features/budget/components/ with 12 bars and horizontal budget goal line
- [ ] T045 [US3] Create ActivityHeatmap.svelte in src/lib/features/budget/components/ with 5-year quarterly grid
- [ ] T046 [US3] Integrate budget widgets into src/routes/my-dashboard/+page.svelte

**Checkpoint**: User Story 3 complete - Dashboard shows all three budget visualizations

---

## Phase 6: User Story 4 - Add Extra Budget (Priority: P2)

**Goal**: Allow users to inject one-time funds into specific months

**Independent Test**: Add $50 extra budget to March, verify March available budget increases by $50

### Backend Implementation

- [ ] T047 [US4] Implement AddExtraBudgetUseCase in src-tauri/src/budget/application/add_extra_budget.rs with validation
- [ ] T048 [US4] Create add_extra_budget Tauri command in src-tauri/src/budget/interface/command_handlers.rs
- [ ] T049 [US4] Create remove_extra_budget Tauri command in src-tauri/src/budget/interface/command_handlers.rs
- [ ] T050 [US4] Create get_extra_budgets Tauri command in src-tauri/src/budget/interface/command_handlers.rs

### Frontend Implementation

- [ ] T051 [US4] Add addExtraBudget(), removeExtraBudget(), getExtraBudgets() to budget.service.ts
- [ ] T052 [US4] Add extra budget state management to BudgetState.svelte.ts
- [ ] T053 [US4] Create ExtraBudgetModal.svelte in src/lib/features/budget/components/ with amount input and reason field
- [ ] T054 [US4] Add "Add Extra Budget" button to BudgetTable.svelte that opens modal for selected month

**Checkpoint**: User Story 4 complete - Extra budget entries can be added and removed

---

## Phase 7: User Story 5 - Budget Management Page (Priority: P2)

**Goal**: Provide detailed budget breakdown table and historical archive

**Independent Test**: Navigate to my-budget page, verify 12-month table and 5-year accordion display correctly

### Backend Implementation (mostly done in earlier phases)

- [ ] T055 [US5] Verify get_monthly_budget_records returns data for specified year (not just current)

### Frontend Implementation

- [ ] T056 [US5] Enhance BudgetTable.svelte with status column (Projected/In-Progress/Completed) and styling
- [ ] T057 [US5] Create HistoricalArchive.svelte accordion component in src/lib/features/budget/components/ showing past 5 years
- [ ] T058 [US5] Add year toggle/selector to my-budget page header
- [ ] T059 [US5] Integrate HistoricalArchive.svelte into src/routes/my-budget/+page.svelte

**Checkpoint**: User Story 5 complete - Full budget management page with current year and history

---

## Phase 8: User Story 6 - Track Spending by Category (Priority: P2)

**Goal**: Show spending breakdown by category in quarterly summaries

**Independent Test**: Record purchases in different categories, click on Q1 in heatmap, verify modal shows category breakdown

### Backend Implementation

- [ ] T060 [US6] Create QuarterlySummary and CategorySpending DTOs in src-tauri/src/budget/domain/quarterly_summary.rs
- [ ] T061 [US6] Create Quarter enum (Q1/Q2/Q3/Q4) in src-tauri/src/budget/domain/quarter.rs
- [ ] T062 [US6] Implement SQL query for quarterly spending with category breakdown in src-tauri/src/budget/infrastructure/database.rs
- [ ] T063 [US6] Implement get_quarterly_summaries in src-tauri/src/budget/application/historical_query.rs
- [ ] T064 [US6] Create get_quarterly_summaries Tauri command in src-tauri/src/budget/interface/command_handlers.rs

### Frontend Implementation

- [ ] T065 [US6] Add getQuarterlySummaries() to budget.service.ts
- [ ] T066 [US6] Add quarterly summary state to BudgetState.svelte.ts
- [ ] T067 [US6] Create QuarterlySummaryModal.svelte in src/lib/features/budget/components/ with category breakdown chart
- [ ] T068 [US6] Add click handler to ActivityHeatmap.svelte to open QuarterlySummaryModal for selected quarter

**Checkpoint**: User Story 6 complete - Category breakdowns visible in quarterly summaries

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final integration, cleanup, and verification

- [ ] T069 [P] Run cargo fmt on all budget module files
- [ ] T070 [P] Run cargo clippy and fix any warnings in src-tauri/src/budget/
- [ ] T071 [P] Run pnpm format on all frontend budget files
- [ ] T072 [P] Run pnpm lint and fix any errors in src/lib/features/budget/
- [ ] T073 Run pnpm check to verify TypeScript types
- [ ] T074 Verify all Paraglide messages are used (no hardcoded strings in components)
- [ ] T075 Update src/lib/features/budget/README.md to document the implemented feature
- [ ] T076 Run quickstart.md validation checklist

---

## Dependencies & Execution Order

### Phase Dependencies

```
Phase 1 (Setup) ────────────────────────────────────────────────────────►
                │
                ▼
Phase 2 (Foundational) ─────────────────────────────────────────────────►
                │
                ├──────────────────┬─────────────────┐
                ▼                  ▼                 ▼
Phase 3 (US1)     Phase 4 (US2)      Phase 5 (US3)
Set Budget        Rollover           Dashboard
    │                 │                  │
    ▼                 ▼                  ▼
                    MVP COMPLETE
                         │
    ┌────────────────────┼────────────────────┐
    ▼                    ▼                    ▼
Phase 6 (US4)     Phase 7 (US5)        Phase 8 (US6)
Extra Budget      Management Page      Categories
                         │
                         ▼
              Phase 9 (Polish)
```

### User Story Dependencies

| Story                 | Depends On                     | Can Parallel With |
| --------------------- | ------------------------------ | ----------------- |
| US1 (Set Budget)      | Phase 2                        | -                 |
| US2 (Rollover)        | Phase 2                        | US1, US3          |
| US3 (Dashboard)       | Phase 2                        | US1, US2          |
| US4 (Extra Budget)    | US2 (for rollover integration) | US5, US6          |
| US5 (Management Page) | US1, US2                       | US4, US6          |
| US6 (Categories)      | US2, US3                       | US4, US5          |

### MVP Scope

**Minimum Viable Product (Phases 1-5)**:

- User can set yearly/monthly budget
- Rollover calculations work correctly
- Dashboard shows donut, bar chart, and heatmap

**Enhanced Features (Phases 6-8)**:

- Extra budget injections
- Detailed management page with history
- Category breakdown in quarterly summaries

---

## Parallel Execution Examples

### Phase 2 (Foundational) - Maximum Parallelism

```bash
# Agent A: Domain types
T006, T007, T008, T009 (all independent)

# Agent B: Domain aggregates (after T006-T008)
T010, T011, T012

# Agent C: Infrastructure (after T010-T012)
T013, T014, T015, T016, T017

# Agent D: Interface (after T013)
T018
```

### MVP Implementation - Parallel by User Story

```bash
# After Phase 2 completion:

# Team A: User Story 1 (Set Budget)
T019 → T020 → T021 → T022 → T023 → T024 → T025 → T026 → T027

# Team B: User Story 2 (Rollover) - can start after T019
T028 → T029 → T030 → T031 → T032 → T033 → T034 → T035 → T036

# Team C: User Story 3 (Dashboard) - can start after T030
T037 → T038 → T039 → T040 → T041 → T042 → T043 → T044 → T045 → T046
```

---

## Summary

| Phase             | Task Count | Parallel Tasks | Estimated Time |
| ----------------- | ---------- | -------------- | -------------- |
| Setup             | 5          | 2              | 0.5 day        |
| Foundational      | 13         | 4              | 1 day          |
| US1: Set Budget   | 9          | 0              | 0.5 day        |
| US2: Rollover     | 9          | 0              | 0.5 day        |
| US3: Dashboard    | 10         | 0              | 1 day          |
| US4: Extra Budget | 8          | 0              | 0.5 day        |
| US5: Management   | 5          | 0              | 0.5 day        |
| US6: Categories   | 9          | 0              | 0.5 day        |
| Polish            | 8          | 4              | 0.5 day        |
| **Total**         | **76**     | **10**         | **~5 days**    |

### MVP vs Full Feature

- **MVP (US1-US3)**: 46 tasks, ~3 days
- **Full Feature (US1-US6 + Polish)**: 76 tasks, ~5 days
