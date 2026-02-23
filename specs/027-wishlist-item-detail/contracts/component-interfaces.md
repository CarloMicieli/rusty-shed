# Component Contracts: Wishlist Item Detail View

**Feature**: 027-wishlist-item-detail
**Date**: 2026-02-23

---

## New Component: WishlistItemSidebar

**File**: `src/lib/features/wishlists/components/WishlistItemSidebar.svelte`

### Props Interface

```typescript
interface Props {
  item: WishlistItem; // From $lib/bindings — wishlist item data
  wishlistName: string; // Parent wishlist's display name
}
```

### Renders

Two sections:

**Section 1 — Wish List Details**

- **List**: `wishlistName` (plain text)
- **Priority**: human-readable label from `item.priority` enum (LOW/NORMAL/HIGH) with badge indicator
- **Status**: human-readable label from `item.status` enum (WANTED/ON_ORDER/PURCHASED/IGNORED) with badge
- **Desired Price**: formatted `item.desiredPrice` with currency, or "Not set" if null
- **Purchased Price**: formatted `item.purchasedPrice` with currency; only shown when `item.purchasedPrice` is non-null (i.e., status is PURCHASED)

**Section 2 — Personal Context**

- **Added**: `item.addedDate` formatted as human-readable date (e.g., "Feb 23, 2026")
- **Notes**: `item.notes` text, truncated if very long; section hidden if null

### Visual Spec

```
┌──────────────────────────────────┐
│ WISH LIST DETAILS                │
│ List          My Wish List       │
│ Priority      [HIGH badge]       │
│ Status        [WANTED badge]     │
│ Desired Price €Price.Amount      │
│ Purchased Price (if set)         │
├──────────────────────────────────┤
│ PERSONAL CONTEXT                 │
│ Added     Feb 23, 2026           │
│ Notes     Lorem ipsum...         │
└──────────────────────────────────┘
```

### Empty States

| Field                   | Empty display                                    |
| ----------------------- | ------------------------------------------------ |
| `desiredPrice = null`   | "Not set"                                        |
| `purchasedPrice = null` | Section row hidden                               |
| `notes = null`          | Personal Context section hidden (or empty state) |

---

## Modified Component: WishlistItemCard

**File**: `src/lib/features/wishlists/components/WishlistItemCard.svelte`

### Updated Props Interface

```typescript
interface Props {
  item: WishlistItem;
  wishlistId: string; // NEW — parent wishlist ID for navigation
  onRemove?: (id: string) => void;
  onMove?: (id: string) => void;
  onPurchase?: (id: string) => void;
}
```

### Behavior Change

The card area (excluding action buttons) becomes clickable. Clicking navigates to:

```
/wishlists/{wishlistId}/items/{item.id}
```

Action buttons (`onRemove`, `onMove`, `onPurchase`) call `stopPropagation()` to prevent triggering the card navigation.

---

## Modified Component: WishlistItems

**File**: `src/lib/features/wishlists/components/WishlistItems.svelte`

### Updated Props Interface

```typescript
interface Props {
  wishlistId: string; // NEW — propagated to WishlistItemCard
  items: WishlistItem[];
  onRemove: (id: string) => void;
  onMove: (id: string) => void;
}
```

`wishlistId` is passed through to each `WishlistItemCard` instance.

---

## New Route: Wishlist Item Detail Page

**File**: `src/routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte`

### Data Dependencies

| Source          | Command                                | Returns                                        |
| --------------- | -------------------------------------- | ---------------------------------------------- |
| Wishlist + item | `getWishlistById(wishlistId)`          | `WishlistView` (contains `items[]` and `name`) |
| Model details   | `getRailwayModelById(railwayModelId)`  | `RailwayModelView`                             |
| Model image     | `getRailwayModelImage(railwayModelId)` | `RailwayModelImageResponse`                    |

### Page States

| State     | Condition           | Display                               |
| --------- | ------------------- | ------------------------------------- |
| Loading   | `loading === true`  | Spinner + "Loading item details..."   |
| Not found | `notFound === true` | Error card + back button              |
| Error     | `error !== null`    | Error card with message + back button |
| Success   | All data loaded     | Two-panel layout                      |

### Layout

```
┌────────────────────────────────────────────────────┐
│ ← Back to Wishlists                                │
├───────────────────────────┬────────────────────────┤
│                           │                        │
│   RailwayModelCard        │  WishlistItemSidebar   │
│   (main content area)     │  (right panel)         │
│                           │                        │
└───────────────────────────┴────────────────────────┘
```

---

## Configuration Change: Navigation Config

**File**: `src/lib/components/navigation/config.ts`

### Change

Add `additionalPrefixes: ['/wishlists']` to the wishlists navigation entry so the "Wishlists" nav item remains highlighted when on `/wishlists/*/items/*` pages.

```typescript
// Before:
{
  id: 'wishlists',
  label: () => m.app_wishlists(),
  icon: Heart,
  href: '/my-wishlists',
  isPrimary: true
}

// After:
{
  id: 'wishlists',
  label: () => m.app_wishlists(),
  icon: Heart,
  href: '/my-wishlists',
  isPrimary: true,
  additionalPrefixes: ['/wishlists']
}
```

---

## i18n Keys Required

**File**: `messages/en.json`

New keys to add:

```json
{
  "wishlist_item_back": "Back to Wishlists",
  "wishlist_item_not_found": "Item not found",
  "wishlist_item_not_found_message": "This wishlist item does not exist or has been removed.",
  "wishlist_item_loading": "Loading item details...",
  "wishlist_item_error": "Failed to load item",
  "wishlist_item_section_details": "Wish List Details",
  "wishlist_item_wishlist_name": "List",
  "wishlist_item_status": "Status",
  "wishlist_item_purchased_price": "Purchased Price",
  "wishlist_item_price_not_set": "Not set",
  "wishlist_item_section_personal_context": "Personal Context",
  "wishlist_item_added_date": "Added",
  "wishlist_item_notes": "Notes",
  "wishlist_item_status_wanted": "Wanted",
  "wishlist_item_status_on_order": "On Order",
  "wishlist_item_status_purchased": "Purchased",
  "wishlist_item_status_ignored": "Ignored"
}
```

**Existing keys to reuse** (no new keys needed):

- `wishlist_field_priority` → sidebar "Priority" label
- `wishlist_field_desired_price` → sidebar "Desired Price" label
- `wishlist_priority_low/normal/high` → priority enum labels
- `collection_item_not_recorded` → generic empty state fallback
