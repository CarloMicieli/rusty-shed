# Component Contract: ModelCard

**Component**: `ModelCard.svelte`  
**Location**: `src/lib/features/dashboard/components/ModelCard.svelte`  
**Purpose**: Display a single railway model as a compact horizontal card with thumbnail, badges, and navigation

**Design Pattern**: Horizontal layout (16:9 thumbnail + details) for information density in "industrial-luxe" dark theme

## Props Interface

```typescript
interface ModelCardProps {
  /** The model data to display */
  model: ModelCard;
}
```

## Component Responsibilities

### 1. Display Model Thumbnail (16:9 Aspect Ratio)

**What**: Show model image or placeholder in landscape 16:9 aspect ratio

**Why 16:9**: Standard photo/video ratio that showcases model details effectively while maintaining reasonable card height

**How**:

```svelte
<script>
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { TrainFront } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let imageSrc = $state<string | null>(null);
  let imageError = $state(false);
  let isLoading = $state(true);

  onMount(async () => {
    if (model.thumbnailPath) {
      try {
        imageSrc = convertFileSrc(model.thumbnailPath);
      } catch {
        imageError = true;
      }
    } else {
      imageError = true;
    }
    isLoading = false;
  });
</script>

<!-- 16:9 Aspect Ratio thumbnail (landscape) -->
<div class="aspect-video w-40 flex-shrink-0 overflow-hidden rounded bg-zinc-800">
  {#if isLoading}
    <div class="skeleton h-full w-full"></div>
  {:else if imageError || !imageSrc}
    <div class="flex h-full items-center justify-center">
      <TrainFront size={32} class="text-zinc-600" />
    </div>
  {:else}
    <img src={imageSrc} alt={model.productCode} class="h-full w-full object-cover" />
  {/if}
</div>
```

**Aspect Ratio**: 16:9 (w-40 aspect-video, approximately 160px × 90px)  
**Placeholder**: TrainFront icon in zinc-600 for visual continuity

---

### 2. Display Manufacturer & Product Code (Industrial-Luxe Styling)

**What**: Show brand identity with "Rusty Shed" color palette

**How**:

```svelte
<div class="flex min-w-0 flex-col justify-center">
  <span class="text-xs font-bold text-orange-400 uppercase">
    {model.manufacturer}
  </span>
  <span class="text-sm font-medium text-white">
    {model.productCode}
  </span>
  <p class="truncate text-xs text-zinc-400">
    {truncatedDescription}
  </p>
</div>
```

**Color Palette**:

- Manufacturer: `text-orange-400` (copper/amber accent)
- Product Code: `text-white` (primary text)
- Description: `text-zinc-400` (secondary text)

**Typography**:

- Manufacturer: Bold, uppercase, extra-small
- Product Code: Medium weight, small
- Description: Regular, extra-small, truncated

---

### 3. Show Condition Badge (Absolute Positioned)

**What**: High-contrast badge in top-right corner of card

**How**:

```svelte
<script>
  const conditionConfig = $derived(
    {
      NEW: {
        variant: 'default' as const, // Orange/amber variant
        label: m.dashboard_condition_new()
      },
      PRE_OWNED: {
        variant: 'secondary' as const, // Gray variant
        label: m.dashboard_condition_preowned()
      },
      UNKNOWN: {
        variant: 'secondary' as const,
        label: m.dashboard_condition_unknown()
      }
    }[model.condition]
  );
</script>

<div class="absolute top-2 right-2">
  <Badge variant={conditionConfig.variant} class="text-xs">
    {conditionConfig.label}
  </Badge>
</div>
```

**Badge Positioning**: Absolute top-2 right-2 (floats above card content)

**Badge Colors**:

- **NEW**: `variant="default"` → Orange/amber (aligns with accent color)
- **PRE_OWNED**: `variant="secondary"` → Gray
- **UNKNOWN**: `variant="secondary"` → Gray

---

### 4. Display Truncated Description

**What**: Show model description, gracefully truncated to prevent layout breaks

**How**:

```svelte
<script>
  const truncatedDescription = $derived(
    model.description.length > 50 ? model.description.slice(0, 47) + '...' : model.description
  );
</script>

<p class="truncate text-xs text-zinc-400">
  {truncatedDescription}
</p>
```

**Truncation**: 50 chars max + `truncate` class for CSS overflow handling

---

### 5. Enable Click Navigation (Horizontal Card as Button)

**What**: Entire card is clickable button with hover state

**How**:

```svelte
<script>
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  function handleClick() {
    goto(resolve(`/models/${model.id}`));
  }
</script>

<button
  type="button"
  onclick={handleClick}
  class="relative flex cursor-pointer gap-3 rounded border border-zinc-800 bg-zinc-900/50 p-2 text-left transition-colors hover:bg-zinc-800/70"
>
  <!-- Card content here -->
</button>
```

**Interaction States**:

- Base: `bg-zinc-900/50 border-zinc-800`
- Hover: `hover:bg-zinc-800/70` (lighter background)
- Transition: `transition-colors` (smooth color change)
- Cursor: `cursor-pointer`

---

## Visual Hierarchy (Horizontal Layout)

```
┌─ ModelCard (button) ─────────────────────────────────┐
│                                          [Badge]      │ ← Absolute positioned
│  ┌──────────┐  Manufacturer (orange-400, uppercase)  │
│  │          │  Product Code (white, medium weight)   │
│  │   IMG    │  Description text truncated...         │
│  │   16:9   │       (zinc-400)                       │
│  └──────────┘                                         │
│  ~160×90px                                            │
└───────────────────────────────────────────────────────┘
     ↑            ↑
  Thumbnail    Details (flex column)
```

**Layout**: `flex gap-3` (horizontal: thumbnail left, details right)  
**Badge**: Floats above content in top-right corner

---

## Styling Requirements (Industrial-Luxe Theme)

**Container**:

```css
relative flex gap-3 p-2 rounded bg-zinc-900/50 border border-zinc-800
hover:bg-zinc-800/70 transition-colors cursor-pointer text-left
```

**Color System**:

- Background: `bg-zinc-900/50` (dark with transparency)
- Border: `border-zinc-800`
- Hover: `bg-zinc-800/70` (lighter on interaction)
- Accent: `text-orange-400` (manufacturer names)
- Primary: `text-white` (product codes)
- Secondary: `text-zinc-400` (descriptions)
- Muted: `text-zinc-500` (notes, metadata)

**Optional Glassmorphism** (for purchase container, not individual cards):

```css
bg-white/5 backdrop-blur-md
```

---

## Accessibility

- **Semantic HTML**: `<button>` wrapper for clickability
- **Alt Text**: Image alt uses product code (concise identifier)
- **Keyboard Navigation**: Tab-navigable, Enter key activates
- **Focus Visible**: Browser default focus ring
- **Text Alignment**: `text-left` ensures proper reading flow

## Testing Checklist

- [ ] Renders with valid model data
- [ ] Displays 16:9 aspect ratio thumbnail (w-40 aspect-video)
- [ ] Shows TrainFront placeholder when thumbnailPath is null
- [ ] Shows placeholder when image fails to load
- [ ] Displays skeleton loader while image loading
- [ ] Shows manufacturer in orange-400 (uppercase, bold)
- [ ] Shows product code in white (medium weight)
- [ ] Displays correct condition badge variant (default for NEW, secondary for PRE_OWNED)
- [ ] Badge positioned absolute top-right
- [ ] Truncates descriptions longer than 50 characters
- [ ] Navigates to correct model detail page on click
- [ ] Hover state shows background transition to zinc-800/70
- [ ] Keyboard Enter key triggers navigation
- [ ] Card maintains consistent height in horizontal layout
- [ ] Works in grid: 1-col mobile, 3-col desktop

## Dependencies

- `$lib/bindings` (ModelCard type)
- `$lib/paraglide/messages.js` (i18n)
- `$lib/components/Badge.svelte` (shadcn-svelte)
- `@tauri-apps/api/core` (convertFileSrc)
- `lucide-svelte` (TrainFront icon for placeholder)
- `$app/navigation` (goto, resolve)

## Component Signature

```svelte
<script lang="ts">
  import type { ModelCard } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import { TrainFront } from 'lucide-svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';

  interface Props {
    model: ModelCard;
  }

  let { model }: Props = $props();

  // State
  let imageSrc = $state<string | null>(null);
  let imageError = $state(false);
  let isLoading = $state(true);

  // Derived
  const truncatedDescription = $derived(
    model.description.length > 50 ? model.description.slice(0, 47) + '...' : model.description
  );

  const conditionConfig = $derived(
    {
      NEW: { variant: 'default' as const, label: m.dashboard_condition_new() },
      PRE_OWNED: { variant: 'secondary' as const, label: m.dashboard_condition_preowned() },
      UNKNOWN: { variant: 'secondary' as const, label: m.dashboard_condition_unknown() }
    }[model.condition]
  );

  // Handlers
  function handleClick() {
    goto(resolve(`/models/${model.id}`));
  }

  onMount(async () => {
    if (model.thumbnailPath) {
      try {
        imageSrc = convertFileSrc(model.thumbnailPath);
      } catch {
        imageError = true;
      }
    } else {
      imageError = true;
    }
    isLoading = false;
  });
</script>

<button
  type="button"
  onclick={handleClick}
  class="relative flex cursor-pointer gap-3 rounded border border-zinc-800 bg-zinc-900/50 p-2 text-left transition-colors hover:bg-zinc-800/70"
>
  <!-- Condition Badge (absolute positioned) -->
  <div class="absolute top-2 right-2">
    <Badge variant={conditionConfig.variant} class="text-xs">
      {conditionConfig.label}
    </Badge>
  </div>

  <!-- Thumbnail (1:1 square, 80×80px) -->
  <div class="h-20 w-20 flex-shrink-0 overflow-hidden rounded bg-zinc-800">
    {#if isLoading}
      <div class="skeleton h-full w-full"></div>
    {:else if imageError || !imageSrc}
      <div class="flex h-full items-center justify-center">
        <TrainFront size={32} class="text-zinc-600" />
      </div>
    {:else}
      <img src={imageSrc} alt={model.productCode} class="h-full w-full object-cover" />
    {/if}
  </div>

  <!-- Model Details (vertical stack) -->
  <div class="flex min-w-0 flex-col justify-center">
    <span class="text-xs font-bold text-orange-400 uppercase">
      {model.manufacturer}
    </span>
    <span class="text-sm font-medium text-white">
      {model.productCode}
    </span>
    <p class="truncate text-xs text-zinc-400">
      {truncatedDescription}
    </p>
  </div>
</button>
```

## Edge Cases

| Scenario                       | Expected Behavior                                          |
| ------------------------------ | ---------------------------------------------------------- |
| `thumbnailPath` is null        | Show TrainFront placeholder icon (zinc-600)                |
| Image fails to load            | Show TrainFront placeholder icon                           |
| Description is empty           | Show "No description available"                            |
| Description is very long       | Truncate at 50 chars with ellipsis                         |
| Manufacturer name is very long | Allow wrapping (no truncation needed in horizontal layout) |
| Product code is missing        | Show "N/A"                                                 |
| Model ID is invalid            | Log error, disable navigation                              |

## Performance Notes

- **Image Loading**: Deferred until component mounts (non-blocking)
- **Lazy Loading**: Not required for visible cards (only 3 per group × 3 groups = 9 max)
- **Derived Values**: Memoized via Svelte 5 `$derived` (automatic)
- **Event Handlers**: Single click handler, no inline functions
- **Layout**: Pure CSS (no JS measurements)

---

### 2. Display Manufacturer & Product Code

**What**: Show brand identity prominently below image

**How**:

```svelte
<div class="mt-2 flex items-center justify-between gap-2">
  <p class="text-surface-100 text-sm font-bold">
    {model.manufacturer}
  </p>
  <p class="text-surface-400 font-mono text-xs">
    {model.productCode}
  </p>
</div>
```

**Visual Priority**: Manufacturer name is primary (bold), product code is secondary (mono font, smaller)

---

### 3. Show Condition Badge

**What**: High-contrast badge indicating New/Pre-owned/Unknown status

**How**:

```svelte
<script>
  const conditionConfig = $derived(
    {
      NEW: {
        variant: 'success' as const,
        label: m.dashboard_condition_new()
      },
      PRE_OWNED: {
        variant: 'info' as const,
        label: m.dashboard_condition_preowned()
      },
      UNKNOWN: {
        variant: 'secondary' as const,
        label: m.dashboard_condition_unknown()
      }
    }[model.condition]
  );
</script>

<Badge variant={conditionConfig.variant} class="text-xs">
  {conditionConfig.label}
</Badge>
```

**Badge Colors**:

- **NEW**: Green (variant-filled-success)
- **PRE_OWNED**: Blue (variant-filled-info)
- **UNKNOWN**: Gray (variant-filled-secondary)

---

### 4. Display Truncated Description

**What**: Show model description, gracefully truncated if too long

**How**:

```svelte
<script>
  const truncatedDescription = $derived(
    model.description.length > 50 ? model.description.slice(0, 47) + '...' : model.description
  );
</script>

<p class="text-surface-300 line-clamp-2 text-sm">
  {truncatedDescription}
</p>
```

**Truncation Rules**:

- Max 50 characters before ellipsis
- Use Tailwind `line-clamp-2` for multi-line overflow
- Ensure description doesn't break grid layout

---

### 5. Enable Click Navigation

**What**: Make entire card clickable, navigate to model details page

**How**:

```svelte
<script>
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  function handleClick() {
    goto(resolve(`/models/${model.id}`));
  }
</script>

<button
  type="button"
  onclick={handleClick}
  class="card hover:bg-surface-700/50 w-full cursor-pointer space-y-2 p-4 text-left transition-colors"
>
  <!-- Card content here -->
</button>
```

**Interaction**:

- Hover state: Background lightens
- Cursor: Pointer on hover
- Click: Navigate to `/models/{model.id}`
- Keyboard: Enter key triggers navigation

---

## Visual Hierarchy

```
┌─ ModelCard ──────────────────────┐
│                                   │
│  ┌────────────────────────────┐  │
│  │      [Thumbnail Image]     │  │ ← 16:9 aspect ratio
│  │         or Placeholder     │  │
│  └────────────────────────────┘  │
│                                   │
│  Roco                     62150   │ ← Manufacturer + Code
│  ┌───────┐                        │
│  │ New   │                        │ ← Condition badge
│  └───────┘                        │
│  Electric locomotive BR 193...    │ ← Description (truncated)
│                                   │
└───────────────────────────────────┘
```

## Styling Requirements

- **Container**: shadcn-svelte `card` class with padding
- **Hover Effect**: Background transition on hover (`hover:bg-surface-700/50`)
- **Image Container**: Fixed aspect ratio (16:9), rounded corners
- **Text Alignment**: Left-aligned, proper spacing between elements
- **Badge Position**: Below manufacturer info, aligned left

## Accessibility

- **Semantic HTML**: Use `<button>` wrapper for clickability
- **Alt Text**: Image has descriptive alt text (model description)
- **Keyboard Navigation**: Tab-navigable, Enter key activates
- **ARIA Labels**: Button has implicit label via content
- **Focus Visible**: Browser default focus ring on keyboard navigation

## Testing Checklist

- [ ] Renders with valid model data
- [ ] Displays thumbnail image when thumbnailPath provided
- [ ] Shows placeholder icon when thumbnailPath is null
- [ ] Shows placeholder icon when image fails to load
- [ ] Displays skeleton loader while image loading
- [ ] Shows manufacturer name prominently
- [ ] Shows product code in monospace font
- [ ] Displays correct condition badge (New/Pre-owned/Unknown)
- [ ] Truncates descriptions longer than 50 characters
- [ ] Navigates to correct model detail page on click
- [ ] Hover state shows visual feedback
- [ ] Keyboard Enter key triggers navigation
- [ ] Card maintains consistent height across grid
- [ ] Responsive: adapts to 1-col, 2-col, 3-col layouts

## Dependencies

- `$lib/bindings` (ModelCard type)
- `$lib/paraglide/messages.js` (i18n)
- `$lib/components/Badge.svelte` (shadcn-svelte)
- `@tauri-apps/api/core` (convertFileSrc)
- `lucide-svelte` (TrainFront icon for placeholder)
- `$app/navigation` (goto, resolve)

## Component Signature

```svelte
<script lang="ts">
  import type { ModelCard } from '$lib/bindings';
  import { Badge } from '$lib/components';
  import { TrainFront } from 'lucide-svelte';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';
  import * as m from '$lib/paraglide/messages.js';
  import { onMount } from 'svelte';

  interface Props {
    model: ModelCard;
  }

  let { model }: Props = $props();

  // State
  let imageSrc = $state<string | null>(null);
  let imageError = $state(false);
  let isLoading = $state(true);

  // Derived
  const truncatedDescription = $derived(
    model.description.length > 50 ? model.description.slice(0, 47) + '...' : model.description
  );

  const conditionConfig = $derived(
    {
      NEW: { variant: 'success' as const, label: m.dashboard_condition_new() },
      PRE_OWNED: { variant: 'info' as const, label: m.dashboard_condition_preowned() },
      UNKNOWN: { variant: 'secondary' as const, label: m.dashboard_condition_unknown() }
    }[model.condition]
  );

  // Handlers
  function handleClick() {
    goto(resolve(`/models/${model.id}`));
  }

  onMount(async () => {
    if (model.thumbnailPath) {
      try {
        imageSrc = convertFileSrc(model.thumbnailPath);
      } catch {
        imageError = true;
      }
    } else {
      imageError = true;
    }
    isLoading = false;
  });
</script>

<button
  type="button"
  onclick={handleClick}
  class="card hover:bg-surface-700/50 w-full cursor-pointer space-y-2 p-4 text-left transition-colors"
>
  <!-- Implementation here -->
</button>
```

## Edge Cases

| Scenario                       | Expected Behavior                  |
| ------------------------------ | ---------------------------------- |
| `thumbnailPath` is null        | Show placeholder icon (TrainFront) |
| Image fails to load            | Show placeholder icon (TrainFront) |
| Description is empty           | Show "No description available"    |
| Description is very long       | Truncate at 50 chars with ellipsis |
| Manufacturer name is very long | Allow wrapping, no truncation      |
| Product code is missing        | Show "N/A"                         |
| Model ID is invalid            | Log error, disable navigation      |

## Performance Notes

- **Image Loading**: Deferred until component mounts (prevents blocking)
- **Lazy Loading**: Consider IntersectionObserver for off-screen cards (future optimization)
- **Derived Values**: Memoized via Svelte 5 `$derived` (automatic)
- **Event Handlers**: Single click handler, no inline functions

## Lazy Loading Enhancement (Optional)

For future optimization when many cards are rendered:

```svelte
<script>
  let imgElement: HTMLDivElement;
  let isVisible = $state(false);

  onMount(() => {
    const observer = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting) {
        isVisible = true;
        observer.disconnect();
      }
    });

    if (imgElement) observer.observe(imgElement);

    return () => observer.disconnect();
  });
</script>

<div bind:this={imgElement}>
  {#if isVisible}
    <!-- Load image -->
  {:else}
    <!-- Placeholder -->
  {/if}
</div>
```
