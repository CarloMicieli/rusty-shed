# Implementation Plan: Acquisition Flow

**Branch**: `038-acquisition-flow` | **Date**: 2026-03-12 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/038-acquisition-flow/spec.md`

## Summary

Replace the "Add Railway Model" dashboard quick-action with a "New Acquisition" side drawer that
records a batch of purchased railway models in one session. Each item in the batch either creates a
new catalog entry (if the manufacturer + product code is unknown) or reuses an existing one —
determined by the deterministic `RailwayModelId` derived from those two fields. All items are
persisted as collection entries with purchase info in a single backend call via a new
`record_acquisition` Tauri command backed by a new `RecordAcquisition` use case. A global Ctrl+N
shortcut (pending dependency approval) opens the drawer from any screen.

## Technical Context

**Language/Version**: TypeScript 5.9.3 (frontend), Rust 2024 edition / rust-version 1.93.0 (backend)
**Primary Dependencies**:

- Frontend: SvelteKit + Svelte 5.48.2, Tailwind CSS v4, shadcn-svelte, Paraglide 2.7.1, lucide-svelte
- Backend: Tauri v2.9.x, sqlx, tauri-specta, validator
- New (pending approval): `tauri-plugin-global-shortcut = "2"` (Ctrl+N shortcut only)

**Storage**: SQLite via sqlx — reuses existing tables (`railway_models`, `collection_items`,
`purchase_infos`). No new migrations.

**Testing**: Vitest + happy-dom (frontend), cargo test (backend)

**Target Platform**: Desktop (Linux / macOS / Windows) via Tauri 2.0

**Project Type**: Tauri desktop app — frontend in `src/`, backend in `src-tauri/`

**Performance Goals**: Drawer open < 300ms (SC-005); backend `record_acquisition` < 200ms p95 for
batches of ≤ 20 items (Tauri SLO from constitution)

**Constraints**:

- All business/domain logic in Rust (Domain Logic Location law)
- No hardcoded UI strings — all via Paraglide
- specta bindings must be regenerated after any command change
- No `unwrap()` in Rust production paths
- Dependency additions require user approval (CLAUDE.md)

**Scale/Scope**: Single-user desktop app; typical batch = 1–10 items per session

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design._

| Principle                       | Status  | Evidence                                                                                                        |
| ------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------- |
| Database (Persistence)          | ✅ PASS | Reuses existing sqlx-managed SQLite tables; no ad-hoc schema changes; migrations untouched                      |
| State / Domain Event Tracking   | ✅ PASS | `RecordAcquisition` use case calls `collection.add_item()` which records domain events; repo drains atomically  |
| API Design / Transport Boundary | ✅ PASS | `RecordAcquisitionArgs` derives `Debug, Clone, Validate, specta::Type, Deserialize`; specta binding regenerated |
| Domain Logic Location           | ✅ PASS | Upsert logic, ID derivation, batch processing all in Rust use case; frontend is rendering + UX only             |
| Code Quality                    | ✅ PASS | Paraglide for all strings; strict TypeScript; cargo clippy -D warnings; no `unwrap()`                           |
| Testing Standards               | ✅ PASS | Unit tests required for `RecordAcquisition` use case; component tests for `AcquisitionDrawer`                   |
| UX Consistency                  | ✅ PASS | Follows existing drawer pattern (AddModelDrawer); Paraglide messages; same scroll-lock pattern                  |
| Performance Requirements        | ✅ PASS | Backend command stays within 200ms SLO; no long-running work on UI thread                                       |
| Safe Rust Practices             | ✅ PASS | `Result<T, E>` throughout; no panics; no `unsafe`                                                               |

**Post-design re-check**: All gates still pass. No violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/038-acquisition-flow/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 — all unknowns resolved
├── data-model.md        # Phase 1 — entities, Rust types, frontend types
├── quickstart.md        # Phase 1 — step-by-step for implementors
├── contracts/
│   ├── rust-command.md  # record_acquisition command contract
│   └── frontend-state.md # AcquisitionDrawer component contracts + Paraglide keys
├── checklists/
│   └── requirements.md  # Spec quality checklist (all passing)
└── tasks.md             # Phase 2 output (/speckit.tasks — NOT created here)
```

### Source Code

```text
src-tauri/src/collecting/
├── application/
│   ├── mod.rs                          # add: pub mod record_acquisition
│   └── record_acquisition.rs           # NEW: RecordAcquisition use case
├── interface/
│   ├── command_args.rs                 # EDIT: add RecordAcquisitionArgs, AcquisitionItemArgs
│   └── command_handlers.rs             # EDIT: add record_acquisition handler
src-tauri/src/
└── lib.rs                              # EDIT: register record_acquisition in collect_commands!

src/lib/features/acquisition/           # NEW feature module
├── types.ts                            # AcquisitionFormState, AcquisitionItemEntry, BatchDefaults
├── AcquisitionState.svelte.ts          # Service/context wrapper
├── AcquisitionDrawer.svelte            # Root drawer component
└── components/
    ├── AcquisitionHeader.svelte        # Sticky header: seller, date, batch defaults
    ├── AcquisitionItemCard.svelte      # Per-item card: all model fields + price + actions
    └── AcquisitionFooter.svelte        # Sticky footer: Add Item + Finalize

src/lib/bindings.ts                     # AUTO-GENERATED — regenerate via pnpm tauri dev
messages/en.json                        # EDIT: add ~28 new acquisition_* keys
src/routes/dashboard/+page.svelte       # EDIT: replace button, mount AcquisitionDrawer
src/routes/+layout.svelte               # EDIT (shortcut only): listen("open-acquisition-drawer")
```

## Implementation Phases

### Phase A — Backend (no frontend changes)

1. Add `RecordAcquisitionArgs` + `AcquisitionItemArgs` to `command_args.rs`
2. Implement `RecordAcquisition` use case in `record_acquisition.rs`
3. Add `record_acquisition` command handler to `command_handlers.rs`
4. Register in `lib.rs`
5. `cargo check && cargo clippy && cargo test`
6. Run `pnpm tauri dev` briefly to regenerate `src/lib/bindings.ts`

### Phase B — Frontend Core

1. Add Paraglide keys to `messages/en.json`; run `pnpm prepare`
2. Create `src/lib/features/acquisition/types.ts`
3. Create `AcquisitionState.svelte.ts`
4. Create `AcquisitionItemCard.svelte`
5. Create `AcquisitionHeader.svelte`
6. Create `AcquisitionFooter.svelte`
7. Create `AcquisitionDrawer.svelte` (composes above)
8. `pnpm check && pnpm lint`

### Phase C — Dashboard Integration

1. Update `src/routes/dashboard/+page.svelte`: rename button, wire drawer
2. `pnpm test` — verify existing dashboard tests still pass

### Phase D — Global Shortcut

1. ~~Add Cargo.toml dependency~~ ✅ Added (`tauri-plugin-global-shortcut = "2"`)
2. Add capability entry to `src-tauri/capabilities/default.json`
3. Register plugin + shortcut in `lib.rs`
4. Add layout-level event listener in `src/routes/+layout.svelte`
5. Full regression test

## Open Questions / Blockers

_None. All phases unblocked._
