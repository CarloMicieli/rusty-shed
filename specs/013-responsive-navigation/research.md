# Phase 0: Research - Responsive Navigation System

**Feature**: Responsive Navigation System  
**Date**: February 5, 2026  
**Status**: Complete

## Research Summary

This document consolidates research findings for implementing a responsive navigation system with desktop sidebar and mobile bottom bar patterns using Svelte 5, Tailwind 4, and shadcn-svelte.

## 1. Responsive Navigation Patterns

### Decision: Combined Desktop Sidebar + Mobile Bottom Bar

**Rationale**:

- Desktop users benefit from persistent sidebar showing all features (reduces cognitive load)
- Mobile users expect bottom navigation (thumb-friendly zone, industry standard)
- Breakpoint at 768px (Tailwind `md:`) aligns with tablet/desktop distinction
- CSS-only responsive switching avoids JavaScript complexity and ensures smooth transitions

**Alternatives Considered**:

- **Hamburger menu on both**: Rejected - hides features unnecessarily on desktop, adds extra clicks
- **Collapsible sidebar**: Rejected - out of scope; feature focuses on mobile optimization, not desktop space savings
- **Tab bar on all viewports**: Rejected - doesn't scale to 9 features on mobile (overcrowded)

**Implementation Approach**:

- Use Tailwind responsive utilities: `hidden lg:flex` for desktop sidebar, `lg:hidden` for mobile bottom bar
- Maintain two separate components for clarity: `SidebarNavigation.svelte` and `BottomNavigation.svelte`
- Both components share navigation item data structure to ensure consistency

**References**:

- Tailwind Responsive Design: https://tailwindcss.com/docs/responsive-design
- Material Design Bottom Navigation: https://m3.material.io/components/navigation-bar/overview

---

## 2. Bottom Sheet / Drawer for "More" Menu

### Decision: shadcn-svelte Sheet Component

**Rationale**:

- shadcn-svelte already integrated in project (see `components.json`, constitution)
- Sheet component provides accessible, keyboard-navigable drawer out-of-the-box
- Follows Radix UI primitives for accessibility (ARIA roles, focus management)
- Works well on mobile with touch gestures and backdrop dismissal
- Consistent with existing design system (uses project's design tokens)

**Alternatives Considered**:

- **Custom Svelte modal**: Rejected - reinventing the wheel; shadcn Sheet handles accessibility, focus trapping, and animations
- **Popover component**: Rejected - popovers are for contextual menus, not full navigation; poor mobile UX
- **Native `<dialog>` element**: Rejected - requires custom styling and accessibility features that Sheet provides

**Implementation Approach**:

- Import `Sheet` from `$lib/components/ui/sheet`
- Trigger: "More" button in mobile bottom bar
- Content: List of 4 secondary features (Maintenance, Depot, Digital DCC, Railway Tracks)
- Sheet opens from bottom (mobile convention); auto-close on navigation or backdrop click
- State management: Simple boolean `$state` rune for open/closed

**References**:

- shadcn-svelte Sheet: https://www.shadcn-svelte.com/docs/components/sheet
- Radix UI Dialog Primitive: https://www.radix-ui.com/primitives/docs/components/dialog

---

## 3. Icon Management and Consistency

### Decision: lucide-svelte with Centralized Icon Mapping

**Rationale**:

- lucide-svelte already used in codebase (see existing `SidebarNavigation.svelte`)
- Provides all required icons: LayoutDashboard, TrainFront, Wallet, Heart, Wrench, Warehouse, Cpu, TrainTrack, Ellipsis
- Tree-shakable: only imports icons actually used
- Consistent size and stroke width across all icons
- TypeScript support for icon component props

**Alternatives Considered**:

- **Custom SVG icons**: Rejected - unnecessary maintenance; lucide has excellent model railway adjacent icons
- **Icon font (e.g., Font Awesome)**: Rejected - bundle size overhead, lucide tree-shakes better
- **Mixed icon sources**: Rejected - inconsistent visual language

**Implementation Approach**:

- Define navigation item type with `icon` field storing lucide component reference
- Centralize icon mapping in navigation configuration array
- Use consistent size across all contexts: `size={20}` for list items, `size={24}` for mobile bottom bar (larger tap targets)
- Icons from lucide-svelte:
  - Home: `LayoutDashboard`
  - Collection: `TrainFront`
  - Finance: `Wallet`
  - Wishlists: `Heart`
  - Maintenance: `Wrench`
  - Depot: `Warehouse`
  - Digital (DCC): `Cpu`
  - Railway Tracks: `TrainTrack`
  - More: `Ellipsis`

**References**:

- lucide-svelte documentation: https://lucide.dev/guide/packages/lucide-svelte
- Icon selection guide: https://lucide.dev/icons

---

## 4. Active State Detection and Routing

### Decision: SvelteKit `$page` Store with Path Matching

**Rationale**:

- `$page.url.pathname` provides reactive current route
- Existing codebase uses this pattern (see current `SidebarNavigation.svelte`)
- Handles exact matches (`/my-dashboard`) and prefix matches (`/my-tracks/...`)
- Reactive: automatically updates when route changes
- Type-safe with TypeScript

**Alternatives Considered**:

- **Custom route context**: Rejected - `$page` store is built-in and optimized
- **URL parsing manually**: Rejected - error-prone and less reactive
- **Global state management**: Rejected - overengineering for navigation; `$page` is sufficient

**Implementation Approach**:

- Import `page` store: `import { page } from '$app/stores';`
- Derive pathname: `const pathname = $derived($page.url.pathname as string);`
- Active detection logic:
  - Primary features: exact match (`pathname === '/my-dashboard'`)
  - Railway Tracks: prefix match (`pathname.startsWith('/my-tracks')`) - handles subroutes
  - More button: active if pathname matches any secondary feature (`/my-maintenance`, `/my-depot`, `/my-digital-roster`, `/my-tracks`)
- Apply active styles: `class:bg-primary={isActive}` (Tailwind conditional classes)

**Edge Case Handling**:

- More button active state: Check if current route is in secondary features list
- Deep links to secondary features: More menu should highlight active feature when opened
- Viewport transitions: Active state persists across responsive breakpoint changes (no flicker)

**References**:

- SvelteKit `$page` store: https://svelte.dev/docs/kit/$app-stores#page
- Svelte 5 `$derived`: https://svelte.dev/docs/svelte/$derived

---

## 5. Localization and Translation Keys

### Decision: Paraglide-JS with New Translation Keys

**Rationale**:

- Paraglide-JS already integrated (@inlang/paraglide-js 2.7.1)
- Constitution mandates: "All user-facing strings MUST be supplied via Paraglide"
- Existing messages in `messages/en.json` and `messages/it.json`
- Type-safe message functions via `$lib/paraglide/messages.js`

**New Translation Keys Required**:

- `app_home`: "Home" (replaces `app_dashboard`: "Dashboard")
- `app_finance`: "Finance" (replaces `budget_title`: "Budget Tracking")
- `app_wishlists`: "Wishlists" (existing `app_wishlists`: "Wish Lists" - update if needed)
- `app_digital_dcc`: "Digital (DCC)" (replaces `app_digital_roster`: "My Digital Rolling Stocks")
- `app_depot`: "Depot" (existing - keep)
- `app_railway_tracks`: "Railway Tracks" (replaces `app_tracks`: "My Tracks")
- `app_maintenance`: "Maintenance" (replaces `app_maintenance`: "My Maintenance" - update if needed)
- `app_more`: "More" (new - for mobile bottom bar)

**Deprecation Strategy**:

- Mark old keys as deprecated in comments: `// DEPRECATED: Use app_home instead`
- Remove old keys after migration complete and verified
- Update all references in codebase to use new keys

**Implementation Approach**:

- Add new keys to `messages/en.json` and `messages/it.json`
- Import messages: `import * as m from '$lib/paraglide/messages.js';`
- Use message functions: `{m.app_home()}`, `{m.app_finance()}`, etc.
- Ensure reactive updates: Wrap navigation in `{#key locale}` block to re-render on language change

**References**:

- Paraglide-JS documentation: https://inlang.com/m/gerre34r/library-inlang-paraglideJs
- Existing messages: `/messages/en.json`, `/messages/it.json`

---

## 6. Responsive Breakpoints and Viewport Detection

### Decision: Tailwind `md:` Breakpoint (768px)

**Rationale**:

- Tailwind default `md:` breakpoint at 768px is industry standard
- Separates portrait tablets/phones (<768px) from landscape tablets/desktops (≥768px)
- Aligns with spec requirement: "responsive breakpoint at 768px"
- CSS-only solution: no JavaScript viewport listeners needed

**Alternatives Considered**:

- **Custom breakpoint (e.g., 640px or 1024px)**: Rejected - 768px is widely accepted standard
- **JavaScript `matchMedia`**: Rejected - CSS handles this efficiently; JS adds complexity
- **Multiple breakpoints (sm, md, lg)**: Rejected - feature only needs two layouts (mobile vs desktop)

**Implementation Approach**:

- Desktop sidebar: `class="hidden md:flex"` (hidden on mobile, flex on desktop)
- Mobile bottom bar: `class="md:hidden"` (visible on mobile, hidden on desktop)
- No JavaScript required: Tailwind generates responsive CSS at build time
- Smooth transitions: Add `transition-all duration-300` if visual feedback needed (debounce not required - CSS is instant)

**Edge Case**:

- Rapid viewport resizing: CSS handles instantly; no flicker due to Tailwind's mobile-first approach
- 320px minimum width: Test on smallest devices; ensure 5-slot bottom bar doesn't overflow (tap targets may need size reduction)

**References**:

- Tailwind Responsive Design: https://tailwindcss.com/docs/responsive-design
- Tailwind Default Breakpoints: https://tailwindcss.com/docs/screens

---

## 7. Component Architecture and Reusability

### Decision: Shared Navigation Item Type + Dedicated Components

**Rationale**:

- DRY principle: Define navigation items once, render in multiple contexts
- Type safety: TypeScript interface ensures consistency
- Maintainability: Adding/removing features updates single source of truth
- Separation of concerns: Desktop sidebar, mobile bottom bar, and More menu have distinct UX needs

**Data Structure** (from data-model.md):

```typescript
interface NavigationItem {
  id: string; // Unique identifier (e.g., 'home', 'collection')
  label: () => string; // Paraglide message function
  icon: typeof SvelteComponent; // lucide-svelte component
  href: string; // Route path (e.g., '/my-dashboard')
  isPrimary: boolean; // true = mobile bottom bar, false = More menu
}
```

**Component Breakdown**:

- **SidebarNavigation.svelte**: Desktop-only; renders all 9 features in vertical list
- **BottomNavigation.svelte**: Mobile-only; renders 4 primary features + More button
- **MoreMenu.svelte**: Mobile sheet/drawer; renders 4 secondary features
- **NavigationItem.svelte** (optional): Reusable list item component if complexity increases

**Implementation Approach**:

- Define navigation items array in centralized file (`src/lib/components/navigation/config.ts`)
- Import and filter items based on context (primary vs secondary)
- Pass items as props to each component for flexibility
- Each component handles its own active state styling

**Alternatives Considered**:

- **Single mega-component**: Rejected - violates separation of concerns; hard to maintain
- **Global navigation store**: Rejected - overkill for static navigation; props are simpler
- **Page-level navigation logic**: Rejected - navigation should be layout-level concern

**References**:

- Svelte 5 Components: https://svelte.dev/docs/svelte/legacy-component-api
- TypeScript with Svelte: https://svelte.dev/docs/typescript

---

## 8. Accessibility and Mobile Usability

### Decision: Follow WCAG 2.1 AA Guidelines

**Rationale**:

- Constitution mentions accessibility checks in component tests
- Mobile tap targets must be ≥44px (Apple/Google guidelines)
- Keyboard navigation required for desktop sidebar
- Screen reader support via semantic HTML and ARIA labels

**Accessibility Requirements**:

- **Tap Targets**: Minimum 44x44px on mobile bottom bar (current spec uses 64px height = h-16)
- **Keyboard Navigation**: Desktop sidebar items focusable via Tab; activated via Enter/Space
- **Semantic HTML**: Use `<nav>` for navigation containers, `<a>` for links (not `<button>`)
- **ARIA Labels**: Add `aria-label` to More button ("Open more features menu")
- **Focus Management**: Sheet component handles focus trapping when open
- **Color Contrast**: Active state must meet 4.5:1 contrast ratio (Tailwind `bg-primary` + `text-primary-foreground`)

**Implementation Approach**:

- Bottom bar: `class="h-16"` provides 64px height (exceeds 44px minimum)
- Desktop sidebar: `<a>` elements are keyboard-navigable by default
- More button: `<button aria-label={m.app_more_aria()}>` with screen reader text
- Active state: `aria-current="page"` on active navigation link
- Sheet: Use shadcn Sheet's built-in accessibility features (focus trap, ESC to close, backdrop dismissal)

**Testing Checklist**:

- [ ] Keyboard navigation works on desktop (Tab, Enter)
- [ ] Screen reader announces navigation items correctly
- [ ] Mobile tap targets are ≥44px (verify with browser dev tools)
- [ ] Color contrast meets WCAG AA (use contrast checker)
- [ ] More menu opens/closes with keyboard (Space/Enter) and touch

**References**:

- WCAG 2.1 AA: https://www.w3.org/WAI/WCAG21/quickref/
- Mobile Accessibility: https://www.w3.org/WAI/standards-guidelines/mobile/
- Touch Target Sizes: https://www.w3.org/WAI/WCAG21/Understanding/target-size.html

---

## 9. Performance Considerations

### Decision: CSS-First Responsive Design with Minimal JavaScript

**Rationale**:

- Spec requires: "<300ms navigation transitions, <200ms bottom sheet"
- CSS transitions are GPU-accelerated and performant
- Svelte 5 reactivity is optimized for minimal re-renders
- No heavy JavaScript listeners or computations

**Performance Optimizations**:

- **Responsive Switching**: CSS media queries (instant, no JavaScript)
- **Active State**: Derived state from `$page.url.pathname` (reactive but cheap)
- **Bottom Sheet**: shadcn Sheet uses CSS transforms (GPU-accelerated)
- **Icon Rendering**: lucide-svelte tree-shakes unused icons (small bundle)
- **Transitions**: `transition-all duration-300` for smooth visual feedback (under 300ms requirement)

**Potential Bottlenecks & Mitigations**:

- **Viewport Resize**: CSS handles instantly; no debouncing needed
- **Sheet Animation**: shadcn Sheet optimized; test on low-end devices if needed
- **Too Many Re-renders**: Use `$derived` instead of `$effect` to avoid unnecessary computations
- **Large Navigation List**: 9 items is small; no virtualization needed

**Testing Requirements**:

- Performance profiling in Chrome DevTools (measure transition times)
- Test on mobile devices (iOS Safari, Android Chrome)
- Verify no layout shift (CLS) during responsive breakpoint changes
- Ensure bottom sheet opens in <200ms (measure with Performance API if needed)

**References**:

- Svelte 5 Performance: https://svelte.dev/docs/svelte/reactivity
- CSS Transitions: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Transitions
- Web Performance: https://web.dev/vitals/

---

## 10. Migration Strategy and Backward Compatibility

### Decision: Incremental Migration with Feature Flag (Optional)

**Rationale**:

- Existing `SidebarNavigation.svelte` and `BottomNavigation.svelte` will be replaced
- Translation key changes require updates across components
- Low risk: navigation is isolated component, doesn't affect domain logic

**Migration Steps**:

1. **Add New Translation Keys**: Update `messages/en.json` and `messages/it.json`
2. **Update SidebarNavigation.svelte**: Rename features, update icons, ensure 9 items displayed
3. **Update BottomNavigation.svelte**: Implement 5-slot layout + More button
4. **Create MoreMenu.svelte**: Bottom sheet for secondary features
5. **Update Active State Logic**: Ensure More button active when on secondary feature routes
6. **Test Responsive Behavior**: Verify transitions at 768px breakpoint
7. **Remove Deprecated Keys**: Clean up old translation keys after migration

**Backward Compatibility**:

- **Routes**: No changes to routing structure (`/my-dashboard`, `/my-collection`, etc.)
- **State**: No state management changes; uses existing `$page` store
- **APIs**: No Tauri command changes (frontend-only)
- **Data**: No database migrations (frontend-only)

**Rollback Plan**:

- If issues arise, revert commits to restore old navigation
- Translation keys can coexist during migration (old + new)
- No data loss risk (frontend-only changes)

**References**:

- SvelteKit Backward Compatibility: https://svelte.dev/docs/kit/migrating

---

## Summary of Research Findings

| Research Area          | Decision                              | Key Rationale                                                        |
| ---------------------- | ------------------------------------- | -------------------------------------------------------------------- |
| Responsive Pattern     | Desktop Sidebar + Mobile Bottom Bar   | Maximizes feature visibility on desktop; mobile-optimized bottom bar |
| Bottom Sheet           | shadcn-svelte Sheet Component         | Accessible, integrated, handles focus/gestures out-of-the-box        |
| Icons                  | lucide-svelte with Centralized Config | Tree-shakable, consistent, already in project                        |
| Active State           | `$page.url.pathname` with `$derived`  | Reactive, built-in, type-safe                                        |
| Localization           | Paraglide-JS with New Keys            | Constitution-mandated; existing integration                          |
| Responsive Breakpoint  | Tailwind `md:` (768px)                | Industry standard; CSS-only solution                                 |
| Component Architecture | Shared Type + Dedicated Components    | DRY, type-safe, separation of concerns                               |
| Accessibility          | WCAG 2.1 AA + ≥44px Tap Targets       | Keyboard navigation, screen readers, mobile usability                |
| Performance            | CSS-First, Minimal JavaScript         | GPU-accelerated transitions, Svelte 5 reactivity, <300ms requirement |
| Migration              | Incremental with Translation Key Adds | Low risk, no routing changes, rollback-friendly                      |

**Next Steps**: Proceed to Phase 1 (data-model.md, contracts, quickstart.md) with research findings applied.
