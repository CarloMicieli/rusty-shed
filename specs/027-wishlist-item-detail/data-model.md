# Data Model: Wishlist Item Detail View

**Feature**: 027-wishlist-item-detail
**Date**: 2026-02-23

---

## Overview

This feature introduces no new database tables or Rust domain types. The data model is entirely a **frontend view composition** derived from existing backend types returned by `getWishlistById`.

---

## Source Types (from `src/lib/bindings.ts`)

### WishlistView

Returned by `commands.getWishlistById(wishlistId)`. Contains everything needed for the detail page.

```typescript
type WishlistView = {
  // From WishlistPreview:
  id: WishlistId; // Parent wishlist ID (used in URL)
  name: string; // Display name for sidebar "List" field
  notes: string | null;
  isDefault: boolean;
  count: bigint;
  updatedAt: string; // ISO datetime
  totalValue: Partial<{ [key in Currency]: bigint }>;

  // Extended:
  items: WishlistItem[]; // Array of items in this wishlist
};
```

### WishlistItem

The core entity for this feature. All sidebar fields come from here.

```typescript
type WishlistItem = {
  id: WishlistItemId; // Used in URL: items/[itemId]
  railwayModelId: RailwayModelId; // Used to fetch RailwayModelView + image
  priority: WishlistPriority; // "LOW" | "NORMAL" | "HIGH"
  status: WishlistStatus; // "WANTED" | "ON_ORDER" | "PURCHASED" | "IGNORED"
  addedDate: string; // "YYYY-MM-DD" — displayed in Personal Context
  removedDate: string | null;
  notes: string | null; // Displayed in Personal Context
  desiredPrice: MonetaryAmount | null; // Sidebar "Desired Price" field
  purchasedPrice: MonetaryAmount | null; // Sidebar "Purchased Price" (if PURCHASED)
};
```

### MonetaryAmount

```typescript
type MonetaryAmount = {
  amount: bigint; // Price in smallest currency unit (cents/pence/etc.)
  currency: string; // ISO 4217 currency code (e.g., "EUR", "GBP")
};
```

### RailwayModelView

Fetched via `commands.getRailwayModelById(railwayModelId)`. Passed to `toRailwayModel()` mapper for the model card.

### RailwayModelImageResponse

Fetched via `commands.getRailwayModelImage(railwayModelId)`. Contains `imagePath: string | null`.

---

## Composed View (Frontend-only)

The page component assembles a `WishlistItemDetailState` from multiple sources:

```typescript
// Page-local state (not a shared type — local $state variables)
{
  wishlistView: WishlistView | null; // From getWishlistById
  wishlistItem: WishlistItem | null; // Looked up from wishlistView.items
  model: RailwayModelView | null; // From getRailwayModelById
  imageResponse: RailwayModelImageResponse | null; // From getRailwayModelImage
  loading: boolean;
  error: string | null;
  notFound: boolean;
}
```

---

## WishlistItemSidebar Props

```typescript
interface WishlistItemSidebarProps {
  item: WishlistItem; // The wishlist item (priority, status, prices, dates, notes)
  wishlistName: string; // Parent wishlist's display name
}
```

---

## WishlistItemCard Updated Props

New `wishlistId` prop added to enable navigation:

```typescript
interface WishlistItemCardProps {
  item: WishlistItem;
  wishlistId: string; // NEW: parent wishlist ID for navigation URL
  onRemove?: (id: string) => void;
  onMove?: (id: string) => void;
  onPurchase?: (id: string) => void;
}
```

---

## Enum Display Mapping

### WishlistPriority → Label

| Enum value | Display label | i18n key                   |
| ---------- | ------------- | -------------------------- |
| `LOW`      | "Low"         | `wishlist_priority_low`    |
| `NORMAL`   | "Normal"      | `wishlist_priority_normal` |
| `HIGH`     | "High"        | `wishlist_priority_high`   |

### WishlistStatus → Label

| Enum value  | Display label | i18n key                         |
| ----------- | ------------- | -------------------------------- |
| `WANTED`    | "Wanted"      | `wishlist_item_status_wanted`    |
| `ON_ORDER`  | "On Order"    | `wishlist_item_status_on_order`  |
| `PURCHASED` | "Purchased"   | `wishlist_item_status_purchased` |
| `IGNORED`   | "Ignored"     | `wishlist_item_status_ignored`   |

---

## Data Flow

```
URL: /wishlists/{wishlistId}/items/{itemId}
          │
          ▼
[itemId]/+page.svelte (onMount)
          │
          ├─► getWishlistById(wishlistId)        ──► WishlistView
          │     └─ wishlistView.items.find(...)  ──► WishlistItem
          │
          ├─► getRailwayModelById(railwayModelId) ──► RailwayModelView
          │     (parallel)
          └─► getRailwayModelImage(railwayModelId) ──► RailwayModelImageResponse
                (parallel)
          │
          ▼
    toRailwayModel(modelView, null, imageResponse)  ──► RailwayModel
          │
          ▼
    ┌─────────────────────────────────────┐
    │  RailwayModelCard (left/main)       │
    └─────────────────────────────────────┘
    ┌─────────────────────────────────────┐
    │  WishlistItemSidebar (right)        │
    │  ├─ Section: Wish List Details      │
    │  │   ├─ List: wishlistView.name     │
    │  │   ├─ Priority: item.priority     │
    │  │   ├─ Status: item.status         │
    │  │   ├─ Desired Price: desiredPrice │
    │  │   └─ Purchased Price (if set)    │
    │  └─ Section: Personal Context      │
    │      ├─ Added: item.addedDate       │
    │      └─ Notes: item.notes           │
    └─────────────────────────────────────┘
```
