# Quickstart Guide: Responsive Navigation System

**Feature**: Responsive Navigation System  
**Date**: February 5, 2026  
**For**: Developers implementing or maintaining the navigation components

---

## Overview

The responsive navigation system provides a **desktop sidebar** (all features visible) and a **mobile bottom bar** (4 primary features + More menu for secondary features). This guide helps you understand, implement, and extend the navigation system.

### Key Concepts

- **Desktop**: Full sidebar with 9 features visible at ≥768px (`md:` breakpoint)
- **Mobile**: Bottom bar with 5 slots (4 primary + More button) at <768px
- **More Menu**: Bottom sheet on mobile containing 4 secondary features
- **Configuration-Driven**: All navigation items defined in a single source of truth
- **Type-Safe**: TypeScript interfaces ensure correctness
- **Localized**: All labels via Paraglide-JS

---

## Quick Navigation

- [Getting Started](#getting-started) - Set up navigation in your project
- [Adding a New Feature](#adding-a-new-feature) - Add a navigation item
- [Customizing Styles](#customizing-styles) - Adjust appearance
- [Testing](#testing) - Write tests for navigation
- [Troubleshooting](#troubleshooting) - Common issues and solutions

---

## Getting Started

### Prerequisites

- Node.js and pnpm installed
- SvelteKit project with Svelte 5
- Tailwind CSS 4.x configured
- shadcn-svelte components installed
- Paraglide-JS for localization

### Installation Steps

**1. Install Dependencies** (if not already installed)

```bash
pnpm add lucide-svelte
# shadcn-svelte Sheet component
pnpm exec shadcn-svelte@latest add sheet
```

**2. Create Navigation Configuration**

Create `src/lib/components/navigation/config.ts`:

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

export const PRIMARY_ITEMS = NAVIGATION_ITEMS.filter((item) => item.isPrimary);
export const SECONDARY_ITEMS = NAVIGATION_ITEMS.filter((item) => !item.isPrimary);
```

**3. Create Type Definitions**

Create `src/lib/components/navigation/types.ts`:

```typescript
import type { ComponentType } from 'svelte';

export interface NavigationItem {
  id: string;
  label: () => string;
  icon: ComponentType;
  href: string;
  isPrimary: boolean;
  badgeCount?: number;
  usePrefixMatch?: boolean;
}

export interface MoreMenuProps {
  open: boolean;
  onClose: () => void;
  items: NavigationItem[];
}
```

**4. Add Translation Keys**

Update `messages/en.json`:

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

Update `messages/it.json` with Italian translations.

**5. Implement Components**

See [Component Implementation](#component-implementation) section below.

**6. Add to Layout**

In `src/routes/+layout.svelte`:

```svelte
<script lang="ts">
  import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';
  import BottomNavigation from '$lib/components/BottomNavigation.svelte';
</script>

<div class="flex h-screen">
  <SidebarNavigation />

  <main class="flex-1 overflow-auto">
    <slot />
  </main>

  <BottomNavigation />
</div>
```

---

## Component Implementation

### SidebarNavigation.svelte (Desktop)

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { NAVIGATION_ITEMS } from './navigation/config';
  import { localeStore } from '$lib/stores/locale';

  const locale = $derived($localeStore);
  const pathname = $derived($page.url.pathname as string);

  function isActive(item: NavigationItem): boolean {
    return item.usePrefixMatch ? pathname.startsWith(item.href) : pathname === item.href;
  }
</script>

{#key locale}
  <nav class="hidden h-full w-64 flex-col border-r border-border bg-sidebar p-4 md:flex">
    <div class="mb-8 flex items-center gap-3 px-4">
      <TrainFront class="text-primary" size={32} />
      <h2 class="h3 font-bold tracking-tight text-sidebar-foreground uppercase">
        {m.app_name()}
      </h2>
    </div>

    <ul class="space-y-2">
      {#each NAVIGATION_ITEMS as item (item.id)}
        {@const active = isActive(item)}
        <li>
          <a
            href={resolve(item.href)}
            class="flex w-full items-center justify-start gap-3 rounded-lg px-4 py-2.5 text-sm font-medium transition-colors"
            class:bg-primary={active}
            class:text-primary-foreground={active}
            class:text-sidebar-foreground={!active}
            class:hover:bg-sidebar-accent={!active}
            aria-current={active ? 'page' : undefined}
          >
            <svelte:component this={item.icon} size={20} />
            <span class="tracking-wide">{item.label()}</span>
            {#if item.badgeCount}
              <Badge variant="outline" class="ml-auto">{item.badgeCount}</Badge>
            {/if}
          </a>
        </li>
      {/each}
    </ul>
  </nav>
{/key}
```

### BottomNavigation.svelte (Mobile)

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { Ellipsis } from 'lucide-svelte';
  import { PRIMARY_ITEMS, SECONDARY_ITEMS } from './navigation/config';
  import MoreMenu from './navigation/MoreMenu.svelte';
  import * as m from '$lib/paraglide/messages.js';
  import { localeStore } from '$lib/stores/locale';

  const locale = $derived($localeStore);
  const pathname = $derived($page.url.pathname as string);

  let moreMenuOpen = $state(false);

  function isActive(item: NavigationItem): boolean {
    return item.usePrefixMatch ? pathname.startsWith(item.href) : pathname === item.href;
  }

  const moreButtonActive = $derived(SECONDARY_ITEMS.some((item) => isActive(item)));

  function toggleMoreMenu() {
    moreMenuOpen = !moreMenuOpen;
  }

  function closeMoreMenu() {
    moreMenuOpen = false;
  }
</script>

<div class="fixed right-0 bottom-0 left-0 z-50 border-t-2 border-primary bg-card md:hidden">
  {#key locale}
    <div class="flex h-16 items-center justify-around">
      {#each PRIMARY_ITEMS as item (item.id)}
        {@const active = isActive(item)}
        <a
          href={resolve(item.href)}
          class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95"
          class:text-primary={active}
          class:text-muted-foreground={!active}
          aria-current={active ? 'page' : undefined}
        >
          {#if active}
            <div
              class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
            ></div>
          {/if}
          <svelte:component this={item.icon} size={20} />
          <span class="text-[10px] font-bold tracking-wider uppercase">{item.label()}</span>
        </a>
      {/each}

      <button
        onclick={toggleMoreMenu}
        class="relative flex h-full w-full flex-col items-center justify-center gap-1 transition-all active:scale-95"
        class:text-primary={moreButtonActive}
        class:text-muted-foreground={!moreButtonActive}
        aria-label={m.app_more_aria()}
      >
        {#if moreButtonActive}
          <div
            class="absolute top-0 left-1/2 h-0.5 w-8 -translate-x-1/2 rounded-b-full bg-primary"
          ></div>
        {/if}
        <Ellipsis size={20} />
        <span class="text-[10px] font-bold tracking-wider uppercase">{m.app_more()}</span>
      </button>
    </div>
  {/key}
</div>

<MoreMenu open={moreMenuOpen} onClose={closeMoreMenu} items={SECONDARY_ITEMS} />
```

### MoreMenu.svelte (Bottom Sheet)

```svelte
<script lang="ts">
  import { page } from '$app/stores';
  import { resolve } from '$app/paths';
  import { Sheet, SheetContent } from '$lib/components/ui/sheet';
  import type { MoreMenuProps } from './navigation/types';

  let { open, onClose, items }: MoreMenuProps = $props();

  const pathname = $derived($page.url.pathname as string);

  function isActive(item: NavigationItem): boolean {
    return item.usePrefixMatch ? pathname.startsWith(item.href) : pathname === item.href;
  }

  function handleItemClick() {
    onClose();
  }
</script>

<Sheet {open} onOpenChange={(isOpen) => !isOpen && onClose()}>
  <SheetContent side="bottom" class="p-6">
    <ul class="space-y-2">
      {#each items as item (item.id)}
        {@const active = isActive(item)}
        <li>
          <a
            href={resolve(item.href)}
            onclick={handleItemClick}
            class="flex w-full items-center gap-3 rounded-lg px-4 py-3 text-sm font-medium transition-colors"
            class:bg-primary={active}
            class:text-primary-foreground={active}
            class:hover:bg-accent={!active}
            aria-current={active ? 'page' : undefined}
          >
            <svelte:component this={item.icon} size={20} />
            <span>{item.label()}</span>
          </a>
        </li>
      {/each}
    </ul>
  </SheetContent>
</Sheet>
```

---

## Adding a New Feature

### Step 1: Add Translation Keys

Update `messages/en.json` and `messages/it.json`:

```json
{
  "app_my_new_feature": "My New Feature"
}
```

### Step 2: Import Icon

In `config.ts`:

```typescript
import { MyNewIcon } from 'lucide-svelte';
```

### Step 3: Add to NAVIGATION_ITEMS

In `config.ts`, add to the array:

```typescript
{
  id: 'my-new-feature',
  label: () => m.app_my_new_feature(),
  icon: MyNewIcon,
  href: '/my-new-feature',
  isPrimary: false  // or true for mobile bottom bar
}
```

**Important**: If adding a primary item, ensure exactly 4 items have `isPrimary: true`.

### Step 4: Create Route

Create the route file: `src/routes/my-new-feature/+page.svelte`

### Step 5: Test

```bash
pnpm run dev
# Navigate to desktop and mobile views to verify
```

---

## Customizing Styles

### Active State Colors

Active state uses `bg-primary` and `text-primary-foreground` from your Tailwind theme.

To customize, update `tailwind.config.js`:

```javascript
export default {
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: 'hsl(var(--primary))',
          foreground: 'hsl(var(--primary-foreground))'
        }
      }
    }
  }
};
```

### Icon Sizes

Desktop sidebar: `size={20}`  
Mobile bottom bar: `size={20}` (compact)  
More menu: `size={20}` (consistent)

Adjust in component templates if needed.

### Bottom Bar Height

Current: `h-16` (64px)  
Minimum for accessibility: 44px tap targets

Change in `BottomNavigation.svelte`:

```svelte
<div class="flex h-20 items-center justify-around">
```

---

## Testing

### Unit Tests

Create `src/__tests__/components/navigation/SidebarNavigation.test.ts`:

```typescript
import { render } from '@testing-library/svelte';
import { expect, test } from 'vitest';
import SidebarNavigation from '$lib/components/SidebarNavigation.svelte';

test('renders all navigation items', () => {
  const { container } = render(SidebarNavigation);
  const navItems = container.querySelectorAll('nav li');
  expect(navItems.length).toBe(9); // All items
});

test('applies active state to current route', () => {
  // Mock $page store with current route
  // Assert active styles applied
});
```

### Manual Testing Checklist

- [ ] Desktop sidebar shows all 9 features
- [ ] Mobile bottom bar shows 4 primary features + More
- [ ] More menu opens with 4 secondary features
- [ ] Active state highlights correctly on all viewports
- [ ] Navigation works with keyboard (Tab, Enter)
- [ ] Tap targets are ≥44px on mobile
- [ ] Localization updates when language changes
- [ ] Responsive transition is smooth at 768px breakpoint

---

## Troubleshooting

### More Menu Not Opening

**Symptom**: Tapping More button does nothing.

**Solution**: Check that `moreMenuOpen` state is toggled:

```svelte
function toggleMoreMenu() {
  moreMenuOpen = !moreMenuOpen;
  console.log('More menu open:', moreMenuOpen); // Debug
}
```

### Active State Not Updating

**Symptom**: Navigation item stays active after navigating away.

**Solution**: Ensure `pathname` is derived from `$page` store:

```svelte
const pathname = $derived($page.url.pathname as string);
```

### Icons Not Rendering

**Symptom**: Navigation items show no icons.

**Solution**:

1. Verify lucide-svelte is installed: `pnpm add lucide-svelte`
2. Check icon import in `config.ts`
3. Ensure `svelte:component this={item.icon}` syntax is correct

### Bottom Bar Hidden on Mobile

**Symptom**: Bottom bar doesn't appear on small screens.

**Solution**: Check Tailwind class: `md:hidden` (visible on mobile, hidden on desktop).

### Translation Keys Not Working

**Symptom**: Labels show function names instead of text.

**Solution**:

1. Run `pnpm run prepare` to compile Paraglide messages
2. Ensure message keys exist in `messages/en.json`
3. Verify import: `import * as m from '$lib/paraglide/messages.js';`

---

## Performance Tips

### Minimize Re-renders

Use `$derived` instead of `$effect`:

```svelte
// Good: Reactive derivation
const pathname = $derived($page.url.pathname);

// Avoid: Side effect (triggers on every change)
$effect(() => {
  pathname = $page.url.pathname;
});
```

### Optimize Icon Imports

lucide-svelte is tree-shakable. Only import icons you use:

```typescript
// Good: Named imports (tree-shakable)
import { LayoutDashboard, TrainFront } from 'lucide-svelte';

// Avoid: Default import (bundles all icons)
import * as Icons from 'lucide-svelte';
```

### Avoid Inline Functions in Templates

```svelte
<!-- Good: Define function in <script> -->
<button onclick={toggleMoreMenu}>More</button>

<!-- Avoid: Inline arrow function (creates new function on each render) -->
<button onclick={() => (moreMenuOpen = !moreMenuOpen)}>More</button>
```

---

## Architecture Decisions

### Why Configuration-Driven?

**Benefit**: Add/remove features by editing `config.ts` only. No need to update multiple components.

### Why Separate Desktop and Mobile Components?

**Benefit**: Different UX requirements. Desktop sidebar and mobile bottom bar have distinct layouts, behaviors, and styling. Separation keeps code clean.

### Why CSS-Only Responsiveness?

**Benefit**: Faster than JavaScript viewport listeners. Browser-native media queries handle breakpoint changes instantly.

### Why shadcn-svelte Sheet?

**Benefit**: Accessible, keyboard-navigable, focus-trapped bottom sheet with minimal code. Integrates with existing design system.

---

## Further Reading

- [Svelte 5 Documentation](https://svelte.dev/docs/svelte)
- [SvelteKit Routing](https://svelte.dev/docs/kit/routing)
- [Tailwind Responsive Design](https://tailwindcss.com/docs/responsive-design)
- [shadcn-svelte Components](https://www.shadcn-svelte.com/)
- [lucide-svelte Icons](https://lucide.dev/guide/packages/lucide-svelte)
- [Paraglide-JS Localization](https://inlang.com/m/gerre34r/library-inlang-paraglideJs)

---

## Questions or Issues?

- **Architecture questions**: Refer to `research.md` and `data-model.md`
- **Type definitions**: See `contracts/navigation-types.ts`
- **Implementation details**: Check `plan.md`

For bugs or feature requests, open an issue in the project repository.
