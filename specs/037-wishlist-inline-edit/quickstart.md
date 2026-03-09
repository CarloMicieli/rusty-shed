# Quickstart: Wishlist Item Sidebar Inline Editing

**Feature**: 037-wishlist-inline-edit  
**Branch**: `037-wishlist-inline-edit`

---

## Prerequisites

- Rust toolchain `1.93+` (`rustup show`)
- `pnpm` (10.x)
- Running dev environment: `pnpm tauri dev`

---

## Step 1 — Add missing UI components

The calendar and popover shadcn-svelte components are needed for the date-picker. Add them before any other changes:

```bash
pnpm dlx shadcn-svelte@latest add calendar
pnpm dlx shadcn-svelte@latest add popover
```

Confirm new directories appear under `src/lib/components/ui/calendar/` and `src/lib/components/ui/popover/`.

> **Hard constraint**: Do not install any other npm/pnpm packages without explicit user approval.

---

## Step 2 — Backend: domain event + aggregate method

**File**: `src-tauri/src/wishlist/domain/wishlist_event.rs`

Add a new variant to `WishlistEvent`:

```rust
ItemUpdated {
    item_id: WishlistItemId,
    priority: Option<WishlistPriority>,
    status: Option<WishlistStatus>,
    desired_price: Option<Option<MonetaryAmount>>,
    added_date: Option<NaiveDate>,
},
```

**File**: `src-tauri/src/wishlist/domain/wishlist.rs`

Add `update_item(...)` method to the `Wishlist` aggregate:

```rust
pub fn update_item(
    &mut self,
    item_id: &WishlistItemId,
    priority: Option<WishlistPriority>,
    status: Option<WishlistStatus>,
    desired_price: Option<Option<MonetaryAmount>>,
    added_date: Option<NaiveDate>,
) -> Result<(), DomainError>
```

The method validates at least one field is set, finds the item, emits `WishlistEvent::ItemUpdated`, and applies the event.

---

## Step 3 — Backend: use case

**New file**: `src-tauri/src/wishlist/application/update_wishlist_item.rs`

```rust
pub struct UpdateWishlistItemUseCase;

impl UpdateWishlistItemUseCase {
    pub async fn execute(
        uow: &mut UnitOfWork,
        input: UpdateWishlistItemInput,
    ) -> Result<WishlistItem, DomainError>
}
```

Load the wishlist by ID, call `wishlist.update_item(...)`, save (draining pending events), return the updated item.

Register in: `src-tauri/src/wishlist/application/mod.rs`.

---

## Step 4 — Backend: transport Args + command handler

**File**: `src-tauri/src/wishlist/interface/command_args.rs`

Add `UpdateWishlistItemArgs` struct (see [contracts/update_wishlist_item.md](../contracts/update_wishlist_item.md)).

**File**: `src-tauri/src/wishlist/interface/command_handlers.rs`

Add `update_wishlist_item(...)` handler. Add `inputs.rs` entry for `UpdateWishlistItemInput`.

**File**: `src-tauri/src/lib.rs` (or wherever commands are registered)

Register `update_wishlist_item` in the `tauri::Builder::invoke_handler(...)` call and in the `specta_typescript` type export.

---

## Step 5 — Sync TypeScript bindings

```bash
pnpm tauri dev   # triggers specta rebuild; ^C after bindings regenerate
```

Confirm `src/lib/bindings.ts` now contains `updateWishlistItem` and `UpdateWishlistItemArgs`.

---

## Step 6 — Frontend: `WishlistItemSidebar.svelte`

Key changes to `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`:

1. **Add props**: `defaultCurrency: string`, `onUpdate: (updated: WishlistItem) => void`
2. **Add reactive state**: `let activeField = $state<'priority' | 'status' | 'price' | 'date' | null>(null)`
3. **Add optimistic save helper**: calls `commands.updateWishlistItem(...)`, calls `onUpdate(result)` on success, reverts + shows `toaster.error(...)` on failure
4. **Priority row**: wrap `<dd>` in hover group; show `InlineSelect` or native `<select>` (shadcn `Select`) when `activeField === 'priority'`
5. **Status row**: same pattern as priority
6. **Desired Price row**: show numeric `Input` with currency label when `activeField === 'price'`; validate on Enter/blur; allow clear
7. **Added date row**: show `Popover + Calendar` when `activeField === 'date'`; `maxDate = today`; auto-close on date selection

---

## Step 7 — i18n: add missing message keys

Add to `messages/en.json` and `messages/it.json` any new string keys required for:

- Inline error messages (e.g., `wishlist_inline_price_invalid`, `wishlist_inline_date_future`)
- Accessibility labels (e.g., `wishlist_inline_edit_priority_label`)

Run `pnpm run sync-i18n` (or `pnpm prepare`) to regenerate paraglide output.

---

## Step 8 — Tests

**Rust unit tests** (`src-tauri/src/wishlist/domain/wishlist.rs` test module):

- `update_item` emits correct event
- `update_item` with all-null input returns error
- `update_item` with future `added_date` returns error

**Rust integration tests** (`src-tauri/src/wishlist/application/update_wishlist_item.rs` or existing integration fixture):

- Repository correctly processes `ItemUpdated` event and only updates affected columns
- Persisted data is retrievable in the updated state

**Frontend Vitest** (`src/__tests__/features/wishlists/WishlistItemSidebar.test.ts`):

- Clicking Priority field → dropdown visible
- Selecting a priority → `onUpdate` callback called; field returns to read-only
- Escape → no `onUpdate` call, `activeField` reset to null
- Invalid price → inline error shown, no IPC call
- Clear price → IPC called with `desiredPriceAmount: null`
- Calendar constrained to past dates

---

## Step 9 — Verify

```bash
# Format & lint
pnpm format
pnpm lint
pnpm check

# Frontend tests
pnpm test

# Rust
pnpm run rust:clippy
pnpm run rust:test
```

All checks MUST pass with zero errors before committing.

---

## Commit Convention

```
feat(wishlist): add inline editing for priority, status, price, and date in item sidebar
```
