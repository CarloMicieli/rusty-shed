# Implementation Plan: Mark Wishlist Item as Purchased

**Branch**: `028-mark-purchased` | **Date**: 2026-02-24 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/028-mark-purchased/spec.md`

---

## Summary

Adds a "Purchase" action to wishlist item preview cards and the detail page. Clicking it opens a dialog collecting price (required), purchase date (defaults to today), seller (optional, from existing sellers list), and condition (optional dropdown). On confirmation, an Application Service coordinator orchestrates an atomic transition: the Wishlist Aggregate validates and marks the item as Purchased, and the Collection Aggregate creates the new collection entry. The operation uses the existing `purchase_wishlist_item` Tauri command, extended to carry condition data.

**Architectural approach (user-specified):** Coordinator Pattern — `PurchaseWishlistItemService` manages the transaction, delegating validation responsibilities to each Aggregate.

---

## Technical Context

**Language/Version**: Rust edition 2024, rust-version 1.93.0 (backend); TypeScript 5.9.3 (frontend)
**Primary Dependencies**: Tauri 2.9.x, sqlx (SQLite), specta/tauri-specta, Svelte 5 + SvelteKit, Tailwind CSS 4, shadcn-svelte, sveltekit-superforms, Zod, Paraglide-JS
**Storage**: SQLite via sqlx — no new migrations required (all columns already exist)
**Testing**: `cargo test` (Rust), Vitest + @testing-library/svelte (frontend)
**Target Platform**: Desktop (Linux, macOS, Windows) via Tauri 2
**Performance Goals**: IPC command round-trip < 200ms (Constitution SLO for UI-critical read/write ops); dialog open < 100ms
**Constraints**: All user-facing strings via Paraglide (hardcoded text forbidden); no `unwrap()`/`expect()` in production Rust; no direct DB access from frontend
**Scale/Scope**: Single-user desktop app; wishlist item count O(100s); sellers list O(10–100)

---

## Constitution Check

_GATE: Must pass before proceeding._

| Law / Principle                                                                                   | Status         | Notes                                                                                                 |
| ------------------------------------------------------------------------------------------------- | -------------- | ----------------------------------------------------------------------------------------------------- |
| **Database (Persistence)**: SQLite via sqlx, migrations for new storage                           | ✅ Pass        | No new tables/columns required — all schema already exists                                            |
| **Domain Event Tracking**: Aggregates emit events; repos drain + persist atomically               | ⚠️ Gap → Fixed | `WishlistEvent::ItemPurchased` is missing. Plan adds this event and infrastructure handler.           |
| **API / Transport Boundary**: Tauri IPC + specta types, `Args` validates at boundary              | ✅ Pass        | Existing `PurchaseWishlistArgs` extended with `condition: Option<String>`, `TryFrom` validation added |
| **Domain Logic Location**: Business rules in Rust, frontend is render-only                        | ✅ Pass        | Wishlist aggregate validates item status; Collection aggregate validates collection rules             |
| **Code Quality**: clippy `-D warnings`, fmt, no unwrap in prod                                    | ✅ Pass        | All new code follows existing patterns                                                                |
| **Testing Standards**: unit tests for business logic, integration tests for cross-layer contracts | ✅ Pass        | New tests for `purchase_item()` aggregate method and service; existing tests updated                  |
| **User Experience Consistency**: Paraglide strings, shared design tokens                          | ✅ Pass        | Dialog uses Paraglide for all labels; shadcn-svelte components for UI                                 |
| **Performance Requirements**: UI-critical IPC < 200ms                                             | ✅ Pass        | Purchase is a single-row update + single-row insert; no expensive queries                             |
| **Safe Rust Practices**: `Result<T,E>`, no panics, clippy/fmt                                     | ✅ Pass        | New domain method returns `Result<(), DomainError>`                                                   |

---

## Project Structure

### Documentation (this feature)

```text
specs/028-mark-purchased/
├── plan.md              ← this file
├── spec.md
├── research.md          ← Phase 0 output
├── data-model.md        ← Phase 1 output
├── quickstart.md        ← Phase 1 output
├── contracts/
│   └── purchase-wishlist-item.md
└── checklists/
    └── requirements.md
```

### Source Code (affected files)

```text
src-tauri/src/wishlist/
├── domain/
│   ├── wishlist_event.rs        ← ADD ItemPurchased variant
│   └── wishlist.rs              ← ADD purchase_item() method
├── infrastructure/
│   └── [repository impl]        ← ADD ItemPurchased event handler (SQL UPDATE)
├── application/
│   └── purchase_wishlist_item.rs ← REFACTOR to use purchase_item() + pass conditions
└── interface/
    ├── command_handlers.rs       ← UPDATE purchase_wishlist_item handler
    └── [command_args file]       ← EXTEND PurchaseWishlistArgs + TryFrom

src/
├── lib/
│   ├── bindings.ts                        ← AUTO-GENERATED (regenerated by build)
│   └── features/wishlist/
│       ├── components/
│       │   ├── PurchaseDialog.svelte      ← NEW
│       │   ├── WishlistItemCard.svelte    ← ADD Purchase button
│       │   └── [detail page component]   ← ADD Purchase button
│       └── WishlistController.svelte.ts  ← ADD purchase dialog state + handler

messages/
└── en.json                                ← ADD new i18n keys
```

---

## Phase 0 Research Output

→ See [research.md](research.md) — all unknowns resolved, no blockers.

**Key findings**:

- `PurchaseWishlistItemService` already exists but has two gaps: missing `ItemPurchased` domain event, and condition fields not forwarded
- `AddCollectionItemInput` already supports condition — no collection-side changes needed
- `get_sellers` and `get_settings` Tauri commands already exist and will be reused
- No new DB migrations required
- Existing `purchase_wishlist_item` command is extended (not replaced)

---

## Phase 1 Design

### Data Model

→ See [data-model.md](data-model.md) for full type definitions.

**Summary of changes**:

**Backend (Rust)**:

- `WishlistEvent::ItemPurchased { item_id, purchased_price }` — new event variant
- `Wishlist::purchase_item(item_id, price) -> Result<(), DomainError>` — validates status, emits event
- `PurchaseWishlistItemCommand { ..., purchase_condition, model_condition }` — extended command type
- `PurchaseWishlistArgs { ..., condition: Option<String> }` — extended transport args
- Condition decomposition: `"PreOwnedVeryGood"` → `(PreOwned, VeryGood)`

**Frontend (TypeScript/Svelte)**:

- `PurchaseFormSchema` (Zod) — client-side validation (price required, date not future)
- `PurchaseDialogState` — reactive state in wishlist feature layer
- `PURCHASE_CONDITION_OPTIONS` — constant for dropdown population

### API Contracts

→ See [contracts/purchase-wishlist-item.md](contracts/purchase-wishlist-item.md)

**Command**: `purchase_wishlist_item(PurchaseWishlistArgs)` — extended (existing)
**Reads**: `get_sellers()` — unchanged (existing)
**Reads**: `get_settings()` — unchanged (existing)

### Frontend Component Architecture

```
WishlistItemCard.svelte
  └── "Purchase" button (shown when status == Wanted || OnOrder)
      └── dispatches event / calls controller → opens PurchaseDialog

WishlistItemDetailPage.svelte (or equivalent)
  └── "Purchase" button (same condition)
      └── same dispatch

PurchaseDialog.svelte
  ├── Loads sellers via get_sellers() on mount
  ├── Loads currency via get_settings() on mount
  ├── Form fields: price (number input), date (date picker), seller (select), condition (select)
  ├── Client validation: superforms + Zod (PurchaseFormSchema)
  ├── On submit: invoke('purchase_wishlist_item', args)
  ├── On success: close dialog, emit invalidation signal to wishlist state
  └── On error: keep dialog open, show inline error message

WishlistController.svelte.ts
  └── purchaseDialog: { open, wishlistId, wishlistItemId, ... }
  └── openPurchaseDialog(item) / closePurchaseDialog()
  └── handlePurchaseSuccess() → refresh wishlist state
```

### Paraglide i18n Keys Required

Add to `messages/en.json`:

```json
"purchaseDialog.title": "Record Purchase",
"purchaseDialog.price.label": "Price Paid",
"purchaseDialog.price.placeholder": "0.00",
"purchaseDialog.date.label": "Purchase Date",
"purchaseDialog.seller.label": "Seller",
"purchaseDialog.seller.placeholder": "Select a seller…",
"purchaseDialog.condition.label": "Condition",
"purchaseDialog.condition.placeholder": "Select condition…",
"purchaseDialog.condition.new": "New",
"purchaseDialog.condition.preOwnedLikeNew": "Pre-Owned – Like New",
"purchaseDialog.condition.preOwnedVeryGood": "Pre-Owned – Very Good",
"purchaseDialog.condition.preOwnedGood": "Pre-Owned – Good",
"purchaseDialog.condition.preOwnedAcceptable": "Pre-Owned – Acceptable",
"purchaseDialog.submit": "Record Purchase",
"purchaseDialog.cancel": "Cancel",
"purchaseDialog.error.priceRequired": "Price is required",
"purchaseDialog.error.futureDateForbidden": "Purchase date cannot be in the future",
"purchaseDialog.error.saveFailed": "Failed to save purchase. Please try again.",
"purchaseDialog.success": "Purchase recorded",
"wishlistCard.purchaseButton": "Purchase"
```

---

## Complexity Tracking

No constitution violations. The `ItemPurchased` event addition resolves an existing gap rather than introducing new complexity.
