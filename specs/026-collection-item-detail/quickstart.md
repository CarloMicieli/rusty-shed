# Quickstart: Collection Item Detail View

**Branch**: `026-collection-item-detail` | **Date**: 2026-02-22

## Prerequisites

- Rust toolchain (1.93.0+), `cargo`, `cargo fmt`, `cargo clippy`
- Node.js + `pnpm` (pnpm@10.27.0)
- The app builds and runs (`pnpm tauri dev`)
- A populated SQLite database (at least one collection item with purchase info)

---

## Development Setup

```bash
# Ensure you are on the feature branch
git checkout 026-collection-item-detail

# Install frontend dependencies (if not already done)
pnpm install

# Start development server
pnpm tauri dev
```

---

## Delivery Order (implement in this sequence)

### Step 1 — Global Collection Store

Create `src/lib/state/collection.svelte.ts`:

```typescript
import { commands } from '$lib/bindings';
import type { CollectionView, CollectionItemView } from '$lib/bindings';

class CollectionStore {
  items = $state<CollectionItemView[]>([]);
  collection = $state<CollectionView | null>(null);
  loading = $state(false);

  getItemById(id: string): CollectionItemView | undefined {
    return this.items.find((item) => item.id === id);
  }

  async fetch(): Promise<void> {
    if (this.items.length > 0) return; // cache hit
    await this.refresh();
  }

  async refresh(): Promise<void> {
    if (this.loading) return;
    this.loading = true;
    try {
      const result = await commands.getCollection();
      if (result.status === 'ok') {
        this.collection = result.data;
        this.items = result.data.items;
      }
    } finally {
      this.loading = false;
    }
  }
}

export const collectionStore = new CollectionStore();
```

Update `+layout.svelte` to call `collectionStore.fetch()` instead of `collectionState.fetchCollection()` in the startup preload. The existing `CollectionState` context can delegate to the store's `refresh()` for `addRailwayModel` and `deleteItem` mutations.

### Step 2 — Navigation Active State

**`src/lib/components/navigation/types.ts`** — add `additionalPrefixes?`:

```typescript
export type NavigationItem = {
  id: string;
  label: () => string;
  icon: ComponentType;
  href: string;
  isPrimary: boolean;
  usePrefixMatch?: boolean;
  additionalPrefixes?: string[]; // NEW
};
```

**`src/lib/components/navigation/utils.ts`** — update `isActive()`:

```typescript
export function isActive(item: NavigationItem, pathname: string): boolean {
  if (item.usePrefixMatch && pathname.startsWith(item.href)) return true;
  if (item.additionalPrefixes?.some((p) => pathname.startsWith(p))) return true;
  return pathname === item.href;
}
```

**`src/lib/components/navigation/config.ts`** — add `additionalPrefixes` to collection:

```typescript
{
  id: 'collection',
  label: () => m.app_collection(),
  icon: TrainFront,
  href: '/my-collection',
  isPrimary: true,
  additionalPrefixes: ['/collection']   // NEW
}
```

### Step 3 — Update Collection Card Navigation

In `src/lib/features/collection/CollectionDashboard.svelte`, change `handleCardClick`:

```typescript
// Before:
goto(`/models/${item.railwayModel.railwayModelId}`);

// After:
goto(`/collection/${item.id}`);
```

### Step 4 — New Route: Collection Item Detail Page

Create directory: `src/routes/collection/[itemId]/`

Create `src/routes/collection/[itemId]/+page.svelte`:

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { collectionStore } from '$lib/state/collection.svelte';
  import { commands } from '$lib/bindings';
  import { ArrowLeft } from 'lucide-svelte';
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import CollectionItemSidebar from '$lib/features/collection/components/CollectionItemSidebar.svelte';
  import { toRailwayModel } from '$lib/features/collection/utils/modelViewMapper';
  import type {
    RailwayModelView,
    RailwayModelImageResponse,
    SellerView,
    CollectionItemView
  } from '$lib/bindings';

  const itemId = $page.params.itemId;

  let collectionItem = $state<CollectionItemView | null>(null);
  let model = $state<RailwayModelView | null>(null);
  let imageResponse = $state<RailwayModelImageResponse | null>(null);
  let seller = $state<SellerView | null>(null);
  let loading = $state(true);
  let error = $state<string | null>(null);

  const displayModel = $derived(
    model ? toRailwayModel(model, collectionItem, imageResponse) : null
  );

  onMount(async () => {
    try {
      // 1. Ensure collection is loaded (cache-first)
      await collectionStore.fetch();
      collectionItem = collectionStore.getItemById(itemId) ?? null;

      if (!collectionItem) {
        error = 'Item not found';
        return;
      }

      const railwayModelId = collectionItem.railwayModel.railwayModelId;
      const sellerId =
        collectionItem.purchaseInfo?.kind === 'purchased'
          ? collectionItem.purchaseInfo.data.seller
          : null;

      // 2. Parallel fetch: model card + seller
      const [modelResult, imageResult, sellerResult] = await Promise.all([
        commands.getRailwayModelById(railwayModelId),
        commands.getRailwayModelImage(railwayModelId),
        sellerId ? commands.getSellerById(sellerId) : Promise.resolve({ status: 'ok', data: null })
      ]);

      if (modelResult.status === 'ok') model = modelResult.data;
      if (imageResult.status === 'ok') imageResponse = imageResult.data;
      if (sellerResult.status === 'ok') seller = sellerResult.data;
    } catch (e) {
      error = e instanceof Error ? e.message : 'An error occurred';
    } finally {
      loading = false;
    }
  });
</script>
```

### Step 5 — New Sidebar Component

Create `src/lib/features/collection/components/CollectionItemSidebar.svelte` with four sections:

- **AcquisitionSection**: Seller (name, optional link), date (formatted), price with currency
- **ConditionSection**: modelCondition, boxCondition, purchaseCondition badges
- **OperationalSection**: per `OwnedRollingStockView`: DCC address + parsed decoder URN
- **PersonalContextSection**: `addedDate` (formatted), `notes` (line-clamp-3 preview)

Each section must handle `null` data gracefully with a "not recorded" empty state.

### Step 6 — Remove Old Route

Delete `src/routes/models/` directory (entire folder including `+page.svelte` and any
`+page.ts` files). Verify no other route imports or links reference `/models/`.

---

## Verification Checklist

```bash
# 1. Type check
pnpm check

# 2. Lint
pnpm lint

# 3. Unit tests
pnpm test

# 4. Manual: navigate collection → item → back
#    Verify "Collection" nav stays highlighted throughout

# 5. Manual: refresh browser on /collection/{itemId}
#    Verify item loads correctly (store re-fetches on empty)

# 6. Manual: item with no purchase info
#    Verify sidebar shows graceful empty state (no crashes)

# 7. Manual: item with DCC address
#    Verify DCC number is prominently displayed

# 8. Rust (if any backend changes made)
cargo fmt && cargo clippy && cargo test
```

---

## File Change Summary

| Action | File Path                                                             |
| ------ | --------------------------------------------------------------------- |
| CREATE | `src/lib/state/collection.svelte.ts`                                  |
| CREATE | `src/routes/collection/[itemId]/+page.svelte`                         |
| CREATE | `src/lib/features/collection/components/CollectionItemSidebar.svelte` |
| UPDATE | `src/lib/components/navigation/types.ts`                              |
| UPDATE | `src/lib/components/navigation/utils.ts`                              |
| UPDATE | `src/lib/components/navigation/config.ts`                             |
| UPDATE | `src/lib/features/collection/CollectionDashboard.svelte`              |
| UPDATE | `src/routes/+layout.svelte` (use `collectionStore.fetch()`)           |
| REMOVE | `src/routes/models/[...modelId]/+page.svelte`                         |
