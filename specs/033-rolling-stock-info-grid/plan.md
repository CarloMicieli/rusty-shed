# Implementation Plan: Rolling Stock Information Grid

**Branch**: `033-rolling-stock-info-grid` | **Date**: 2026-03-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/033-rolling-stock-info-grid/spec.md`

## Summary

Restructure `RollingStockCard.svelte` from an unstructured definition-list layout into a fixed 5-row × 3-column information grid that surfaces all 15 rolling stock attributes (currently 7 are hidden behind the `RollingStockSpecsDrawer`). Each field gains click-to-edit inline editing using existing `InPlaceEdit` (text/numeric), `InPlaceSelectEdit` (enumerated), or a new `InPlaceBooleanEdit` (FeatureFlag toggles) primitive. All new fields persist via the existing `updateRollingStockSpecifications` Tauri command. No backend changes are required.

## Technical Context

**Language/Version**: TypeScript 5.9 (strict) · Rust 1.93 (edition 2024)
**Primary Dependencies**: Svelte 5 (Runes only) · Tailwind CSS v4 · shadcn-svelte · Tauri 2.9.x · Paraglide 2.7.1
**Storage**: SQLite via sqlx — no schema changes required (all fields already persisted)
**Testing**: Vitest 4 (frontend, happy-dom) · cargo test (backend)
**Target Platform**: Desktop (Tauri) — Linux / macOS / Windows
**Performance Goals**: Inline save round-trip < 200ms p95 (per constitution SLO for UI-critical commands)
**Constraints**: All UI strings via Paraglide; specta-generated types only; no `unwrap()` in Rust
**Scale/Scope**: Single feature-level component change; ~1 Svelte component, 1 new primitive, message keys

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design._

| Principle                | Status  | Notes                                                                       |
| ------------------------ | ------- | --------------------------------------------------------------------------- |
| Modular Library-First    | ✅ PASS | New `InPlaceBooleanEdit` is self-contained and independently testable       |
| Deterministic Interfaces | ✅ PASS | Using existing specta-generated bindings; no new IPC surface                |
| Test-First Emphasis      | ✅ PASS | Plan includes unit tests for `InPlaceBooleanEdit` and component tests       |
| Code Quality             | ✅ PASS | Strict TS, Clippy, Prettier — no new Rust code                              |
| Testing Standards        | ✅ PASS | Vitest unit tests for new component; integration tests via existing harness |
| UX Consistency           | ✅ PASS | Reuses InPlaceEdit/InPlaceSelectEdit patterns; Paraglide for all strings    |
| Performance              | ✅ PASS | `updateRollingStockSpecifications` is an existing command within SLO        |
| Safe Rust Practices      | ✅ N/A  | No Rust changes planned                                                     |
| Database Law             | ✅ N/A  | No schema changes; all fields already exist                                 |
| State Management Law     | ✅ N/A  | No domain model changes                                                     |
| API/Transport Law        | ✅ PASS | Only using existing specta-generated `UpdateRollingStockSpecificationsArgs` |
| Domain Logic Location    | ✅ PASS | All business logic stays in Rust; frontend is display + UX only             |

**Post-design re-check**: No violations found. Feature is purely a frontend restructuring exercise leveraging existing backend infrastructure.

## Project Structure

### Documentation (this feature)

```text
specs/033-rolling-stock-info-grid/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── tauri-commands.md
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
src/
├── lib/
│   ├── components/
│   │   ├── InPlaceBooleanEdit.svelte       # NEW — FeatureFlag toggle primitive
│   │   └── model-details/
│   │       └── RollingStockCard.svelte     # MODIFIED — grid layout + 7 new fields
└── paraglide/
    └── messages.js                         # AUTO-GENERATED (from messages/en.json)

messages/
└── en.json                                 # MODIFIED — add ~8 new rolling_stock_field_* keys

src-tauri/
└── (no changes required)
```

**Structure Decision**: Single-project Tauri + SvelteKit layout. Changes confined to the frontend `src/` layer. Existing `updateRollingStockSpecifications` Tauri command and its specta bindings are consumed as-is.
