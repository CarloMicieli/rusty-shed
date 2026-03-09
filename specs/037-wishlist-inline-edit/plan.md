# Implementation Plan: Wishlist Item Sidebar Inline Editing

**Branch**: `037-wishlist-inline-edit` | **Date**: 2026-03-09 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/037-wishlist-inline-edit/spec.md`

## Summary

Add hover-and-click inline editing to `WishlistItemSidebar.svelte` for four fields — Priority (dropdown), Status (dropdown), Desired Price (numeric input with currency), and Added Date (date-picker constrained to past dates). The List field remains read-only. Requires a new `update_wishlist_item` Tauri command backed by a new `WishlistEvent::ItemUpdated` domain event and `UpdateWishlistItemUseCase`. A `calendar` and `popover` shadcn-svelte component must be added. No database schema changes are needed.

## Technical Context

**Language/Version**: Rust 1.93 (backend), TypeScript 5.9 + Svelte 5.48 (frontend)  
**Primary Dependencies**: Tauri 2.9, shadcn-svelte (bits-ui calendar), specta, sqlx, Vitest  
**Storage**: SQLite via sqlx — existing `wishlist_items` table; no migration required  
**Testing**: Vitest 4 (happy-dom) for frontend; cargo test + rstest for Rust  
**Target Platform**: Desktop (Linux / macOS / Windows via Tauri)  
**Project Type**: Tauri desktop app (frontend in `src/`, backend in `src-tauri/`)  
**Performance Goals**: IPC write command < 200ms (Constitution SLO for UI-critical operations)  
**Constraints**: No new npm dependencies without user approval; no `unwrap()` in Rust; strict TypeScript; all strings via Paraglide  
**Scale/Scope**: Single sidebar component; one new Tauri command; four editable fields

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Law                               | Status             | Evidence                                                                                                                       |
| --------------------------------- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| **Persistence via SQLite/sqlx**   | ✅ PASS            | No new table; existing `wishlist_items` columns used. Repository UPDATE query via event drain                                  |
| **Domain Event Tracking**         | ✅ PASS            | New `WishlistEvent::ItemUpdated` variant; aggregate `update_item()` method emits it; repository drains atomically              |
| **Tauri IPC + specta types**      | ✅ PASS            | New `update_wishlist_item` command with `UpdateWishlistItemArgs` deriving `specta::Type`; TypeScript bindings regenerated      |
| **No business logic in frontend** | ✅ PASS            | Date range enforcement, price validation, and domain constraint checks are in Rust; frontend does UX-only pre-validation hints |
| **Paraglide strings**             | ✅ PASS            | New i18n keys added for inline error messages and a11y labels                                                                  |
| **No hardcoded strings**          | ✅ PASS            | All user-visible strings via `m.*()` message functions                                                                         |
| **No `unwrap()` in Rust**         | ✅ PASS - REQUIRED | Plan specifies `Result<T, E>` throughout                                                                                       |
| **Code Quality — clippy/fmt**     | ✅ PASS - REQUIRED | Workflow step 9 enforces zero clippy warnings                                                                                  |
| **Test-First**                    | ✅ PASS            | Domain unit tests + repository integration tests + Vitest component tests planned                                              |

**Post-Phase-1 re-check**: Architecture is unchanged from initial check. Double-Option `desired_price` field in the domain event follows existing `MonetaryAmount` patterns and adds no complexity violations.

## Project Structure

### Documentation (this feature)

```text
specs/037-wishlist-inline-edit/
├── plan.md              ← this file
├── spec.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── update_wishlist_item.md
├── checklists/
│   └── requirements.md
└── tasks.md             # Phase 2 output (/speckit.tasks — not yet created)
```

### Source Code

```text
src-tauri/src/wishlist/
├── domain/
│   ├── wishlist.rs                      # ADD update_item() method + apply_event arm
│   └── wishlist_event.rs                # ADD ItemUpdated variant
├── application/
│   ├── mod.rs                           # EXPORT UpdateWishlistItemUseCase
│   ├── inputs.rs                        # ADD UpdateWishlistItemInput
│   └── update_wishlist_item.rs          # NEW use case file
└── interface/
    ├── command_args.rs                  # ADD UpdateWishlistItemArgs
    ├── command_handlers.rs              # ADD update_wishlist_item handler
    └── mod.rs                           # EXPORT UpdateWishlistItemArgs

src/lib/
├── components/
│   └── ui/
│       ├── calendar/                    # NEW (pnpm dlx shadcn-svelte add calendar)
│       └── popover/                     # NEW (pnpm dlx shadcn-svelte add popover)
├── features/wishlists/
│   └── components/
│       └── WishlistItemSidebar.svelte   # MODIFY — add inline edit UX

src/routes/wishlists/[wishlistId]/items/[itemId]/
└── +page.svelte                         # MODIFY — pass defaultCurrency prop to WishlistItemSidebar

messages/
├── en.json                              # ADD new inline-edit i18n keys
└── it.json                              # ADD new inline-edit i18n keys

src/__tests__/features/wishlists/
└── WishlistItemSidebar.test.ts          # MODIFY — add inline-edit test cases
```

**Structure Decision**: Tauri desktop app layout. Backend domain/application/interface layers strictly separated per hexagonal architecture. Frontend feature-modular under `src/lib/features/wishlists/`.

## Complexity Tracking

_No Constitution violations — no complexity justification required._
