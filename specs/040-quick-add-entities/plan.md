# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: [e.g., Python 3.11, Swift 5.9, Rust 1.75 or NEEDS CLARIFICATION]  
**Primary Dependencies**: [e.g., FastAPI, UIKit, LLVM or NEEDS CLARIFICATION]  
**Storage**: [if applicable, e.g., PostgreSQL, CoreData, files or N/A]  
**Testing**: [e.g., pytest, XCTest, cargo test or NEEDS CLARIFICATION]  
**Target Platform**: [e.g., Linux server, iOS 15+, WASM or NEEDS CLARIFICATION]
**Project Type**: [single/web/mobile - determines source structure]  
**Performance Goals**: [domain-specific, e.g., 1000 req/s, 10k lines/sec, 60 fps or NEEDS CLARIFICATION]  
**Constraints**: [domain-specific, e.g., <200ms p95, <100MB memory, offline-capable or NEEDS CLARIFICATION]  
**Scale/Scope**: [domain-specific, e.g., 10k users, 1M LOC, 50 screens or NEEDS CLARIFICATION]

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

[Gates determined based on constitution file]

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
# Implementation Plan: On-the-Fly Entity Quick-Add

**Branch**: `040-quick-add-entities` | **Date**: 2026-05-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/040-quick-add-entities/spec.md`

## Summary

Add contextual `+` trigger buttons beside the Manufacturer, Seller, and Buyer dropdowns in the Acquisition, Collection Item, and Wishlist Item forms. Clicking a trigger opens a lightweight `QuickAddShell` drawer (z-110) that slides over the parent form (dimmed to z-105 scrim + opacity-70). The user enters only a name (required) plus optional website and country; a reactive client-side duplicate check blocks submission until the name is unique. On success the backend returns the full entity object; the frontend pushes it directly into the parent form's local state array and auto-selects it in the dropdown. A toast confirms. Cancelling restores the parent form with zero data loss.

**This feature is frontend-only.** All new Rust CRUD commands (`create_manufacturer`) and schema migrations (LOWER() unique indexes) are owned by the prerequisite feature **041-entity-management**.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend, zero changes in 040) / TypeScript 5.9.3 strict
**Primary Dependencies**: Svelte 5.48.2 + SvelteKit, Tauri 2.11.1, specta 2.0.0-rc.25, svelte-sonner (toaster), sveltekit-superforms + Zod 4 (parent forms), garde 0.22.1 (backend validation)
**Storage**: SQLite via sqlx 0.8.6 — no schema changes in this feature
**Testing**: Vitest 4.0.18 (happy-dom), cargo test
**Target Platform**: Desktop (Tauri); mobile viewport adaptation for P3 story
**Project Type**: Web + Rust Tauri application
**Performance Goals**: Duplicate-check warning within 500 ms (client-side, effectively instant); `create_manufacturer`/`create_seller` commands < 200 ms (backend SLO per constitution)
**Constraints**: No full list re-fetch after quick-add; parent form state must survive drawer open/close; one quick-add drawer open at a time
**Scale/Scope**: ≤ 500 manufacturers, ≤ 500 sellers — client-side filtering is viable

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked post-design — all gates pass._

| Principle | Status | Notes |
|---|---|---|
| **Database — REQUIRED** | ✅ Pass | No new tables or migrations in 040. LOWER() indexes delivered by 041. |
| **State Management / Domain Events** | ✅ Pass | No new aggregates. Parent form local `$state` arrays are not domain aggregates. |
| **Transport Boundary — REQUIRED** | ✅ Pass | All IPC calls use `commands.*` (specta-generated). `CreateManufacturerArgs` follows ADR-8 (`Args` suffix, `garde::Validate`). |
| **Domain Logic in Rust** | ✅ Pass | Duplicate check uses client-side filter as UX hint only; the DB unique constraint enforces authoritative integrity server-side. |
| **Modular, Library-First** | ✅ Pass | `QuickAddShell` and `QuickAddEntityForm` are independent, reusable components. |
| **Test-First / Testing Standards** | ✅ Pass | 7 component tests specified in quickstart.md. |
| **Paraglide / No Hardcoded Strings** | ✅ Pass | 13 new i18n keys defined (en + it). |
| **Code Quality (clippy / svelte-check)** | ✅ Pass | Zero new Rust code; TypeScript strict mode enforced. |
| **UX Consistency** | ✅ Pass | Uses existing `toaster`, `DrawerShell` patterns; new `QuickAddShell` follows same visual language. |
| **Performance** | ✅ Pass | Client-side check is O(n) on ≤ 500 items; no new blocking I/O on UI thread. |

**No complexity violations.** No new projects, no Repository pattern additions, no new domain aggregates.

## Project Structure

### Documentation (this feature)

```text
specs/040-quick-add-entities/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 complete
├── data-model.md        # Phase 1 complete
├── quickstart.md        # Phase 1 complete
├── contracts/
│   └── ipc-commands.md  # Phase 1 complete
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Web + Rust (Tauri) — Option 2

# NEW files (frontend only)
src/lib/schemas/
└── quick-add-form.ts               ← Zod schema for QuickAdd form

src/lib/components/drawer/
└── QuickAddShell.svelte            ← Stacked drawer panel (z-110) + scrim (z-105)

src/lib/features/quick-add/
├── QuickAddEntityForm.svelte       ← Unified form for manufacturer / seller / buyer
└── types.ts                        ← QuickAddTarget, QuickAddState, QuickAddFormValues

# MODIFIED files (frontend)
src/lib/components/drawer/
└── DrawerShell.svelte              ← Add optional `dimmed?: boolean` prop

src/lib/features/acquisition/
├── AcquisitionDrawer.svelte        ← Mount QuickAddShell; handle onSuccess/onCancel
└── components/
    ├── AcquisitionItemCard.svelte  ← Add [+] trigger beside manufacturer Select
    └── AcquisitionBatchFields.svelte ← Add [+] trigger beside seller SearchableSelect

src/lib/features/wishlists/
└── AddWishlistItemDrawer.svelte    ← Add [+] trigger beside manufacturer Select

src/lib/features/collection/components/
└── AddCollectionItemDrawer.svelte  ← Add [+] trigger beside manufacturer Select

# MODIFIED files (i18n)
messages/en.json                    ← 13 new quick_add_* keys
messages/it.json                    ← 13 new quick_add_* keys

# NO Rust changes in this feature
# src-tauri/                        ← untouched; write commands owned by 041
```

**Structure Decision**: Web application layout (Option 2). New components are scoped to `src/lib/` following the existing `features/` and `components/` conventions. No new SvelteKit routes needed — all UI is drawer-based.

## Complexity Tracking

No constitution violations. No complexity justification required.
