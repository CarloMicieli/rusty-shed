# Research: Railway Model Preview Card Component

**Date**: 2026-02-12 (Updated)
**Feature**: Railway Model Preview Card Component
**Purpose**: Technical research to resolve implementation decisions based on existing codebase patterns

## Research Areas

### 1. Svelte 5 Runes Patterns for Presentational Components

**Decision**: Use `$props()` for component props with TypeScript interface, `$derived()` for computed values, and `createEventDispatcher` for events

**Rationale**:

- Svelte 5 runes provide explicit, type-safe reactivity
- `$props()` with TypeScript interface gives compile-time prop validation
- `$derived()` is more performant than reactive statements for computed values
- Event dispatcher pattern is standard for parent-child communication

**Pattern**:

```typescript
<script lang="ts">
  import type { RailwayModel } from './types';

  interface Props {
    model: RailwayModel;
    onDelete?: (modelId: string) => void;
  }

  let { model, onDelete }: Props = $props();

  // Derived state
  const hasPhoto = $derived(!!model.photoUrl);
  const truncatedRoadNumber = $derived(
    model.roadNumber && model.roadNumber.length > 25
      ? model.roadNumber.substring(0, 22) + '...'
      : model.roadNumber
  );
</script>
```

**Alternatives considered**:

- Traditional reactive statements (`$:`) - less explicit, harder to type
- Stores for component state - overkill for presentational components

### 2. Responsive Layout Strategy (Desktop vs Mobile)

**Decision**: Use Tailwind container queries with `@container` directive and CSS Grid for layout

**Rationale**:

- Container queries allow component to adapt based on parent width, not viewport
- CSS Grid provides precise control over thumbnail + content layout
- Tailwind 4.x has native container query support
- Mobile layout (vertical stack) is achieved by switching grid template

**Pattern**:

```svelte
<div class="@container">
  <div class="grid gap-4 @sm:grid-cols-1 @lg:grid-cols-[auto_1fr]">
    <!-- Thumbnail on left (desktop) or top (mobile) -->
    <div class="aspect-video">...</div>

    <!-- Content on right (desktop) or bottom (mobile) -->
    <div>...</div>
  </div>
</div>
```

**Alternatives considered**:

- Media queries (`md:`, `lg:`) - less flexible, tied to viewport not container
- Flexbox - less precise control over aspect ratios and alignment

### 3. Category-Specific Placeholder Icon Strategy

**Decision**: Use lucide-svelte icon components with conditional rendering based on category enum

**Rationale**:

- lucide-svelte provides clean, scalable SVG icons
- Conditional rendering is simple and type-safe
- Fallback to generic Train icon when category unknown
- Icons are lightweight and load quickly

**Pattern**:

```typescript
import { Train, Zap, Box } from 'lucide-svelte';

const placeholderIcon = $derived(() => {
  if (!model.photoUrl) {
    switch (model.category) {
      case 'SteamLocomotive':
        return Train;
      case 'ElectricLocomotive':
        return Zap;
      case 'Wagon':
        return Box;
      default:
        return Train; // Generic fallback
    }
  }
  return null;
});
```

**Alternatives considered**:

- Static image files - harder to maintain, less flexible theming
- Custom SVG components - more work, lucide has good coverage
- Icon fonts - less accessible, harder to customize

### 4. Image Aspect Ratio Enforcement (16:9)

**Decision**: Use Tailwind `aspect-video` (16/9) utility with `object-cover` for image fitting

**Rationale**:

- `aspect-video` enforces 16:9 ratio declaratively
- `object-cover` ensures image fills container without distortion
- Works consistently across browsers
- Preserves aspect ratio even if source image is different

**Pattern**:

```svelte
<div class="bg-surface-200 aspect-video overflow-hidden rounded-lg">
  {#if model.photoUrl}
    <img src={model.photoUrl} alt={model.series} class="h-full w-full object-cover" />
  {:else}
    <div class="flex h-full w-full items-center justify-center">
      <svelte:component this={placeholderIcon} class="text-surface-500 h-16 w-16" />
    </div>
  {/if}
</div>
```

**Alternatives considered**:

- CSS padding-top hack - less readable, harder to maintain
- JavaScript calculations - unnecessary complexity, layout shift risk

### 5. Truncation with Click/Hover Tooltip Pattern

**Decision**: Use shadcn-svelte `Tooltip` component with conditional truncation

**Rationale**:

- shadcn-svelte Tooltip is already in project dependencies
- Accessible by default (keyboard navigation, ARIA attributes)
- Works on both hover (desktop) and click/touch (mobile)
- Consistent with rest of UI library

**Pattern**:

```svelte
<script>
  import { Tooltip } from '$lib/components/ui/tooltip';

  const shouldTruncate = $derived(model.roadNumber && model.roadNumber.length > 25);
  const displayRoadNumber = $derived(
    shouldTruncate ? model.roadNumber!.substring(0, 22) + '...' : model.roadNumber || '---'
  );
</script>

{#if shouldTruncate}
  <Tooltip.Root>
    <Tooltip.Trigger class="font-mono text-sm">
      # {displayRoadNumber}
    </Tooltip.Trigger>
    <Tooltip.Content>
      {model.roadNumber}
    </Tooltip.Content>
  </Tooltip.Root>
{:else}
  <span class="font-mono text-sm">
    # {displayRoadNumber}
  </span>
{/if}
```

**Alternatives considered**:

- CSS `title` attribute - less accessible, poor mobile support
- Custom tooltip - reinventing the wheel, more testing needed
- Modal for long text - overkill for this use case

### 6. Badge Layout Pattern (Handling Missing Data)

**Decision**: Use flexbox with conditional rendering to omit missing badges

**Rationale**:

- Flexbox wraps badges naturally if space constrained
- Conditional rendering (`{#if}`) is simple and explicit
- No empty elements in DOM for missing data
- Consistent spacing with `gap-2` utility

**Pattern**:

```svelte
<div class="mt-2 flex flex-wrap gap-2">
  {#if model.scale}
    <Badge variant="secondary">{model.scale}</Badge>
  {/if}

  {#if model.powerMethod}
    <Badge variant="secondary">{model.powerMethod}</Badge>
  {/if}

  {#if model.purchaseDate}
    <Badge variant="secondary">
      {m.components_purchaseDate()}: {formatDate(model.purchaseDate)}
    </Badge>
  {/if}
</div>
```

**Alternatives considered**:

- Always render with `hidden` class - creates empty DOM nodes
- Grid layout - less flexible for varying badge counts
- Single "combined" badge - loses granularity

### 7. Overlay Badge Positioning (Unit Count, Digital Features)

**Decision**: Use absolute positioning with Tailwind utilities inside relative container

**Rationale**:

- Absolute positioning keeps overlays on top of thumbnail without layout shift
- `relative` parent establishes positioning context
- Tailwind utilities (`top-2`, `right-2`, `bottom-2`, `left-2`) provide consistent spacing
- z-index ensures overlays stay above image

**Pattern**:

```svelte
<div class="relative aspect-video">
  <!-- Image/placeholder -->
  <img ... />

  <!-- Digital features (top-left) -->
  {#if model.digitalFeatures}
    <div class="absolute top-2 left-2 z-10 flex gap-1">
      {#if model.digitalFeatures.includes('Sound')}
        <div class="rounded-full bg-black/60 p-1">
          <Volume2 class="h-4 w-4 text-white" />
        </div>
      {/if}
      {#if model.digitalFeatures.includes('DCC')}
        <div class="rounded-full bg-black/60 p-1">
          <Zap class="h-4 w-4 text-white" />
        </div>
      {/if}
    </div>
  {/if}

  <!-- Unit count (bottom-right) -->
  {#if model.unitCount && model.unitCount > 1}
    <div
      class="absolute right-2 bottom-2 z-10 rounded-full bg-black/60 px-2 py-1 text-xs font-medium text-white"
    >
      ×{model.unitCount}
    </div>
  {/if}
</div>
```

**Alternatives considered**:

- CSS Grid with overlapping areas - more complex, less intuitive
- SVG overlay elements - overkill, harder to style
- Separate overlay container - requires careful sizing coordination

### 8. Delete Button with Confirmation Dialog

**Decision**: Use shadcn-svelte `AlertDialog` component triggered by Button click

**Rationale**:

- AlertDialog is purpose-built for confirmation flows
- Accessible by default (focus trap, keyboard navigation, ARIA)
- Prevents accidental deletions with explicit confirm/cancel actions
- Consistent with application's design system

**Pattern**:

```svelte
<script>
  import { AlertDialog } from '$lib/components/ui/alert-dialog';
  import { Button } from '$lib/components/ui/button';
  import { Trash2 } from 'lucide-svelte';

  let showDeleteDialog = $state(false);

  function handleDeleteConfirm() {
    onDelete?.(model.id);
    showDeleteDialog = false;
  }
</script>

<AlertDialog.Root bind:open={showDeleteDialog}>
  <AlertDialog.Trigger asChild let:builder>
    <Button builders={[builder]} variant="ghost" size="icon">
      <Trash2 class="h-4 w-4" />
    </Button>
  </AlertDialog.Trigger>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>{m.components_deleteConfirmTitle()}</AlertDialog.Title>
      <AlertDialog.Description>
        {m.components_deleteConfirmMessage({ model: model.series })}
      </AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>{m.common_cancel()}</AlertDialog.Cancel>
      <AlertDialog.Action on:click={handleDeleteConfirm}>
        {m.common_delete()}
      </AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
```

**Alternatives considered**:

- Browser `confirm()` dialog - not customizable, poor UX
- Custom modal component - reinventing the wheel
- Inline confirmation UI - clutters card layout

## Summary

All technical decisions are resolved with established patterns from the project's tech stack:

- Svelte 5 runes for reactivity
- Tailwind CSS container queries for responsive layout
- shadcn-svelte components for UI primitives
- lucide-svelte for icons
- Paraglide for i18n

No external dependencies or novel patterns are required. Implementation can proceed to Phase 1 (Design & Contracts).

---

## Addendum: Codebase Pattern Verification (2026-02-12)

### Actual Patterns Found in Codebase

**Card Styling** (from StatsCard.svelte and Dashboard):

```svelte
<!-- Stats/Summary cards -->
<div class="card gauge-frame space-y-3 p-5 ring-1 ring-border/40">

<!-- Interactive cards -->
<button class="group card hover:ring-primary-500 w-full overflow-hidden
               transition-all duration-200 hover:scale-[1.02] hover:ring-2
               active:scale-[0.98]">
```

**Responsive Grid** (from Dashboard +page.svelte):

```svelte
<div class="grid grid-cols-2 gap-4 lg:grid-cols-3">
```

**Note**: Dashboard uses standard Tailwind breakpoints (`lg:`, `sm:`), not container queries. Container query approach from original research is valid but not currently used in the codebase.

**Recommendation**: Use standard Tailwind breakpoints to match existing patterns.

**Image Handling** (from SmartImage.svelte):

- Project has `SmartImage.svelte` component with:
  - Async image path resolution via `$effect()`
  - `placeholder` and `error` snippet slots
  - Integration with Tauri's `convertFileSrc()`
  - Loading states with repeating-linear-gradient background

**Recommendation**: Reuse SmartImage component or follow its pattern for consistency.

**Blueprint Placeholder** (from Dashboard):

```svelte
<div
  class="blueprint-panel text-surface-200 flex h-full w-full
            flex-col items-center justify-center gap-2 rounded-lg text-center"
>
  <span class="text-xs font-semibold tracking-[0.3em] uppercase">BLUEPRINT</span>
</div>
```

**Testing Patterns** (from RollingStockCard.test.ts):

- Mock `$lib/paraglide/messages.js` with vi.mock
- Use @testing-library/svelte with render, screen, fireEvent
- Test structure: Rendering, User Interactions, Accessibility, Edge Cases
- Coverage target: 60%+ for UI components

**Type Bindings**:

- Types available in `src/lib/bindings.ts` via specta/tauri-specta
- `RailwayModel`, `CategoryType`, `PowerMethod`, `ScaleType` types confirmed

**Verified Patterns**:
✅ Svelte 5 runes ($props, $state, $derived, $effect) - Confirmed in existing components
✅ lucide-svelte icons - Confirmed (Train, Zap, Volume2, Trash2, etc.)
✅ shadcn-svelte Badge, Button, AlertDialog - Confirmed
✅ Paraglide i18n with `* as m from '$lib/paraglide/messages.js'` - Confirmed

**Patterns to Adjust**:
⚠️ Responsive layout: Use standard breakpoints (`sm:`, `lg:`) instead of container queries
⚠️ Road number truncation: Tooltip component not found in codebase - use simpler approach with $state toggle
⚠️ Card styling: Add `gauge-frame ring-1 ring-border/40` classes to match Dashboard pattern
