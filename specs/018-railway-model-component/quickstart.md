# Quickstart: Reusable Railway Model Component

**Feature**: 018-railway-model-component
**Date**: 2026-02-11

## Overview

This guide shows how to use the `RailwayModelCard` component in your SvelteKit pages to display railway model information.

---

## Installation

No installation required - the component is part of the project's component library.

**Location**: `src/lib/components/RailwayModelCard.svelte`

---

## Basic Usage

### 1. Import the Component

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import type { RailwayModel } from '$lib/bindings';

  let model: RailwayModel = $state({
    id: 1,
    manufacturer: 'Rivarossi',
    product_code: 'HR2906',
    scale: 'H0',
    era: 'IIIb',
    power_method: 'DC',
    category: 'Locomotive',
    description: 'Electric locomotive E.656 in original green livery',
    image_path: 'images/railway_models/1/hr2906.jpg',
    status: 'InCollection',
    rolling_stock: [
      {
        id: 1,
        railway_model_id: 1,
        series_code: 'E.656',
        series_name: 'I Serie',
        category: 'Electric Locomotive',
        subcategory: 'Bo-Bo-Bo',
        road_number: '656 001',
        depot: 'Milano Centrale',
        livery: 'Verde FS',
        control_type: 'Digital',
        dcc_interface: '21-pin MTC',
        coupling_type: 'NEM 362'
      }
    ]
  });
</script>

<RailwayModelCard {model} />
```

---

## Props API

### Required Props

| Prop    | Type           | Description                       |
| ------- | -------------- | --------------------------------- |
| `model` | `RailwayModel` | The railway model data to display |

### Optional Props

| Prop              | Type                      | Default     | Description                        |
| ----------------- | ------------------------- | ----------- | ---------------------------------- |
| `editable`        | `boolean`                 | `false`     | Enable image upload functionality  |
| `class`           | `string`                  | `''`        | Additional CSS classes for styling |
| `onImageUploaded` | `(path: string) => void`  | `undefined` | Callback when image is uploaded    |
| `onError`         | `(error: string) => void` | `undefined` | Callback when errors occur         |

---

## Examples

### Example 1: Read-Only Display (Collection Page)

Display a railway model from the collection without editing capabilities.

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import type { RailwayModel } from '$lib/bindings';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let modelId = $state(1);
  let model = $state<RailwayModel | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      model = await invoke<RailwayModel>('get_railway_model', {
        args: { id: modelId }
      });
    } catch (err) {
      console.error('Failed to load model:', err);
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <p>Loading...</p>
{:else if model}
  <RailwayModelCard {model} />
{:else}
  <p>Model not found</p>
{/if}
```

---

### Example 2: Editable Display (Edit Page)

Allow image upload with callbacks for state updates.

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import type { RailwayModel } from '$lib/bindings';
  import { invoke } from '@tauri-apps/api/core';
  import { toaster } from '$lib/toaster';
  import * as m from '$lib/paraglide/messages';

  let model = $state<RailwayModel>({
    /* ... */
  });

  async function handleImageUploaded(imagePath: string) {
    // Update the model in the database
    try {
      await invoke('update_railway_model_image', {
        args: {
          id: model.id,
          image_path: imagePath
        }
      });

      // Update local state
      model = { ...model, image_path: imagePath };

      // Show success message
      toaster.success(m.image_uploaded_successfully());
    } catch (err) {
      console.error('Failed to update model:', err);
      toaster.error(m.error_updating_image());
    }
  }

  function handleError(error: string) {
    toaster.error(error);
  }
</script>

<RailwayModelCard
  {model}
  editable={true}
  onImageUploaded={handleImageUploaded}
  onError={handleError}
/>
```

---

### Example 3: Custom Styling

Apply custom classes for layout customization.

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';
  import type { RailwayModel } from '$lib/bindings';

  let model = $state<RailwayModel>({
    /* ... */
  });
</script>

<!-- Full-width card with shadow -->
<RailwayModelCard {model} class="w-full shadow-lg" />

<!-- Compact card in a grid layout -->
<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
  <RailwayModelCard {model} class="compact" />
  <RailwayModelCard {model} class="compact" />
</div>
```

---

### Example 4: Single-Unit Model (Direct Display)

For models with only one rolling stock unit, the component automatically displays details directly without tabs.

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';

  let singleLocomotive = $state({
    id: 2,
    manufacturer: 'Fleischmann',
    product_code: '732101',
    scale: 'N',
    era: 'V',
    power_method: 'DC',
    category: 'Locomotive',
    description: 'DB AG electric locomotive class 101',
    image_path: null,
    status: 'Wishlist',
    rolling_stock: [
      {
        id: 2,
        railway_model_id: 2,
        series_code: '101',
        series_name: null,
        category: 'Electric Locomotive',
        subcategory: 'Bo-Bo',
        road_number: '101 001',
        depot: null,
        livery: 'Traffic Red',
        control_type: 'Analog',
        dcc_interface: null,
        coupling_type: 'Rapido'
      }
    ]
  });
</script>

<!-- Component automatically detects single-unit and displays inline -->
<RailwayModelCard model={singleLocomotive} />
```

**Result**: No tabs shown, rolling stock details appear directly under global specifications.

---

### Example 5: Multi-Unit Set (Tabbed Display)

For sets with multiple rolling stock units, the component displays a tabbed interface.

```svelte
<script lang="ts">
  import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';

  let passengerSet = $state({
    id: 3,
    manufacturer: 'Roco',
    product_code: '74141',
    scale: 'H0',
    era: 'IV',
    power_method: null,
    category: 'Passenger Set',
    description: 'ÖBB Railjet 3-car set',
    image_path: 'images/railway_models/3/railjet.jpg',
    status: 'InCollection',
    rolling_stock: [
      {
        id: 3,
        railway_model_id: 3,
        series_code: 'Railjet',
        series_name: 'Control Car',
        category: 'Passenger Car',
        subcategory: 'Control Car',
        road_number: '80 90 73 90 004-2',
        depot: null,
        livery: 'Railjet Red/Grey',
        control_type: null,
        dcc_interface: null,
        coupling_type: 'NEM 362'
      },
      {
        id: 4,
        railway_model_id: 3,
        series_code: 'Railjet',
        series_name: 'Bistro Car',
        category: 'Passenger Car',
        subcategory: 'Restaurant Car',
        road_number: '61 80 88-90 101-3',
        depot: null,
        livery: 'Railjet Red/Grey',
        control_type: null,
        dcc_interface: null,
        coupling_type: 'NEM 362'
      },
      {
        id: 5,
        railway_model_id: 3,
        series_code: 'Railjet',
        series_name: 'Standard Car',
        category: 'Passenger Car',
        subcategory: '2nd Class',
        road_number: '61 80 21-90 012-7',
        depot: null,
        livery: 'Railjet Red/Grey',
        control_type: null,
        dcc_interface: null,
        coupling_type: 'NEM 362'
      }
    ]
  });
</script>

<!-- Component automatically detects multi-unit and displays tabs -->
<RailwayModelCard model={passengerSet} />
```

**Result**: Two tabs shown ("Railway Model Details" and "Rolling Stock List"), expandable rows for each unit.

---

## Component Behavior

### Automatic Display Mode Selection

The component automatically chooses the display mode based on rolling stock count:

- **Single-unit mode** (1 rolling stock unit): Rolling stock details shown directly under global specs, no tabs
- **Multi-unit mode** (2+ rolling stock units): Tabbed interface with expandable rolling stock list

### Image Handling

- **No Image**: Displays placeholder with upload controls (if `editable=true`)
- **With Image**: Displays image with replace controls (if `editable=true`)
- **Upload Progress**: Shows loading indicator during upload
- **Upload Errors**: Displays error toast via `onError` callback

### Responsive Behavior

- **Mobile (< 768px)**: Vertical layout, full-width tabs, stacked rolling stock rows
- **Tablet (768px - 1024px)**: Hybrid layout, 2-column rolling stock grid
- **Desktop (> 1024px)**: Horizontal header, constrained image width, 2-column rolling stock grid

---

## Internationalization (i18n)

All user-facing strings use Paraglide. Required message keys:

```typescript
// Component labels
m.railway_model_details(); // "Railway Model Details"
m.rolling_stock_list(); // "Rolling Stock List"
m.upload_image(); // "Upload Image"
m.drag_drop_image_here(); // "Drag and drop an image here"

// Field labels
m.manufacturer(); // "Manufacturer"
m.product_code(); // "Product Code"
m.scale(); // "Scale"
m.era(); // "Era"
m.power_method(); // "Power Method"
m.category(); // "Category"
m.description(); // "Description"
m.series_code(); // "Series Code"
m.road_number(); // "Road Number"
m.depot(); // "Depot"
m.livery(); // "Livery"
m.control_type(); // "Control Type"
m.dcc_interface(); // "DCC Interface"
m.coupling_type(); // "Coupling Type"

// Error messages
m.error_invalid_image_format(); // "Invalid image format..."
m.error_image_too_large(); // "Image file is too large..."
m.error_upload_failed(); // "Upload failed. Please try again."
```

---

## Styling Conventions

The component follows project styling conventions from MEMORY.md:

```svelte
<!-- Card container: matches Dashboard style -->
<div class="card gauge-frame ring-1 ring-border/40">
  <!-- ... -->
</div>

<!-- Content sections: matches Dashboard style -->
<div class="rounded-lg border border-white/10 bg-black/20 p-4">
  <!-- ... -->
</div>

<!-- Search/filter inputs: horizontal layout -->
<div class="flex items-center gap-2">
  <SearchIcon class="h-4 w-4" />
  <input type="text" class="flex-1" />
  <button>Clear</button>
</div>
```

---

## Testing Your Integration

### Manual Testing Checklist

- [ ] Component renders with valid model data
- [ ] Single-unit model displays inline (no tabs)
- [ ] Multi-unit model displays tabs
- [ ] Image placeholder shown when `image_path` is null
- [ ] Image upload button visible when `editable=true`
- [ ] Drag-drop visual feedback works
- [ ] Image upload succeeds with valid file
- [ ] Error toast shown for invalid file type
- [ ] Error toast shown for oversized file
- [ ] `onImageUploaded` callback fires with correct path
- [ ] Component responsive on mobile (320px)
- [ ] Component responsive on tablet (768px)
- [ ] Component responsive on desktop (1280px+)
- [ ] Tab switching works (multi-unit models)
- [ ] Rolling stock rows expand/collapse (multi-unit models)
- [ ] All text uses Paraglide (no hardcoded strings)

### Automated Testing

```typescript
// Example Vitest test
import { describe, it, expect } from 'vitest';
import { render } from '@testing-library/svelte';
import RailwayModelCard from '$lib/components/RailwayModelCard.svelte';

describe('RailwayModelCard', () => {
  it('renders single-unit model without tabs', () => {
    const model = createMockModel({ rolling_stock_count: 1 });
    const { container } = render(RailwayModelCard, { props: { model } });

    expect(container.querySelector('[role="tablist"]')).toBeNull();
  });

  it('renders multi-unit model with tabs', () => {
    const model = createMockModel({ rolling_stock_count: 3 });
    const { container } = render(RailwayModelCard, { props: { model } });

    expect(container.querySelector('[role="tablist"]')).not.toBeNull();
  });
});
```

---

## Troubleshooting

### Issue: Image upload fails with "Model not found"

**Solution**: Ensure the `railway_model_id` exists in the database before uploading.

```typescript
// Check model exists before enabling upload
let modelExists = $derived(model?.id && model.id > 0);
```

---

### Issue: Component doesn't update after image upload

**Solution**: Ensure you're updating the reactive state in the `onImageUploaded` callback:

```typescript
function handleImageUploaded(imagePath: string) {
  // ❌ Don't do this (mutates object)
  model.image_path = imagePath;

  // ✅ Do this (creates new object reference)
  model = { ...model, image_path: imagePath };
}
```

---

### Issue: Tabs not showing for multi-unit model

**Solution**: Verify `rolling_stock` array has 2+ entries:

```typescript
console.log('Rolling stock count:', model.rolling_stock?.length);
```

---

### Issue: Image upload shows "Invalid file path" error

**Solution**: Ensure you're passing the absolute path from Tauri's file dialog:

```typescript
import { open } from '@tauri-apps/plugin-dialog';

const selected = await open({
  /* ... */
});
if (selected) {
  // ✅ selected.path is an absolute path
  await uploadImage(selected.path);
}
```

---

## Performance Tips

- **Lazy Load Images**: Use `SmartImage` component with lazy loading for large collections
- **Virtualize Long Lists**: If displaying 20+ rolling stock units, consider virtualizing the list
- **Debounce Search**: If adding search/filter, debounce input events

---

## Related Documentation

- [Feature Specification](./spec.md) - Full feature requirements and user stories
- [Data Model](./data-model.md) - Entity definitions and relationships
- [API Contract: Upload Model Image](./contracts/upload-model-image.md) - Backend command documentation
- [Research](./research.md) - Technical decisions and alternatives considered

---

## Support

For issues or questions:

1. Check the [Troubleshooting](#troubleshooting) section above
2. Review the [Component Behavior](#component-behavior) section
3. Consult [data-model.md](./data-model.md) for type definitions
4. Check [CLAUDE.md](/CLAUDE.md) for project conventions
