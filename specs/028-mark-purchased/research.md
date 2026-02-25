# Research: Mark Wishlist Item as Purchased (028)

**Date**: 2026-02-24
**Branch**: `028-mark-purchased`

---

## 1. Existing Infrastructure Audit

### 1.1 `PurchaseWishlistItemService` (Already Exists)

**File**: `src-tauri/src/wishlist/application/purchase_wishlist_item.rs`

A `PurchaseWishlistItemService` with `move_wishlist_item()` already exists. It:

- Accepts `MoveWishlistItemId` (has: `collection_id`, `wishlist_id`, `wishlist_item_id`, `purchase_price`, `purchase_date`, `seller_id`)
- Calls `AddCollectionItem::execute()` to create the collection entry
- Sets item `status = WishlistStatus::Purchased` and `purchased_price`
- Persists both aggregates in one UoW transaction

**Gap**: `MoveWishlistItemId` does **not** carry condition fields — it passes `None` for `purchase_condition`, `model_condition`, and `box_condition`. The condition data from the new dialog is lost.

**Gap**: The service directly mutates the wishlist item fields without calling an aggregate method that emits a domain event. No `ItemPurchased` event exists in `WishlistEvent`. This violates the Constitution's Domain Event Tracking law.

### 1.2 `purchase_wishlist_item` Tauri Command (Already Exists)

**File**: `src-tauri/src/wishlist/interface/command_handlers.rs`

Command `purchase_wishlist_item(state, input: PurchaseWishlistArgs)` is registered. It calls the service above. No UI currently triggers it for the "purchase with modal" flow — the command exists but was wired for a different internal use case.

### 1.3 `AddCollectionItemInput` (Ready for Condition)

**File**: `src-tauri/src/collecting/application/add_collection_item.rs`

`AddCollectionItemInput` already accepts:

```
purchase_condition: Option<PurchaseCondition>
model_condition: Option<ModelCondition>
box_condition: Option<BoxCondition>
```

`CollectionEvent::RailwayModelAdded` already carries all condition fields. **No collection-side changes required for condition support.** Only the service needs to forward the values.

### 1.4 `WishlistEvent` Enum (Gap)

**File**: `src-tauri/src/wishlist/domain/wishlist_event.rs`

Current variants: `Created`, `Renamed`, `ItemAdded`, `ItemRemoved`, `ItemMoved`, `MarkedDefault`.

**Missing**: `ItemPurchased`. Constitution requires aggregates to emit domain events for every state change persisted. Adding this variant is required.

### 1.5 Sellers Domain

**Command**: `get_sellers()` → `Vec<SellerView>` — already exists. The frontend can call this to populate the seller autocomplete field.

**SellerId format**: `trn:seller:{slug}` — must be passed as-is in the args.

### 1.6 Settings / Default Currency

**Command**: `get_settings()` → `UserSettings` — already exists.
`UserSettings.currency: String` — defaults to `"EUR"`. Frontend reads this on dialog open to display the currency label and include the currency code in the submission payload.

### 1.7 `MonetaryAmount` Encoding

Prices are stored as `i64` (minor currency units, e.g. cents) + `Currency` enum. The frontend must submit `price_amount: i64` (e.g., `1050` for €10.50) and `price_currency: String` (e.g., `"EUR"`).

---

## 2. Design Decisions

### Decision 1 — Add `WishlistEvent::ItemPurchased`

- **What**: New event variant `ItemPurchased { item_id: WishlistItemId, purchased_price: MonetaryAmount }`
- **Why**: Constitution requires all state changes to flow through domain events. The existing code bypasses this for item status updates.
- **How**: Infrastructure layer handles this event with a targeted SQL UPDATE on `wishlist_items`.

### Decision 2 — Add `Wishlist::purchase_item()` Aggregate Method

- **What**: Method that validates the item is in an active status (Wanted/OnOrder), updates state, and emits `ItemPurchased`.
- **Why**: Aligns with user's "Coordinator Approach" — the **Wishlist Aggregate is responsible for validating the transition** and marking the item. Business rule enforcement stays in the domain.
- **How**: `purchase_item(&mut self, item_id: &WishlistItemId, price: MonetaryAmount) -> Result<(), DomainError>`

### Decision 3 — Unified Condition Enum in Interface Layer

- **What**: A `PurchaseConditionOption` string-based selection (e.g., `"New"`, `"PreOwnedLikeNew"`, …) is accepted by the Tauri command args. The `TryFrom` impl on `Args → Input` decomposes it into `(Option<PurchaseCondition>, Option<ModelCondition>)`.
- **Why**: Keeps the domain clean (separate PurchaseCondition + ModelCondition). The combined picker is purely a UX concern.
- **Mapping**:
  | UI Label | condition string | purchase_condition | model_condition |
  |---|---|---|---|
  | New | `New` | `New` | `None` |
  | Pre-Owned – Like New | `PreOwnedLikeNew` | `PreOwned` | `NearMint` |
  | Pre-Owned – Very Good | `PreOwnedVeryGood` | `PreOwned` | `VeryGood` |
  | Pre-Owned – Good | `PreOwnedGood` | `PreOwned` | `Good` |
  | Pre-Owned – Acceptable | `PreOwnedAcceptable` | `PreOwned` | `Fair` |

### Decision 4 — Reuse Existing `purchase_wishlist_item` Command

- **What**: Extend (not replace) the existing `PurchaseWishlistArgs` struct and handler.
- **Why**: Command is already registered in `lib.rs` and specta-exported. Adding fields is backward-compatible during development (all new fields are optional or have defaults).

### Decision 5 — Currency from Frontend via Settings

- **What**: Frontend reads `get_settings().currency` on dialog open and includes `price_currency` in the submission payload.
- **Why**: The backend `MonetaryAmount` requires both amount and currency. The frontend derives the currency from settings (no user selection), so it remains accurate with zero user friction.

### Decision 6 — No New DB Migrations Needed

- **What**: All tables required (`wishlist_items`, `collection_items`, `purchase_info`) already exist with the correct columns.
- **Why**: The infrastructure for condition fields in collection_items was added in prior features. The wishlist_items status and purchased_price columns already exist (proven by the existing service tests).

---

## 3. Frontend Architecture Decision

The purchase dialog is triggered from two locations: the `WishlistItemCard` (preview) and the wishlist item detail page. The pattern used across other features is to host shared dialog state in the feature's controller.

- **Decision**: Add a `purchaseDialog` state block to the existing `WishlistController.svelte.ts` (or create a new `PurchaseDialogController.svelte.ts` for isolation).
- **Dialog component**: New `PurchaseDialog.svelte` component under `src/lib/features/wishlist/components/`.
- **Validation**: superforms + Zod on the frontend, `garde` validation on the Rust Args struct.
