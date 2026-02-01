# Implementation Plan: My Maintenance Page

**Branch**: `007-my-maintenance` | **Date**: January 30, 2026 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/007-my-maintenance/spec.md`

## Summary

Add a "My Maintenance" page to the app that displays the top 10 rolling stocks requiring maintenance, sorted by due date with visual urgency indicators. The page provides two quick actions: create a new maintenance card and log a maintenance event. **The backend commands already exist** - this feature primarily requires frontend implementation.

## Technical Context

**Language/Version**: Rust 1.x (backend - existing), TypeScript/Svelte 5 (frontend)  
**Primary Dependencies**: Tauri 2, Svelte 5 Runes, Tailwind 4, Skeleton 4.x, Paraglide-JS  
**Storage**: SQLite via sqlx (existing schema)  
**Testing**: Vitest (frontend), cargo test (backend - existing tests)  
**Target Platform**: Desktop (Tauri 2)
**Project Type**: Tauri Desktop App (frontend + backend monorepo)  
**Performance Goals**: Page load < 2 seconds, maintenance card list renders immediately  
**Constraints**: Offline-capable, single-user desktop application  
**Scale/Scope**: Single user collection, typically < 1000 rolling stock items

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

- ✅ Uses existing project structure patterns (DDD layers in Rust, features in Svelte)
- ✅ Backend commands already implemented (`get_maintenance_dashboard`, `add_maintenance_card`, `add_maintenance_event`)
- ✅ Database schema already exists (migrations/0004_create_maintenance_tables.sql)
- ✅ TypeScript bindings already generated (`MaintenanceCardView`, `AddMaintenanceArgs`)

## Existing Backend Implementation

The following backend components are **already implemented**:

### Domain Layer (`src-tauri/src/maintenance/domain/`)

- `MaintenanceCard` - Aggregate root with events
- `MaintenanceCardId` - Typed identifier
- `MaintenanceEvent` - Maintenance event entity
- `MaintenanceType` - Enum for maintenance types
- `MaintenanceRepository` - Repository trait

### Application Layer (`src-tauri/src/maintenance/application/`)

- `GetMaintenanceDashboard` - Retrieve due/overdue cards
- `AddMaintenanceCard` - Create new maintenance card
- `AddMaintenanceEvent` - Log maintenance event

### Interface Layer (`src-tauri/src/maintenance/interface/`)

- `get_maintenance_dashboard` - Tauri command
- `add_maintenance_card` - Tauri command
- `add_maintenance_event` - Tauri command
- `MaintenanceCardView` - Frontend-facing view model

### Database (`src-tauri/migrations/`)

- `maintenance_cards` table
- `maintenance_events` table

## Frontend Implementation Required

The frontend needs:

1. **Route**: `/my-maintenance` page
2. **State Management**: MaintenanceState.svelte.ts with service integration
3. **Components**:
   - Page component with quick actions and card list
   - Maintenance card display component with urgency indicators
   - Add Maintenance Card modal
   - Add Maintenance Event modal
4. **Localization**: Paraglide messages for all UI text

## Project Structure

### Documentation (this feature)

```text
specs/007-my-maintenance/
├── plan.md              # This file
├── spec.md              # Feature specification
├── checklists/          # Quality checklists
│   └── requirements.md  # Requirements checklist
└── tasks.md             # Task breakdown (to be created)
```

### Source Code (repository root)

```text
# Backend (EXISTING - no changes needed)
src-tauri/
├── src/maintenance/
│   ├── domain/              # ✅ Complete
│   ├── application/         # ✅ Complete
│   ├── infrastructure/      # ✅ Complete
│   └── interface/           # ✅ Complete
└── migrations/
    └── 0004_create_maintenance_tables.sql  # ✅ Complete

# Frontend (TO BE IMPLEMENTED)
src/
├── routes/
│   └── my-maintenance/      # NEW - page route
│       └── +page.svelte
├── lib/features/maintenance/
│   ├── MaintenanceState.svelte.ts     # NEW - state management
│   ├── MaintenanceService.ts          # NEW - Tauri command wrapper
│   ├── components/                    # NEW - UI components
│   │   ├── MaintenanceCardList.svelte
│   │   ├── MaintenanceCardItem.svelte
│   │   ├── AddMaintenanceCardModal.svelte
│   │   └── AddMaintenanceEventModal.svelte
│   └── index.ts                       # NEW - feature exports
└── lib/paraglide/messages/            # UPDATE - i18n messages

messages/
├── en.json                  # UPDATE - English translations
└── it.json                  # UPDATE - Italian translations
```

**Structure Decision**: This follows the existing project structure with Svelte 5 features in `src/lib/features/` and routes in `src/routes/`. The backend is complete and only frontend work is required.

## Complexity Tracking

No violations - using established project patterns.

## Dependencies

- **US1 (View Maintenance Overview)**: No dependencies - can start immediately
- **US2 (Create Maintenance Card)**: Depends on US1 page structure being in place
- **US3 (Add Maintenance Event)**: Depends on US1 page structure being in place

## Parallel Execution Opportunities

- Components (MaintenanceCardItem, modals) can be developed in parallel
- Localization messages can be added in parallel with component development
- State management service can be developed alongside components

## Implementation Strategy

1. **MVP (Phase 3)**: Deliver User Story 1 first - view-only maintenance overview
2. **Increment 1 (Phase 4)**: Add maintenance card creation capability
3. **Increment 2 (Phase 5)**: Add maintenance event logging
4. **Polish (Phase 6)**: Cross-cutting concerns, empty states, error handling refinement
