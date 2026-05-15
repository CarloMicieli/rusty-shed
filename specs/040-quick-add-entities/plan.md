# Implementation Plan: On-the-Fly Entity Quick-Add

**Branch**: `040-quick-add-entities` | **Date**: 2026-05-15 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/040-quick-add-entities/spec.md`

## Summary

Implement an end-to-end Quick-Add flow for Manufacturer, Seller, and Buyer directly inside Acquisition, Collection Item, and Wishlist forms. The feature adds contextual `+` triggers, stacked quick-add UI, client-side duplicate prevention, backend create commands with case-insensitive uniqueness enforcement, immediate local-state insertion and auto-selection of the new entity, localized feedback messages, and full verification coverage.

This plan is self-contained for Feature 040 and includes all required backend, frontend, i18n, bindings, and testing work needed to deliver the feature.

## Technical Context

**Language/Version**: Rust 1.93.0, TypeScript 5.9.3 (strict), Svelte 5.48.2 (Runes only)
**Primary Dependencies**: Tauri 2.11.1, specta 2.0.0-rc.25, sqlx 0.8.6, garde 0.22.1, Zod 4, sveltekit-superforms, svelte-sonner, Paraglide-JS
**Storage**: SQLite
**Testing**: `pnpm svelte-check`, Vitest 4.0.18, `cargo test`, `cargo clippy -- -D warnings`
**Target Platform**: Tauri desktop (Linux/macOS/Windows) + mobile viewport behavior for quick-add bottom-sheet
**Project Type**: Tauri + Svelte monorepo
**Performance Goals**:
- Duplicate warning visible within 500 ms after user input
- Create commands complete within 200 ms p95 in normal local usage
- Quick-add completion flow in under 60 seconds for first-time users
**Constraints**:
- No hardcoded user-facing strings (Paraglide keys required in `en` and `it`)
- No `any` in TypeScript
- No `unwrap()` in Rust
- One quick-add drawer active at a time
- Parent form data must remain intact across open/save/cancel/error
**Scale/Scope**:
- Up to ~500 manufacturers and ~500 sellers loaded in parent form state
- Create flows for: manufacturer (all target forms), seller (acquisition), buyer (acquisition, mapped to seller domain)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-checked post-design — all gates pass._

| Principle | Status | Notes |
|---|---|---|
| Database-first correctness | PASS | Adds/aligns case-insensitive uniqueness for manufacturer and seller names. |
| Domain logic in Rust | PASS | Create operations, validation, and conflict handling stay in backend command/service layer. |
| Typed transport boundary | PASS | Uses specta-generated command bindings and typed args/results. |
| Frontend state safety | PASS | Parent forms keep local state; quick-add is additive and non-destructive. |
| Localization completeness | PASS | Every new key added in both `messages/en.json` and `messages/it.json`. |
| Testing integrity | PASS | Adds/updates unit/integration tests; no skipped/deleted tests. |
| Code quality gates | PASS | Requires `cargo clippy -- -D warnings`, `pnpm svelte-check`, lint and tests clean. |

## Project Structure

### Documentation (this feature)

```text
specs/040-quick-add-entities/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── ipc-commands.md
└── tasks.md
```

### Source Code (repository root)

```text
# Backend (Rust/Tauri) - create + validation + uniqueness
src-tauri/src/
├── catalog/
│   ├── interface/manufacturers.rs              # add create_manufacturer command + args DTO
│   ├── application/                            # create use case wiring (if needed by current architecture)
│   └── domain/                                 # invariants/error mapping (if needed by current architecture)
├── sellers/
│   ├── interface/command_handlers.rs           # ensure create_seller quick-add path supports buyer context
│   ├── application/                            # service wiring if updates required
│   └── domain/                                 # invariants/error mapping if updates required
└── lib.rs / command registration modules       # register any new command exports

src-tauri/migrations/
└── <new_migration>.sql                         # case-insensitive unique indexes on manufacturer/seller names

# Frontend (Svelte/Tauri)
src/lib/
├── components/drawer/
│   ├── DrawerShell.svelte                      # optional dimmed mode
│   └── QuickAddShell.svelte                    # stacked drawer/bottom-sheet shell
├── features/quick-add/
│   ├── QuickAddEntityForm.svelte               # shared quick-add form component
│   └── types.ts                                # quick-add target/state/types
├── schemas/
│   └── quick-add-form.ts                       # zod schema for quick-add input
├── features/acquisition/
│   ├── AcquisitionDrawer.svelte
│   └── components/
│       ├── AcquisitionItemCard.svelte
│       └── AcquisitionBatchFields.svelte
├── features/collection/components/
│   └── AddCollectionItemDrawer.svelte
├── features/wishlists/
│   └── AddWishlistItemDrawer.svelte
└── bindings.ts                                 # regenerated via specta

messages/
├── en.json
└── it.json

src/__tests__/
├── quick-add/
│   └── QuickAddEntityForm.test.ts
└── acquisition/
    └── AcquisitionDrawer.test.ts
```

## Implementation Scope (Feature 040)

### 1. Backend Commands and Validation

1. Add `create_manufacturer` Tauri command in manufacturer interface module.
2. Define `CreateManufacturerArgs` (specta type + serde + garde validation).
3. Enforce trimmed non-empty name and canonicalized optional fields.
4. Return full created `Manufacturer` object on success.
5. Map duplicate conflicts to serializable `CommandError::Conflict` (or project-equivalent conflict variant).
6. Ensure `create_seller` supports quick-add payload shape for both Seller and Buyer context (buyer uses seller domain record).
7. Register exposed commands in Tauri command list.

### 2. Database Uniqueness Guarantees

1. Add migration for case-insensitive uniqueness:
- Manufacturer: unique index on `LOWER(name)`.
- Seller: unique index on `LOWER(name)`.
2. Keep migration idempotent and safe for existing data.
3. Document or handle index creation failures caused by existing duplicates (explicit conflict remediation path in migration comments or pre-check logic).

### 3. Specta and Typed Frontend Integration

1. Run `pnpm specta:generate` after Rust command/type changes.
2. Use only generated command APIs from `src/lib/bindings.ts`.
3. No manual type redefinitions for command payloads/results.

### 4. Quick-Add UI Shell and Form Reuse

1. Implement `QuickAddShell.svelte` for desktop stacked drawer (`z-[110]`) and scrim (`z-[105]`).
2. Implement mobile behavior as bottom-sheet (~80% height) with swipe-to-dismiss and keyboard-safe save action visibility.
3. Extend `DrawerShell.svelte` with optional dimmed mode so parent form is visible but non-interactive during quick-add.
4. Implement `QuickAddEntityForm.svelte` as shared create-only form with fields:
- Required: Name
- Optional: Website, Country
5. Ensure quick-add shows no edit/delete controls.

### 5. Parent Form Wiring (All Required Contexts)

1. Acquisition:
- Manufacturer quick-add trigger and auto-select result.
- Seller quick-add trigger and auto-select result.
- Buyer quick-add trigger and auto-select result (mapped to seller create).
2. Collection Item:
- Manufacturer quick-add trigger and auto-select result.
3. Wishlist Item:
- Manufacturer quick-add trigger and auto-select result.
4. Enforce single active quick-add instance per parent form session.

### 6. Duplicate Check and Save Rules

1. Duplicate detection is case-insensitive using already-loaded local list.
2. Save disabled when:
- name is empty/whitespace
- duplicate detected
- request in progress
3. Backend remains source of truth; race conflicts from concurrent writes must display error and keep drawer data intact.

### 7. Success/Error UX and i18n

1. Add all quick-add UI/toast/validation keys to `messages/en.json` and `messages/it.json`.
2. Use Paraglide messages only in UI text and errors.
3. Show non-blocking success toast identifying entity type and name.
4. On failure, keep drawer open and preserve entered values.

### 8. Testing and Verification

1. Frontend tests (minimum):
- Save disabled on empty/whitespace name.
- Save disabled on duplicate (case-insensitive).
- Successful create calls correct command by target type.
- Success inserts entity locally and auto-selects parent field.
- Cancel/dismiss preserves parent form state.
- Failed save keeps drawer open with entered values and error message.
2. Backend tests (minimum):
- `create_manufacturer` validation failures.
- Duplicate conflict behavior (case-insensitive).
- Successful insert returns full entity.
3. Run full checks:
- `pnpm specta:generate`
- `pnpm svelte-check`
- `pnpm lint`
- `pnpm test`
- `cargo test`
- `cargo clippy -- -D warnings`

## Delivery Gates

Feature 040 is complete only when all are true:

1. Quick-add works in Acquisition, Collection, and Wishlist contexts for required entity targets.
2. Parent form data is preserved across quick-add open/save/cancel/error flows.
3. Backend create + conflict handling + case-insensitive uniqueness are implemented and verified.
4. `messages/en.json` and `messages/it.json` both contain all new keys.
5. Specta bindings are regenerated and used.
6. All verification commands pass with zero warnings/errors relevant to changed scope.

## Complexity Tracking

No constitution violations and no additional complexity exemptions required.
