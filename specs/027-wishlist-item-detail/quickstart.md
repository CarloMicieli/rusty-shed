# Quickstart: Wishlist Item Detail View

**Feature**: 027-wishlist-item-detail
**Branch**: `027-wishlist-item-detail`

---

## Prerequisites

- Branch `027-wishlist-item-detail` checked out
- `pnpm install` completed
- `pnpm tauri dev` working

---

## Scope Summary

**Frontend-only feature.** No Rust/backend changes required. All data is available via the existing `getWishlistById` Tauri command.

**Files to create** (2):

- `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`
- `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`

**Files to modify** (4):

- `src/lib/components/navigation/config.ts`
- `src/lib/features/wishlists/components/WishlistItemCard.svelte`
- `src/lib/features/wishlists/components/WishlistItems.svelte`
- `messages/en.json`

---

## Implementation Order

### Step 1 — Add i18n keys

Add the new `wishlist_item_*` keys to `messages/en.json`. See `contracts/component-interfaces.md` for the full list. Run `pnpm format` after to check Paraglide compilation.

### Step 2 — Fix navigation config

In `src/lib/components/navigation/config.ts`, add `additionalPrefixes: ['/wishlists']` to the wishlists nav entry. This ensures "Wishlists" stays highlighted on the new detail page.

### Step 3 — Create WishlistItemSidebar component

Create `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`.

**Reference**: `src/lib/features/collection/components/CollectionItemSidebar.svelte` — follow the same section/definition-list pattern.

Key differences from CollectionItemSidebar:

- Props: `item: WishlistItem`, `wishlistName: string` (no seller)
- Section 1 "Wish List Details": list name, priority badge, status badge, desired price, purchased price (if set)
- Section 2 "Personal Context": added date, notes

Use existing `wishlist_priority_*` message keys for priority labels.

### Step 4 — Update WishlistItems to pass wishlistId

In `src/lib/features/wishlists/components/WishlistItems.svelte`, add `wishlistId: string` to props and pass it down to each `<WishlistItemCard>`.

### Step 5 — Add navigation to WishlistItemCard

In `src/lib/features/wishlists/components/WishlistItemCard.svelte`:

1. Add `wishlistId: string` to props
2. Add an `onclick` handler on the card container that calls `goto('/wishlists/${wishlistId}/items/${item.id}')`
3. Add `onclick:stopPropagation()` or `event.stopPropagation()` on the existing action buttons to prevent triggering card navigation

### Step 6 — Create the detail page route

Create `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`.

**Reference**: `src/routes/collection/[itemId]/+page.svelte` — mirror the structure exactly.

Key implementation points:

```typescript
// Route params
const wishlistId = $page.params.wishlistId as string;
const itemId = $page.params.itemId as string;

// onMount data loading
const wishlistView = await commands.getWishlistById(wishlistId);
const wishlistItem = wishlistView?.items.find((i) => i.id === itemId) ?? null;

// Parallel fetches
const [model, imageResponse] = await Promise.all([
  commands.getRailwayModelById(wishlistItem.railwayModelId),
  commands.getRailwayModelImage(wishlistItem.railwayModelId)
]);

// Mapper (status: 'Wishlist' when collectionItem is null)
const railwayModel = toRailwayModel(model, null, imageResponse);
```

Layout: two-panel — `RailwayModelCard` on left, `WishlistItemSidebar` on right.

Back button: `goto('/my-wishlists')`.

---

## Verification Checklist

After implementation:

- [ ] `pnpm lint` — passes
- [ ] `pnpm check` — passes (TypeScript + svelte-check)
- [ ] Navigate to wishlists page → click item card → lands on `/wishlists/{id}/items/{id}`
- [ ] "Wishlists" nav item highlighted on detail page
- [ ] Back button returns to `/my-wishlists`
- [ ] Model card displays model name, manufacturer, scale
- [ ] Sidebar shows wishlist name, priority badge, desired price
- [ ] Item with no desired price shows "Not set" without error
- [ ] Item with `purchasedPrice` shows purchased price row
- [ ] Invalid item URL shows "Item not found" state

---

## Reference Files

| What                               | Where                                                                 |
| ---------------------------------- | --------------------------------------------------------------------- |
| Collection detail page (reference) | `src/routes/collection/[itemId]/+page.svelte`                         |
| CollectionItemSidebar (reference)  | `src/lib/features/collection/components/CollectionItemSidebar.svelte` |
| Navigation config                  | `src/lib/components/navigation/config.ts`                             |
| toRailwayModel mapper              | `src/lib/features/collection/utils/modelViewMapper.ts`                |
| WishlistItemCard (to modify)       | `src/lib/features/wishlists/components/WishlistItemCard.svelte`       |
| WishlistItems (to modify)          | `src/lib/features/wishlists/components/WishlistItems.svelte`          |
| Auto-generated bindings            | `src/lib/bindings.ts`                                                 |
