# Quickstart: Railway Model Preview Card Component

**Feature**: Railway Model Preview Card Component
**Date**: 2026-02-12
**Purpose**: Quick implementation guide for using the RailwayModelPreviewCard component

---

## Basic Usage

### Import and Render

```svelte
<script lang="ts">
  import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';
  import type { CollectionItemView } from '$lib/bindings';

  // Your collection item data
  let item: CollectionItemView = $state(/* ... */);

  // Map collection item to component props
  const cardData = $derived({
    id: item.railwayModel.railwayModelId,
    manufacturer: item.railwayModel.manufacturer,
    productCode: item.railwayModel.productCode,
    series: item.rollingStocks[0]?.series ?? null,
    category: item.railwayModel.category,
    roadNumber: item.rollingStocks[0]?.roadNumber ?? null,
    scale: item.railwayModel.scale,
    powerMethod: null, // Not available in CollectionItemView
    era: item.railwayModel.epoch,
    purchaseDate: item.purchaseInfo?.purchaseDate ?? null,
    photoUrl: null, // TODO: Add image resolution logic
    unitCount: item.rollingStocks.length,
    digitalFeatures: [] // TODO: Map from rolling stock digital setup
  });

  function handleDelete(modelId: string) {
    console.log('Delete model:', modelId);
    // Implement deletion logic
  }
</script>

<RailwayModelPreviewCard model={cardData} onDelete={handleDelete} />
```

---

## Responsive Grid Layout

### Collection View Example

```svelte
<script lang="ts">
  import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';
  import type { CollectionItemView } from '$lib/bindings';

  let collection = $state<CollectionItemView[]>([]);

  // Map function to convert CollectionItemView to component props
  function toCardData(item: CollectionItemView) {
    return {
      id: item.railwayModel.railwayModelId,
      manufacturer: item.railwayModel.manufacturer,
      productCode: item.railwayModel.productCode,
      series: item.rollingStocks[0]?.series ?? null,
      category: item.railwayModel.category,
      roadNumber: item.rollingStocks[0]?.roadNumber ?? null,
      scale: item.railwayModel.scale,
      powerMethod: null,
      era: item.railwayModel.epoch,
      purchaseDate: item.purchaseInfo?.purchaseDate ?? null,
      photoUrl: null,
      unitCount: item.rollingStocks.length,
      digitalFeatures: []
    };
  }

  async function deleteModel(modelId: string) {
    // Call backend deletion command
    await invoke('remove_collection_item', { args: { id: modelId } });
    // Refresh collection
    collection = collection.filter((item) => item.id !== modelId);
  }
</script>

<!-- Responsive grid: 1 column on mobile, 2 on small screens, 3 on large -->
<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  {#each collection as item (item.id)}
    <RailwayModelPreviewCard model={toCardData(item)} onDelete={deleteModel} />
  {/each}
</div>
```

---

## Handling Missing Data

### Example with Minimal Data

```svelte
<script lang="ts">
  import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';

  // Minimal model with missing fields
  const minimalModel = {
    id: 'model-123',
    manufacturer: null, // Will display "Unknown"
    productCode: null,
    series: null,
    category: 'Unknown', // Will show generic train icon
    roadNumber: null, // Will display "---"
    scale: null, // Badge will be omitted
    powerMethod: null, // Badge will be omitted
    era: null, // Badge will be omitted
    purchaseDate: null, // Badge will be omitted
    photoUrl: null, // Will show category placeholder icon
    unitCount: 1, // Won't show badge (only shows when > 1)
    digitalFeatures: [] // Won't show overlay icons
  };
</script>

<RailwayModelPreviewCard model={minimalModel} />
```

The component gracefully handles all missing fields:

- ✅ Manufacturer: Displays "Unknown" (via i18n)
- ✅ Road Number: Displays "---" (via i18n)
- ✅ Photo: Shows category-specific placeholder icon
- ✅ Optional badges: Omitted from DOM entirely

---

## Edge Cases

### Long Road Numbers

```svelte
<script lang="ts">
  const modelWithLongRoadNumber = {
    id: 'model-456',
    manufacturer: 'Märklin',
    productCode: '37586',
    series: 'Class 66',
    category: 'ElectricLocomotive',
    roadNumber: '12 34 56 78 90 12 34 56 78 90 123' // 35 characters
    // ... other fields
  };
</script>

<RailwayModelPreviewCard model={modelWithLongRoadNumber} />
```

**Behavior**:

- Road numbers > 25 characters are truncated to 22 chars + "..."
- Click/hover reveals full road number (via Tooltip or state toggle)

### Multiple Digital Features

```svelte
<script lang="ts">
  const modelWithDigital = {
    id: 'model-789',
    // ... other fields
    digitalFeatures: ['Sound', 'DCC'] // Multiple features
  };
</script>

<RailwayModelPreviewCard model={modelWithDigital} />
```

**Behavior**:

- Icons stack horizontally in top-left corner
- Sound icon (speaker) + DCC icon (bolt) displayed side-by-side

---

## Customization

### Custom CSS Class

```svelte
<RailwayModelPreviewCard
  model={cardData}
  onDelete={handleDelete}
  class="shadow-lg hover:shadow-xl"
/>
```

### Without Delete Button

```svelte
<!-- Omit onDelete prop to hide delete button -->
<RailwayModelPreviewCard model={cardData} />
```

---

## Testing

### Unit Test Example

```typescript
import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';

describe('RailwayModelPreviewCard', () => {
  const mockModel = {
    id: 'test-001',
    manufacturer: 'Märklin',
    productCode: '37586',
    series: 'Class 66',
    category: 'DieselLocomotive',
    roadNumber: '66 001',
    scale: 'H0',
    powerMethod: 'DCC',
    era: 'VI',
    purchaseDate: '2024-06-15',
    photoUrl: null,
    unitCount: 1,
    digitalFeatures: ['Sound']
  };

  it('should render manufacturer and product code', () => {
    render(RailwayModelPreviewCard, {
      props: { model: mockModel }
    });

    expect(screen.getByText(/Märklin/)).toBeInTheDocument();
    expect(screen.getByText(/37586/)).toBeInTheDocument();
  });

  it('should call onDelete when confirmed', async () => {
    const onDelete = vi.fn();
    render(RailwayModelPreviewCard, {
      props: { model: mockModel, onDelete }
    });

    const deleteButton = screen.getByLabelText(/delete/i);
    await fireEvent.click(deleteButton);

    // Confirm in dialog
    const confirmButton = screen.getByText(/delete/i);
    await fireEvent.click(confirmButton);

    expect(onDelete).toHaveBeenCalledWith('test-001');
  });

  it('should display placeholder for missing photo', () => {
    render(RailwayModelPreviewCard, {
      props: { model: { ...mockModel, photoUrl: null } }
    });

    // Check for placeholder icon (SVG)
    const placeholder = screen.getByRole('img', { hidden: true });
    expect(placeholder).toBeInTheDocument();
  });

  it('should truncate long road numbers', () => {
    const longRoadNumber = '12 34 56 78 90 12 34 56 78 90 123';
    render(RailwayModelPreviewCard, {
      props: { model: { ...mockModel, roadNumber: longRoadNumber } }
    });

    // Should show truncated version
    expect(screen.getByText(/\.\.\./)).toBeInTheDocument();
    // Should not show full number initially
    expect(screen.queryByText(longRoadNumber)).not.toBeInTheDocument();
  });
});
```

---

## Common Patterns

### Loading State

```svelte
<script lang="ts">
  let isLoading = $state(true);
  let collection = $state<CollectionItemView[]>([]);

  onMount(async () => {
    isLoading = true;
    collection = await fetchCollection();
    isLoading = false;
  });
</script>

{#if isLoading}
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
    {#each Array(6) as _, index}
      <div class="skeleton rounded-container h-48" key={index}></div>
    {/each}
  </div>
{:else}
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
    {#each collection as item (item.id)}
      <RailwayModelPreviewCard model={toCardData(item)} onDelete={deleteModel} />
    {/each}
  </div>
{/if}
```

### Empty State

```svelte
{#if collection.length === 0}
  <div class="blueprint-panel text-surface-200 rounded-container p-10 text-center">
    <p class="text-base font-semibold">{m.collection_empty()}</p>
    <p class="text-surface-300 mt-2 text-sm">
      {m.collection_empty_message()}
    </p>
  </div>
{:else}
  <!-- Render cards -->
{/if}
```

---

## Performance Tips

1. **Use `{#key}` blocks** in loops to help Svelte track items efficiently:

   ```svelte
   {#each collection as item (item.id)}
     <RailwayModelPreviewCard model={toCardData(item)} />
   {/each}
   ```

2. **Lazy load images** (SmartImage component handles this automatically)

3. **Virtualize long lists** for collections with 100+ items (consider using a virtual list library)

4. **Memoize derived data** with `$derived` to avoid recomputing on every render

---

## Troubleshooting

### Images Not Showing

**Problem**: Photos not displaying even when `photoUrl` is provided

**Solution**: Ensure you're using Tauri's `convertFileSrc()` for local file paths:

```svelte
<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/core';

  const resolvedPhotoUrl = $derived(model.photoUrl ? convertFileSrc(model.photoUrl) : null);

  const cardData = {
    // ...
    photoUrl: resolvedPhotoUrl
  };
</script>
```

### Delete Button Not Showing

**Problem**: Delete button is not visible

**Solution**: Make sure you're passing the `onDelete` prop:

```svelte
<RailwayModelPreviewCard
  model={cardData}
  onDelete={handleDelete}  <!-- Required for delete button -->
/>
```

### Road Number Not Truncating

**Problem**: Long road numbers overflow the identification plate

**Solution**: Component handles truncation automatically. If still seeing issues, check that:

- Road number field is a string
- Component is using the latest version with truncation logic

---

## Next Steps

- Read the [full specification](./spec.md) for detailed requirements
- Review [data model documentation](./data-model.md) for type definitions
- Check [research notes](./research.md) for implementation decisions

---

**Version**: 1.0
**Last Updated**: 2026-02-12
**Maintainer**: Rusty Shed Team
