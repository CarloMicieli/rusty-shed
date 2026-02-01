# Implementation Plan: Add Railway Model to Wishlist

**Branch**: `003-add-model-wishlist` | **Date**: 2026-01-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-add-model-wishlist/spec.md`

**Note**: Rust backend implementation is complete. This plan focuses on **frontend-only** changes.

## Summary

Create a side drawer UI component that allows users to add a new railway model to a wishlist. The drawer collects railway model data (manufacturer, product code, description, category, scale, power method, epoch), optional rolling stocks, and wishlist item details (desired price, priority). On submission, the frontend calls the existing `addRailwayModelToWishList` Tauri command. Additionally, update the "My Wishlists" page styling to match "My Collection".

## Technical Context

**Language/Version**: TypeScript 5.9.3, Svelte 5.48.2  
**Primary Dependencies**: SvelteKit/Vite 7.3.1, Tailwind CSS 4.1.18, Skeleton UI 4.x, Paraglide-JS 2.7.1  
**Storage**: N/A (frontend-only; Rust/SQLite backend already implemented)  
**Testing**: Vitest 4.0.18 with happy-dom  
**Target Platform**: Tauri desktop app (Windows/macOS/Linux)  
**Project Type**: Web frontend (Svelte) + Rust backend (Tauri)  
**Performance Goals**: Form submission completes within 200ms (UI responsiveness)  
**Constraints**: All user-facing strings via Paraglide; no hardcoded text  
**Scale/Scope**: 1 new drawer component, 1 service method, page styling updates

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                                    | Status  | Notes                                                                                   |
| -------------------------------------------- | ------- | --------------------------------------------------------------------------------------- |
| **Modular, Library-First Design**            | ✅ PASS | New drawer component will be self-contained in `src/lib/features/wishlists/components/` |
| **Deterministic Interfaces & Observability** | ✅ PASS | Uses existing `AddRailwayModelToWishListArgs` type from specta-generated bindings       |
| **Test-First Emphasis**                      | ✅ PASS | Will add Vitest tests for form validation and component behavior                        |
| **Code Quality**                             | ✅ PASS | TypeScript strict mode, ESLint, Prettier enforced                                       |
| **Testing Standards**                        | ✅ PASS | Unit tests for component logic; integration tests for service layer                     |
| **User Experience Consistency**              | ✅ PASS | Matches existing ItemDrawer pattern from Collection; uses Skeleton UI                   |
| **Performance Requirements**                 | ✅ PASS | No heavy operations; dropdown data loaded via existing commands                         |
| **Safe Rust Practices**                      | N/A     | Frontend-only changes                                                                   |
| **Simplicity & Semantic Versioning**         | ✅ PASS | No breaking changes; additive feature                                                   |
| **Database (Persistence)**                   | N/A     | Backend already implemented                                                             |
| **State Management / Persistence Strategy**  | N/A     | Backend already implemented                                                             |
| **API Design & Transport Boundary**          | ✅ PASS | Uses existing `AddRailwayModelToWishListArgs` with specta bindings                      |
| **Domain Logic Location**                    | ✅ PASS | All business logic in Rust backend; frontend is presentation only                       |

**Gate Result**: ✅ ALL GATES PASSED — Proceed to Phase 0

## Project Structure

### Documentation (this feature)

```text
specs/003-add-model-wishlist/
├── plan.md              # This file
├── research.md          # Phase 0 output - frontend patterns research
├── data-model.md        # Phase 1 output - form state and type mappings
├── quickstart.md        # Phase 1 output - implementation guide
├── contracts/           # Phase 1 output - component interface specs
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
# Frontend (Svelte/TypeScript)
src/
├── lib/
│   ├── bindings.ts                      # Auto-generated Tauri command types (READ-ONLY)
│   ├── paraglide/messages.js            # Generated i18n messages (READ-ONLY)
│   └── features/
│       └── wishlists/
│           ├── WishlistState.svelte.ts  # MODIFY: Add addRailwayModelToWishlist method
│           ├── WishlistsDashboard.svelte # MODIFY: Add button, integrate drawer
│           └── components/
│               ├── WishlistHeader.svelte # MODIFY: Add "Add railway model" button
│               ├── AddRailwayModelDrawer.svelte # NEW: Side drawer component
│               └── RollingStockEntry.svelte     # NEW: Rolling stock sub-form
├── routes/
│   └── my-wishlists/
│       └── +page.svelte                 # May need minor styling updates
└── __tests__/
    └── lib/
        └── features/
            └── wishlists/
                └── AddRailwayModelDrawer.test.ts # NEW: Component tests

# Messages (i18n)
messages/
├── en.json                              # MODIFY: Add new UI strings
└── it.json                              # MODIFY: Add new UI strings
```

**Structure Decision**: Follow existing feature-grouped pattern. New drawer component mirrors the `ItemDrawer.svelte` pattern from `collection/components/`. State management extends `WishlistState.svelte.ts` class.

## Complexity Tracking

> No violations — all gates passed.

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| _None_    | —          | —                                    |
