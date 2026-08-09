# Collection Page — UX/UI Assessment

> **Scope:** Mobile-first assessment of the existing Collection page (`CollectionDashboard.svelte`) and its component tree.
> **Approach:** Inspect-only — no redesign from scratch; recommendations are grounded in the actual implementation.

---

## 1. Current Implementation Overview

### Structure

The page is built as a single `CollectionDashboard.svelte` component composed of the following layers, top to bottom:

1. **PageHeader** — 3-tier (subtitle / h1 / description) + action buttons (Add Model, Filter toggle on mobile)
2. **StatChip grid** — 6 category counters (locomotives, passenger cars, freight cars, train sets, railcars, EMU)
3. **Lifecycle status chips** — preordered / active / sold (3 clickable quick-filter buttons)
4. **Active filter chips row** — badge + X for each active scale/epoch/category/company/tag filter
5. **View mode switcher** — icon-only grid / table toggle (right-aligned)
6. **Main content** — VirtualGrid (grid mode) or CollectionTableView (table mode), or loading skeleton, or empty state, or no-results state
7. **Mobile filter panel** — full-width `<aside>` rendered **below** the content area when `showFilterSidebar = true`
8. **Desktop filter sidebar** — collapsible right sidebar (280px ↔ 60px icon rail), sticky, full-height
9. **AddCollectionItemDrawer** — bottom drawer for add workflow

### Components

| Component | Role |
|---|---|
| `RailwayModelPreviewCard` | Primary card: image area, metadata grid, overlays |
| `ItemCard.svelte` | **Deprecated** — no longer used on the main page |
| `CollectionTableView` | Responsive `<table>` with thumbnail / brand+model / road number / scale / era / type / status |
| `FilterPanel` | Mobile filter: search + scale (single-select) + tags only |
| `ControlPanel` | Desktop filter sidebar: status + scale (multi-select) + companies + categories + epochs |
| `VirtualGrid` | Custom row virtualizer with ResizeObserver + scroll listener |
| `AddCollectionItemDrawer` | Multi-section add form (model info + rolling stock + purchase) |
| `EmptyState` | Empty collection state with CTA |
| `PageHeader` | 3-tier header, consistent with app |

### State

- `status` filter defaults to `'active'` (sold and preorder items hidden by default, silently)
- All filter state is in-memory; no persistence between sessions
- Collection fetch uses a cache-hit guard (`if (this.#collection !== null) return`), so no re-fetch on back-navigation unless forced

---

## 2. Visual Design Assessment

### Layout

The header section occupies a **disproportionate amount of the upper screen** on mobile. The 3-tier PageHeader + the 6 StatChips (2 columns × 3 rows on mobile) + 3 lifecycle chips pushes the first actual model card below the fold on virtually all small phones. On a standard 390px viewport, the effective first scroll position before seeing a card is approximately 350px+.

Desktop layout is much better: the sticky right sidebar is a sensible pattern, the collapsible icon-rail is a thoughtful touch, and the main content area uses available width well.

### Typography

- Metadata labels (`text-[10px]`) are at the edge of readability on small phones, especially in bright environments.
- Description text (`text-sm` / 14px) is appropriate as the primary identifier.
- The manufacturer/product code row uses `text-xs font-semibold text-zinc-200` / `font-mono text-xs text-zinc-500` — the contrast differential is appropriate.
- `line-clamp-2` on description is correct.

### Color

The amber/primary accent is used consistently and sparingly: "Add Railway Model" button, active filter badges, card hover border, sidebar active pills, power method badge. Appropriate restraint.

The lifecycle status chips (preordered = amber, active = emerald, sold = rose) use color effectively to communicate meaning. However, they do not visually communicate that they are **interactive filters**, not just informational counters.

The `isSold` grayscale treatment (`opacity-70 grayscale`) is well-executed.

### Icons

- Lucide icons are used consistently throughout.
- The delete icon in `PreviewCardActions` is `h-3.5 w-3.5` (≈14px) inside a `h-7 w-7` (28px) button — borderline acceptable.
- View-mode switcher icons are 14px inside ~28px buttons — at the low end.
- Icons are semantically meaningful and not decorative clutter.

---

## 3. Collection Item / Card Assessment

### Current card information hierarchy

**Top section:**
- Manufacturer + product code (zinc-200/zinc-500, xs)
- Description / series (h3, line-clamp-2, prominent) — **primary identifier**
- Delete button (hover-only)

**Image area (aspect-video):**
- Photo or category icon placeholder on technical grid background
- DCC overlay (top-left) · Sound overlay (top-left) · Power method badge (top-right)
- Unit count badge (bottom-left) · Condition badge (bottom-right)

**Metadata grid below image:**
- Category / Scale / Era (3-column, center-aligned)
- Purchase date + Price (2-column, conditional)

### Strengths

- Blueprint technical grid background gives the card an inventory feel, not an e-commerce feel.
- DCC/Sound overlays deliver high-value collector information with minimal visual noise.
- `isSold` grayscale is effective differentiation.
- Hover amber border glow is a subtle and appropriate affordance.

### Weaknesses

- **Railway company is absent.** For a model railway collector, the railway company (DB, FS, SNCF) is often the primary browsing axis. Its omission is the single most significant identification gap on the card.
- **Center-aligned metadata** in the 3-column row creates an awkward visual rhythm. Left-aligned would be more scannable.
- **Purchase date is collector-noise** on the browse view. It is financial/temporal context, not identification context.
- **Price on the card borders e-commerce.** Price is financially relevant but not identification-relevant. It competes for space with more useful fields.
- On mobile (`< sm:`), all three metadata columns stack as full-width rows — cards become very tall.

The deprecated `ItemCard.svelte` still exists in the codebase and should be removed.

### Table view

Better than expected: thumbnails included, road number as a dedicated column is a strong collector choice, responsive column hiding is sensible.

**Issue:** The "STATUS" column header is misleading — it actually shows power method (AC/DC/DCC), not lifecycle status. Only when `isSold` is true does a lifecycle badge appear.

---

## 4. Mobile UX Assessment

### Critical: Filter Panel Position

`showFilterSidebar` defaults to `true`. On mobile, the filter `<aside>` renders **below the entire main content area**, not as an overlay. When the filter is open (which it is by default), a user must scroll past all their collection cards to reach the filter options.

This is the most significant mobile UX problem on the page.

### Filter Toggle Button Placement

The filter toggle (`md:hidden`) is in the page header. After scrolling into the collection, there is no persistent way to open the filter without scrolling back to the top. For a large collection this is a real usability problem.

### View Mode Switcher Touch Target

The grid/table toggle buttons have ~28px hit area — below the recommended 44px minimum.

### Add Model Button

The "Add Railway Model" button is in the header. Once the user scrolls down, the primary action is no longer accessible. There is no FAB or persistent CTA in the scrolled state.

### One-Handed Use

All interactive controls (filter toggle, add button, view switcher) are at the top of the page. For large phones, reaching the top one-handed while browsing cards is physically awkward.

---

## 5. Information Architecture

### Clear at a glance

- This is the user's collection
- Per-category counts (StatChips)
- Preordered / active / sold counts (lifecycle chips)

### Not clear

- **Total item count** — no single "You have 47 models" indicator.
- **Current filter state when scrolled** — no persistent filter indicator on mobile.
- **That lifecycle chips are filters**, not just counters.
- **That the collection defaults to showing only "active" items** — preorders and sold items are silently hidden until the user discovers this.

---

## 6. Search, Filtering, and Sorting

### Filter asymmetry between mobile and desktop

| Filter dimension | Mobile (FilterPanel) | Desktop (ControlPanel) |
|---|---|---|
| Search | ✅ | ❌ |
| Status | ❌ | ✅ |
| Scale | ✅ single-select | ✅ multi-select |
| Tags | ✅ | ❌ |
| Companies | ❌ | ✅ |
| Categories | ❌ | ✅ |
| Epochs | ❌ | ✅ |

Mobile users cannot filter by railway company, category, or epoch. Desktop users have no search. The same filter dimension (scale) has different interaction models on each breakpoint.

### Sorting

No sorting is implemented. A simple "sort by" control (date added, manufacturer, era) would meaningfully improve large-collection browsability.

### Filter persistence

Filters are not persisted between sessions. Each app launch resets to `status: 'active'`.

### Clearing filters

`clearFilters()` resets `status` back to `'active'`. Clicking "Clear all filters" while on the "Sold" view silently returns to the active view with no visual acknowledgement.

---

## 7. List vs Grid

### Grid mode (default)

On a standard phone viewport (390px), the grid produces effectively **1 column**. Cards are tall (aspect-ratio image + metadata rows). Scrolling through a 1-column grid of tall cards is slow for finding a specific item.

The grid is the correct default for visual browsing and recognition — the image area is the key advantage.

### Table mode

More scan-friendly for large collections. More items per screen height. Road number as a dedicated column is good. Responsive column hiding is well-implemented.

Road number shows only `rollingStocks[0]?.roadNumber` — multi-unit sets are not fully represented.

**Grid is the correct default.** Table should be an equally accessible secondary mode.

---

## 8. Interaction Design

### Card tap → detail page

`goto(\`/collection/${item.id.split(':').pop()}\`)` — clean. Minor coupling to the backend ID format.

### Delete from card (grid mode)

Flow: hover → icon appears → AlertDialog → confirm → delete.

**On mobile, hover does not exist.** The delete icon is permanently invisible. Mobile users cannot delete from the grid without navigating to the detail page first.

### Lifecycle chips

Chips are clickable but have no selected/active visual state after clicking. The only feedback is that collection content changes. Combined with the `'active'` default, this creates an invisible filter.

---

## 9. States

### Normal state
Works well.

### Empty state
`EmptyState` component is appropriate. Communicates what happened, why, and what to do.

### No-results state
`NoResults` snippet: X icon + "No items match these filters" + "Clear all filters" button. Adequate but does not explain which filters are active.

### Loading state
6 `animate-pulse rounded-xl bg-muted h-56` placeholders. The `h-56` (224px) underestimates the actual card height (~340px), causing a visual size discontinuity on load.

### Error state
**No error state is rendered.** When `fetchCollection()` fails, the page shows the empty state ("Add your first model"). This is misleading and alarming for a user with an existing collection.

### Delete / remove
Delete calls `remove_collection_item` (sets `removedDate`, does not destroy data) but the confirmation dialog uses `m.common_delete()` — semantically inconsistent with the actual operation.

---

## 10. Accessibility

### Touch targets

| Element | Approximate size | Meets 44px? |
|---|---|---|
| View mode switcher buttons | ~28px | ❌ |
| PreviewCardActions delete button | 28px (h-7 w-7) | ❌ |
| Active filter chip close buttons | ~22px | ❌ |
| Lifecycle chips | ~32px height | ❌ |
| Filter pills (ControlPanel) | ~24–26px | ❌ |

Most interactive elements in the filter areas and card actions are below the 44px touch target recommendation.

### Screen reader
- Cards use `role="button" tabindex={0}` on a `div` — valid but a native `<button>` would be more appropriate.
- Image alt text is handled correctly (blurred BG: `alt="" aria-hidden="true"`; foreground: descriptive alt).
- Digital feature overlays have `aria-label` — correct.

### Color-only indicators
Lifecycle chips use color to indicate status, but the label text carries the meaning independently — acceptable.

### Semantic structure
`<aside>`, `<section>`, `<h4>`, `<thead>/<tbody>` all used correctly.

---

## 11. Design System Consistency

### FilterPanel vs ControlPanel asymmetry

Two separate filter components for the same feature with different filter dimensions, different scale selection models (single vs. multi-select), and different visual implementations (Badge components vs. raw Tailwind buttons).

### Active filter chip badges

`Badge variant="default"` with an embedded `<button>` inside — semantically mixed; may cause unexpected behavior in some screen readers.

### StatChip vs GaugeStatCard

`StatChip` (inline snippet in CollectionDashboard) and `GaugeStatCard` (used in `CollectionSummary.svelte`) both display label + count statistics with completely different visual treatments. `CollectionSummary.svelte` appears unused on the collection page.

### Inline styles vs Tailwind

The desktop sidebar uses inline `style="width: ..."` and `style="opacity: ..."` for CSS transitions (necessary for dynamic JavaScript values) — creates a minor pattern inconsistency but is technically justified.

---

## 12. Performance Considerations

### VirtualGrid

Correct solution for large collections. Renders only visible rows (+ 3 overscan rows). DOM node count stays bounded regardless of collection size.

**Potential issue:** `itemHeight={340}` is an estimate. If actual rendered height differs (e.g., when price row is absent), there could be scroll position jumping. The estimate should match the most common card height.

### Image loading

Blob URL caching prevents redundant IPC calls on virtual scroll remounts — well-optimized.

**Potential issue:** No concurrency limit on image fetch `$effect`. For 2,000+ models, rapid scrolling could generate a large spike of concurrent IPC requests.

### Filtering

Client-side O(n) filter on every state change. Acceptable for 2,000 items; may become perceptible beyond that scale.

### Collection fetch

Every mutation (add, delete) calls `forceRefresh()` — a full re-fetch of the entire collection. For large collections, delta updates would be more efficient.

---

## 13. Model Railway Collector Perspective

The card currently exposes:

| Field | On card | Notes |
|---|---|---|
| Manufacturer | ✅ | |
| Product code | ✅ | |
| Description | ✅ | Primary identifier |
| Scale | ✅ | |
| Category | ✅ (generic) | |
| Era | ✅ | |
| Power method | ✅ (badge overlay) | |
| DCC | ✅ (icon overlay) | |
| Sound | ✅ (icon overlay) | |
| Condition | ✅ (badge overlay) | |
| Purchase date | ✅ | Should be on detail page only |
| Price | ✅ | Should be on detail page only |
| **Railway company** | ❌ | Primary collector browsing axis — missing |
| **Road number** | ❌ | Key identification field — missing |

For a serious collector, the primary browsing mental model is "show me my DB locomotives" or "is this the BR 01.10 or the BR 01?" The current card cannot support this without navigating to the detail page.

---

## 14. Visual Hierarchy Assessment

### Current hierarchy

1. **Primary:** Description (h3, line-clamp-2, medium weight)
2. **Near-primary:** Manufacturer + product code (xs, zinc-200/zinc-500)
3. **Image area:** Photo / placeholder, overlays
4. **Tertiary:** Category / Scale / Era (center-aligned, [10px] labels)
5. **Quaternary:** Purchase date + Price (conditional, bottom)

### Recommended adjustment

1. **Primary:** Description
2. **Secondary:** Manufacturer · Product Code + **Railway Company / Road Number**
3. **Image area:** Unchanged
4. **Tertiary:** Scale / Era / Category
5. **Remove from card:** Purchase date, Price (→ detail page)

---

## 15. Competitive Benchmarking

### Patterns the current implementation gets right

- Active filter chips that can be individually removed (standard: Google Shopping, Apple Files)
- Side panel that collapses to an icon rail (standard: VS Code, Notion)
- VirtualGrid for large lists (standard: all modern list apps)
- Destructive action confirmation dialog (standard: iOS/Android system patterns)

### Established patterns the current implementation misses

- **Mobile filter as bottom sheet** — virtually all modern iOS/Android apps use a bottom sheet or full-screen modal for mobile filter panels, not an inline panel below the content.
- **Persistent FAB for primary action** — Google Keep, Material You apps, iOS apps with a "new" primary action all use a floating CTA that stays accessible while scrolling.
- **Item count in the header** — any inventory or list app shows "47 items" near the list header.

---

## 16. Prioritized Recommendations

### P0 — Critical

#### P0-1: Mobile filter panel appends below content instead of overlaying it

**Problem:** `showFilterSidebar` defaults to `true`. On mobile, the filter `<aside>` renders after the full collection content. Users must scroll past all their cards to reach filters.

**Why it matters:** Filters are effectively unusable on mobile for any collection with more than a few items.

**Recommendation:** Replace the inline `<aside class="md:hidden">` with a bottom-sheet drawer (or full-screen overlay). Default `showFilterSidebar` to `false` on mobile. The filter toggle already exists — it just needs to open an overlay, not append to the DOM.

**Expected impact:** Filters become usable on mobile.

**Complexity:** Medium

---

#### P0-2: Search is absent from the desktop sidebar (ControlPanel)

**Problem:** `FilterPanel` (mobile only) has the search input. `ControlPanel` (desktop) does not. Desktop users cannot search their collection.

**Why it matters:** Search is typically the fastest path to a specific item. Its absence on desktop is a functional regression.

**Recommendation:** Add the search input to `ControlPanel` as the first filter section. Consider merging `FilterPanel` and `ControlPanel` into a single `CollectionFilterPanel` component.

**Expected impact:** Desktop users gain search. Filter behavior becomes consistent across breakpoints.

**Complexity:** Low–Medium

---

#### P0-3: No error state when collection fetch fails

**Problem:** When `fetchCollection()` fails, the UI displays the empty state ("Add your first model") with no indication of error.

**Why it matters:** A user with a large collection encountering a database error will believe their collection has been lost.

**Recommendation:** Add an explicit error state, tracked via a `#hasError` field in `CollectionState`. Show a clear error message with a retry button, separate from the empty state.

**Expected impact:** Errors become transparent and recoverable.

**Complexity:** Low

---

### P1 — High Priority

#### P1-1: Mobile and desktop filter panels expose different filter dimensions

**Problem:** Mobile users cannot filter by railway company, category, or epoch. Desktop users cannot search.

**Why it matters:** Mobile users have a fundamentally inferior filtering experience.

**Recommendation:** Both panels should expose all filter dimensions. The mobile bottom-sheet (per P0-1) has vertical space for the additional sections.

**Expected impact:** Full filtering capability on mobile. Feature parity across breakpoints.

**Complexity:** Medium

---

#### P1-2: Delete is inaccessible on mobile (grid view)

**Problem:** `PreviewCardActions` delete button uses `opacity-0 group-hover:opacity-100`. Hover does not exist on touch devices.

**Why it matters:** Deleting items is a core collection management operation. Mobile users must navigate to the detail page for every delete.

**Recommendation:** On touch devices, always show the delete button at reduced opacity, or add a swipe-to-reveal action in table mode, or show it on long press.

**Expected impact:** Mobile users can manage their collection from the grid.

**Complexity:** Low–Medium

---

#### P1-3: Lifecycle chips do not show which one is active

**Problem:** The three lifecycle chips function as filter buttons but have no selected/active visual state. The `status: 'active'` default is silent.

**Why it matters:** Users cannot tell which status filter is currently active. They may browse their collection not realizing sold or preordered items are hidden.

**Recommendation:** Add an active indicator to the currently selected chip. Add a subtle "Showing: Active Items" label or indicator beneath the chips.

**Expected impact:** Filter state becomes legible. Silent defaults become visible.

**Complexity:** Low

---

#### P1-4: Railway company absent from the collection card

**Problem:** The card does not show the railway company (DB, FS, SNCF, SBB). This is the primary identification axis for most serious collectors.

**Why it matters:** A collector thinks "my FS locomotives" first. The card shows scale and category but not company, making the most differentiating field invisible.

**Recommendation:** Add railway company below the description (or replacing purchase date in the metadata grid). Use `item.rollingStocks[0]?.railwayCompanyName`.

**Expected impact:** Cards become significantly more scannable for identification.

**Complexity:** Low (data is already available in `CollectionItemView`)

---

#### P1-5: No total item count visible on the page

**Problem:** There is no "47 models" indicator. The user must mentally add up the StatChips.

**Why it matters:** A collection management app should immediately communicate collection size. Also useful when filters are active: "Showing 12 of 47."

**Recommendation:** Add `{filteredItems.length} models` (or `{filteredItems.length} of {rawItems.length}`) near the view mode switcher.

**Expected impact:** Immediate orientation. Active filter feedback.

**Complexity:** Low

---

### P2 — Medium Priority

#### P2-1: Add Model button scrolls off screen — no persistent primary action on mobile

**Recommendation:** Add a floating action button (FAB) anchored `fixed bottom-6 right-6` that appears after scrolling past the page header.

**Complexity:** Low

---

#### P2-2: View mode switcher touch targets are too small (~28px)

**Recommendation:** Increase padding to `p-2` or `p-2.5` (≥36px per button, ideally 44px).

**Complexity:** Trivial

---

#### P2-3: StatChip grid displaces collection content below the fold on mobile

**Recommendation:** On mobile, collapse the 6-chip grid to a single horizontally-scrollable summary row, or limit to the top 3 categories.

**Complexity:** Low–Medium

---

#### P2-4: `clearFilters()` silently resets status to 'active'

**Recommendation:** Preserve the current `status` when clearing dimension filters (scales, companies, categories, epochs, tags, query). Or provide a separate "clear dimension filters" action.

**Complexity:** Low

---

#### P2-5: Table view "STATUS" column header is misleading

**Recommendation:** Rename the column to "POWER" or "SYSTEM." It shows power method (AC/DC/DCC), not lifecycle status.

**Complexity:** Trivial

---

#### P2-6: Replace purchase date with road number on the card

**Recommendation:** Replace the purchase date metadata slot with `item.rollingStocks[0]?.roadNumber`. Purchase date belongs on the detail page only.

**Complexity:** Low

---

### P3 — Optional

#### P3-1: Active filter close buttons have small touch targets (~22px)

Increase to `size={16}` with `p-1` padding (≈32px).

**Complexity:** Trivial

---

#### P3-2: Remove deprecated `ItemCard.svelte`

Carries a `@deprecated` JSDoc, is no longer used on the main page, adds maintenance noise.

**Complexity:** Trivial

---

#### P3-3: Add a sort control

A minimal "Sort by: Date Added / Manufacturer / Era" selector would improve large-collection browsability without significant complexity.

**Complexity:** Medium

---

#### P3-4: Image loading concurrency limit

For 500+ model collections, add a semaphore or request queue to the image loading `$effect` to cap concurrent IPC calls (e.g., max 8 concurrent).

**Complexity:** Medium

---

#### P3-5: Persist filter state across sessions

Store the last-used filter configuration in Tauri's local storage so returning users see their preferred view immediately.

**Complexity:** Low

---

## 17. Preserve What Works

### Keep

- **VirtualGrid virtualization** — correct solution for large collections; well-implemented with ResizeObserver and scroll-parent detection.
- **Image blob URL caching** — smart optimization that prevents redundant IPC calls on virtual scroll remounts.
- **Blueprint/technical grid aesthetic on cards** — distinctly not e-commerce; feels correct for a collector's tool.
- **DCC/Sound overlays on the image area** — high-value information delivered with minimal visual noise.
- **`isSold` grayscale treatment** — effective visual differentiation without explicit text labels.
- **AlertDialog for delete confirmation** — prevents accidental deletion.
- **Desktop collapsible sidebar with icon rail** — a polished, practical pattern for advanced filtering.
- **Active filter chip row with individual remove buttons** — correct pattern for multi-dimension filter management.
- **Amber primary accent used sparingly** — strong visual identity without visual noise.
- **Table view** — well-structured; responsive column hiding is appropriate; thumbnail inclusion is good.
- **PageHeader 3-tier structure** — consistent with the rest of the app.
- **Paraglide i18n** — all user-facing strings are externalized correctly.

### Improve

- `FilterPanel` and `ControlPanel` — merge into a single component with full filter coverage on both breakpoints.
- Lifecycle chips — add active/selected visual state.
- Card metadata row — replace purchase date with road number; add railway company.
- StatChip grid — collapse on mobile to prevent content displacement.
- Loading skeleton height — match actual card height (340px, not 224px).

### Replace

- Mobile filter panel (inline below content) — replace with bottom sheet overlay.

### Remove

- `ItemCard.svelte` — deprecated, unused, adds maintenance noise.
- Purchase date from card — tertiary financial information; belongs on the detail page.
- Price from card *(optional)* — reinforces an e-commerce feeling in a collector's inventory.

---

## 18. Final Assessment

| Dimension | Score |
|---|---|
| Overall UX | **5.5 / 10** |
| Visual Design | **7.5 / 10** |
| Mobile Usability | **4.0 / 10** |
| Information Architecture | **5.5 / 10** |
| Accessibility | **5.0 / 10** |
| Performance / Scalability | **7.0 / 10** |

The visual design and desktop experience are genuinely strong — the blueprint aesthetic is coherent, the dark theme is well-executed, and the desktop sidebar with its icon-rail collapse is a thoughtful detail. The mobile experience has structural problems (filter panel position, missing FAB, hover-only delete) that significantly undercut the visual quality.

### Top 5 Improvements

1. **Replace the inline mobile filter panel with a bottom-sheet overlay** — fixes the most damaging UX problem.
2. **Add search to the desktop ControlPanel and unify filter coverage across both panels** — closes the most surprising functional gap.
3. **Add railway company to the collection card** — highest-value identification change for a collector; data already available.
4. **Add visible active/selected state to lifecycle status chips** — makes the silent default filter visible.
5. **Add a total item count indicator near the grid** — one line of code; high information value.

### Recommended Implementation Order

1. P0-1 — Mobile filter overlay (unblocks the entire mobile experience)
2. P0-2 — Add search to ControlPanel (closes the most surprising functional gap)
3. P1-3 — Lifecycle chip active state (low effort, high legibility improvement)
4. P1-4 — Railway company on card (immediately improves collection scanability)
5. P1-5 — Item count (trivial; ship together with P1-3 or P1-4)
6. P0-3 — Error state (must not ship without this)

### What I Would Not Change

- VirtualGrid implementation
- Card blueprint/grid aesthetic
- DCC/Sound/power method overlay system
- AlertDialog for delete confirmation
- Desktop sidebar collapsible behavior and icon-rail collapse
- Paraglide i18n integration
- Amber primary accent palette
- `isSold` grayscale visual treatment
- Add-model drawer architecture
