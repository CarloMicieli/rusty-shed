# Quickstart: Track Inventory Feature Implementation

**Feature**: 005-track-inventory  
**Created**: 2026-01-30

## Prerequisites

- Branch: `005-track-inventory` (already created)
- Rust backend: Mostly complete
- Frontend: Empty placeholder

## Implementation Order

Follow this sequence to maintain a working build at each step:

### Phase 1: Rust Changes (Minor)

#### Step 1.1: Add Migration for track_type

Create `src-tauri/migrations/0007_add_track_type_to_products.sql`:

```sql
ALTER TABLE track_products ADD COLUMN track_type TEXT;
UPDATE track_products SET track_type = 'STRAIGHT' WHERE track_type IS NULL;
```

#### Step 1.2: Update TrackProduct Domain

In `src-tauri/src/tracks_inventory/domain/track_product.rs`, add:

```rust
use super::track_type::TrackType;

pub struct TrackProduct {
    // ... existing fields ...
    pub track_type: TrackType,  // ADD THIS
}
```

#### Step 1.3: Update Infrastructure

In `src-tauri/src/tracks_inventory/infrastructure/entities.rs`, update `TrackProductRow`:

```rust
pub struct TrackProductRow {
    // ... existing fields ...
    pub track_type: Option<String>,  // ADD THIS
}
```

Update repository queries to include `track_type` column.

#### Step 1.4: Create View Structs

Create `src-tauri/src/tracks_inventory/application/views.rs`:

```rust
use crate::core::domain::length::Length;
use crate::core::domain::monetary_amount::MonetaryAmount;
use crate::tracks_inventory::domain::{TrackCode, TrackId, TrackInventoryId, TrackPurchaseId, TrackType};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct TrackInventoryListItem {
    pub id: TrackInventoryId,
    pub name: String,
    pub description: Option<String>,
    pub total_items: i64,
    pub total_quantity: i64,
}

// ... other view structs as per data-model.md
```

#### Step 1.5: Create Query Handlers

Create `src-tauri/src/tracks_inventory/interface/query_handlers.rs`:

```rust
#[tauri::command]
#[specta::specta]
pub async fn get_track_inventories(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<TrackInventoryListItem>, CommandError> {
    // Implementation
}
```

#### Step 1.6: Verify Rust Changes

```bash
cd src-tauri
cargo fmt
cargo clippy -- -D warnings
cargo test
```

### Phase 2: Frontend Setup

#### Step 2.1: Create Route Structure

```bash
mkdir -p src/routes/my-tracks/\[id\]
```

Create placeholder pages:

- `src/routes/my-tracks/+page.svelte`
- `src/routes/my-tracks/[id]/+page.svelte`

#### Step 2.2: Create Service

Create `src/lib/features/track-inventory/services/TrackInventoryService.svelte.ts`:

```typescript
import { setContext, getContext } from 'svelte';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import type { TrackInventoryListItem, TrackInventoryView } from '$lib/bindings';

const SERVICE_KEY = Symbol('track-inventory-service');

export class TrackInventoryService {
  #inventories = $state<TrackInventoryListItem[]>([]);
  #selectedInventory = $state<TrackInventoryView | null>(null);
  #isLoading = $state(false);

  get inventories() {
    return this.#inventories;
  }
  get selectedInventory() {
    return this.#selectedInventory;
  }
  get isLoading() {
    return this.#isLoading;
  }

  async fetchInventories(): Promise<void> {
    this.#isLoading = true;
    try {
      const result = await safeInvoke<TrackInventoryListItem[]>('get_track_inventories');
      if (result.ok) {
        this.#inventories = result.data;
      }
    } finally {
      this.#isLoading = false;
    }
  }
}

export function setTrackInventoryService(service?: TrackInventoryService): TrackInventoryService {
  const instance = service ?? new TrackInventoryService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

export function getTrackInventoryService(): TrackInventoryService {
  const service = getContext<TrackInventoryService>(SERVICE_KEY);
  if (!service) {
    throw new Error('TrackInventoryService not found in context');
  }
  return service;
}
```

#### Step 2.3: Add Localization Keys

Add to `messages/en.json`:

```json
{
  "app_tracks": "My Tracks",
  "track_inventories_title": "Track Inventories",
  "track_inventories_empty_title": "No Track Inventories",
  "track_inventories_empty_caption": "Create your first track inventory to start managing your track collection.",
  "track_create_inventory": "Create Inventory",
  "track_add_purchase": "Add Purchase",
  "track_quantity": "Quantity",
  "track_required": "Required",
  "track_in_stock": "In Stock"
}
```

Add corresponding Italian translations to `messages/it.json`.

#### Step 2.4: Update Navigation

In `src/lib/features/navigation/components/SidebarNavigation.svelte`, add after "My Wishlists":

```svelte
<li>
  <a
    href={resolve('/my-tracks')}
    class="hover:variant-soft-primary btn w-full justify-start gap-3"
    class:variant-filled-primary={pathname === '/my-tracks'}
    class:variant-ghost-surface={pathname !== '/my-tracks'}
  >
    <Tram size={20} />
    <span class="font-medium tracking-wide">{m.app_tracks()}</span>
  </a>
</li>
```

Do the same for `BottomNavigation.svelte`.

#### Step 2.5: Implement List Page

Create `src/routes/my-tracks/+page.svelte`:

```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { getTrackInventoryService } from '$lib/features/track-inventory';
  import * as m from '$lib/paraglide/messages.js';

  const service = getTrackInventoryService();
  const inventories = $derived(service.inventories);
  const isLoading = $derived(service.isLoading);

  onMount(() => {
    service.fetchInventories();
  });
</script>

<div class="p-6">
  <h1 class="mb-6 h2">{m.track_inventories_title()}</h1>

  {#if isLoading}
    <p>Loading...</p>
  {:else if inventories.length === 0}
    <!-- Empty state -->
  {:else}
    <!-- Inventory list -->
  {/if}
</div>
```

### Phase 3: Complete Components

#### Step 3.1: Create Components

Build these components in order:

1. `InventoryCard.svelte` - Single inventory summary
2. `InventoryList.svelte` - Grid of inventory cards
3. `CreateInventoryDialog.svelte` - Modal for creating inventory
4. `InventoryDetail.svelte` - Full inventory view with items
5. `PurchaseHistory.svelte` - Purchase list
6. `AddPurchaseDialog.svelte` - Modal for adding purchase

#### Step 3.2: Wire Up Detail Page

`src/routes/my-tracks/[id]/+page.svelte`:

```svelte
<script lang="ts">
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { getTrackInventoryService } from '$lib/features/track-inventory';

  const service = getTrackInventoryService();
  const id = $derived(page.params.id);

  onMount(() => {
    service.fetchInventory(id);
  });
</script>
```

### Phase 4: Testing & Verification

#### Step 4.1: Rust Tests

```bash
pnpm rust:test
```

#### Step 4.2: Frontend Tests

Create `src/__tests__/features/track-inventory/TrackInventoryService.test.ts`:

```typescript
import { describe, it, expect, vi } from 'vitest';
// Test service methods
```

```bash
pnpm test
```

#### Step 4.3: Full Verification

```bash
pnpm format
pnpm lint
pnpm check
pnpm test
pnpm rust:format
pnpm rust:clippy
pnpm rust:test
```

## Regenerate TypeScript Bindings

After Rust changes, regenerate bindings:

```bash
pnpm tauri build  # Or pnpm dev to trigger specta generation
```

Check that new types appear in `src/lib/bindings.ts`.

## Checklist

- [ ] Migration 0007 created and tested
- [ ] `track_type` added to `TrackProduct`
- [ ] View structs created in application module
- [ ] Query handlers implemented
- [ ] TypeScript bindings regenerated
- [ ] Navigation updated (both components)
- [ ] Localization keys added (en + it)
- [ ] Service created with context provider
- [ ] Route pages created
- [ ] Components implemented
- [ ] Tests passing
- [ ] All linters/formatters passing
