# Data Model: Collection Item Detail View

**Branch**: `026-collection-item-detail` | **Date**: 2026-02-22

## Overview

This feature is **read-only** on the frontend. No new database tables, migrations, or Rust domain
aggregates are introduced. All data is already persisted and exposed via existing Tauri commands.

The data model here describes the **frontend view layer** — the types consumed by the new page
and sidebar component, and the global store interface.

---

## Frontend State: `CollectionStore` (module singleton)

**File**: `src/lib/state/collection.svelte.ts`

```
CollectionStore
├── items: CollectionItemView[]        # $state — full collection, populated by fetch()
├── collection: CollectionView | null  # $state — raw response (summary, totalValue, items)
├── loading: boolean                   # $state — true while fetching
│
├── fetch(): Promise<void>             # Calls getCollection() only if items is empty
├── refresh(): Promise<void>           # Always calls getCollection() (bypasses cache)
└── getItemById(id: string): CollectionItemView | undefined
```

**Transitions**:

- `Initial` → `loading: true` → `loaded (items populated)` → `loading: false`
- `loaded` + `fetch()` called → no-op (cache hit)
- `loaded` + `refresh()` called → re-fetches regardless of cache

---

## Domain Entities (existing, read from Rust layer)

### CollectionItemView

The primary entity for the detail page. All sidebar sections are derived from this type.

| Field               | Type                        | Sidebar Section      | Notes                                  |
| ------------------- | --------------------------- | -------------------- | -------------------------------------- |
| `id`                | `CollectionItemId`          | —                    | Used as route parameter                |
| `railwayModel`      | `CollectionRailwayModel`    | —                    | Provides `railwayModelId` for card     |
| `addedDate`         | `string` (ISO YYYY-MM-DD)   | Personal Context     | Formatted as "Feb 22, 2026"            |
| `removedDate`       | `string \| null`            | —                    | Guards active-item check               |
| `purchaseCondition` | `PurchaseCondition \| null` | Condition & Grading  | Badge: New / Second-hand               |
| `modelCondition`    | `ModelCondition \| null`    | Condition & Grading  | Label: New / Like New / Weathered      |
| `boxCondition`      | `BoxCondition \| null`      | Condition & Grading  | Label: Mint / Good / Poor / etc.       |
| `notes`             | `string \| null`            | Personal Context     | Truncated preview (line-clamp-3)       |
| `rollingStocks`     | `OwnedRollingStockView[]`   | Operational Snapshot | May be empty; each entry has digital   |
| `purchaseInfo`      | `PurchaseInfo \| null`      | Acquisition Summary  | Tagged union; `purchased` variant used |

### PurchaseInfo (tagged union)

```
PurchaseInfo
├── { kind: 'purchased'; data: PurchasedInfo }
├── { kind: 'sold';      data: SoldInfo }
└── { kind: 'preOrdered'; data: PreOrderInfo }
```

Only the `purchased` variant is displayed in the Acquisition Summary for now.

**`PurchasedInfo` fields used**:
| Field | Type | Display |
|----------------|-----------------------|----------------------------------|
| `purchaseDate` | `string` (YYYY-MM-DD) | "Feb 22, 2026" |
| `price` | `MonetaryAmount \| null` | Symbol + formatted amount |
| `seller` | `SellerId \| null` | Resolved to `SellerView` on load |

### SellerView (fetched separately)

Fetched via `getSellerById(id)` using the `SellerId` from `PurchasedInfo.seller`.

| Field        | Type             | Display                             |
| ------------ | ---------------- | ----------------------------------- |
| `name`       | `string`         | Plain text or anchor text for URL   |
| `websiteUrl` | `string \| null` | If present, name becomes `<a>` link |

### OwnedRollingStockView (per rolling stock)

| Field     | Type                   | Display                       |
| --------- | ---------------------- | ----------------------------- |
| `digital` | `DigitalSetup \| null` | Null → "not configured" state |

**`DigitalSetup` fields used**:
| Field | Type | Display |
|-----------------------|-------------|------------------------------------------------|
| `dcc_address` | `number` | Prominent number badge (e.g., "87") |
| `installed_decoder_id`| `DecoderId` | Parse URN: `trn:decoder:{mfr}:{code}` → display|

### RailwayModelView (for model card)

Fetched via `getRailwayModelById(railwayModelId)`. Passed into `<RailwayModelCard>` via the
existing `toRailwayModel()` mapper. No changes to this entity or the card component.

### RailwayModelImageResponse (for model card image)

Fetched via `getRailwayModelImage(railwayModelId)`. Passed into `<RailwayModelCard>` unchanged.

---

## Frontend Component Hierarchy

```
/collection/[itemId]/+page.svelte          # Route page
├── CollectionItemDetailLayout             # Two-panel flex container
│   ├── <RailwayModelCard>                 # Left panel (existing component, unchanged)
│   └── <CollectionItemSidebar>            # Right panel (NEW component)
│       ├── AcquisitionSection             # Seller, date, price
│       ├── ConditionSection               # modelCondition, boxCondition, purchaseCondition
│       ├── OperationalSection             # DCC address + decoder per rolling stock
│       └── PersonalContextSection         # addedDate, notes preview
```

---

## State Flow on Page Load

```
1. User navigates to /collection/{itemId}

2. Page checks collectionStore.items:
   a. If populated → find item by id immediately (no IPC call)
   b. If empty (direct refresh) → call collectionStore.fetch() → wait

3. With collectionItem in hand:
   a. Extract railwayModelId from item.railwayModel.railwayModelId
   b. Parallel IPC calls:
      - getRailwayModelById(railwayModelId)      → for model card
      - getRailwayModelImage(railwayModelId)     → for model image
      - getSellerById(sellerId) if seller != null → for acquisition sidebar

4. Render:
   - Left panel: RailwayModelCard with model + image
   - Right panel: CollectionItemSidebar with item data + resolved seller
```

---

## Navigation Config Change

**File**: `src/lib/components/navigation/types.ts`

Add `additionalPrefixes?: string[]` to `NavigationItem`.

**File**: `src/lib/components/navigation/config.ts`

Update the `'collection'` entry:

```
{
  id: 'collection',
  href: '/my-collection',
  additionalPrefixes: ['/collection'],   // ← NEW
  ...
}
```

**File**: `src/lib/components/navigation/utils.ts`

Update `isActive()` to check `item.additionalPrefixes`:

```
if (item.additionalPrefixes?.some(p => pathname.startsWith(p))) return true;
```
