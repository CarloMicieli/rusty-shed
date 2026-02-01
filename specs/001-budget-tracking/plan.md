# Implementation Plan: Budget Tracking

**Branch**: `001-budget-tracking` | **Date**: January 30, 2026 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-budget-tracking/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement a budget tracking feature that enables rail hobbyists to set yearly/monthly hobby budgets, track spending against those budgets with roll-over mechanics, and visualize spending patterns through dashboard widgets (donut chart, bar chart, 5-year heatmap) and a dedicated management page. The feature leverages the existing `MonetaryAmount`, `Currency`, and `Category` domain types while introducing new budget-specific aggregates with domain event persistence.

## Technical Context

**Language/Version**: Rust 1.93.0 (edition 2024) backend, TypeScript 5.9.3 frontend  
**Primary Dependencies**: Tauri 2.9.x, SQLite via sqlx, Svelte 5.48.2, Skeleton UI 4.x, Tailwind CSS 4.x, specta/tauri-specta for type generation  
**Storage**: SQLite (via sqlx with `sqlx::migrate!` embedded migrations, `PRAGMA foreign_keys = ON`)  
**Testing**: Vitest for frontend, cargo test for Rust; vitest-coverage with V8 provider  
**Target Platform**: Desktop (Tauri) - Linux, macOS, Windows  
**Project Type**: Web app (Svelte frontend) + Rust backend (Tauri)  
**Performance Goals**: <200ms for read queries (dashboard budget stats), <1s for rollover recalculation chain  
**Constraints**: All domain logic in Rust backend; frontend for rendering only; Paraglide for all user strings; offline-capable local SQLite

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Architectural Laws Compliance

| Law                         | Status       | Evidence                                                                                                                                                  |
| --------------------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Database (SQLite/sqlx)**  | ✅ COMPLIANT | Migration file `0007_create_budget_schema.sql` will be added in `/migrations`; `sqlx::migrate!` used; `PRAGMA foreign_keys = ON` enforced                 |
| **Domain Event Tracking**   | ✅ COMPLIANT | `BudgetConfiguration` and `MonthlyBudgetRecord` aggregates will use `pending_events: Vec<BudgetEvent>` pattern; repositories drain and persist atomically |
| **Tauri IPC + specta**      | ✅ COMPLIANT | All transport DTOs use `Args`/`Input`/`Query` conventions per ADR 8; `specta::Type` + `serde::Deserialize` derived; `validator::Validate` at boundary     |
| **Domain Logic in Backend** | ✅ COMPLIANT | All rollover calculations, budget validation, and spending aggregation performed in Rust domain layer; frontend only renders and calls Tauri commands     |

### Code Quality & Standards

| Principle                | Status | Notes                                                                                        |
| ------------------------ | ------ | -------------------------------------------------------------------------------------------- |
| Test-First               | ✅     | Unit tests for domain, integration tests for repository, frontend tests for controllers      |
| Modular/Library-First    | ✅     | New `budget/` feature module under `src-tauri/src/` following existing patterns              |
| Deterministic Interfaces | ✅     | OpenAPI-style contracts documented in `contracts/`; TypeScript bindings generated via specta |
| UX Consistency           | ✅     | Paraglide for strings; Skeleton UI components; design tokens from shared library             |
| Performance Requirements | ✅     | Dashboard budget queries <200ms SLO; rollover recalculation <1s                              |

## Project Structure

### Documentation (this feature)

```text
specs/001-budget-tracking/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── budget-api.md    # Tauri command contracts
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Backend (Rust/Tauri)
src-tauri/
├── migrations/
│   └── 0007_create_budget_schema.sql    # New: budget tables
├── src/
│   ├── budget/                          # New: budget feature module
│   │   ├── mod.rs
│   │   ├── domain/
│   │   │   ├── mod.rs
│   │   │   ├── budget_configuration.rs  # Aggregate: yearly/monthly config
│   │   │   ├── budget_events.rs         # Domain events
│   │   │   ├── monthly_budget_record.rs # Aggregate: per-month data
│   │   │   ├── extra_budget_entry.rs    # Value object
│   │   │   ├── quarterly_summary.rs     # Read model
│   │   │   └── repository.rs            # Repository trait
│   │   ├── application/
│   │   │   ├── mod.rs
│   │   │   ├── set_budget.rs            # Use case: configure budget
│   │   │   ├── add_extra_budget.rs      # Use case: inject extra funds
│   │   │   ├── budget_query.rs          # Query: dashboard stats
│   │   │   └── historical_query.rs      # Query: 5-year quarterly data
│   │   ├── infrastructure/
│   │   │   ├── mod.rs
│   │   │   ├── entities.rs              # DB row types
│   │   │   ├── mappers.rs               # row_to_* pure functions
│   │   │   ├── database.rs              # SQL queries
│   │   │   └── repositories.rs          # Repository impl
│   │   └── interface/
│   │       ├── mod.rs
│   │       ├── command_args.rs          # Transport DTOs (Args)
│   │       └── command_handlers.rs      # Tauri #[command] handlers
│   └── lib.rs                           # Register budget module

# Frontend (Svelte/TypeScript)
src/
├── lib/
│   ├── features/
│   │   └── budget/
│   │       ├── index.ts                 # Public exports
│   │       ├── BudgetState.svelte.ts    # Controller with $state/$derived
│   │       ├── components/
│   │       │   ├── BudgetDonutChart.svelte
│   │       │   ├── YearlySpendingChart.svelte
│   │       │   ├── ActivityHeatmap.svelte
│   │       │   ├── BudgetTable.svelte
│   │       │   ├── ExtraBudgetModal.svelte
│   │       │   └── QuarterlySummaryModal.svelte
│   │       └── services/
│   │           └── budget.service.ts    # Tauri invoke wrappers
│   ├── bindings.ts                      # Auto-generated (specta)
│   └── components/                      # Shared UI components
├── routes/
│   ├── my-dashboard/
│   │   └── +page.svelte                 # Update: integrate budget widgets
│   └── my-budget/                       # New: budget management page
│       ├── +page.svelte
│       └── +layout.svelte
└── messages/
    ├── en.json                          # Budget-related strings
    └── it.json                          # Budget-related strings
```

**Structure Decision**: Follows existing feature-grouped DDD pattern. Backend mirrors `collecting/`, `dashboard/` structure with domain → application → infrastructure → interface layers. Frontend mirrors `dashboard/` pattern with controller class + components + services.

## Complexity Tracking

> No constitution violations requiring justification. Design follows established patterns.
