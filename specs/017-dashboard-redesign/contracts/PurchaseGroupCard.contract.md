# Component Contract: PurchaseGroupCard

**Component**: `PurchaseGroupCard.svelte`  
**Location**: `src/lib/features/dashboard/components/PurchaseGroupCard.svelte`  
**Purpose**: Display a single purchase group container with metadata header and horizontal model cards grid

**Design Pattern**: Industrial-luxe dark theme with glassmorphism effects and contextual purchase header

## Props Interface

```typescript
interface PurchaseGroupCardProps {
  /** The purchase group data to display */
  group: PurchaseGroup;
}
```

## Component Responsibilities

### 1. Display Purchase Metadata Header

**What**: Show contextual information about the purchase event with "Rusty Shed" styling

**Visual Design**:

- Left side: Date (📅) + Seller (🏪) with icons
- Right side: User notes in italic quotation marks
- Border-bottom separator (`border-white/5`)

**How**:

```svelte
<div class="mb-4 flex items-center justify-between border-b border-white/5 pb-2">
  <div class="flex items-center gap-4 text-sm text-zinc-400">
    <span class="flex items-center gap-1">
      📅 {formatDate(group.purchaseDate)}
    </span>
    {#if group.sellerName}
      <span class="flex items-center gap-1">
        🏪 {group.sellerName}
      </span>
    {:else}
      <span class="flex items-center gap-1">
        🏪 {m.dashboard_seller_unknown()}
      </span>
    {/if}
  </div>
  {#if group.notes}
    <span class="text-xs text-zinc-500 italic">"{group.notes}"</span>
  {/if}
</div>
```

**Color Palette**:

- Date/Seller text: `text-zinc-400` (secondary)
- Notes: `text-zinc-500` (muted, italic)
- Separator: `border-white/5` (subtle)

**Date Formatting**: Human-readable (e.g., "January 15, 2026") via `date-fns` or similar

---

### 2. Render Horizontal Model Cards Grid

**What**: Display up to 3 model cards in responsive grid with horizontal card layout

**Grid System**:

```html
grid grid-cols-1 md:grid-cols-3 gap-4
<!-- Mobile: 1 column stacked -->
<!-- Desktop: 3 columns side-by-side -->
```

**How**:

```svelte
<div class="grid grid-cols-1 gap-4 md:grid-cols-3">
  {#each group.modelCards as model (model.id)}
    <ModelCard {model} />
  {/each}
</div>
```

**Constraints**:

- Maximum 3 cards displayed (enforced by backend)
- Each card uses horizontal layout (see ModelCard.contract.md)
- Gap between cards: 1rem (gap-4)

---

### 3. Show "More Items" Indicator

**What**: Display "+N more models..." message when purchase has more than 3 models

**Styling**: Centered, italic, muted color

**How**:

```svelte
{#if group.totalCount > group.modelCards.length}
  <div class="flex items-center justify-center text-sm text-zinc-500 italic">
    +{group.totalCount - group.modelCards.length} more models...
  </div>
{/if}
```

**Example Output**: "+2 more models..."

---

## Visual Hierarchy (Industrial-Luxe Theme)

```
┌─ PurchaseGroupCard ──────────────────────────────────────────────┐
│ rounded-lg border border-white/10 bg-black/20 p-4                │
│                                                                   │
│  ┌─ Header ─────────────────────────────────────────────────┐   │
│  │ 📅 January 15, 2026  🏪 Milan Model Trains              │   │
│  │                           "Birthday gift from Maria"     │   │
│  └──────────────────────────────────────────────────────────┘   │
│     ↑ border-b border-white/5 pb-2                              │
│                                                                   │
│  ┌─ Grid (grid-cols-1 md:grid-cols-3 gap-4) ───────────────┐   │
│  │ [ModelCard] [ModelCard] [ModelCard]                      │   │
│  │  Horizontal  Horizontal  Horizontal                      │   │
│  │   (16:9)      (16:9)      (16:9)                         │   │
│  └───────────────────────────────────────────────────────────┘   │
│                                                                   │
│  +2 more models... (text-zinc-500, italic, centered)             │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

## Styling Requirements (Industrial-Luxe Theme)

**Container** (Purchase Group):

```css
rounded-lg border border-white/10 bg-black/20 p-4
```

**Optional Glassmorphism Enhancement**:

```css
bg-white/5 backdrop-blur-md
/* Creates frosted glass effect for depth */
```

**Header Separator**:

```css
border-b border-white/5 pb-2
/* Subtle divider between metadata and cards */
```

**Text Colors** (aligned with "Rusty Shed" palette):

- Primary: `text-white` (headings)
- Secondary: `text-zinc-400` (date, seller)
- Muted: `text-zinc-500` (notes, "+N more")
- Accent: `text-orange-400` or `text-orange-500` (links, manufacturer names in child cards)

---

## Accessibility

- **Semantic HTML**: Use `<article>` for purchase group, `<header>` for metadata
- **ARIA Labels**: None required (content is self-explanatory via emojis + text)
- **Keyboard Navigation**: Inherits from child ModelCard components (button elements)

## Testing Checklist

- [ ] Renders with valid purchase group data
- [ ] Header displays date with 📅 emoji
- [ ] Header displays seller name with 🏪 emoji
- [ ] Shows "Unknown source" when seller_name is null
- [ ] Shows notes section in italic quotation marks when present
- [ ] Notes section hidden when notes is null
- [ ] Renders 1-3 model cards in horizontal layout
- [ ] Shows "+N more models..." indicator when totalCount > 3
- [ ] Does not show "+N more" when totalCount <= 3
- [ ] Responsive grid: 1 col on mobile, 3 on desktop
- [ ] Formats date using locale (via Paraglide or date-fns)
- [ ] Empty notes don't render extra spacing
- [ ] Container uses border-white/10 and bg-black/20
- [ ] Header separator uses border-white/5

## Dependencies

- `ModelCard.svelte` (child component - horizontal layout)
- `$lib/paraglide/messages.js` (i18n)
- `date-fns` (or similar) for date formatting
- Tailwind CSS for styling (no custom CSS required)

## Component Signature

```svelte
<script lang="ts">
  import type { PurchaseGroup } from '$lib/bindings';
  import ModelCard from './ModelCard.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { format } from 'date-fns';

  interface Props {
    group: PurchaseGroup;
  }

  let { group }: Props = $props();

  const formatDate = (isoDate: string) => {
    return format(new Date(isoDate), 'MMMM d, yyyy');
  };

  const hasMoreItems = $derived(group.totalCount > group.modelCards.length);
  const moreItemsCount = $derived(group.totalCount - group.modelCards.length);
</script>

<article class="rounded-lg border border-white/10 bg-black/20 p-4">
  <!-- Purchase Header -->
  <div class="mb-4 flex items-center justify-between border-b border-white/5 pb-2">
    <div class="flex items-center gap-4 text-sm text-zinc-400">
      <span class="flex items-center gap-1">
        📅 {formatDate(group.purchaseDate)}
      </span>
      {#if group.sellerName}
        <span class="flex items-center gap-1">
          🏪 {group.sellerName}
        </span>
      {:else}
        <span class="flex items-center gap-1">
          🏪 {m.dashboard_seller_unknown()}
        </span>
      {/if}
    </div>
    {#if group.notes}
      <span class="text-xs text-zinc-500 italic">"{group.notes}"</span>
    {/if}
  </div>

  <!-- Model Cards Grid (horizontal cards) -->
  <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
    {#each group.modelCards as model (model.id)}
      <ModelCard {model} />
    {/each}
  </div>

  <!-- More Items Indicator -->
  {#if hasMoreItems}
    <div class="mt-4 flex items-center justify-center text-sm text-zinc-500 italic">
      +{moreItemsCount} more models...
    </div>
  {/if}
</article>
```

## Edge Cases

| Scenario                     | Expected Behavior                                            |
| ---------------------------- | ------------------------------------------------------------ |
| `sellerName` is null         | Display "Unknown source" with 🏪 emoji                       |
| `notes` is null              | Do not render notes span (no extra spacing)                  |
| `totalCount` equals 3        | No "+N more" indicator shown                                 |
| `totalCount` is 1            | Single horizontal model card, no grid issues                 |
| Very long notes (>100 chars) | Allow wrapping (no truncation, notes are valuable context)   |
| Very long seller name        | Allow wrapping (no truncation, seller identity is important) |
| Empty modelCards array       | Should not occur (backend enforces 1-3 cards)                |

## Performance Notes

- No heavy computations required
- Date formatting happens once per group (memoized via `$derived`)
- Grid layout handled by CSS (no JS layout calculations)
- Child ModelCard components handle their own image loading

## UI Polish Recommendations

**Glassmorphism Variant** (optional enhancement):

```svelte
<article class="rounded-lg border border-white/10 bg-white/5 p-4 backdrop-blur-md">
  <!-- Creates frosted glass effect -->
</article>
```

**Interaction Enhancement** (future):

- Add hover effect to entire group container
- Subtle scale or glow on hover
- Click to expand/collapse when more than 3 models

**Empty State** (should not occur, but good practice):

```svelte
{#if group.modelCards.length === 0}
  <div class="py-8 text-center text-zinc-500">No models in this purchase group</div>
{/if}
```
