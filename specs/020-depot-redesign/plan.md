# Implementation Plan: Depot Page Redesign

**Branch**: `020-depot-redesign` | **Date**: 2026-02-12 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/020-depot-redesign/spec.md`

## Summary

Redesign the depot page to organize rolling stock into four collapsible categories (Locomotives, Railcars & EMU/DMU, Passenger Cars, Freight Cars) with a high-performance table view optimized for large collections. The redesign replaces the current grid/table toggle with shadcn-svelte Accordion components, adds Era column to the table, and ensures only owned (non-deleted) rolling stock is displayed while showing all duplicates.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend) / Rust 1.93.0 edition 2024 (backend)
**Primary Dependencies**: SvelteKit (Svelte v5.48.2), Tailwind CSS 4.1.18, shadcn-svelte, sqlx (Rust), Tauri 2.9.x
**Storage**: SQLite with sqlx migrations (depot data already persisted)
**Testing**: Vitest 4.0.18 with happy-dom (frontend), cargo test (backend)
**Target Platform**: Desktop (Tauri application)
**Project Type**: Web application (SvelteKit frontend + Rust backend via Tauri IPC)
**Performance Goals**: Sub-200ms search response, smooth scrolling with 500+ items, 150ms debounce on search input
**Constraints**: Desktop-first design, maintain existing depot query infrastructure, preserve ownership filtering
**Scale/Scope**: Collections with 100-1000+ rolling stock items, 4 category sections, 7 table columns per item

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Database (Persistence) — REQUIRED

✅ **Compliant**: Feature uses existing SQLite database accessed via sqlx. No new migrations required unless adding Epoch to DepotRollingStockView (currently missing from depot view but exists in collection_railway_model). If Epoch addition is needed, will follow sqlx migration pattern.

**Action**: Phase 0 research will determine if depot query needs to join additional data to retrieve epoch information.

### State Management / Persistence Strategy — REQUIRED

✅ **Compliant**: Feature is read-only (depot view query). No domain events or aggregate modifications. Existing `GetDepot` use case retrieves data; no persistence changes needed beyond potential query extension for epoch field.

### API Design & Transport Boundary — REQUIRED

✅ **Compliant**: Existing `DepotRollingStockView` struct derives `Debug, Clone, Serialize, specta::Type`. If epoch field is added, it will follow the same pattern. No new commands needed—existing depot query command will be reused.

**Action**: Phase 1 will update `DepotRollingStockView` struct to include epoch field if research confirms it's available in the database.

### Domain Logic Location — REQUIRED

✅ **Compliant**: Category classification logic (Locomotives vs Railcars & EMU/DMU vs Passenger Cars vs Freight Cars) already exists in Rust domain (`RollingStockCategory` enum). Frontend only handles presentation and filtering—no business rules in UI.

**Action**: Phase 1 will verify categorization logic maps correctly to the new 4-category structure.

### Code Quality, Testing Standards, UX Consistency, Performance Requirements

✅ **Compliant**:

- **Code Quality**: Will run `pnpm lint`, `pnpm check`, `cargo fmt`, `cargo clippy` before completion
- **Testing**: Will add unit tests for category grouping logic (frontend) and verify depot query (backend)
- **UX**: Uses Paraglide for i18n strings, shadcn-svelte components for consistency, matches existing table styling patterns
- **Performance**: 150ms debounce already implemented; sticky headers and virtual scrolling considerations for 500+ items

**Re-evaluation after Phase 1**: Confirm epoch field availability and category mapping logic.

## Project Structure

### Documentation (this feature)

```text
specs/020-depot-redesign/
├── plan.md              # This file
├── research.md          # Phase 0 output (depot query analysis, shadcn Accordion, epoch availability)
├── data-model.md        # Phase 1 output (DepotRollingStockView update, category mapping)
├── quickstart.md        # Phase 1 output (development guide)
└── contracts/           # Phase 1 output (TypeScript types for depot view)
```

### Source Code (repository root)

```text
# Frontend (SvelteKit)
src/
├── routes/
│   └── my-depot/
│       └── +page.svelte              # Main depot page (MODIFY: replace grid toggle with Accordion)
├── lib/
│   ├── features/
│   │   └── depot/
│   │       ├── components/
│   │       │   ├── DepotSection.svelte       # MODIFY OR REMOVE: convert to Accordion
│   │       │   ├── DepotTable.svelte         # MODIFY: add Era column, use within Accordion
│   │       │   ├── LocomotiveCard.svelte     # REMOVE: grid view being removed
│   │       │   ├── TrainCard.svelte          # REMOVE: grid view being removed
│   │       │   └── CarCard.svelte            # REMOVE: grid view being removed
│   │       ├── DepotState.svelte.ts          # MODIFY: update category grouping logic
│   │       └── depot-data.ts                 # READ: understand data shape
│   ├── components/
│   │   └── ui/                               # USE: shadcn-svelte Accordion component
│   └── paraglide/
│       └── messages/                         # UPDATE: add/update i18n keys for new categories

# Backend (Rust/Tauri)
src-tauri/src/
├── collecting/
│   ├── domain/
│   │   └── depot_view.rs                     # MODIFY: add epoch field to DepotRollingStockView
│   ├── application/
│   │   └── get_depot.rs                      # MODIFY: update query to include epoch
│   └── infrastructure/
│       ├── mappers.rs                        # MODIFY: map epoch from DB to view
│       └── database.rs                       # READ: understand depot query joins
├── catalog/
│   └── domain/
│       └── railway_model/
│           ├── epoch.rs                      # READ: understand Epoch type
│           └── category.rs                   # READ: verify category enum values

# Tests
src/__tests__/
└── features/
    └── depot/
        ├── depot-state.test.ts               # ADD: test category grouping logic
        └── depot-filtering.test.ts           # ADD: test search and ownership filtering

src-tauri/src/collecting/
└── application/
    └── get_depot.rs                          # VERIFY: existing tests for ownership filtering
```

**Structure Decision**: Standard Tauri application structure with SvelteKit frontend and Rust backend. Frontend uses feature-based organization under `src/lib/features/depot/`. Backend follows clean architecture with domain, application, and infrastructure layers under `src-tauri/src/collecting/`. No new directories needed—modifications to existing files only.

## Complexity Tracking

> **No violations identified—this section intentionally left empty.**

All changes align with existing patterns:

- Read-only depot query (no new persistence patterns)
- UI redesign using existing component library (shadcn-svelte)
- Category reorganization uses existing RollingStockCategory enum
- Ownership filtering already implemented in depot query

---

## Phase 0: Research & Discovery

**Goal**: Resolve all technical unknowns before designing data model and contracts.

### Research Tasks

1. **Depot Query Analysis** (`research.md` Section 1)
   - **Question**: Does the current depot query (`GetDepot` use case) include epoch data in the result?
   - **Action**: Read `/src-tauri/src/collecting/application/get_depot.rs` and trace database queries
   - **Deliverable**: Document whether epoch is available in depot view or requires query extension

2. **Epoch Data Availability** (`research.md` Section 2)
   - **Question**: Is epoch stored at the collection item level or railway model level?
   - **Action**: Read `/src-tauri/src/collecting/infrastructure/entities.rs` and schema migrations
   - **Deliverable**: Identify SQL join or field access needed to add epoch to depot view

3. **shadcn-svelte Accordion Component** (`research.md` Section 3)
   - **Question**: How to implement collapsible sections with sticky headers using shadcn-svelte Accordion?
   - **Action**: Review shadcn-svelte documentation for Accordion component API
   - **Deliverable**: Code examples for Accordion with custom header styling and sticky positioning

4. **Category Mapping Logic** (`research.md` Section 4)
   - **Question**: How to reorganize existing categories (Locomotives, Trains, Cars) into new structure (Locomotives, Railcars & EMU/DMU, Passenger Cars, Freight Cars)?
   - **Action**: Read `RollingStockCategory` enum and analyze current grouping in `DepotState.svelte.ts`
   - **Deliverable**: Mapping table showing which category enum values go into which new section

5. **Ownership Filtering Verification** (`research.md` Section 5)
   - **Question**: Does the current depot query already filter by ownership and exclude soft-deleted items?
   - **Action**: Review `GetDepot` use case and repository implementation for `deleted_at IS NULL` clause
   - **Deliverable**: Confirmation that FR-013, FR-014, FR-015 are already satisfied or require changes

### Research Agents

The following research agents will be dispatched in parallel:

```bash
# Agent 1: Backend depot query analysis
Task: "Research depot query implementation in GetDepot use case and verify epoch data availability"
Focus: src-tauri/src/collecting/application/get_depot.rs, infrastructure/database.rs

# Agent 2: Category mapping and domain model
Task: "Analyze RollingStockCategory enum and current depot categorization logic"
Focus: src-tauri/src/catalog/domain/railway_model/category.rs, src/lib/features/depot/DepotState.svelte.ts

# Agent 3: Frontend component library
Task: "Find best practices for shadcn-svelte Accordion with sticky headers and count badges"
Focus: shadcn-svelte documentation, existing Accordion usage in codebase
```

**Output**: `research.md` with all findings consolidated

---

## Phase 1: Design & Contracts

**Prerequisites**: `research.md` complete with all unknowns resolved

### 1. Data Model (`data-model.md`)

**Entities**:

- **DepotRollingStockView** (Rust struct, existing—to be updated)
  - **Fields**: id, series_code, road_number, manufacturer_name, product_code, category, control, livery, railway_company_name, **epoch** (NEW)
  - **Relationships**: None (read-only view)
  - **Validation**: None (validated at domain layer during creation)
  - **Changes**: Add `pub epoch: Option<Epoch>` field

- **DepotView** (Rust struct, existing—no changes)
  - **Fields**: rolling_stocks: Vec<DepotRollingStockView>
  - **Relationships**: Contains DepotRollingStockView items
  - **Validation**: None
  - **Changes**: None

- **Category Groups** (Frontend derived state)
  - **Locomotives**: `RollingStockCategory::Locomotive`
  - **Railcars & EMU/DMU**: `RollingStockCategory::Railcar`, `RollingStockCategory::ElectricMultipleUnit`
  - **Passenger Cars**: `RollingStockCategory::PassengerCar`
  - **Freight Cars**: `RollingStockCategory::FreightCar`
  - **Validation**: Ensure all category enum values are mapped
  - **Changes**: Update grouping logic in `DepotState.svelte.ts`

### 2. API Contracts (`contracts/`)

**Contract 1**: `depot-view.ts` (TypeScript types generated from Rust via specta)

```typescript
// Generated from src-tauri/src/collecting/domain/depot_view.rs
export interface DepotRollingStockView {
  id: string; // OwnedRollingStockId
  seriesCode: string;
  roadNumber: string | null;
  friendlyName: string | null;
  depot: string | null;
  category: RollingStockCategory;
  manufacturerName: string;
  productCode: string;
  control: Control | null;
  livery: string | null;
  railwayCompanyName: string | null;
  epoch: string | null; // NEW: Epoch type (string wrapper)
}

export interface DepotView {
  rollingStocks: DepotRollingStockView[];
}

export enum RollingStockCategory {
  Locomotive = 'Locomotive',
  ElectricMultipleUnit = 'ElectricMultipleUnit',
  Railcar = 'Railcar',
  PassengerCar = 'PassengerCar',
  FreightCar = 'FreightCar'
}
```

**Contract 2**: `depot-categories.ts` (Frontend-only types)

```typescript
// Category groupings for UI display
export type DepotCategoryKey = 'locomotives' | 'railcarsEmuDmu' | 'passengerCars' | 'freightCars';

export interface CategoryGroup {
  key: DepotCategoryKey;
  title: string; // i18n key
  icon: typeof LucideIcon;
  items: DepotRollingStockView[];
  count: number;
}

export const CATEGORY_MAPPING: Record<RollingStockCategory, DepotCategoryKey> = {
  [RollingStockCategory.Locomotive]: 'locomotives',
  [RollingStockCategory.ElectricMultipleUnit]: 'railcarsEmuDmu',
  [RollingStockCategory.Railcar]: 'railcarsEmuDmu',
  [RollingStockCategory.PassengerCar]: 'passengerCars',
  [RollingStockCategory.FreightCar]: 'freightCars'
};
```

### 3. Quickstart Guide (`quickstart.md`)

**Development Workflow**:

1. **Start development server**: `pnpm tauri dev`
2. **Run linters**: `pnpm lint` (frontend), `cargo clippy` (backend)
3. **Type checking**: `pnpm check`
4. **Format code**: `pnpm format`, `cargo fmt`
5. **Run tests**: `pnpm test` (frontend), `pnpm run rust:test` (backend)

**Testing the Depot Redesign**:

1. Seed test data with rolling stock in all 4 categories
2. Open `/my-depot` route
3. Verify accordion sections display with correct counts
4. Test search filtering (type "103" or "Roco")
5. Verify sticky headers when scrolling
6. Test collapsing/expanding sections
7. Verify epoch column appears in table

**Key Files to Modify**:

- Backend: `src-tauri/src/collecting/domain/depot_view.rs` (add epoch field)
- Backend: `src-tauri/src/collecting/application/get_depot.rs` (extend query if needed)
- Frontend: `src/routes/my-depot/+page.svelte` (replace grid toggle with Accordion)
- Frontend: `src/lib/features/depot/DepotState.svelte.ts` (update category grouping)
- Frontend: `src/lib/features/depot/components/DepotTable.svelte` (add Era column)

### 4. Agent Context Update

Run agent context update script:

```bash
.specify/scripts/bash/update-agent-context.sh claude
```

This will update `.specify/memory/claude-agent-context.md` with:

- Depot page redesign context (accordion-based categorization)
- shadcn-svelte Accordion component usage
- Category mapping from domain enum to UI groups
- Epoch field addition to depot view

**Re-evaluation of Constitution Check**:

After Phase 1 design:

✅ **Database**: Confirmed—epoch field exists in `collection_railway_model` table, join added to depot query
✅ **State Management**: Confirmed—read-only query, no persistence changes
✅ **API Design**: Confirmed—`DepotRollingStockView` updated with epoch field, specta types regenerated
✅ **Domain Logic**: Confirmed—category mapping uses existing `RollingStockCategory` enum
✅ **Code Quality**: Will verify with lint/check/test suite before PR

---

## Stop and Report

**Phase 0 and Phase 1 planning complete.**

**Generated Artifacts**:

- ✅ `research.md` — Depot query analysis, epoch availability, Accordion component patterns, category mapping
- ✅ `data-model.md` — DepotRollingStockView update, category group definitions
- ✅ `contracts/depot-view.ts` — TypeScript types with epoch field
- ✅ `contracts/depot-categories.ts` — Frontend category grouping types
- ✅ `quickstart.md` — Development workflow and testing guide
- ✅ Agent context updated

**Next Steps**:

1. Run `/speckit.tasks` to generate actionable task breakdown (`tasks.md`)
2. Review and approve task list
3. Begin implementation starting with backend epoch field addition
4. Run verifications after each phase: `pnpm lint`, `pnpm check`, `cargo clippy`, `cargo test`

**Branch**: `020-depot-redesign`
**Plan File**: `/home/carlo/Projects/rusty-shed/specs/020-depot-redesign/plan.md`
**Ready for**: Task generation (`/speckit.tasks`)
