# Implementation Plan: Digital Rolling Stock Management

**Branch**: `006-digital-rolling-stocks` | **Date**: 2026-01-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/006-digital-rolling-stocks/spec.md`

## Summary

Build a "My Digital Rolling Stocks" page to manage the user's digital roster. The feature includes a summary showing the percentage of digitalized rolling stock (excluding dummies, counting factory-fitted DCC_SOUND/DCC_FITTED), a filterable list of digital rolling stock indexed by DCC address (with category, railway, scale, power method), inline DCC address editing with duplicate warnings, and a right-sliding decoder installation panel.

## Technical Context

**Language/Version**: Rust 2024 edition (rust-version 1.93.0) for backend, TypeScript 5.9.3 for frontend  
**Primary Dependencies**: Tauri 2.9.x, sqlx (SQLite), tokio, specta/tauri-specta (backend); SvelteKit, Svelte 5.48.2, Tailwind 4, Skeleton UI 4.x (frontend)  
**Storage**: SQLite via sqlx (existing database with `digital_rolling_stocks`, `decoders`, `owned_rolling_stocks`, `rolling_stocks` tables)  
**Testing**: cargo test (backend), vitest with happy-dom (frontend)  
**Target Platform**: Desktop (Tauri 2)  
**Project Type**: Tauri desktop app (Rust backend + SvelteKit frontend)  
**Performance Goals**: <200ms for read queries (Constitution requirement)  
**Constraints**: All user strings via Paraglide, all domain logic in Rust  
**Scale/Scope**: Single user, local SQLite database

### Existing Implementation (DO NOT MODIFY without approval)

The `dcc_inventory` domain is already partially implemented:

**Backend (Rust) - Already exists:**

- `DigitalRollingStock` aggregate with domain events
- `Decoder` master record entity
- `DccAddress` value object (validated 1-9999)
- `DecoderId`, `DigitalRollingStockId` typed IDs
- `DecoderType` enum (Plain, Sound, Function, MultiProtocol)
- `DigitalRollingStockRepository` with:
  - `find_by_id`, `save`, `find_all_decoders`, `find_all_digital_rolling_stocks`
- Use cases:
  - `NewDigitalRollingStockUseCase` (creates new entry)
  - `ChangeDccAddressUseCase` (updates address)
  - `ChangeDecoderUseCase` (replaces decoder)
  - `GetDigitalRollingStocksUseCase` (returns all views)
- Tauri commands (already exposed):
  - `new_digital_rolling_stock`
  - `change_dcc_address`
  - `change_decoder`
  - `get_digital_rolling_stocks`

**Frontend (TypeScript bindings) - Already generated:**

- All commands available in `bindings.ts`
- Types: `DigitalRollingStockView`, `DecoderView`, `DccAddress`, etc.

### Gaps to Address

**Backend additions needed:**

1. **Enhanced `DigitalRollingStockView`**: Current view lacks rolling stock details (category, railway company, scale, power method, road number). Need to enrich with joined catalog data.
2. **Digital Summary Query**: New query to calculate digital percentage (excluding dummies, counting DCC_SOUND/DCC_FITTED).
3. **Duplicate DCC Address Check**: Query to check if a DCC address is already in use.
4. **Get Decoders Command**: Expose `find_all_decoders` as a Tauri command for the dropdown.
5. **Filter by decoder type**: Repository should exclude Function decoders from roster list.

**Frontend additions needed:**

1. **New route**: `/my-digital-roster` (or similar)
2. **Navigation item**: Add to SidebarNavigation and BottomNavigation
3. **Controller class**: `DigitalRosterController.svelte.ts` with Svelte 5 runes
4. **Page components**: Summary section, filterable table, DCC address edit
5. **Decoder installation panel**: Right-sliding drawer component
6. **Paraglide messages**: All new UI strings

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Check (Phase 0 Gate) ✅

| Principle                            | Status  | Notes                                                                                                                  |
| ------------------------------------ | ------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**           | ✅ PASS | Using existing SQLite + sqlx. No new migrations expected (tables exist). If schema changes needed, will add migration. |
| **State Management / Domain Events** | ✅ PASS | Using existing `DigitalRollingStock` aggregate with `pending_events`. New queries are read-only.                       |
| **API Design & Transport**           | ✅ PASS | New Tauri commands will follow ADR 8 (Args → Input pattern), specta types.                                             |
| **Domain Logic in Rust**             | ✅ PASS | All business logic (percentage calculation, duplicate detection) in Rust backend.                                      |
| **Type Generation**                  | ✅ PASS | Using specta/tauri-specta for TypeScript bindings.                                                                     |
| **Test-First Emphasis**              | ✅ PASS | Will add unit tests for new queries, integration tests for repositories.                                               |
| **Code Quality**                     | ✅ PASS | Must pass cargo clippy, cargo fmt, eslint, prettier.                                                                   |
| **User Experience Consistency**      | ✅ PASS | Using Paraglide for strings, Skeleton UI components, right-sliding drawer pattern.                                     |
| **Performance Requirements**         | ✅ PASS | Queries designed for <200ms. Summary is a single aggregate query.                                                      |

### Post-Design Check (Phase 1 Gate) ✅

| Principle                            | Status  | Notes                                                                                                          |
| ------------------------------------ | ------- | -------------------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**           | ✅ PASS | No schema changes required. Using existing tables with enhanced queries. See [data-model.md](./data-model.md). |
| **State Management / Domain Events** | ✅ PASS | New commands are read-only or use existing aggregate events. No new event types needed.                        |
| **API Design & Transport**           | ✅ PASS | 4 new commands defined in [contracts/tauri-commands.md](./contracts/tauri-commands.md) following ADR 8.        |
| **Domain Logic in Rust**             | ✅ PASS | Summary calculation, duplicate detection, filtering all in Rust. Frontend is presentation only.                |
| **Type Generation**                  | ✅ PASS | All new types derive `specta::Type`. Bindings will regenerate automatically.                                   |
| **Test-First Emphasis**              | ✅ PASS | Test plan included in [quickstart.md](./quickstart.md).                                                        |
| **Code Quality**                     | ✅ PASS | Verification checklist in quickstart.md.                                                                       |
| **User Experience Consistency**      | ✅ PASS | All UI strings defined as Paraglide keys in research.md.                                                       |
| **Performance Requirements**         | ✅ PASS | Queries use indexed JOINs. Summary is single aggregation query.                                                |

**All gates pass. Ready for Phase 2: Task Generation (`/speckit.tasks`).**

**All gates pass. Proceeding to Phase 0.**

## Project Structure

### Documentation (this feature)

```text
specs/006-digital-rolling-stocks/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (API contracts)
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Backend (Rust - src-tauri/src/)
src-tauri/src/
├── dcc_inventory/
│   ├── application/
│   │   ├── get_digital_rolling_stocks.rs   # Extend view with catalog data
│   │   ├── get_digital_summary.rs          # NEW: Summary query
│   │   ├── get_decoders.rs                 # NEW: Expose decoder list
│   │   ├── check_duplicate_address.rs      # NEW: Duplicate check
│   │   └── views.rs                        # Extend DigitalRollingStockView
│   ├── interface/
│   │   ├── command_handlers.rs             # Add new Tauri commands
│   │   └── command_args.rs                 # Add new Args types
│   └── infrastructure/
│       └── sqlite_digital_rolling_stock_repository.rs  # Extend queries

# Frontend (TypeScript/Svelte - src/)
src/
├── routes/
│   └── my-digital-roster/                  # NEW: Feature route
│       ├── +page.svelte
│       └── +page.server.ts                 # Pre-render SSR stub if needed
├── lib/
│   ├── features/
│   │   └── digital-roster/                 # NEW: Feature module
│   │       ├── DigitalRosterController.svelte.ts
│   │       ├── DigitalRosterState.svelte.ts
│   │       ├── components/
│   │       │   ├── DigitalSummary.svelte
│   │       │   ├── DigitalRosterTable.svelte
│   │       │   ├── DccAddressEditor.svelte
│   │       │   └── DecoderInstallDrawer.svelte
│   │       └── index.ts
│   └── components/
│       ├── SidebarNavigation.svelte        # Add nav item
│       └── BottomNavigation.svelte         # Add nav item
└── messages/
    ├── en.json                             # Add new messages
    └── it.json                             # Add new messages
```

**Structure Decision**: Feature-grouped DDD architecture. Backend follows domain → application → infrastructure layering. Frontend uses per-feature controller pattern with Svelte 5 reactivity.

## Complexity Tracking

> No Constitution violations requiring justification.

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A       | N/A        | N/A                                  |
