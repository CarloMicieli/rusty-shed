# Phase 1: Data Model - Responsive Navigation System

**Feature**: Responsive Navigation System  
**Date**: February 5, 2026  
**Status**: Complete

## Overview

This document defines the data structures, state management, and type contracts for the responsive navigation system. The navigation system is purely frontend-based with no backend persistence, focusing on reactive state and type-safe component interactions.

---

## 1. Navigation Item Entity

### Purpose

Represents a single navigable feature in the application. Navigation items are used across desktop sidebar, mobile bottom bar, and More menu contexts.

### Structure

```typescript
/**
 * Represents a single navigation feature
 */
interface NavigationItem {
  /**
   * Unique identifier for the navigation item
   * @example 'home', 'collection', 'finance'
   */
  id: string;

  /**
   * Paraglide message function that returns the localized label
   * @example () => m.app_home()
   */
  label: () => string;

  /**
   * lucide-svelte icon component
   * @example LayoutDashboard, TrainFront, Wallet
   */
  icon: typeof SvelteComponent;

  /**
   * Route path for navigation
   * @example '/my-dashboard', '/my-collection'
   */
  href: string;

  /**
   * Determines if item appears in mobile bottom bar (true) or More menu (false)
   */
  isPrimary: boolean;

  /**
   * Optional: Badge count to display (e.g., wishlist count)
   */
  badgeCount?: number;

  /**
   * Optional: Use prefix matching for active state (for routes with subroutes)
   * @default false (uses exact match)
   * @example true for '/my-tracks' to match '/my-tracks/all', '/my-tracks/n-scale', etc.
   */
  usePrefixMatch?: boolean;
}
```

### Validation Rules

- `id`: Must be unique across all navigation items
- `label`: Must be a function returning a string (Paraglide message function)
- `icon`: Must be a valid Svelte component (lucide-svelte icon)
- `href`: Must start with `/` (absolute path)
- `isPrimary`: Boolean; exactly 4 items must have `isPrimary: true`

### Example Instances

```typescript
import {
  LayoutDashboard,
  TrainFront,
  Wallet,
  Heart,
  Wrench,
  Warehouse,
  Cpu,
  TrainTrack
} from 'lucide-svelte';
import * as m from '$lib/paraglide/messages.js';

const navigationItems: NavigationItem[] = [
  {
    id: 'home',
    label: () => m.app_home(),
    icon: LayoutDashboard,
    href: '/my-dashboard',
    isPrimary: true
  },
  {
    id: 'collection',
    label: () => m.app_collection(),
    icon: TrainFront,
    href: '/my-collection',
    isPrimary: true
  },
  {
    id: 'finance',
    label: () => m.app_finance(),
    icon: Wallet,
    href: '/my-budget',
    isPrimary: true
  },
  {
    id: 'wishlists',
    label: () => m.app_wishlists(),
    icon: Heart,
    href: '/my-wishlists',
    isPrimary: true,
    badgeCount: undefined // Set dynamically from wishlist context
  },
  {
    id: 'maintenance',
    label: () => m.app_maintenance(),
    icon: Wrench,
    href: '/my-maintenance',
    isPrimary: false
  },
  {
    id: 'depot',
    label: () => m.app_depot(),
    icon: Warehouse,
    href: '/my-depot',
    isPrimary: false
  },
  {
    id: 'digital-dcc',
    label: () => m.app_digital_dcc(),
    icon: Cpu,
    href: '/my-digital-roster',
    isPrimary: false
  },
  {
    id: 'railway-tracks',
    label: () => m.app_railway_tracks(),
    icon: TrainTrack,
    href: '/my-tracks',
    isPrimary: false,
    usePrefixMatch: true // Matches '/my-tracks/all', '/my-tracks/n-scale', etc.
  }
];
```

---

## 2. Viewport State

### Purpose

Tracks the current device/viewport context to determine which navigation layout to render.

### Structure

**Note**: This is a **derived** concept, not explicit state. Tailwind CSS handles viewport detection automatically via media queries. No JavaScript state management needed.

### CSS-Based Implementation

```svelte
<!-- Desktop Sidebar: visible at md (≥768px) and above -->
<nav class="hidden md:flex">...</nav>

<!-- Mobile Bottom Bar: visible below md (<768px) -->
<nav class="md:hidden">...</nav>
```

### Rationale

- **Performance**: CSS media queries are instant; no JavaScript overhead
- **Simplicity**: No state synchronization or event listeners required
- **Reliability**: Browser-native; handles rapid viewport changes gracefully

---

## 3. More Menu State

### Purpose

Controls the open/closed state of the bottom sheet/drawer containing secondary navigation features on mobile.

### Structure

```typescript
/**
 * More menu open/closed state
 * Managed by Svelte 5 $state rune in BottomNavigation.svelte
 */
let moreMenuOpen = $state<boolean>(false);
```

### State Transitions

```
CLOSED ──[tap More button]──> OPEN
OPEN ──[tap secondary feature]──> CLOSED + navigate to feature
OPEN ──[tap backdrop]──> CLOSED
OPEN ──[press ESC]──> CLOSED
OPEN ──[viewport resize to desktop]──> CLOSED (optional: auto-close on breakpoint change)
```

### Management

- **Where**: Local state in `BottomNavigation.svelte`
- **How**: Svelte 5 `$state` rune
- **Sharing**: Passed as prop to `MoreMenu.svelte` component

```svelte
<script lang="ts">
  let moreMenuOpen = $state(false);

  function toggleMoreMenu() {
    moreMenuOpen = !moreMenuOpen;
  }

  function closeMoreMenu() {
    moreMenuOpen = false;
  }
</script>

<button onclick={toggleMoreMenu}>More</button>

<MoreMenu open={moreMenuOpen} onClose={closeMoreMenu} items={secondaryItems} />
```

---

## 4. Active State Detection

### Purpose

Determines which navigation item is currently active based on the user's current route.

### Structure

**Derived State** (not stored):

```typescript
/**
 * Derived active state for a navigation item
 * @param item - Navigation item to check
 * @param pathname - Current route pathname (from $page.url.pathname)
 * @returns true if item is active
 */
function isActive(item: NavigationItem, pathname: string): boolean {
  if (item.usePrefixMatch) {
    return pathname.startsWith(item.href);
  }
  return pathname === item.href;
}
```

### Special Case: More Button Active State

The More button should show active state if **any** secondary feature is currently active.

```typescript
/**
 * Determines if More button should show active state
 * @param secondaryItems - List of secondary navigation items
 * @param pathname - Current route pathname
 * @returns true if any secondary feature is active
 */
function isMoreButtonActive(secondaryItems: NavigationItem[], pathname: string): boolean {
  return secondaryItems.some((item) => isActive(item, pathname));
}
```

### Implementation in Components

```svelte
<script lang="ts">
  import { page } from '$app/stores';

  const pathname = $derived($page.url.pathname as string);
  const itemActive = $derived(isActive(navItem, pathname));
</script>

<a href={navItem.href} class:bg-primary={itemActive} aria-current={itemActive ? 'page' : undefined}>
  ...
</a>
```

---

## 5. Translation Messages

### Purpose

Provides localized labels for all navigation features.

### Required Message Keys

Add to `messages/en.json` and `messages/it.json`:

```json
{
  "app_home": "Home",
  "app_collection": "Collection",
  "app_finance": "Finance",
  "app_wishlists": "Wishlists",
  "app_maintenance": "Maintenance",
  "app_depot": "Depot",
  "app_digital_dcc": "Digital (DCC)",
  "app_railway_tracks": "Railway Tracks",
  "app_more": "More",
  "app_more_aria": "Open more features menu"
}
```

### Italian Translations (`messages/it.json`):

```json
{
  "app_home": "Home",
  "app_collection": "Collezione",
  "app_finance": "Finanze",
  "app_wishlists": "Liste dei Desideri",
  "app_maintenance": "Manutenzione",
  "app_depot": "Deposito",
  "app_digital_dcc": "Digitale (DCC)",
  "app_railway_tracks": "Binari",
  "app_more": "Altro",
  "app_more_aria": "Apri menu funzionalità aggiuntive"
}
```

### Deprecated Keys (to remove after migration):

```json
// DEPRECATED - Remove after migration complete
{
  "app_dashboard": "Dashboard", // Use app_home instead
  "budget_title": "Budget Tracking", // Use app_finance instead
  "app_digital_roster": "My Digital Rolling Stocks", // Use app_digital_dcc instead
  "app_tracks": "My Tracks" // Use app_railway_tracks instead
}
```

---

## 6. Navigation Configuration

### Purpose

Centralized source of truth for all navigation items, ensuring consistency across desktop and mobile layouts.

### File Location

`src/lib/components/navigation/config.ts`

### Implementation

```typescript
import {
  LayoutDashboard,
  TrainFront,
  Wallet,
  Heart,
  Wrench,
  Warehouse,
  Cpu,
  TrainTrack
} from 'lucide-svelte';
import * as m from '$lib/paraglide/messages.js';
import type { NavigationItem } from './types';

/**
 * Complete navigation configuration for the application
 * Items are ordered as they should appear in the UI
 */
export const NAVIGATION_ITEMS: NavigationItem[] = [
  {
    id: 'home',
    label: () => m.app_home(),
    icon: LayoutDashboard,
    href: '/my-dashboard',
    isPrimary: true
  },
  {
    id: 'collection',
    label: () => m.app_collection(),
    icon: TrainFront,
    href: '/my-collection',
    isPrimary: true
  },
  {
    id: 'finance',
    label: () => m.app_finance(),
    icon: Wallet,
    href: '/my-budget',
    isPrimary: true
  },
  {
    id: 'wishlists',
    label: () => m.app_wishlists(),
    icon: Heart,
    href: '/my-wishlists',
    isPrimary: true
  },
  {
    id: 'maintenance',
    label: () => m.app_maintenance(),
    icon: Wrench,
    href: '/my-maintenance',
    isPrimary: false
  },
  {
    id: 'depot',
    label: () => m.app_depot(),
    icon: Warehouse,
    href: '/my-depot',
    isPrimary: false
  },
  {
    id: 'digital-dcc',
    label: () => m.app_digital_dcc(),
    icon: Cpu,
    href: '/my-digital-roster',
    isPrimary: false
  },
  {
    id: 'railway-tracks',
    label: () => m.app_railway_tracks(),
    icon: TrainTrack,
    href: '/my-tracks',
    isPrimary: false,
    usePrefixMatch: true
  }
];

/**
 * Filter helper: Get primary navigation items (mobile bottom bar)
 */
export const PRIMARY_ITEMS = NAVIGATION_ITEMS.filter((item) => item.isPrimary);

/**
 * Filter helper: Get secondary navigation items (More menu)
 */
export const SECONDARY_ITEMS = NAVIGATION_ITEMS.filter((item) => !item.isPrimary);
```

### Usage in Components

```svelte
<script lang="ts">
  import { NAVIGATION_ITEMS, PRIMARY_ITEMS, SECONDARY_ITEMS } from './navigation/config';

  // Desktop sidebar uses all items
  const desktopItems = NAVIGATION_ITEMS;

  // Mobile bottom bar uses only primary items
  const mobileItems = PRIMARY_ITEMS;

  // More menu uses only secondary items
  const moreMenuItems = SECONDARY_ITEMS;
</script>
```

---

## 7. Component State Summary

### SidebarNavigation.svelte

- **Input Props**: None (uses centralized config)
- **Local State**: None (fully derived from `$page` store and config)
- **External Dependencies**: `$page` store, `NAVIGATION_ITEMS` config, Paraglide messages

### BottomNavigation.svelte

- **Input Props**: None (uses centralized config)
- **Local State**: `moreMenuOpen` (boolean, controls More menu)
- **External Dependencies**: `$page` store, `PRIMARY_ITEMS` and `SECONDARY_ITEMS` config, Paraglide messages

### MoreMenu.svelte

- **Input Props**:
  - `open: boolean` - Controls visibility
  - `onClose: () => void` - Callback to close menu
  - `items: NavigationItem[]` - Secondary navigation items
- **Local State**: None (controlled component)
- **External Dependencies**: `$page` store (for active state), shadcn-svelte Sheet component

---

## 8. Data Flow Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                   Navigation Configuration                   │
│                   (config.ts - Source of Truth)              │
│  - NAVIGATION_ITEMS: All 9 features                         │
│  - PRIMARY_ITEMS: 4 primary features                        │
│  - SECONDARY_ITEMS: 4 secondary features                    │
└────────────────────┬────────────────────────────────────────┘
                     │
         ┌───────────┴───────────┐
         │                       │
         ▼                       ▼
┌─────────────────────┐  ┌─────────────────────┐
│ SidebarNavigation   │  │ BottomNavigation    │
│ (Desktop ≥768px)    │  │ (Mobile <768px)     │
│                     │  │                     │
│ Uses:               │  │ Uses:               │
│ - NAVIGATION_ITEMS  │  │ - PRIMARY_ITEMS     │
│   (all 9)           │  │   (4 primary)       │
│                     │  │ - moreMenuOpen      │
│ Derives:            │  │   (local $state)    │
│ - Active state from │  │                     │
│   $page.url.pathname│  │ Derives:            │
│                     │  │ - Active state from │
│                     │  │   $page.url.pathname│
│                     │  │ - More button active│
│                     │  │   from SECONDARY_   │
│                     │  │   ITEMS             │
└─────────────────────┘  └──────────┬──────────┘
                                    │
                                    ▼
                         ┌─────────────────────┐
                         │ MoreMenu            │
                         │ (Bottom Sheet)      │
                         │                     │
                         │ Props:              │
                         │ - open (boolean)    │
                         │ - onClose (fn)      │
                         │ - items (SECONDARY_ │
                         │   ITEMS)            │
                         │                     │
                         │ Derives:            │
                         │ - Active state from │
                         │   $page.url.pathname│
                         └─────────────────────┘
```

---

## 9. State Lifecycle and Transitions

### Application Load

1. `NAVIGATION_ITEMS` config is imported (static, no initialization needed)
2. Components render based on viewport (CSS media queries)
3. Active state is derived from initial route (`$page.url.pathname`)

### User Navigation

1. User clicks/taps navigation item
2. SvelteKit navigates to new route
3. `$page.url.pathname` updates reactively
4. Active state re-derives automatically (Svelte 5 `$derived`)
5. Components re-render with new active state

### More Menu Interaction (Mobile)

1. User taps "More" button → `moreMenuOpen = true`
2. Bottom sheet opens with secondary items
3. User taps secondary item:
   - SvelteKit navigates to route
   - `onClose()` callback fires → `moreMenuOpen = false`
   - Sheet closes
4. Alternative: User taps backdrop or presses ESC:
   - `onClose()` callback fires → `moreMenuOpen = false`
   - Sheet closes without navigation

### Viewport Resize

1. User resizes browser window crossing 768px breakpoint
2. CSS media queries instantly show/hide appropriate navigation
3. No JavaScript state updates needed
4. More menu automatically hidden on desktop (CSS: `md:hidden`)

---

## 10. Type Safety Guarantees

### Compile-Time Checks

- TypeScript ensures `NavigationItem` fields are correctly typed
- Svelte component type-checks icon props
- Paraglide message functions return strings (type-safe)

### Runtime Validation

- Development-only assertions (optional):
  ```typescript
  if (import.meta.env.DEV) {
    const primaryCount = NAVIGATION_ITEMS.filter((i) => i.isPrimary).length;
    if (primaryCount !== 4) {
      console.warn(`Expected 4 primary items, got ${primaryCount}`);
    }
  }
  ```

### Type Exports

All types exported from `src/lib/components/navigation/types.ts`:

```typescript
export type { NavigationItem };
export type { MoreMenuProps };
export type { NavigationItemProps };
```

---

## Summary

The responsive navigation system uses a **configuration-driven, type-safe, reactive** approach:

1. **Single Source of Truth**: `NAVIGATION_ITEMS` config defines all features
2. **Derived State**: Active state computed from `$page.url.pathname` (no manual tracking)
3. **CSS-First Responsiveness**: Tailwind media queries handle desktop/mobile switching
4. **Minimal Local State**: Only `moreMenuOpen` boolean for bottom sheet
5. **Type Safety**: TypeScript interfaces enforce structure; Paraglide ensures localized messages
6. **No Backend**: Purely frontend; no persistence, APIs, or Tauri commands

This approach ensures **maintainability** (add/remove features in one place), **performance** (minimal JavaScript, reactive updates), and **reliability** (type-checked, no manual state sync).
