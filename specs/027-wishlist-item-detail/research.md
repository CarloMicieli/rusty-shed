# Research: Wishlist Item Detail View

**Feature**: 027-wishlist-item-detail
**Date**: 2026-02-23

---

## Decision 1: Route Structure

**Decision**: Use `/wishlists/[wishlistId]/items/[itemId]` as the SvelteKit dynamic route.

**Rationale**: The URL nests item under its parent wishlist, matching the spec's requirement for `/wishlists/{wishlistId}/items/{id}`. It also provides the `wishlistId` in the URL context without needing an extra lookup, enabling the back-navigation target to be constructed as `/my-wishlists` without any ID resolution.

**Alternatives considered**:

- `/wishlists/[itemId]` — Simpler but loses parent context and conflicts with potential `/wishlists/[wishlistId]` wishlist detail route in the future.
- `/my-wishlists/items/[itemId]` — Flat structure, harder to extend; breaks the resource hierarchy.

---

## Decision 2: Data Loading Strategy

**Decision**: Call `getWishlistById(wishlistId)` on mount, then look up the specific item by `itemId` client-side from the returned `WishlistView.items` array. Fetch `getRailwayModelById` and `getRailwayModelImage` in parallel.

**Rationale**: The existing `getWishlistById` command returns the full `WishlistView` with all items, and crucially also contains the `Wishlist.name` needed for the sidebar. There is no single-item lookup command (`getWishlistItemById`), so a client-side array lookup is the correct approach. This mirrors the collection item detail pattern: `collectionStore.getItemById()` also performs a client-side lookup after a cache-first fetch.

**Alternatives considered**:

- Adding a new `get_wishlist_item_by_id` Rust command — Unnecessary complexity; the existing command is sufficient and the array lookup is trivial.
- Fetching all wishlists and finding the right one — Wasteful; we have the wishlistId in the URL.

---

## Decision 3: Navigation Active State

**Decision**: Add `additionalPrefixes: ['/wishlists']` to the wishlists navigation config entry in `src/lib/components/navigation/config.ts`.

**Rationale**: The `isActive()` utility in `navigation/utils.ts` already supports `additionalPrefixes` for exactly this use case. The collection item detail page uses `additionalPrefixes: ['/collection']` to keep "Collection" highlighted when on `/collection/[itemId]`. Applying the same pattern to wishlists requires a one-line config change.

**Alternatives considered**:

- Using `usePrefixMatch: true` on the `/my-wishlists` href — Would break if the prefixes don't align (`/my-wishlists` vs `/wishlists/...`). `additionalPrefixes` is the correct mechanism.

---

## Decision 4: Model Card Reuse

**Decision**: Reuse the existing `RailwayModelCard` component and `toRailwayModel()` mapper from `src/lib/features/collection/utils/modelViewMapper.ts` with `collectionItem: null`, which sets `status: 'Wishlist'`.

**Rationale**: The mapper already handles the wishlist case explicitly (the `status = collectionItem ? 'InCollection' : 'Wishlist'` branch). No new mapper function is needed.

**Alternatives considered**:

- Creating a separate wishlist-specific mapper — Would duplicate the rolling stock transformation logic for no benefit.

---

## Decision 5: WishlistItemSidebar — No New Backend Commands

**Decision**: The sidebar displays data from `WishlistItem` (priority, status, desired_price, purchased_price, added_date, notes) and `Wishlist.name`. All of this is available from the single `getWishlistById` call result. No new Tauri commands are needed.

**Rationale**: `WishlistView` extends `WishlistPreview` and includes the `items: WishlistItem[]` array. Both the wishlist name and the item fields come from this single response.

---

## Decision 6: Navigation Fix in WishlistItemCard

**Decision**: Add a click handler (wrapping the card in an anchor or calling `goto()`) in `WishlistItemCard.svelte` that navigates to `/wishlists/${wishlistId}/items/${item.id}`. The `wishlistId` is passed as a new prop.

**Rationale**: The card currently has no navigation — only action buttons (`onRemove`, `onMove`, `onPurchase`). Adding navigation requires the parent wishlist ID, which `WishlistItemCard` does not currently receive. `WishlistItems.svelte` (the parent grid component) already knows the active wishlist ID and must pass it down.

**Alternatives considered**:

- Wrapping the whole card in `<a href="...">` — Cleaner HTML semantics but could interfere with nested action buttons; using an outer click handler with `stopPropagation` on buttons is more compatible with the existing card structure.

---

## Decision 7: i18n Keys

**Decision**: Add new `wishlist_item_*` message keys to `messages/en.json` for all user-facing strings in the new detail page and sidebar.

**Existing keys that can be reused**:

- `wishlist_priority_low/normal/high` — Already present; use for priority labels in sidebar.
- `collection_item_not_recorded` — Can reuse for empty field states.
- `collection_item_added_date`, `collection_item_notes` — Can reuse for shared field labels.

**New keys needed**:

- `wishlist_item_back` — Back button label ("Back to Wishlists")
- `wishlist_item_not_found` — Not found heading
- `wishlist_item_not_found_message` — Not found body
- `wishlist_item_loading` — Loading label
- `wishlist_item_error` — Error heading
- `wishlist_item_section_details` — Sidebar section header ("Wish List Details")
- `wishlist_item_wishlist_name` — Field label ("List")
- `wishlist_item_priority` — Field label ("Priority") ← `wishlist_field_priority` already exists; reuse it
- `wishlist_item_desired_price` — Field label ("Desired Price") ← `wishlist_field_desired_price` already exists; reuse it
- `wishlist_item_purchased_price` — Field label ("Purchased Price")
- `wishlist_item_status` — Field label ("Status")
- `wishlist_item_status_wanted` — "Wanted"
- `wishlist_item_status_on_order` — "On Order"
- `wishlist_item_status_purchased` — "Purchased"
- `wishlist_item_status_ignored` — "Ignored"
- `wishlist_item_section_personal_context` — "Personal Context"
- `wishlist_item_added_date` — "Added" (or reuse `collection_item_added_date`)
- `wishlist_item_notes` — "Notes" (or reuse `collection_item_notes`)
- `wishlist_item_price_not_set` — "Not set"

---

## Summary: No Backend Changes Required

This feature is entirely frontend-only:

- No new Tauri commands
- No new database migrations
- No new Rust types or specta bindings
- All necessary data is available via existing `getWishlistById` command

Frontend changes:

1. New route file: `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`
2. New component: `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`
3. Modified: `src/lib/components/navigation/config.ts` (add `additionalPrefixes`)
4. Modified: `src/lib/features/wishlists/components/WishlistItemCard.svelte` (add `wishlistId` prop + navigation)
5. Modified: `src/lib/features/wishlists/components/WishlistItems.svelte` (pass `wishlistId` to cards)
6. Modified: `messages/en.json` (new `wishlist_item_*` keys)
