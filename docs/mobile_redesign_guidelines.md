# Rusty Shed — Mobile Redesign Guidelines

> **Status:** Actionable — Q&A resolved; ready for implementation  
> **Scope:** Mobile-only enhancements (≤ `md` breakpoint, i.e., `< 768 px`) that **must not alter the desktop experience** (`lg:` and above remain untouched).  
> **Stack:** Tauri 2 · Svelte 5 (Runes) · Tailwind CSS v4 · shadcn-svelte (bits-ui) · Paraglide-JS i18n

---

## Codebase Baseline Assessment

This section records objective findings from the initial audit. Every design decision in the four sections that follow is grounded in these observations.

### Layout & Responsiveness

| Concern                  | Current State                                                                                                                                                                                                                                    |
| ------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Layout paradigm**      | Mobile-first in intent but inconsistently applied. Base styles target small screens; `md:` and `lg:` modifiers widen/rearrange. Most breakpoint pivots happen at `md` (768 px) or `lg` (1024 px).                                                |
| **Root shell**           | `+layout.svelte` uses a `flex h-screen` container. On desktop (`lg:flex-row`) the left `SidebarNavigation` is 256 px / 64 px (collapsed). On mobile (`< lg`) the sidebar is `hidden`; `BottomNavigation` is `fixed bottom-0` and always visible. |
| **Main content padding** | `p-4 pb-24 lg:p-8 lg:pb-8` — the `pb-24` clears the 64 px bottom nav plus comfortable breathing room. This is correct but could be tightened using `env(safe-area-inset-bottom)`.                                                                |
| **Header**               | Sticky `<header>` contains a mobile brand mark (`lg:hidden`) and a `SearchBar` that is always rendered. No mobile-specific height constraint; currently `p-4` on all sizes.                                                                      |
| **PageHeader**           | 3-tier component (subtitle → h1 → description). Actions row uses `md:flex-row` to stack buttons on mobile. Correct but the `text-2xl lg:text-3xl` h1 can be aggressively large on 360 px screens.                                                |
| **Collection page**      | `CollectionDashboard` stat chips in `grid-cols-2 sm:grid-cols-3 lg:grid-cols-6` — fine. The desktop filter sidebar is `hidden md:flex`. Mobile shows a full-width `<aside>` panel below content when toggled.                                    |

### Component Density & Touch Targets

| Concern                              | Current State                                                                                                                                                                                                                                                          |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **RailwayModelCard specs bar**       | `grid-cols-[2fr_1fr_1fr_1fr_1fr]` — 5 columns of `text-[9px]` labels and `text-xs` values. On screens narrower than 400 px the columns compress severely and the labels become illegible.                                                                              |
| **RailwayModelPreviewCard metadata** | `grid-cols-1 sm:grid-cols-[2fr_1fr_1fr]` — good; collapses to single column on mobile. Labels are `text-[10px]` uppercase. Acceptable for a card thumbnail but borderline for readability.                                                                             |
| **Filter chip remove buttons**       | `class="rounded-sm p-0.5"` around a 14 px icon — this is approximately 22 × 22 px: well below the 44 × 44 px WCAG minimum touch target.                                                                                                                                |
| **View mode toggle buttons**         | `p-1.5 rounded` — approximately 26 × 26 px. Below minimum.                                                                                                                                                                                                             |
| **Navigation bottom bar**            | `h-16` (64 px) with 5 equal-width items. Each item's touch area is ≈ 64 × 64 px (full height × viewport-width ÷ 5 ≈ 75 px wide at 375 px screen). This is adequate. The `active:scale-95` press feedback is good.                                                      |
| **MoreMenu**                         | Uses shadcn `Sheet side="bottom"` positioned as a floating bubble (`!right-4 !bottom-24 !left-auto !w-72`). Positioning is anchored to the right-side of the screen — this works but is not full-width, limiting touch area on narrow screens.                         |
| **DrawerShell**                      | `fixed inset-y-0 right-0` right-panel drawer. Max widths: `max-w-lg`, `max-w-2xl`, `max-w-3xl`. On a 375 px screen the `max-w-lg` (512 px) drawer is already full-width, but the visual treatment (border-left, enters from the right) is a desktop idiom.             |
| **steampunk-base.css**               | Already sets `min-height: 44px; min-width: 44px` for `button, [role='button']` at `max-width: 1023px`. This blanket rule is a good baseline but does **not** cover small child elements inside buttons (e.g., the filter chip `<button>` wrapping a 14 px `<X>` icon). |

### Drawer & Modal Patterns

| Component                            | Mechanism                                                            | Mobile Adaptation Needed                      |
| ------------------------------------ | -------------------------------------------------------------------- | --------------------------------------------- |
| `DrawerShell`                        | `fixed inset-y-0 right-0` slide-in panel                             | Convert to bottom-sheet on mobile             |
| `MoreMenu`                           | shadcn `Sheet side="bottom"` (floating, partial-width)               | Extend to full-width bottom sheet             |
| `QuickAddShell`                      | Rendered inside `DrawerShell` — effectively a nested right-panel     | Convert to stacked bottom-sheet layer         |
| `RestoreConfirmModal` (cloud backup) | Uses bits-ui `Dialog` (centered modal)                               | Keep as a center sheet / smaller bottom sheet |
| `ImageCropDialog`                    | bits-ui `Dialog` centered                                            | Keep centered (canvas interaction)            |
| `WelcomeWizard`                      | Full-screen onboarding — not assessed; out of scope for initial pass |

### Navigation Structure

- **Desktop:** `SidebarNavigation` — collapsible, icon + label, 9 items + Settings + Debug.
- **Mobile:** `BottomNavigation` — 4 primary items (Dashboard, Collection, Finance, Wishlists) + "More" overflow.
- **Primary items count validated** in `config.ts` (DEV warning if not exactly 4).
- **Settings and Debug** are only accessible from the desktop sidebar; on mobile the user must find them via another path (currently there is no mobile route to Settings from the bottom nav).

### Safe-Area Inset Handling

- `viewport.svelte.ts` programmatically sets `--safe-area-top/bottom/left/right` CSS vars **only when `isMobile === true`** (User-Agent detection).
- `BottomNavigation` uses `pb-safe-area` class — this requires a Tailwind v4 `@plugin` or `@utility` definition; verify it resolves at runtime.
- The global `layout.css` does not add `padding-bottom: env(safe-area-inset-bottom)` to the body or main wrapper. This must be addressed for iPhone notch / home-indicator overlap.

### Typography Scale (Existing)

| Level           | Current Class          | Rendered Size |
| --------------- | ---------------------- | ------------- |
| Page h1         | `text-2xl lg:text-3xl` | 24 px mobile  |
| Section heading | `text-xl` (sidebar)    | 20 px         |
| Card title      | `text-sm font-medium`  | 14 px         |
| Metadata label  | `text-[10px]`          | 10 px         |
| Micro label     | `text-[9px]`           | 9 px          |
| Monospace data  | `font-mono text-xs`    | 12 px         |

Labels at 9–10 px are below the WCAG 2.1 minimum of 14 px (or 12 px bold, small text at 4.5:1 ratio). On high-DPI mobile screens they appear as approximately 18–20 px physical pixels, which is acceptable for Android/iOS, but they remain a risk at system 1× text scaling.

---

## Section 1: Architecture & Navigation Foundations

### 1.1 Mobile Header vs. Footer (Thumb-Zone) Division of Labor

The current header holds a brand mark (mobile) and the search bar. The footer holds primary navigation. This split is correct and must be preserved. The rules below formalize and extend it.

#### Header Zone (Top, thumb-far)

**Responsibility:** Identity, contextual page title, and non-time-critical secondary actions.

| Element                             | Mobile Rule                                                                                                                                                                             | Implementation Hook                                                            |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Brand mark                          | Keep `<TrainFront>` + app name visible when no page title is present                                                                                                                    | Existing `lg:hidden` `div` in `+layout.svelte` header                          |
| Page context                        | Inject a short `<h1>` or page label into the header on inner pages (Collection, Settings, etc.) so the user always knows where they are without reading the `PageHeader` below the fold | New `#snippet pageTitle` forwarded via Svelte context from each `+page.svelte` |
| Search                              | Keep `<SearchBar>` in the header; ensure it expands to full-width on focus (currently renders inline at all sizes)                                                                      | Add `focus-within:w-full sm:w-auto` expansion                                  |
| Secondary actions (e.g., "Add" CTA) | Move page-level CTAs from `PageHeader` into the header right zone for mobile only                                                                                                       | Use `{#if $isMobileViewport}` wrapping or a dedicated mobile header slot       |

**CSS constraint:** Mobile header must have a minimum height of 56 px (`min-h-14`) plus `padding-top: env(safe-area-inset-top, 0px)`. Use `pt-[env(safe-area-inset-top)]` in Tailwind v4.

```html
<!-- Tailwind v4 safe-area header pattern -->
<header
  class="sticky top-0 z-50 min-h-14 border-b"
  style="padding-top: env(safe-area-inset-top, 0px);"
></header>
```

#### Footer Zone (Bottom, thumb-near)

**Responsibility:** Primary navigation and universal quick-action trigger.

Current state is good. Enhancements:

1. **Settings access:** Add a "Settings" item to the mobile bottom nav — either as a 5th primary item (replacing one less-used item) or by including it prominently in the "More" bottom sheet. Currently settings is completely unreachable from the mobile UI without the desktop sidebar.

2. **Primary action FAB:** Consider a Floating Action Button (+) anchored above the bottom nav (`bottom-20`) for the most frequent action on the current page (e.g., "Add to Collection" on the collection page, "New Wishlist Item" on wishlists). This is optional but aligns with the thumb-zone principle.

3. **Bottom nav safe-area:** Enforce `padding-bottom: env(safe-area-inset-bottom, 0px)` inside the nav. Currently `pb-safe-area` is used — confirm the Tailwind v4 utility is registered:

   ```css
   /* In layout.css */
   @utility pb-safe-area {
     padding-bottom: env(safe-area-inset-bottom, 0px);
   }
   ```

4. **Navigation bar total height:** `h-16` + safe-area padding. Main content's `pb-24` should become `pb-[calc(4rem_+_env(safe-area-inset-bottom,_0px))]` to dynamically adapt.

### 1.2 Modal / Dialog → Mobile Bottom Sheet Adaptation

#### Rule: Right-Panel Drawers become Bottom Sheets on Mobile

`DrawerShell` (`fixed inset-y-0 right-0`) is the primary modal container. On small screens it already becomes full-width, but the animation direction (translate-x) and border placement (border-left) are semantically wrong for mobile. The adaptation strategy:

| Breakpoint       | Layout                                       | Animation                            | Max Height                           |
| ---------------- | -------------------------------------------- | ------------------------------------ | ------------------------------------ |
| `< md` (mobile)  | `fixed bottom-0 left-0 right-0` bottom sheet | `translate-y-0` ← `translate-y-full` | `max-h-[90dvh]` with internal scroll |
| `≥ md` (tablet+) | `fixed inset-y-0 right-0` current side panel | `translate-x-0` ← `translate-x-full` | `max-w-lg / 2xl / 3xl`               |

Implementation approach for `DrawerShell.svelte`:

```svelte
<!-- Determine layout based on a reactive isMobile prop or CSS media query -->
<div
  class="fixed z-50 flex flex-col border-primary bg-card shadow-2xl transition-transform duration-300
         bottom-0 left-0 right-0 max-h-[90dvh] rounded-t-2xl border-t-2
         md:inset-y-0 md:bottom-auto md:right-0 md:left-auto md:max-h-none md:rounded-none md:border-l-2 md:border-t-0
         {sizeClass}"
>
```

**Handle bar:** Add a drag handle indicator at the top of the mobile bottom sheet:

```html
<!-- Only visible on mobile -->
<div class="mx-auto mb-2 h-1 w-10 rounded-full bg-border md:hidden" aria-hidden="true"></div>
```

#### Nested Drawers (QuickAdd)

`QuickAddShell` renders a second overlay on top of `DrawerShell`. On mobile this must stack as a second bottom sheet layer (higher `z-index`, slightly shorter `max-h`):

- `DrawerShell` mobile: `z-50`, `max-h-[90dvh]`
- `QuickAddShell` mobile: `z-60`, `max-h-[75dvh]`, slightly dragged-up to reveal the parent sheet edge

**Svelte 5 runes-based stacking — avoid manual `z-index` increments.** Use a module-level `DrawerRegistry` to track active drawers and derive visual offsets reactively:

```ts
// src/lib/states/drawer.svelte.ts
class DrawerRegistry {
  stack = $state<string[]>([]);

  open(id: string) {
    this.stack.push(id);
  }
  close() {
    this.stack.pop();
  }
  getDepth(id: string) {
    return this.stack.indexOf(id);
  }
}
export const drawers = new DrawerRegistry();
```

Inside `DrawerShell.svelte`, bind the depth to inline style rather than static class values so the parent sheet automatically scales back when a child sheet opens — mimicking native iOS modal stacking:

```svelte
<div
  class="fixed bottom-0 left-0 right-0 max-h-[90dvh] rounded-t-2xl border-t-2 bg-card transition-transform duration-300"
  style:transform={drawers.getDepth(id) > 0
    ? `translateY(-${drawers.getDepth(id) * 8}px) scale(${1 - drawers.getDepth(id) * 0.04})`
    : 'none'}
  style:z-index={50 + drawers.getDepth(id)}
>
```

This way a `QuickAdd` overlay automatically pushes the parent `DrawerShell` into the background without any hardcoded `z-60` or `max-h-[75dvh]` magic numbers.

#### Confirmation Dialogs (e.g., Discard, Restore)

Keep centered modal pattern (`Dialog`) for binary decisions — they are short, focused, and centering is appropriate. Apply:

```css
/* Mobile: make dialogs comfortably wide */
.dialog-content {
  @apply mx-4 w-[calc(100%-2rem)] max-w-sm;
}
```

#### ImageCropDialog

This uses a canvas — keep it as a centered dialog. Ensure full-width on mobile (`w-full max-w-[calc(100vw-2rem)]`).

### 1.3 Screen Estate: Horizontal Scroll vs. Vertical Stacking

#### Data Type Classification

| Data Type                        | Structure                                   | Mobile Strategy                                                                                                                                                                                                                          |
| -------------------------------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Collection grid                  | `VirtualGrid` — `itemMinWidth={240}`        | **1 column** on `< sm` (< 640 px), 2 columns on `≥ sm`. The 240 px minimum with 16 px gap on a 375 px screen yields 1 column — this already works. Explicitly enforce with `itemMinWidth={320}` to prevent 2-column layout below 670 px. |
| Category stat chips              | `grid-cols-2 sm:grid-cols-3 lg:grid-cols-6` | Keep as-is — 2 × 3 grid is thumb-friendly.                                                                                                                                                                                               |
| Dashboard stat cards             | `grid-cols-1 md:grid-cols-2 lg:grid-cols-3` | Keep. 1-column on mobile is fine.                                                                                                                                                                                                        |
| Railway model specs bar          | `grid-cols-[2fr_1fr_1fr_1fr_1fr]` (5 cols)  | **Horizontal scroll** — wrap in `overflow-x-auto`. This preserves the desktop fixed layout while allowing mobile users to swipe to see all specs. Alternatively, convert to a 2×3 grid on mobile (see Section 3).                        |
| Settings form                    | `grid md:grid-cols-2`                       | Keep — stacks to 1 column on mobile naturally.                                                                                                                                                                                           |
| Purchase group cards (dashboard) | Vertical list                               | Keep as vertical.                                                                                                                                                                                                                        |
| Filter chips row                 | `flex flex-wrap gap-2`                      | Keep as wrapping flex. Increase chip height to 32 px minimum.                                                                                                                                                                            |
| Acquisition item cards           | Vertical list of form cards                 | Keep vertical.                                                                                                                                                                                                                           |
| Navigation "More" menu           | Floating partial-width bottom sheet         | Convert to full-width bottom sheet (see 1.2).                                                                                                                                                                                            |

#### Horizontal Scroll Guidelines

When using horizontal scroll for dense tabular/spec data:

1. Wrap in a container with `overflow-x-auto -mx-4 px-4` (edge bleed) so the scroll area is flush with screen edges.
2. Add a visual scroll hint: fade gradient on the right edge (`bg-gradient-to-r from-transparent via-transparent to-card/80 pointer-events-none`).
3. Use `scroll-snap-type: x mandatory` and `scroll-snap-align: start` on child columns for deliberate swipe feel.
4. Never hide a scrollbar on mobile — rely on the OS's native scroll indicator.

---

## Section 2: Touch & Interaction Rules

### 2.1 Touch Target Standards

All interactive elements on mobile must meet the following minimums. The `steampunk-base.css` blanket rule covers `button` and `[role='button']` at a global level, but specific Tailwind utility patterns below must be applied component-by-component where the rule is insufficient.

#### Minimum Size Table

| Element Type                                       | Minimum Touch Size                          | Tailwind v4 Implementation               |
| -------------------------------------------------- | ------------------------------------------- | ---------------------------------------- |
| Icon-only button (close, remove chip, toggle view) | 44 × 44 px                                  | `min-h-11 min-w-11` (11 × 4 = 44 px)     |
| Text button / nav item                             | 44 px tall, full available width            | `min-h-11 w-full` or `h-11`              |
| Input field (text, select)                         | 48 px tall                                  | `h-12`                                   |
| Checkbox / radio card                              | 48 px tall, full available width            | `min-h-12`                               |
| Bottom nav item                                    | 64 px tall (current `h-16` ✓)               | No change needed                         |
| Badge remove button (`<X>` icon)                   | 36 × 36 px minimum (relaxed: inside a chip) | `p-2` around the icon instead of `p-0.5` |

#### Specific Violations to Fix (Priority Order)

1. **Filter chip remove buttons** (`CollectionDashboard`): Change `class="rounded-sm p-0.5"` → `class="rounded p-2 -m-1"` (negative margin to avoid expanding the chip itself).
2. **View mode toggle buttons** (grid/table toggle): Change `p-1.5` → `min-h-11 min-w-11 p-2.5` and adjust the wrapper to `h-11`.
3. **Collection filter toggle** (`<Button size="sm" class="md:hidden">`): Button `size="sm"` in shadcn renders at approximately 36 px tall. Override to `h-11` on mobile via `class="h-11 md:h-9"`.

### 2.2 Padding, Margin, and Spacing Rules

#### Content Area

- **Horizontal page padding:** `px-4` (16 px) is the minimum safe gutter on all mobile content. Never drop below `px-4`.
- **Vertical section spacing:** Reduce `space-y-8` to `space-y-6` inside the main scroll area on mobile to use vertical space more efficiently. The `+layout.svelte` currently wraps children in `class="space-y-8"`.
- **Card internal padding:** `p-4` minimum. The current `p-3` on `CardHeader` / `CardContent` is acceptable for preview cards but `p-4` is preferred for full-detail cards.
- **Header section bleed:** The pattern `-mx-4 -mt-4 px-6` (used in `CollectionDashboard` and `settings/+page.svelte`) correctly bleeds the header to screen edges. On mobile, use `px-4` (not `px-6`) to maximize usable width.

#### Safe-Area Insets (Tailwind v4)

Add the following CSS utilities to `layout.css` so they are available project-wide:

```css
/* layout.css additions — safe-area utilities */
@utility pt-safe {
  padding-top: env(safe-area-inset-top, 0px);
}
@utility pb-safe {
  padding-bottom: env(safe-area-inset-bottom, 0px);
}
@utility pl-safe {
  padding-left: env(safe-area-inset-left, 0px);
}
@utility pr-safe {
  padding-right: env(safe-area-inset-right, 0px);
}
```

Usage in layout:

```html
<!-- Header: respect status bar notch -->
<header class="pt-safe sticky top-0 z-50">
  <!-- Bottom nav: respect home indicator -->
  <div class="pb-safe fixed bottom-0">
    <!-- Main scroll area: clear both nav elements -->
    <div
      class="h-full overflow-y-auto p-4 pb-[calc(6rem_+_env(safe-area-inset-bottom,_0px))]"
    ></div>
  </div>
</header>
```

### 2.3 Active States and Touch Feedback

#### System-Wide Touch Feedback Strategy

Rusty Shed's steampunk aesthetic depends on mechanical press feedback. The `active:scale-95` already applied to bottom nav items is the baseline. Extend it consistently:

| Interaction               | Feedback Rule             | Tailwind Utility                                        |
| ------------------------- | ------------------------- | ------------------------------------------------------- |
| Primary action buttons    | Scale + brightness change | `active:scale-95 active:brightness-90`                  |
| Navigation items          | Scale + opacity           | `active:scale-95 active:opacity-80` (already applied ✓) |
| Card row / clickable card | Background flash + scale  | `active:bg-muted/50 active:scale-[0.98]`                |
| Icon-only button          | Opacity pulse             | `active:opacity-60`                                     |
| Destructive button        | Color deepen              | `active:brightness-75`                                  |
| Toggle / chip             | Border flash              | `active:border-primary`                                 |

The `variant-steampunk-lever:active { transform: translateY(2px); }` from `steampunk-base.css` is the model for mechanical press simulation. Apply this class to primary `<Button variant="rusty">` on mobile.

#### Removing / Isolating Desktop Hover Variants

Desktop `hover:` variants create false "sticky hover" states on touch (iOS webkit hover persistence). Strategy:

**Rule:** Never write a bare `hover:*` that changes structural layout or color on elements that are also touch targets.

1. **Global suppression rule** — add to `layout.css`:

   ```css
   @media (hover: none) and (pointer: coarse) {
     /* Disable hover-triggered visual state changes for touch users */
     .hover\:bg-sidebar-accent:hover,
     .hover\:bg-muted:hover,
     .hover\:bg-muted\/40:hover,
     .hover\:text-foreground:hover,
     .hover\:border-primary:hover {
       /* Reset: do not apply hover styles on touch */
       background-color: unset;
       color: unset;
       border-color: unset;
     }
   }
   ```

   > **Note:** A cleaner Tailwind v4 approach is to prefix hover variants with `@media (hover: hover)` using the `@variant` directive. See below.

2. **Tailwind v4 `@variant` for precise-pointer hover** — add to `layout.css`:

   ```css
   /* Native Tailwind v4 touch-safe hover variant */
   @variant touch-hover (&:hover) {
     @media (hover: hover) and (pointer: fine) {
       &:hover {
         @slot;
       }
     }
   }
   ```

   Then replace `hover:bg-sidebar-accent` with `touch-hover:bg-sidebar-accent` in navigation components. This ensures hover styles never fire on touch screens. The `@variant` directive compiles this into a proper Tailwind utility with full purge and tree-shaking support — cleaner than the `@custom-variant` workaround.

   > **Implementation note:** Replacing all `hover:` in the codebase is a large refactor. Prioritize doing it for navigation items, card interactions, and buttons — components where sticky hover is most visible on iOS.

3. **Immediate priority files:**
   - `SidebarNavigation.svelte` — `hover:bg-sidebar-accent`
   - `MoreMenu.svelte` — `hover:bg-muted`
   - `CollectionDashboard.svelte` — stat chip `hover:bg-muted/40`, lifecycle chip `hover:bg-amber-500/14`
   - `RailwayModelPreviewCard.svelte` — `group-hover:*` (card-level hover, iOS will trigger on first tap)

### 2.4 Scroll & Gesture Behavior

- **Momentum scrolling:** Apply `-webkit-overflow-scrolling: touch` via `overscroll-contain` and `scroll-smooth` classes to the main content scroll container.
- **Overscroll bounce:** Use `overscroll-y-none` on the `<main>` scroll container to prevent accidental page-bounce that can look broken in Tauri's WebView.
- **Swipe-to-dismiss drawers:** For mobile bottom sheets (Section 1.2), implement a touch-drag gesture via `@draggable` or pointer events to allow swiping down to close.
- **Virtual grid scroll:** `VirtualGrid` already uses `passive: true` scroll listeners — correct. No change needed.

---

## Section 3: Typography & Visual Density Rules

### 3.1 Mobile Typographic Hierarchy Scale

All type sizes below are for mobile (`< md`). Desktop sizes are unchanged.

| Level                               | Role                                          | Example Location               | Mobile Tailwind Class                                 | Min Size                           |
| ----------------------------------- | --------------------------------------------- | ------------------------------ | ----------------------------------------------------- | ---------------------------------- |
| **T0** — App name                   | Brand wordmark in header                      | `+layout.svelte` header        | `text-sm font-bold tracking-widest uppercase`         | 14 px                              |
| **T1** — Page title                 | Main `<h1>` on each page                      | `PageHeader` title             | `text-xl font-bold`                                   | 20 px                              |
| **T2** — Section title              | Dashboard section headers, card group headers | `DashboardSectionHeader`       | `text-base font-semibold`                             | 16 px                              |
| **T3** — Subtitle / Eyebrow         | Uppercase pre-title label above T1            | `PageHeader` subtitle          | `text-xs font-semibold tracking-widest uppercase`     | 12 px                              |
| **T4** — Body / Description         | Page descriptions, card descriptions          | `PageHeader` description       | `text-sm text-muted-foreground`                       | 14 px                              |
| **T5** — Card primary text          | Model name, manufacturer                      | `RailwayModelPreviewCard` h3   | `text-sm font-medium leading-tight`                   | 14 px                              |
| **T6** — Card secondary text        | Product code, series                          | Manufacturer · ProductCode row | `text-xs text-muted-foreground`                       | 12 px                              |
| **T7** — Metadata label (uppercase) | Column headers in specs bar, stat chip labels | Specs bar, StatChip            | `text-[10px] font-semibold tracking-widest uppercase` | 10 px (physical ≈ 20 px on 2× DPI) |
| **T8** — Metadata value             | Actual value under T7 labels                  | Scale, Era, Category values    | `text-xs font-mono`                                   | 12 px                              |
| **T9** — Micro label                | Nav item labels in bottom bar                 | `BottomNavigation`             | `text-[10px] font-bold tracking-wider uppercase`      | 10 px                              |

#### Rules for Overflow Prevention

1. **Truncate at T5 and below** with `truncate` (single-line) or `line-clamp-2` (two-line). Never allow text to wrap uncontrolled in a fixed-size card.
2. **No text below 10 px** on mobile. The existing `text-[9px]` labels in `RailwayModelCard`'s specs bar must be raised to `text-[10px]` (T7).
3. **Bebas Neue headings** (`font-bebas`) should only appear at **T1** and above on mobile. Using Bebas Neue below 16 px (e.g., in the discard dialog `text-xl`) is acceptable because the font renders wide; do not use it at `text-sm` or below.
4. **JetBrains Mono / Courier Prime** (`font-mono`) should only be used for data values (product codes, prices, scales, eras) — never for labels or descriptions on mobile.
5. **Line height:** Use `leading-tight` (1.25) for headings (T1–T2), `leading-snug` (1.375) for body copy (T4–T5), and `leading-none` (1.0) for single-line data labels (T7–T8). This prevents excess vertical space in dense card layouts.

### 3.2 Maximum Data Density Rules for List/Card Components

#### RailwayModelPreviewCard (Collection Grid)

This is the highest-density component shown in quantity on mobile. Rules:

| Zone                        | Max Lines                            | Truncation                                       | Change Required?                                                                                          |
| --------------------------- | ------------------------------------ | ------------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| Manufacturer + product code | 1 line (flex row, whitespace-nowrap) | Manufacturer `truncate`, product code `shrink-0` | No change — current code is correct.                                                                      |
| Description / series (h3)   | 2 lines                              | `line-clamp-2`                                   | No change — current code is correct.                                                                      |
| Image area                  | Fixed `aspect-video`                 | N/A                                              | No change.                                                                                                |
| Category/Scale/Era metadata | 1 line each                          | `truncate` on values                             | On mobile `< sm`, collapse the 3-column grid to **2 columns** (Category hidden, or merged with a scroll). |
| Purchase date + price       | 1 line each                          | `truncate`                                       | Keep `sm:grid-cols-2`, single column on `< sm`.                                                           |

**Specific fix for < 375 px screens:** The `sm:grid-cols-[2fr_1fr_1fr]` metadata grid renders as `grid-cols-1` at `< sm`. This means Category, Scale, and Era stack vertically — three rows of data below the image. This is fine but adds height. Alternative: show only the 2 most important fields (Scale + Era) in a horizontal row and put Category in a badge overlay on the image.

#### RailwayModelCard Specs Bar (Detail View)

The 5-column `grid-cols-[2fr_1fr_1fr_1fr_1fr]` specs bar is the densest component in the app. On a 375 px screen, each column is ≈ 55–60 px wide after accounting for dividers. This is manageable for data values but the `text-[9px]` labels are below minimum.

**Mobile adaptation (two options — pick one):**

**Option A: Horizontal scroll (lower risk):**

```html
<div class="overflow-x-auto md:overflow-visible">
  <div class="grid min-w-[420px] grid-cols-[2fr_1fr_1fr_1fr_1fr] ...">
    <!-- existing content unchanged -->
  </div>
</div>
```

Minimum width of 420 px forces horizontal scroll on < 420 px screens. Simple to implement without restructuring the component.

**Option B: 2×3 grid reflow (preferred UX):**

```html
<!-- Mobile: 3 columns per row, 2 rows -->
<div class="grid grid-cols-3 md:grid-cols-[2fr_1fr_1fr_1fr_1fr] ...">
  <!-- Category (spanning 1 col), Scale, Era -->
  <!-- Delivery Date (spanning 1 col), Status (spanning 1 col), [empty] -->
</div>
```

Raises label text to `text-[10px]` and value text to `text-xs` — both meeting T7/T8 minimums.

#### CollectionTableView

Table views are inherently problematic on mobile. Rules:

1. **Hide table view toggle on mobile** (`< md`): The view mode switch should only show on `md:flex` and above. On mobile, grid-only or a compact list-row view should be used.
2. If a table must be shown on mobile, use **horizontal scroll** with `min-w-[600px]` on the `<table>` and `overflow-x-auto` on the wrapper, with sticky first column (model name).

#### Dashboard Stats Cards

`grid-cols-1 md:grid-cols-2 lg:grid-cols-3` stacks to single column on mobile. The `StatsCard` component currently uses `h-28` skeleton placeholder. Rule: each stats card must not exceed 80 px on mobile (use `h-20`). Stack 3 cards in a `grid-cols-3` pill-style row instead for tighter layout:

```html
<!-- Mobile: compact 3-col stats strip instead of tall single-column cards -->
<div class="grid grid-cols-3 gap-2 md:grid-cols-2 lg:grid-cols-3"></div>
```

Use `text-lg` for the primary number and `text-[10px]` (T7) for the label in this compact layout.

#### Wishlist / Finance / Maintenance Pages

These pages were not audited in detail, but the same density rules apply:

- List rows: minimum 52 px tall, `py-3` internal padding.
- Numeric values: `font-mono text-sm` (T8 scale).
- Status badges: max 2 badges per row; additional badges truncated with `+N more`.

### 3.3 Mobile Image & Camera Capture Layouts

To bridge the gap between Tauri's desktop drag-and-drop file inputs and native mobile hardware pipelines, all image management containers must transition to a tap-to-capture architecture.

#### Native Hardware File Bridge

- **No drag-and-drop dropzones on mobile:** Remove desktop wrapper classes like `border-dashed` input panels below the `md:` breakpoint. Replace them with single-action tactile trigger buttons.
- **Direct camera execution:** Implement explicit media triggers to skip deep directory browsing on active devices:

  ```html
  <input
    type="file"
    accept="image/*"
    capture="environment"
    class="sr-only"
    id="mobile-camera-capture"
  />
  <label
    for="mobile-camera-capture"
    class="flex h-12 w-full items-center justify-center gap-2 rounded-xl bg-primary font-medium text-primary-foreground active:scale-95"
  >
    <Camera class="h-5 w-5" />
    <span>Capture Model Photo</span>
  </label>
  ```

  The `capture="environment"` attribute opens the rear-facing camera directly on both iOS (`WKWebView`) and Android (`WebView`). Tauri's file system plugin then receives the selected file path via the standard `<input>` change event, keeping the IPC bridge interaction identical to the desktop flow.

#### Image Aspect-Ratio Containers

- **Enforce strict geometry blocks:** In grid and list previews, imagery must be hard-bound to an aspect ratio utility (`aspect-video` or `aspect-[4/3]`) combined with `object-cover`. This safely handles varying smartphone camera capture outputs without inducing unexpected page layout shifts or vertical stretching down to the 375 px baseline:

  ```html
  <!-- Mobile-safe image container -->
  <div class="aspect-video w-full overflow-hidden rounded-lg">
    <img src="{model.imageUrl}" alt="{model.name}" class="h-full w-full object-cover" />
  </div>
  ```

- **Skeleton placeholder geometry:** Image skeleton loaders must use the same `aspect-video` container so the layout does not reflow when the image loads.
- **Missing image fallback:** When no image is available, render a centered icon placeholder inside the same `aspect-video` container (`bg-muted flex items-center justify-center`) rather than collapsing the container height.

---

## Section 4: Discovery Questionnaire

These questions were answered by the product owner on 2026-06-28. All answers are now incorporated into the guidelines above where relevant, and the implementation checklist below is updated accordingly.

---

### Q1: What are the target mobile platforms and OS versions?

**Decision:** Primary targets are **Android (API 26+)** and **iOS (16.4+)**.

- **Form factor baseline:** The absolute lower bound for layout optimization is the **iPhone SE / iPhone 8 grid (375 × 667 px)**. All density calculations in Sections 2 and 3 use this as the minimum.
- **`dvh` unit support:** iOS 16.4+ ships full Dynamic Viewport Unit support. `max-h-[90dvh]` and similar `dvh`-based classes in `DrawerShell` are safe to use without JS-calculated fallbacks.
- **WebView engine:** `WKWebView` (iOS) and Android WebView (API 26+). Both support `env(safe-area-inset-*)`. No custom polyfills needed.

**Impact resolved:** Safe-area utilities (`pt-safe`, `pb-safe`, etc.) in Section 2.2 are confirmed correct. The `dvh` unit is confirmed for all bottom-sheet height constraints.

---

### Q2: Which user workflows occur most frequently on mobile vs. desktop?

**Decision:** The primary mobile use pattern is a hybrid of **(a) looking up a model** on the display floor to confirm ownership or check a running number, and **(c) verifying a wishlist** to avoid duplicate purchases at hobby fairs or train shows.

- **Keep the 4 core nav items:** Dashboard, Collection, Finance, and Wishlists remain the primary mobile bottom-nav items. No item replacement needed.
- **FAB placement:** Do **not** add a global FAB across all pages. Implement a contextual quick-action button exclusively on the **Collection page** and **Wishlist page** (`bottom-20` above the bottom nav).
- **"More" sheet:** Settings and Debug move to the **top** of the `MoreMenu` sheet as full-width rows with chevron arrows. Depot, Digital DCC, Railway Tracks, and Train Formations are deprioritized in the sheet but remain reachable.
- **Maintenance:** Not a primary mobile workflow; it remains in the "More" sheet. No change to primary nav.

**Impact resolved:** Navigation structure and FAB scope are confirmed. Section 1.1's "Consider a FAB" note is now a concrete recommendation scoped to Collection and Wishlist pages only.

---

### Q3: How should the RailwayModelCard detail view behave on mobile?

**Decision:** The mobile detail view is optimized for **readability first**. Editing is not disabled entirely, but inline micro-input patterns are replaced with full-width equivalents.

- **`InPlaceEdit`:** Disable inline editing below the `md:` breakpoint. Replace with an **"Edit Profile"** action button in the card header that opens all editable fields as a full-width form list in a bottom sheet.
- **`BadgePicker`:** Convert popover-based pickers into **choice-chip bottom drawers** on mobile (full-screen option sheet instead of a popover that risks edge clipping).
- **Image upload:** Replace the drag-and-drop zone with a native `<input type="file" accept="image/*" capture="environment">` trigger. This lets Tauri engage the device's native camera/photo library picker on both Android and iOS.
- **Tab layout:** Keep the multi-tab pattern; do not convert to accordion or horizontal swipe for the initial mobile pass. The existing tab bar is functional at 375 px.

**Impact resolved:** `DrawerShell` → bottom-sheet conversion is in scope. `InPlaceEdit` and `BadgePicker` adaptations are confirmed for the initial mobile pass.

---

### Q4: What are the i18n / text-length constraints on mobile?

**Decision:** The app must handle Italian text expansions (20–40% longer than English) gracefully at all breakpoints.

- **Bottom nav labels:** Enforce a strict `truncate` or `overflow-x-auto` on all `<span>` label elements inside `BottomNavigation`. If Italian labels like `"FORMAZIONI"` squeeze flex items on 375 px screens, drop from `text-[10px]` to `text-[9px]` or apply horizontal string clipping via CSS `overflow: hidden; text-overflow: clip`.
- **Header page title:** Apply `max-w-[180px] truncate block` to the injected `<h1>` in the mobile header (Section 1.1). This provides clean ellipsis fallback for long Italian page names without layout breaking.
- **No third language** planned for the current roadmap. The existing 2-language margin is sufficient.

**Impact resolved:** T9 (bottom nav labels) in Section 3.1 now has a confirmed truncation policy. The mobile header title constraint is confirmed at `max-w-[180px]`.

---

### Q5: What is the offline / connectivity constraint for Tauri mobile builds?

**Decision:** The 4-second Tauri bridge timeout remains unchanged. The loading UX is changed from a full-screen blocking spinner to incremental skeletons.

- **Keep the 4-second timeout** — appropriate for all target Android devices including mid-range hardware (API 26+).
- **Mount the root shell navigation instantly** with per-section **skeleton blocks** rather than holding back the full UI behind a global spinner. Each feature area (Dashboard stats, Collection list, Wishlists) loads independently with its own skeleton → data transition.
- **Individual lazy loading** per page section is confirmed for the mobile architecture. `DrawerShell` and `AcquisitionDrawer` state is not restored after OS backgrounding in this initial pass — unsaved state is accepted as lost on background/restore.

**Impact resolved:** Section 1.1 boot sequence note is now concrete: skeleton-first loading is the confirmed pattern. The `SignalFailureView` remains as the hard-failure fallback after 4 seconds.

---

## Appendix: Implementation Checklist (Pre-Code)

All five Q&A responses have been collected from the product owner (2026-06-28). The following items are now confirmed:

- [x] All five Q&A responses collected from the product owner
- [x] Target screen size confirmed (minimum: 375 × 667 px — iPhone SE baseline)
- [ ] `pb-safe-area` Tailwind v4 utility verified at runtime (or replaced with `pb-safe`)
- [x] `dvh` unit support confirmed for target Tauri WebView version (iOS 16.4+, Android API 26+)
- [ ] Desktop regression test baseline captured (screenshot / automated) for:
  - `+layout.svelte` (sidebar, header, content area)
  - `CollectionDashboard.svelte` (grid, filter sidebar)
  - `RailwayModelCard.svelte` (specs bar, tabs)
  - `DrawerShell.svelte` (right panel)
  - `BottomNavigation.svelte` (hidden on desktop `lg:hidden` — confirm it remains hidden)
- [ ] i18n strings for both `en` and `it` spot-checked at all T7–T9 levels on 375 px mockups

---

_Document last updated: 2026-06-28 — Q&A resolved by product owner; status updated to Actionable. Authored from codebase audit of commit HEAD on branch at time of writing._
