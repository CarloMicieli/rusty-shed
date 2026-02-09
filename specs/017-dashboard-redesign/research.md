# Research: Dashboard Collector's Overview Redesign

**Feature**: 017-dashboard-redesign  
**Date**: February 9, 2026  
**Purpose**: Resolve all [NEEDS CLARIFICATION] items and document technology choices

## Overview

This document resolves technical unknowns and documents best practices for implementing purchase-grouped model cards on the dashboard. All research findings inform the data model and implementation approach.

---

## R1: Purchase Grouping Strategy

### Decision

Group models by **purchase event** defined as the combination of `purchase_date` and `seller_id` from the `purchase_infos` table.

### Rationale

**Existing Schema Analysis:**

```sql
-- purchase_infos table (from 0002_create_collection_schema.sql)
purchase_date                         TEXT NOT NULL
seller_id                             TEXT
collection_item_id                    TEXT NOT NULL (FK to collection_items)
```

The `purchase_infos` table already tracks:

- Purchase date (required field)
- Seller reference (optional, FK to sellers table)
- One-to-one relationship with collection_items

**Grouping Logic:**

1. Models with identical `(purchase_date, seller_id)` belong to the same purchase event
2. If `seller_id` is NULL, treat each NULL as a separate event per date (prevents merging unrelated purchases without seller info)
3. Sort groups by `purchase_date DESC` to show most recent first
4. Limit to top 2-3 groups for consistent dashboard height

### Alternatives Considered

- **Group by added_date only**: Rejected because `collection_items.added_date` doesn't capture the actual purchase event relationship. A user could add multiple models on the same day from different sources.
- **Create new purchase_events table**: Rejected because existing schema already supports the relationship implicitly through matching purchase_date + seller_id combinations.
- **Group by transaction_id**: Rejected because no transaction ID exists in current schema, would require migration.

### Implementation Notes

```rust
// Pseudo-SQL for grouping query
SELECT
  pi.purchase_date,
  pi.seller_id,
  s.name as seller_name,
  COUNT(*) as model_count,
  -- Aggregate model data here
FROM purchase_infos pi
LEFT JOIN sellers s ON pi.seller_id = s.id
LEFT JOIN collection_items ci ON pi.collection_item_id = ci.id
LEFT JOIN railway_models rm ON ci.railway_model_id = rm.id
WHERE ci.removed_date IS NULL  -- Only active collection items
GROUP BY pi.purchase_date, pi.seller_id
ORDER BY pi.purchase_date DESC
LIMIT 3
```

**Edge Cases Handled:**

- Models without purchase_info: Fallback to `collection_items.added_date` with NULL seller
- Models with same date but different sellers: Separate groups
- Models with NULL seller_id: Each NULL treated independently per date

---

## R2: Model Card Data Requirements

### Decision

Each model card requires 6 core data points fetched in a single enriched query:

1. **Thumbnail image path** (from railway_models or placeholder)
2. **Manufacturer name** (from manufacturers table)
3. **Product code** (from railway_models)
4. **Condition status** (from collection_items.purchase_condition)
5. **Model description** (railway_models.description or auto-generated title)
6. **Model ID** (for navigation link)

### Rationale

**Data Location Analysis:**

```sql
-- Required JOIN chain
railway_models (id, manufacturer_id, product_code, description, image_path)
  -> manufacturers (id, name)
  -> collection_items (railway_model_id, purchase_condition, model_condition)
  -> purchase_infos (collection_item_id, purchase_date, seller_id)
```

**Query Optimization:**

- Single query with LEFT JOINs prevents N+1 problem
- All data available in existing tables, no schema changes needed
- Image path stored in railway_models; use existing image loading infrastructure

### Alternatives Considered

- **Separate queries per model**: Rejected due to N+1 query problem (would require 10+ queries for 2-3 purchase groups)
- **Cache layer for model cards**: Rejected as premature optimization; grouping already limits results to ~10 models max
- **Pre-computed denormalized view**: Rejected because data freshness requirements (user expects immediate updates after adding models)

### Implementation Notes

```rust
// Rust domain entity structure
pub struct ModelCard {
    pub id: RailwayModelId,
    pub thumbnail_path: Option<String>,
    pub manufacturer: String,
    pub product_code: String,
    pub condition: PurchaseCondition,  // Enum: New, PreOwned, Unknown
    pub description: String,  // Truncate in UI layer to 50 chars
}
```

**Condition Mapping:**

- `purchase_condition` = "NEW" → Badge: "New" (green)
- `purchase_condition` = "PRE_OWNED" → Badge: "Pre-owned" (blue)
- `purchase_condition` = NULL → Badge: "Unknown" (gray)

---

## R3: Image Loading & Placeholder Strategy

### Decision

Reuse existing image loading infrastructure from Feature 014 (Railway Model Details Page) and Feature 015 (Model Image Upload).

### Rationale

**Existing System:**

- Tauri command: `get_railway_model_image(model_id)` returns `RailwayModelImageResponse`
- Images stored at: `{data_dir}/images/{model_id}.{ext}`
- Frontend loads via Tauri asset protocol

**For Dashboard:**

- Query returns `image_path` from `railway_models` table
- If NULL or file not found, display placeholder
- Placeholder options:
  1. Generic locomotive icon (existing in assets)
  2. Manufacturer logo (if available)
  3. First letter of manufacturer in colored circle

### Alternatives Considered

- **Preload all thumbnails**: Rejected due to unnecessary memory usage; lazy loading with IntersectionObserver is sufficient
- **Generate thumbnails on backend**: Rejected because existing full images already optimized for display
- **WebP conversion**: Deferred to separate performance optimization feature

### Implementation Notes

```svelte
<!-- ModelCard.svelte component pattern -->
<script>
  let imageSrc = $state<string | null>(null);
  let imageError = $state(false);

  async function loadImage(modelId: RailwayModelId) {
    try {
      const result = await commands.getRailwayModelImage(modelId);
      if (result.status === 'ok') {
        imageSrc = convertFileSrc(result.data.path);
      } else {
        imageError = true;
      }
    } catch {
      imageError = true;
    }
  }
</script>

{#if imageError}
  <div class="placeholder"><!-- Fallback icon --></div>
{:else if imageSrc}
  <img src={imageSrc} alt={model.description} />
{:else}
  <div class="skeleton"><!-- Loading state --></div>
{/if}
```

---

## R4: Responsive Grid Layout Pattern

### Decision

Use CSS Grid with auto-fit and minmax for responsive model cards within each purchase group.

### Rationale

**shadcn-svelte + Tailwind CSS 4 Capabilities:**

```css
/* Tailwind utilities for responsive grid */
grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4
```

**Breakpoint Strategy:**

- **Mobile (320px-640px)**: Single column, full-width cards
- **Tablet (640px-1024px)**: 2 columns
- **Desktop (1024px+)**: 3 columns max (per purchase group)

**Card Sizing:**

- Min width: 280px (prevents cramped mobile view)
- Max width: Auto (grid handles distribution)
- Aspect ratio: 16:9 for image container (maintains consistency)

### Alternatives Considered

- **Flexbox with wrap**: Rejected because CSS Grid provides better alignment control for card grids
- **Fixed card widths**: Rejected due to poor responsive behavior across device sizes
- **Masonry layout**: Rejected because consistent card heights improve scannability (FR-012)

### Implementation Notes

```svelte
<!-- PurchaseGroupCard.svelte -->
<div class="purchase-group card space-y-4 p-6">
  <!-- Header: Date, Seller, Notes -->
  <header>...</header>

  <!-- Model Cards Grid -->
  <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
    {#each models.slice(0, 3) as model (model.id)}
      <ModelCard {model} />
    {/each}
  </div>

  <!-- +N more indicator -->
  {#if models.length > 3}
    <p class="text-surface-400 text-sm">
      +{models.length - 3} more items
    </p>
  {/if}
</div>
```

---

## R5: Paraglide i18n Message Keys

### Decision

Add new message keys to `messages/en.json` and `messages/it.json` for all dashboard purchase group strings.

### Rationale

**Constitution Requirement:**

> All user-facing strings MUST be supplied via the Paraglide message system

**Required Message Keys:**

```json
{
  "dashboard_purchase_group_title": "Recent Acquisitions",
  "dashboard_purchase_on": "Purchased on {date}",
  "dashboard_seller_from": "from {seller}",
  "dashboard_seller_unknown": "Unknown source",
  "dashboard_purchase_notes": "Notes: {notes}",
  "dashboard_more_items": "+{count} more items",
  "dashboard_condition_new": "New",
  "dashboard_condition_preowned": "Pre-owned",
  "dashboard_condition_unknown": "Unknown",
  "dashboard_empty_purchases": "No recent acquisitions",
  "dashboard_add_first_purchase": "Add your first model to get started"
}
```

### Alternatives Considered

- **Inline English strings with TODO comments**: Rejected, violates constitution
- **Reuse existing collection messages**: Rejected because context differs (purchase groups vs. individual models)

### Implementation Notes

- Run `pnpm prepare` after adding messages to regenerate Paraglide types
- Use `{placeholder}` syntax for dynamic values (dates, counts, seller names)
- Italian translations required before merge (per constitution standards)

---

## R6: Navigation & Scroll Position Preservation

### Decision

Use SvelteKit's built-in scroll restoration with `afterNavigate` lifecycle hook.

### Rationale

**SvelteKit Built-in Features:**

- `$app/navigation` provides `afterNavigate((navigation) => {...})`
- Browser native `history.scrollRestoration = 'manual'` for fine control
- `goto()` function with `keepFocus` option

**FR-016 Requirement:**

> System MUST maintain user's scroll position when navigating back from model details

**Implementation Pattern:**

```typescript
// In dashboard +page.svelte
import { afterNavigate } from '$app/navigation';
import { onMount } from 'svelte';

let scrollY = $state(0);

onMount(() => {
  const savedScroll = sessionStorage.getItem('dashboard-scroll');
  if (savedScroll) {
    scrollY = parseInt(savedScroll, 10);
    window.scrollTo(0, scrollY);
    sessionStorage.removeItem('dashboard-scroll');
  }
});

afterNavigate(() => {
  // Save scroll position before navigation
  sessionStorage.setItem('dashboard-scroll', window.scrollY.toString());
});
```

### Alternatives Considered

- **Global scroll store**: Rejected as over-engineering; SvelteKit handles this natively for most cases
- **Route-specific scroll snapshots**: Rejected because sessionStorage is sufficient for dashboard → model → dashboard flow
- **Third-party scroll library**: Rejected due to minimal requirements and unnecessary dependency

---

## R7: Performance Optimization Strategy

### Decision

Implement lazy loading for model images using IntersectionObserver, defer non-critical data (depot section).

### Rationale

**Performance Target (SC-007):**

> Page load time for dashboard with 10 purchase groups (30 visible models) remains under 2 seconds

**Optimization Approaches:**

1. **Query Optimization**:
   - Index on `purchase_infos(purchase_date DESC)`
   - LIMIT 3 groups at SQL level (not application level)
   - Fetch only required fields (avoid SELECT \*)

2. **Image Loading**:
   - Lazy load images below fold using IntersectionObserver
   - Load first 3 visible models immediately
   - Defer remaining images until scroll

3. **Component Rendering**:
   - Use Svelte 5 `$derived` for computed values (memoization built-in)
   - Avoid reactive re-renders in stable sections (stats, quick actions)

### Alternatives Considered

- **Virtual scrolling**: Rejected, only 2-3 groups visible (not enough items to justify complexity)
- **Service Worker caching**: Rejected as premature optimization (Tauri handles asset caching)
- **Pagination for purchase groups**: Rejected, spec requires "2-3 most recent" as fixed count

### Implementation Notes

```typescript
// Image lazy loading component
import { onMount } from 'svelte';

let imgElement: HTMLImageElement;
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
```

**Query Performance Checklist:**

- ✅ Index on `purchase_infos.purchase_date`
- ✅ Index on `collection_items.railway_model_id` (already exists)
- ✅ LIMIT clause at SQL level
- ✅ WHERE removed_date IS NULL (active items only)

---

## R8: Visual Design System & UI Patterns

### Decision

Implement "industrial-luxe" dark theme with glassmorphism effects, horizontal model cards, and contextual purchase headers.

### Rationale

**Current Dashboard Aesthetic**: Dark theme with copper/amber accents creates a "Rusty Shed" industrial collector's vibe. The redesign must enhance this existing visual language rather than conflict with it.

**Purchase Header Design**:

- **Left Side**: Date (📅 icon) + Seller Name (🏪 icon) - provides context at a glance
- **Right Side**: User notes in italic (e.g., "Birthday gift from Maria") - adds personal story
- **Separator**: Subtle border-bottom (`border-white/5`) to visually separate metadata from cards

**Model Card Layout** (Horizontal, not Vertical):

- **Aspect Ratio**: 1:1 square thumbnail (not 16:9) - saves vertical space, allows 3 per row
- **Layout**: Horizontal flex (image left, details right) - maximizes information density
- **Badge Position**: Absolute positioned top-right (condition: New/Pre-owned)
- **Text Hierarchy**:
  1. Manufacturer (bold, orange/amber - `text-orange-400`)
  2. Product code (medium weight, white)
  3. Description (small, gray, truncated)

**Container Styling**:

```css
/* Purchase group container */
rounded-lg border border-white/10 bg-black/20 p-4

/* Individual model card */
rounded bg-zinc-900/50 border border-zinc-800 p-2

/* Glassmorphism enhancement (optional) */
bg-white/5 backdrop-blur-md
```

### Alternatives Considered

- **Vertical Cards (16:9 images)**: Rejected - consumes too much vertical space, requires excessive scrolling for 2-3 purchase groups
- **Grid Layout without Grouping**: Rejected - loses purchase context and storytelling aspect
- **Light Theme Variant**: Rejected - conflicts with established "Rusty Shed" dark industrial aesthetic

### Implementation Notes

**Color Palette** (aligned with existing dashboard):

- Primary accent: `text-orange-400` / `text-orange-500` (manufacturer names, links)
- Background layers:
  - Base: `bg-black/20` (purchase container)
  - Card: `bg-zinc-900/50` (model card)
  - Borders: `border-white/10` (container), `border-zinc-800` (cards)
- Text colors:
  - Primary: `text-white` (product codes)
  - Secondary: `text-zinc-400` (descriptions, metadata)
  - Muted: `text-zinc-500` (notes, "+N more")

**Badge Variants** (shadcn-svelte):

```typescript
condition === 'NEW' ? 'default' : 'secondary';
// 'default' = orange/amber for New
// 'secondary' = gray for Pre-owned/Unknown
```

**Empty State Placeholder**:

- Use existing "Blueprint" icon pattern from dashboard
- Maintain visual continuity with empty state sections
- Background: `bg-zinc-800` with centered icon

**Responsive Breakpoints**:

```html
grid grid-cols-1 md:grid-cols-3 gap-4
<!-- Mobile: 1 column stacked -->
<!-- Desktop: 3 columns side-by-side -->
```

### UI Polish Recommendations

1. **Glassmorphism Enhancement**:
   - Apply `bg-white/5 backdrop-blur-md` to purchase containers for depth
   - Creates frosted glass effect that fits industrial-luxe aesthetic

2. **Depot Table Relocation**:
   - Keep depot section on dashboard but maintain existing table format
   - Full "Depot" experience already available via sidebar navigation
   - Dashboard shows preview, sidebar link provides comprehensive view

3. **Typography Consistency**:
   - Section headers: `text-xl font-semibold tracking-tight text-white`
   - "View All" links: `text-xs text-orange-500 hover:underline`
   - Maintains alignment with existing dashboard sections

4. **Interaction States**:
   - Model cards: Add `hover:bg-zinc-800/70 transition-colors cursor-pointer`
   - Focus states: Browser default focus ring for keyboard navigation
   - Active state: `active:scale-[0.98]` for tactile feedback

### Code Example (Svelte 5 Pattern)

```svelte
<section class="mt-8">
  <div class="mb-4 flex items-center justify-between">
    <h2 class="text-xl font-semibold tracking-tight text-white">
      {m.dashboard_purchase_group_title()}
    </h2>
    <a href={resolve('/my-collection')} class="text-xs text-orange-500 hover:underline">
      {m.dashboard_view_all()}
    </a>
  </div>

  <div class="space-y-6">
    {#each purchaseGroups as purchase (purchase.id)}
      <div class="rounded-lg border border-white/10 bg-black/20 p-4">
        <!-- Purchase Header -->
        <div class="mb-4 flex items-center justify-between border-b border-white/5 pb-2">
          <div class="flex items-center gap-4 text-sm text-zinc-400">
            <span class="flex items-center gap-1">
              📅 {formatDate(purchase.purchaseDate)}
            </span>
            {#if purchase.sellerName}
              <span class="flex items-center gap-1">
                🏪 {purchase.sellerName}
              </span>
            {:else}
              <span class="flex items-center gap-1">
                🏪 {m.dashboard_seller_unknown()}
              </span>
            {/if}
          </div>
          {#if purchase.notes}
            <span class="text-xs text-zinc-500 italic">"{purchase.notes}"</span>
          {/if}
        </div>

        <!-- Model Cards Grid -->
        <div class="grid grid-cols-1 gap-4 md:grid-cols-3">
          {#each purchase.modelCards as model (model.id)}
            <button
              type="button"
              onclick={() => goto(resolve(`/models/${model.id}`))}
              class="relative flex cursor-pointer gap-3 rounded border border-zinc-800 bg-zinc-900/50 p-2 text-left transition-colors hover:bg-zinc-800/70"
            >
              <!-- Condition Badge -->
              <div class="absolute top-2 right-2">
                <Badge variant={model.condition === 'NEW' ? 'default' : 'secondary'}>
                  {conditionLabel(model.condition)}
                </Badge>
              </div>

              <!-- Thumbnail (1:1 square) -->
              <div class="h-20 w-20 flex-shrink-0 overflow-hidden rounded bg-zinc-800">
                {#if model.thumbnailPath}
                  <img
                    src={convertFileSrc(model.thumbnailPath)}
                    alt={model.productCode}
                    class="h-full w-full object-cover"
                  />
                {:else}
                  <div class="flex h-full items-center justify-center">
                    <TrainFront size={32} class="text-zinc-600" />
                  </div>
                {/if}
              </div>

              <!-- Model Details -->
              <div class="flex min-w-0 flex-col justify-center">
                <span class="text-xs font-bold text-orange-400 uppercase">
                  {model.manufacturer}
                </span>
                <span class="text-sm font-medium text-white">
                  {model.productCode}
                </span>
                <p class="truncate text-xs text-zinc-400">
                  {model.description}
                </p>
              </div>
            </button>
          {/each}

          {#if purchase.totalCount > 3}
            <div class="flex items-center justify-center text-sm text-zinc-500 italic">
              +{purchase.totalCount - 3} more models...
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>
</section>
```

---

## Summary of Research Findings

| Topic                 | Decision                                            | Impact                                               |
| --------------------- | --------------------------------------------------- | ---------------------------------------------------- |
| **Purchase Grouping** | Use (purchase_date, seller_id) combination          | No migration needed, leverages existing schema       |
| **Model Card Data**   | 6 core fields via single enriched query             | Prevents N+1 queries, optimizes performance          |
| **Image Loading**     | Reuse Feature 014/015 infrastructure + lazy loading | Minimal new code, leverages existing patterns        |
| **Layout**            | CSS Grid with Tailwind responsive utilities         | Native browser support, no JS layout libraries       |
| **i18n**              | Paraglide message keys in messages/\*.json          | Constitution compliant, supports Italian translation |
| **Navigation**        | SvelteKit afterNavigate + sessionStorage            | Native scroll restoration, simple implementation     |
| **Performance**       | Query LIMIT, lazy images, indexed columns           | Meets <2s load target for 30 models                  |
| **Visual Design**     | Industrial-luxe dark theme with horizontal cards    | Aligns with existing "Rusty Shed" aesthetic          |

**No [NEEDS CLARIFICATION] items remain.** All technical decisions documented with rationale and alternatives considered. Ready for Phase 1: Design.
