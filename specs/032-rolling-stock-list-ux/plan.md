# Implementation Plan: Rolling Stock List UX

**Branch**: `032-rolling-stock-list-ux` | **Date**: 2026-03-03 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/032-rolling-stock-list-ux/spec.md`

## Summary

Improve the Rolling Stock List component with three complementary UX changes: (1) replace italic placeholder text with a neutral dash ("—") for empty fields, (2) remove the floating Save/Cancel pill from `InPlaceEdit` so saves happen automatically on blur or Enter, and (3) add a unified Side Drawer that lets users create a new catalog rolling stock variant for a Railway Model from both the empty-state and populated-state paths. Backend change: one new Tauri command (`add_rolling_stock_to_model`) wired to the already-present `RailwayModel::add_rolling_stock` domain method.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (strict), Rust edition 2024 (min 1.93.0)
**Primary Dependencies**: Svelte 5.48.2 (Runes), SvelteKit / Vite 7.3.1, Tailwind CSS v4, shadcn-svelte, Tauri 2.9.x, tauri-specta, sqlx (SQLite), Paraglide 2.7.1
**Storage**: SQLite via sqlx — existing `rolling_stocks` table; no new migrations needed (table has all required columns)
**Testing**: Vitest 4.0.18 (happy-dom, frontend), `cargo test` (backend), `pnpm check` + `svelte-check` (types)
**Target Platform**: Desktop (Tauri 2, Linux/macOS/Windows)
**Performance Goals**: IPC read commands <200ms per constitution; inline save calls are write operations on a local SQLite — no performance concern
**Constraints**: No hardcoded UI strings (Paraglide); no `unwrap()` in Rust; `args.validate()` called at transport boundary; `specta::Type` derived on all new Args structs; types regenerated via `pnpm tauri dev`
**Scale/Scope**: Single desktop app, one user; rolling stocks per model typically 1–20 entries

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                                | Status  | Notes                                                                                                         |
| ---------------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------- |
| Modular, Library-First Design            | ✅ PASS | New use case `AddRollingStockToModel` is a self-contained module under `catalog/application/`                 |
| Deterministic Interfaces & Observability | ✅ PASS | New command registered in `specta` builder; `Args` struct derives `Debug, Clone, Validate, Type, Deserialize` |
| Test-First Emphasis                      | ✅ PASS | New unit tests for use-case and updated frontend component tests required                                     |
| Code Quality                             | ✅ PASS | `cargo clippy -D warnings`; `pnpm lint`; `svelte-check` must all pass                                         |
| Testing Standards                        | ✅ PASS | Frontend component tests updated; backend use-case unit tested                                                |
| User Experience Consistency              | ✅ PASS | All new strings via Paraglide; design tokens from existing card/drawer CSS variables                          |
| Performance Requirements                 | ✅ PASS | All write operations are local SQLite; no long-running work; <200ms SLO met                                   |
| Safe Rust Practices                      | ✅ PASS | `Result<T, CommandError>` throughout; no `unwrap()`; no `unsafe`                                              |
| Database (Persistence)                   | ✅ PASS | No new migration needed — `rolling_stocks` table already has all columns; existing FK/PK constraints apply    |
| State Management / Persistence           | ✅ PASS | `RailwayModel::add_rolling_stock` emits `RollingStockAdded` domain event; repository drains event atomically  |
| API Design / Transport Boundary          | ✅ PASS | New `AddRollingStockToModelArgs` follows ADR 8 (`Args` suffix, `Validate`, `specta::Type`)                    |
| Domain Logic Location                    | ✅ PASS | All creation/validation logic stays in Rust; frontend only assembles form values                              |
| Simplicity & Semantic Versioning         | ✅ PASS | Minimal surface change — one new command, one modified component, one modified primitive                      |

## Project Structure

### Documentation (this feature)

```text
specs/032-rolling-stock-list-ux/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── add_rolling_stock_to_model.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src-tauri/src/catalog/
├── application/
│   └── add_rolling_stock_to_model.rs       # NEW: use case
├── domain/railway_model/
│   ├── rolling_stock_params.rs             # UNCHANGED: RollingStockParams already covers all categories
│   └── railway_model.rs                    # UNCHANGED: add_rolling_stock() domain method already exists
└── infrastructure/railway_model/
    └── sqlite_railway_model_repository.rs  # UNCHANGED: RollingStockAdded event handling already implemented
src-tauri/src/commands/
    └── catalogue_commands.rs               # MODIFIED: add new #[tauri::command] fn

src/lib/
├── components/
│   ├── InPlaceEdit.svelte                  # MODIFIED: remove Save/Cancel pill, keep auto-save
│   └── model-details/
│       ├── RollingStockList.svelte         # MODIFIED: empty-state CTA + "Add More" button + drawer wiring
│       └── RollingStockCard.svelte         # MODIFIED: always show all fields with "—" for empty values
├── features/rolling-stock-edit/
│   └── components/
│       └── RollingStockCreateDrawer.svelte # NEW: creation drawer (category + identification form)

messages/
    └── *.json                              # MODIFIED: add new Paraglide keys for create workflow

src/__tests__/components/
    ├── InPlaceEdit.test.ts                 # MODIFIED: update for removed buttons
    └── RollingStockList.test.ts            # MODIFIED: update for new CTA and "Add More" button
src-tauri/src/catalog/application/
    └── add_rolling_stock_to_model.test.rs  # NEW: embedded unit tests
```

## Complexity Tracking

_No constitution violations. No additional justification required._

---

## Phase 0: Research

### Research findings

All unknowns were resolved through codebase exploration. No external references required.

---

### Decision 1: InPlaceEdit pill removal strategy

**Decision**: Remove the floating Save/Cancel pill entirely from `InPlaceEdit.svelte`, remove the `suppressBlurSave` flag, keep `handleBlur → save()` and `Escape → cancel()` as the sole commit/revert paths. The `error` display `mt-7` margin offset (which cleared the pill) is replaced with `mt-1`.

**Rationale**: The feature specification explicitly removes buttons. `handleBlur` already calls `save()` and is already guarded by `suppressBlurSave` — once the pill is gone the guard is no longer needed. Enter-to-save already works via `handleKeydown`. No API shape change.

**Alternatives considered**: Keeping an optional `showButtons` prop — rejected because no other call site currently needs buttons; this adds dead complexity. Creating a separate `InlineField` component — rejected as over-engineering; `InPlaceEdit` is small and the change is a pure subtraction.

---

### Decision 2: RollingStockCard empty-field display

**Decision**: Remove the `{#if editable || rollingStock.livery}` and `{#if editable || rollingStock.railwayCompanyName}` guards. Always render Livery and Railway Company rows. Add a Depot row (always rendered). Use `{value || '—'}` pattern already present for Series and Road Number. `control` row rendered unconditionally with `{rollingStock.control ?? '—'}`.

**Rationale**: The spec states "If a field has no saved value, display '—'". `OwnedRollingStockView` has `livery`, `railwayCompanyName`, `control` as nullable. `depot` is **not** on `OwnedRollingStockView` — it is a catalog-side field on `RollingStockView`. The inline-edit commands (`updateRollingStockIdentification`) do accept `depot`. Adding Depot display in RollingStockCard requires either (a) surfacing `depot` on `OwnedRollingStockView` by updating the Rust view or (b) loading the full `RollingStockView` per card. **Decision: surface `depot` on `OwnedRollingStockView`** — one new nullable field, no migration needed (already in `rolling_stocks` table). This keeps the card lightweight.

**Alternatives considered**: Loading `RollingStockView` per card — rejected; creates N+1 IPC calls per list render. Omitting Depot row — rejected; spec explicitly lists Depot as an example field.

---

### Decision 3: "Add Rolling Stock" drawer — create vs extend

**Decision**: Create a new `RollingStockCreateDrawer.svelte` component rather than extending `RollingStockSpecsDrawer` with a creation mode.

**Rationale**: The creation form needs a Category selector (Locomotive / Passenger Car / etc.) as a first step because `RollingStockParams` is a discriminated enum and the category determines which type-specific fields appear. The existing `RollingStockSpecsDrawer` is an edit-only drawer that loads an existing entry by `rollingStockId`. Mixing creation and edit modes into one component creates significant conditional complexity. A dedicated creation drawer is ~80 lines and maps cleanly to `add_rolling_stock_to_model`.

**Alternatives considered**: Reusing `RollingStockSpecsDrawer` with a `mode: 'create' | 'edit'` prop — rejected; the create path lacks a `rollingStockId` and requires an additional Category step, making the component logic heavily branched. A multi-step wizard — rejected as over-engineering for the current scope.

---

### Decision 4: New backend command scope

**Decision**: Add one new Tauri command `add_rolling_stock_to_model` with a simplified `AddRollingStockToModelArgs` struct. For MVP, the creation form supports a **simplified variant** (matching the existing `SimplifiedRollingStockArgs` pattern): `railway_company_id`, `category` (enum string), `series_code`, `road_number?`, `livery?`, `depot?`, `control?`. Advanced fields (technical specs, DCC, coupling) are accessible via the existing "Edit Specs" drawer after creation.

**Rationale**: The feature spec lists "Series Code, Depot, Livery, Length, Control Type" as the drawer fields. The `RollingStockParams` discriminated enum requires a `locomotive_type` / `electric_multiple_unit_type` / `railcar_type` for some categories. For the MVP creation drawer, accepting `category + series_code` is sufficient to produce a valid entry. Advanced details are already editable via the specs drawer. This avoids a deeply nested form in the creation drawer.

**Alternatives considered**: Full `RollingStockParams` form in the drawer — rejected; the discriminated enum requires 5 different sub-forms and is out of scope for this feature. Using the existing `SimplifiedRollingStockArgs` for owned rolling stock — rejected; that type is for collection/wishlist owned instances, not catalog entries.

---

### Decision 5: OwnedRollingStockView depot field

**Decision**: Add `depot: Option<String>` to `OwnedRollingStockView` (Rust struct) and the corresponding query in `sqlite_railway_model_repository.rs`. Re-run `pnpm tauri dev` to regenerate bindings.

**Rationale**: Required to display Depot inline in `RollingStockCard` without an extra IPC call. The `rolling_stocks` table already has a `depot` column.

**Alternatives considered**: Fetching full `RollingStockView` per card — rejected (N+1 problem). A separate `get_rolling_stock_depot` command — rejected (over-engineering).

---

## Phase 1: Design & Contracts

_See [data-model.md](./data-model.md), [contracts/](./contracts/), and [quickstart.md](./quickstart.md) for full artifacts._

### Key design decisions summary

- **Backend**: New use case `AddRollingStockToModel` + command; `OwnedRollingStockView.depot` field added.
- **Frontend**: `InPlaceEdit` simplified (no buttons); `RollingStockCard` unconditional field rows + depot; `RollingStockList` new CTA + drawer; `RollingStockCreateDrawer` new component.
- **Paraglide**: ~5 new message keys needed (create drawer title, success, error, CTA text, "add more" button).
- **Tests**: Existing `RollingStockList.test.ts` and `InPlaceEdit.test.ts` must be updated; new tests for `RollingStockCreateDrawer`.
