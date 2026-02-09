# Implementation Plan: Dashboard Collector's Overview Redesign

**Branch**: `017-dashboard-redesign` | **Date**: February 9, 2026 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/017-dashboard-redesign/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

The dashboard will be transformed from a data-dump interface into a collector's visual overview by grouping models by purchase events. Instead of showing individual models, the redesigned dashboard will display 2-3 most recent purchase groups (acquisition events), each showing the date, seller, and notes along with up to 3 model cards. Each model card will emphasize visual recognition with thumbnails, manufacturer badges, condition status, and clickable navigation to full details. This approach provides context and story behind acquisitions while maintaining consistent viewport height.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (strict mode), Rust 1.93.0 (edition 2024)  
**Primary Dependencies**: SvelteKit (Svelte 5.48.2), Vite 7.3.1, Tauri 2.9.x, shadcn-svelte, Tailwind CSS 4.1.18  
**Storage**: SQLite via sqlx (existing tables: `purchase_infos`, `collection_items`, `sellers`)  
**Testing**: Vitest 4.0.18 (frontend - happy-dom environment), cargo test (backend)  
**Target Platform**: Tauri 2 desktop application (Linux, macOS, Windows)  
**Project Type**: Web + desktop hybrid (SvelteKit frontend, Rust backend via Tauri IPC)  
**Performance Goals**: <2 seconds dashboard load with 30 models, <200ms for purchase grouping query  
**Constraints**: Responsive design 320px-2560px, offline-capable, maintain scroll position on navigation  
**Scale/Scope**: Display 2-3 purchase groups (~10 models visible), handle collections of 500+ models efficiently

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Gate 1: Database (Persistence) — REQUIRED ✅

**Status**: COMPLIANT

**Justification**: This feature extends existing SQLite schema accessed via sqlx. All necessary tables already exist:

- `purchase_infos` table stores purchase_date, seller_id (existing structure)
- `collection_items` table has added_date field (can be used for grouping)
- `sellers` table provides seller name/type information
- Foreign key enforcement already enabled in application initialization

**No migration needed**: Existing schema supports all required queries (group by purchase date + seller, fetch recent purchases).

### Gate 2: State Management / Persistence Strategy — REQUIRED ✅

**Status**: COMPLIANT

**Justification**: No new aggregates or domain events required. This is a read-only query feature that:

- Uses existing read-model queries via dashboard repository
- No write operations or aggregate mutations
- Extends existing `DashboardSummary` query with purchase grouping logic
- Follows established CQRS read pattern (see `src-tauri/src/dashboard/`)

### Gate 3: API Design & Transport Boundary — REQUIRED ✅

**Status**: COMPLIANT

**Justification**: Extends existing `get_dashboard_summary` Tauri command:

- Maintains existing `DashboardSummary` return type with extended structure
- New types (`PurchaseGroup`, `ModelCard`) will derive `specta::Type, Serialize`
- Follows ADR 8 naming (Query types for reads, Args types if parameters added)
- Type generation via specta already configured in build pipeline
- No new HTTP ports or sidecar processes

### Gate 4: Domain Logic Location — REQUIRED ✅

**Status**: COMPLIANT

**Justification**: All business logic remains in Rust backend:

- Purchase grouping algorithm (group by date + seller) in repository layer
- Date formatting and sorting in Rust
- Frontend only renders pre-grouped data structures
- No business rules in UI components

### Gate 5: Testing Standards ✅

**Status**: COMPLIANT

**Justification**:

- Unit tests will cover grouping logic in repository layer (Rust)
- Integration tests will verify query contracts
- Component tests will verify UI rendering (Vitest)
- No external network dependencies (local SQLite only)

### Gate 6: User Experience Consistency ✅

**Status**: COMPLIANT

**Justification**:

- All strings via Paraglide message system (e.g., `m.dashboard_purchase_group_title()`)
- Uses existing shadcn-svelte components (`Card`, `Badge`)
- Follows established dashboard layout patterns
- Responsive design using Tailwind CSS grid system

### Gate 7: Performance Requirements ✅

**Status**: COMPLIANT

**Justification**:

- Query limited to 2-3 recent purchases (~10 models max) for consistent load time
- Grouping query uses indexed columns (purchase_date, added_date)
- Target: <200ms for grouping query, <2s total page load
- All work on Rust backend (no UI thread blocking)

### Gate 8: Code Quality ✅

**Status**: COMPLIANT

**Justification**:

- TypeScript strict mode enabled
- Rust clippy with `-D warnings` enforced
- All code formatted via prettier/cargo fmt
- Short design summary included in plan (see Summary section)

**RESULT**: ALL GATES PASSED ✅ - Ready for Phase 0 research

## Project Structure

### Documentation (this feature)

```text
specs/017-dashboard-redesign/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── PurchaseGroupCard.contract.md
│   ├── ModelCard.contract.md
│   └── get_dashboard_summary_v2.contract.md
├── checklists/
│   └── requirements.md  # Already created during specification
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Web application with Tauri backend
src-tauri/
├── src/
│   ├── dashboard/
│   │   ├── domain/
│   │   │   ├── dashboard_summary.rs          # Extended with PurchaseGroup
│   │   │   ├── purchase_group.rs             # NEW: Purchase grouping entity
│   │   │   └── model_card.rs                 # NEW: Model card view entity
│   │   ├── infrastructure/
│   │   │   ├── dashboard_repository.rs       # Extended query logic
│   │   │   └── entities.rs                   # SQL row mappers
│   │   └── application/
│   │       └── get_dashboard_summary.rs      # Extended command handler
│   └── lib.rs                                 # Existing Tauri command registration
├── migrations/                                 # No new migrations needed
└── Cargo.toml

src/
├── lib/
│   ├── bindings.ts                            # Auto-generated from specta
│   ├── features/
│   │   └── dashboard/
│   │       ├── DashboardState.svelte.ts       # Existing state management
│   │       ├── components/
│   │       │   ├── PurchaseGroupCard.svelte   # NEW: Purchase group container
│   │       │   ├── ModelCard.svelte           # NEW: Individual model card
│   │       │   ├── StatsCard.svelte           # Existing
│   │       │   ├── RecentItemCard.svelte      # May be repurposed or replaced
│   │       │   └── DepotView.svelte           # Existing - links to full collection
│   │       └── index.ts
│   ├── components/                            # Shared shadcn-svelte components
│   └── paraglide/
│       └── messages.js                        # i18n strings
└── routes/
    ├── my-dashboard/
    │   └── +page.svelte                       # MODIFIED: New layout with purchase groups
    └── my-collection/
        └── +page.svelte                       # Existing full collection view

messages/
├── en.json                                     # NEW: Dashboard purchase group strings
└── it.json                                     # NEW: Italian translations
```

**Structure Decision**: This is a web application with Tauri backend (Option 2 pattern). The dashboard feature follows the established DDD layered architecture:

- **Domain layer** (`src-tauri/src/dashboard/domain/`): Entities and value objects
- **Infrastructure layer** (`src-tauri/src/dashboard/infrastructure/`): Repository with SQL queries
- **Application layer** (`src-tauri/src/dashboard/application/`): Use case / command handlers
- **Presentation layer** (`src/lib/features/dashboard/`): Svelte 5 components with runes

The feature extends the existing dashboard module rather than creating a new bounded context.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**N/A** - All constitution gates passed without violations. No complexity justification required.

---

## Phase 0: Outline & Research ✅

**Status**: COMPLETE

All technical unknowns resolved. See [research.md](./research.md) for detailed decisions:

- **R1**: Purchase grouping strategy (by purchase_date + seller_id)
- **R2**: Model card data requirements (6 core fields, single query)
- **R3**: Image loading infrastructure (reuse existing, lazy loading)
- **R4**: Responsive grid layout (CSS Grid with Tailwind utilities)
- **R5**: Paraglide i18n message keys (11 new keys for EN + IT)
- **R6**: Navigation & scroll position preservation (SvelteKit built-in)
- **R7**: Performance optimization strategy (query limits, indexed columns, lazy images)

**Key Findings**:

- No database migrations required (existing schema sufficient)
- Performance target achievable (<2s load, <200ms queries)
- All patterns leverage existing codebase conventions

---

## Phase 1: Design & Contracts ✅

**Status**: COMPLETE

### Artifacts Generated

1. **[data-model.md](./data-model.md)**: Complete domain entity definitions
   - `PurchaseGroup` (aggregates models by purchase event)
   - `ModelCard` (compact view for card display)
   - `PurchaseCondition` enum (New/PreOwned/Unknown)
   - Extended `DashboardSummary` with `purchase_groups` field

2. **[contracts/](./contracts/)**: Component and API contracts
   - `PurchaseGroupCard.contract.md` - Purchase group container component
   - `ModelCard.contract.md` - Individual model card component
   - `get_dashboard_summary_v2.contract.md` - Extended Tauri command API

3. **[quickstart.md](./quickstart.md)**: 7-phase implementation guide
   - Phase 0: Setup (30 min) - Paraglide messages
   - Phase 1: Backend Domain (1 hr) - Entities & enums
   - Phase 2: Backend Infrastructure (2 hrs) - Repository queries
   - Phase 3: Backend Application (30 min) - Command handler
   - Phase 4: Frontend Components (3 hrs) - Svelte components
   - Phase 5: Frontend Dashboard Page (1 hr) - Page integration
   - Phase 6: Testing (2 hrs) - Unit & integration tests
   - Phase 7: Verification (1 hr) - Performance & acceptance testing
   - **Total**: 11 hours (1.5 developer days)

### Agent Context Updated ✅

Updated `.github/agents/copilot-instructions.md` with:

- TypeScript 5.9.3, Rust 1.93.0 (edition 2024)
- SvelteKit (Svelte 5.48.2), Tailwind CSS 4.1.18
- SQLite schema references (purchase_infos, collection_items, sellers)
- Web + desktop hybrid architecture (Tauri IPC)

---

## Next Steps

**Ready for Implementation** - Use `/speckit.tasks` to generate the task breakdown, or proceed directly with implementation following [quickstart.md](./quickstart.md).

### Pre-Implementation Checklist

- [x] Feature specification validated ([spec.md](./spec.md))
- [x] Constitution compliance verified (all gates passed)
- [x] Technical research complete ([research.md](./research.md))
- [x] Data model defined ([data-model.md](./data-model.md))
- [x] Component contracts documented ([contracts/](./contracts/))
- [x] Implementation guide ready ([quickstart.md](./quickstart.md))
- [x] Agent context updated for Copilot
- [ ] Tasks breakdown generated (`/speckit.tasks` - next step)

### Implementation Branch

Branch `017-dashboard-redesign` is ready for development. Feature directory:

```
/home/carlo/Projects/rusty-shed/specs/017-dashboard-redesign/
```

### Estimated Completion

- **Development**: 11 hours (Phase 0-7 in quickstart.md)
- **Review & Refinement**: 2-3 hours
- **Total**: ~2 developer days (13-14 hours)

---

## Planning Summary

**Planning Completed**: February 9, 2026  
**Feature Branch**: `017-dashboard-redesign`  
**Specification**: [spec.md](./spec.md) (4 user stories, 19 functional requirements, 7 success criteria)

**Constitution Status**: ✅ ALL GATES PASSED

- Database (SQLite/sqlx) ✅
- State Management (Read-only queries) ✅
- API Design (Tauri IPC + specta) ✅
- Domain Logic (Backend Rust) ✅
- Testing Standards ✅
- UX Consistency (Paraglide + shadcn) ✅
- Performance (<2s load, <200ms queries) ✅
- Code Quality (TypeScript strict, Rust clippy) ✅

**Key Decisions**:

- Group by (purchase_date, seller_id) - no migration needed
- Limit to 2-3 groups, 3 models each - consistent viewport height
- Reuse existing image infrastructure - no new services
- Extend DashboardSummary with backward compatibility
- Frontend: Svelte 5 runes, shadcn-svelte components
- Backend: DDD layers, sqlx queries, specta type generation

**Innovation**: Transform data-dump dashboard into collector's visual story by grouping models by purchase events, emphasizing context (date, seller, notes) over raw inventory data.
