# Implementation Plan: Train Formations

**Branch**: `039-train-formations` | **Date**: 2026-03-29 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/039-train-formations/spec.md`

## Summary

Build a Train Formations module that lets users compose ordered consists from a seeded Prototype master catalog. Each formation slot (`FormationElement`) references a mandatory `Prototype` (series, car type, traction classification) and optionally assigns a specific owned physical model (`owned_rolling_stocks`). The frontend renders a horizontal track view with ownership badges, drag-and-drop reordering, and a real-time traction warning. The approach extends the existing hexagonal catalog architecture with a new `trains` domain module in `src-tauri/src/trains/`, a SQLite migration, and a Svelte 5 feature module under `src/lib/features/train-formations/`.

## Technical Context

**Language/Version**: Rust 1.93+ (edition 2024); TypeScript 5.9 strict; Svelte 5.48

**Primary Dependencies**:

- Backend: `sqlx` (async SQLite), `tauri-specta` (type gen), `uuid`, `garde` (v0.22.1)
- Frontend: SvelteKit, Svelte 5 Runes, Tailwind CSS v4, shadcn-svelte, `svelte-dnd-action` (drag-and-drop — **approved 2026-03-29**)

**Storage**: SQLite via `sqlx` migrations. Migration file: `0009_create_train_formations_schema.sql`. Additive `ALTER TABLE` on `owned_rolling_stocks` to add `prototype_id`.

**Testing**: `cargo test` (Rust); `vitest` with `happy-dom` (frontend, under `src/__tests__/`)

**Target Platform**: Desktop (Linux/macOS/Windows) via Tauri 2.0; tablet touch for DnD (SC-007)

**Project Type**: Tauri 2.0 desktop app — single project, frontend (`src/`) + Rust backend (`src-tauri/`)

**Performance Goals**:

- Track view renders 50 cells in <2s (SC-002)
- Traction warning updates <500ms after composition change (SC-003)
- Drawer search results within 300ms of last keystroke (SC-004)

**Constraints**: Fully offline (no network), Paraglide for all strings (EN + IT), no `unwrap()` in Rust, `specta` types only (no manual TS type definitions)

**Scale/Scope**: ~50 units max per formation (SC-002); personal use (one user); 14 Tauri commands, 5 frontend pages/components, 1 migration

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                       | Status  | Notes                                                                                                                                                              |
| ------------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Modular, Library-First**      | ✅ PASS | New `trains` domain module is self-contained under `src-tauri/src/trains/` and `src/lib/features/train-formations/`. No cross-domain coupling added.               |
| **Deterministic Interfaces**    | ✅ PASS | All 14 IPC commands defined in `contracts/tauri-ipc.md`; typed via `tauri-specta`.                                                                                 |
| **Test-First Emphasis**         | ✅ PASS | Rust use-case tests and Vitest component tests specified in `quickstart.md` Phase F.                                                                               |
| **Code Quality**                | ✅ PASS | No `unwrap()`, all `Args` derive `garde::Validate`, CI gates apply.                                                                                                |
| **Testing Standards**           | ✅ PASS | Unit tests for domain logic (traction eval, reorder); integration tests via in-memory SQLite.                                                                      |
| **User Experience Consistency** | ✅ PASS | All strings via Paraglide (EN + IT). UI follows steampunk design tokens.                                                                                           |
| **Performance Requirements**    | ✅ PASS | SC-002/003/004 targets defined. Bulk reorder pattern avoids N+1 DB writes.                                                                                         |
| **Safe Rust**                   | ✅ PASS | `Result<T,E>` error handling throughout; no `unsafe`.                                                                                                              |
| **Database (Persistence)**      | ✅ PASS | SQLite via `sqlx`. Migration file `0009_...sql`. `PRAGMA foreign_keys = ON` already enforced app-wide. `owned_rolling_stocks` alteration is additive non-breaking. |
| **State Management**            | ✅ PASS | Domain event pattern: `TrainFormationEvent` enum; Repository drains events atomically inside transaction.                                                          |
| **API Design / Transport**      | ✅ PASS | ADR 8 conventions: `Args` / `Input` / View model separation. `specta`-typed bindings.                                                                              |
| **Domain Logic Location**       | ✅ PASS | Traction evaluation, year-range validation, uniqueness check — all in Rust use-case layer. Frontend computes only derived display state (`$derived`).              |
| **Paraglide Strings**           | ✅ PASS | `formations_*` namespace keys in `en.json` + `it.json`. No hardcoded UI strings.                                                                                   |

**No violations. No complexity justification required.**

## Project Structure

### Documentation (this feature)

```text
specs/039-train-formations/
├── plan.md              # This file
├── research.md          # Phase 0 — all unknowns resolved
├── data-model.md        # Phase 1 — entities, SQL migration, seed data
├── quickstart.md        # Phase 1 — implementation guide (Phases A–G)
├── contracts/
│   └── tauri-ipc.md     # Phase 1 — 14 Tauri IPC command contracts
└── tasks.md             # Phase 2 output (NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Backend (Rust / Tauri)
src-tauri/
├── migrations/
│   └── 0009_create_train_formations_schema.sql   # NEW
├── src/
│   └── trains/                                    # NEW domain module
│       ├── mod.rs
│       ├── domain/
│       │   ├── prototype/            # Prototype aggregate + repository trait
│       │   ├── formation/            # TrainFormation aggregate + events + element
│       │   └── formation_category/
│       ├── application/              # 14 use-case handlers
│       ├── infrastructure/           # SQLx repos + mappers + seed_data
│       └── interface/                # command_args.rs + command_handlers.rs

# Frontend (SvelteKit / Svelte 5)
src/
├── routes/
│   └── train-formations/
│       ├── +page.svelte              # NEW — formation list
│       └── [id]/
│           └── +page.svelte          # NEW — formation builder
├── lib/
│   ├── features/
│   │   └── train-formations/         # NEW feature module
│   │       ├── index.ts
│   │       ├── TrainFormationState.svelte.ts
│   │       ├── components/           # 13 components (see quickstart.md)
│   │       ├── domain/traction.ts    # isTractionSlot pure function
│   │       ├── services/             # safeInvoke wrappers
│   │       └── types/
│   └── components/navigation/
│       └── config.ts                 # ADD train-formations entry (isPrimary: false)
messages/
├── en.json                           # ADD formations_* keys
└── it.json                           # ADD formations_* Italian translations
```

**Structure Decision**: Follows the established hexagonal pattern (`catalog` module) for backend, and the feature-modular pattern (`collection`, `track-inventory`) for frontend. No new architectural patterns introduced.
