# Implementation Plan: Track Inventory Management

**Branch**: `005-track-inventory` | **Date**: 2026-01-30 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/005-track-inventory/spec.md`

**Note**: Rust backend is largely complete. This plan focuses on frontend implementation and minor Rust adjustments.

## Summary

Implement a "My Tracks" feature allowing users to manage multiple track inventories. Users can create inventories, add purchases (updating stock quantities automatically), view purchase history, and compare stock vs required quantities. The Rust backend provides commands for CRUD operations; this plan adds query handlers with View structs and implements the complete Svelte frontend.

## Technical Context

**Language/Version**: Rust 1.93.0 (backend), TypeScript 5.9.3 (frontend)
**Primary Dependencies**: Tauri 2.9.x, SvelteKit, Svelte 5, Tailwind 4.1, Skeleton UI 4.x, sqlx, specta/tauri-specta
**Storage**: SQLite via sqlx (migration `0006_create_tracks_inventory.sql` exists)
**Testing**: cargo test (Rust), Vitest (frontend)
**Target Platform**: Desktop via Tauri
**Project Type**: Tauri hybrid (Rust backend + SvelteKit frontend)
**Performance Goals**: <200ms for read queries (per Constitution)
**Constraints**: Offline-capable, all persistence via Rust/SQLite
**Scale/Scope**: Single user, multiple inventories

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                            | Status  | Notes                                                                                              |
| ------------------------------------ | ------- | -------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**           | ✅ PASS | Using SQLite via sqlx; migration 0006 exists. Adding `track_type` column via new migration.        |
| **State Management / Domain Events** | ✅ PASS | `TrackInventory` aggregate already implements `pending_events` pattern (see `track_inventory.rs`). |
| **API Design & Transport**           | ✅ PASS | Commands use `Args`/`Input` pattern per ADR-8. Query handlers will return `View` structs.          |
| **Domain Logic Location**            | ✅ PASS | All business logic in Rust; frontend is rendering only.                                            |
| **Modular, Library-First**           | ✅ PASS | Feature is self-contained in `tracks_inventory/` module.                                           |
| **Test-First Emphasis**              | ✅ PASS | Rust tests exist; frontend tests to be added.                                                      |
| **Code Quality**                     | ✅ PASS | Will run clippy, fmt, eslint, prettier.                                                            |
| **UX Consistency**                   | ✅ PASS | Will use Paraglide for all strings, Skeleton UI components.                                        |
| **Performance Requirements**         | ✅ PASS | Simple queries on indexed tables; no N+1 expected.                                                 |

## Project Structure

### Documentation (this feature)

```text
specs/005-track-inventory/
├── plan.md              # This file
├── research.md          # Phase 0 output - Rust changes and patterns
├── data-model.md        # Phase 1 output - Entity definitions
├── quickstart.md        # Phase 1 output - Implementation guide
├── contracts/           # Phase 1 output - API contracts
│   └── api.md           # Query and command interface definitions
└── tasks.md             # Phase 2 output (created by /speckit.tasks)
```

### Source Code (repository root)

```text
# Backend (Rust - mostly complete)
src-tauri/
├── migrations/
│   └── 0007_add_track_type_to_products.sql   # NEW: Add track_type column
└── src/tracks_inventory/
    ├── domain/
    │   ├── track_product.rs                   # MODIFY: Add track_type field
    │   ├── track_type.rs                      # EXISTS: TrackType enum
    │   └── views.rs                           # NEW: View structs for queries
    ├── application/
    │   ├── mod.rs                             # MODIFY: Export view structs and queries
    │   ├── get_track_inventories.rs           # NEW: List all inventories query
    │   ├── get_track_inventory.rs             # NEW: Get single inventory with items
    │   ├── get_track_products.rs              # NEW: List track products query
    │   └── views.rs                           # NEW: Application-layer view structs
    ├── infrastructure/
    │   ├── entities.rs                        # MODIFY: Add track_type to row struct
    │   └── sqlite_track_product_repository.rs # MODIFY: Update queries for track_type
    └── interface/
        ├── query_handlers.rs                  # NEW: Tauri query commands
        └── mod.rs                             # MODIFY: Export query handlers

# Frontend (Svelte - NEW)
src/
├── routes/my-tracks/
│   ├── +page.svelte                           # NEW: Track inventories list page
│   └── [id]/
│       └── +page.svelte                       # NEW: Single inventory detail page
├── lib/features/track-inventory/
│   ├── index.ts                               # MODIFY: Export public API
│   ├── TrackInventoryState.svelte.ts          # NEW: State controller
│   ├── services/
│   │   └── TrackInventoryService.svelte.ts    # NEW: Service class
│   ├── components/
│   │   ├── InventoryList.svelte               # NEW: List of inventories
│   │   ├── InventoryCard.svelte               # NEW: Inventory summary card
│   │   ├── InventoryDetail.svelte             # NEW: Detail view with items
│   │   ├── PurchaseHistory.svelte             # NEW: Purchase history list
│   │   ├── AddPurchaseDialog.svelte           # NEW: Add purchase modal
│   │   └── CreateInventoryDialog.svelte       # NEW: Create inventory modal
│   ├── domain/
│   │   └── types.ts                           # NEW: Frontend domain types
│   └── README.md                              # MODIFY: Update from placeholder
└── lib/features/navigation/components/
    ├── SidebarNavigation.svelte               # MODIFY: Add My Tracks link
    └── BottomNavigation.svelte                # MODIFY: Add My Tracks link

# Localization
messages/
├── en.json                                    # MODIFY: Add track inventory strings
└── it.json                                    # MODIFY: Add track inventory strings
```

**Structure Decision**: Follows established feature-grouped pattern with DDD layering in Rust and controller-based state management in Svelte. View structs will be defined in the application layer as requested.

## Complexity Tracking

No constitution violations. Standard feature implementation.
