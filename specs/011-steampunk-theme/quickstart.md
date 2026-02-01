# Quickstart: Modern Steampunk Theme System

**Feature**: 011-steampunk-theme  
**Created**: 2026-01-30

## Overview

This guide covers implementing the dual "Modern Steampunk" theme system with light/dark variants, backend persistence, and themed UI components.

## Prerequisites

- Node.js 18+ and pnpm 10.x
- Rust 1.93.0+ with Tauri CLI
- Existing Rusty Shed development environment

## Quick Links

| Artifact      | Path                                                       |
| ------------- | ---------------------------------------------------------- |
| Specification | [spec.md](spec.md)                                         |
| Research      | [research.md](research.md)                                 |
| Data Model    | [data-model.md](data-model.md)                             |
| API Contract  | [contracts/theme-settings.ts](contracts/theme-settings.ts) |

---

## Phase 1: Database Migration

### 1.1 Create Migration File

```bash
# Create migration file
touch src-tauri/migrations/0007_add_theme_setting.sql
```

**Content:**

```sql
-- Add theme setting column with system as default
ALTER TABLE settings ADD COLUMN theme TEXT NOT NULL DEFAULT 'system';
```

### 1.2 Update Rust Settings Module

Modify `src-tauri/src/settings.rs`:

1. Add `ThemeValue` enum
2. Add `theme` field to `SettingsDto` and `UpdateSettingsPayload`
3. Update SQL queries to include `theme` column
4. Add `parse_theme()` helper function

---

## Phase 2: Theme CSS Files

### 2.1 Create Theme Directory

```bash
mkdir -p src/lib/themes
```

### 2.2 File Structure

```
src/lib/themes/
├── steampunk-base.css    # Shared textures, variants, fonts
├── steampunk-light.css   # Parchment & Brass tokens
└── steampunk-dark.css    # Iron & Copper tokens
```

### 2.3 Base Theme (steampunk-base.css)

Contains:

- Font family custom properties
- Texture gradient definitions
- `variant-steampunk-*` CSS classes
- Responsive texture disabling
- Focus/accessibility styles

### 2.4 Import in layout.css

```css
@import 'tailwindcss';
@plugin '@tailwindcss/forms';
@plugin '@tailwindcss/typography';

/* Steampunk themes (replace cerberus) */
@import '$lib/themes/steampunk-base.css';
@import '$lib/themes/steampunk-light.css';
@import '$lib/themes/steampunk-dark.css';

/* Skeleton base (no theme import) */
@import '@skeletonlabs/skeleton';
```

---

## Phase 3: Theme Store

### 3.1 Create Theme Store

**File:** `src/lib/stores/themeStore.svelte.ts`

```typescript
import { commands } from '$lib/bindings';

type ThemeValue = 'steampunk-light' | 'steampunk-dark' | 'system';
type ResolvedTheme = 'light' | 'dark';

function createThemeStore() {
  let current = $state<ThemeValue>('system');
  let resolved = $state<ResolvedTheme>('dark');
  let isLoading = $state(true);

  function resolveTheme(theme: ThemeValue): ResolvedTheme {
    if (theme === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return theme === 'steampunk-dark' ? 'dark' : 'light';
  }

  function applyTheme(theme: ResolvedTheme) {
    document.body.dataset.theme = `steampunk-${theme}`;
  }

  return {
    get current() {
      return current;
    },
    get resolved() {
      return resolved;
    },
    get isLoading() {
      return isLoading;
    },

    async initialize() {
      const result = await commands.getSettings();
      if (result.status === 'ok') {
        current = result.data.theme as ThemeValue;
        resolved = resolveTheme(current);
        applyTheme(resolved);
      }
      isLoading = false;
    },

    async setTheme(theme: ThemeValue) {
      // Optimistic update
      current = theme;
      resolved = resolveTheme(theme);
      applyTheme(resolved);

      // Persist (requires full settings payload)
      // Implementation depends on settings page integration
    }
  };
}

export const themeStore = createThemeStore();
```

### 3.2 Initialize in Layout

**File:** `src/routes/+layout.svelte`

```svelte
<script lang="ts">
  import { themeStore } from '$lib/stores/themeStore.svelte';
  import { onMount } from 'svelte';

  onMount(() => {
    themeStore.initialize();

    // Listen for OS theme changes when using 'system'
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = () => {
      if (themeStore.current === 'system') {
        themeStore.setTheme('system'); // Re-resolve
      }
    };
    mediaQuery.addEventListener('change', handleChange);

    return () => mediaQuery.removeEventListener('change', handleChange);
  });
</script>
```

---

## Phase 4: Font Loading

### 4.1 Update app.html

```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />

    <!-- Google Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link
      href="https://fonts.googleapis.com/css2?family=Cinzel+Decorative:wght@400;700&family=Courier+Prime:wght@400;700&family=Spectral:wght@400;500;600&display=swap"
      rel="stylesheet"
    />

    %sveltekit.head%
  </head>
  <body data-sveltekit-preload-data="hover" data-theme="steampunk-dark">
    <div style="display: contents">%sveltekit.body%</div>
  </body>
</html>
```

---

## Phase 5: Steampunk Components

### 5.1 Component Directory

```bash
mkdir -p src/lib/components/steampunk
```

### 5.2 RivetedCard Example

**File:** `src/lib/components/steampunk/RivetedCard.svelte`

```svelte
<script lang="ts">
  interface Props {
    variant?: 'panel' | 'plate' | 'frame';
    rivets?: 'corners' | 'edges' | 'none';
    padding?: 'none' | 'sm' | 'md' | 'lg';
    class?: string;
  }

  let {
    variant = 'panel',
    rivets = 'corners',
    padding = 'md',
    class: className = '',
    children
  } = $props<Props>();

  const paddingClasses = {
    none: '',
    sm: 'p-2',
    md: 'p-4',
    lg: 'p-6'
  };
</script>

<div
  class="variant-steampunk-riveted {paddingClasses[padding]} {className}"
  data-variant={variant}
  data-rivets={rivets}
>
  {@render children?.()}
</div>
```

### 5.3 Barrel Export

**File:** `src/lib/components/steampunk/index.ts`

```typescript
export { default as RivetedCard } from './RivetedCard.svelte';
export { default as ToggleValve } from './ToggleValve.svelte';
export { default as PressureGauge } from './PressureGauge.svelte';
export { default as RailDivider } from './RailDivider.svelte';
```

---

## Phase 6: Settings Page Integration

### 6.1 Add Theme Selector

**File:** `src/routes/my-settings/+page.svelte` (excerpt)

```svelte
<script lang="ts">
  import { themeStore } from '$lib/stores/themeStore.svelte';
  import * as m from '$lib/paraglide/messages';
</script>

<label class="label">
  <span>{m.settings_theme_label()}</span>
  <select
    class="select"
    value={themeStore.current}
    onchange={(e) => themeStore.setTheme(e.currentTarget.value)}
  >
    <option value="system">{m.settings_theme_system()}</option>
    <option value="steampunk-light">{m.settings_theme_light()}</option>
    <option value="steampunk-dark">{m.settings_theme_dark()}</option>
  </select>
</label>
```

---

## Phase 7: Localization

### 7.1 Add Theme Messages

**File:** `messages/en.json` (add entries)

```json
{
  "settings_theme_label": "Theme",
  "settings_theme_system": "System",
  "settings_theme_light": "Parchment & Brass",
  "settings_theme_dark": "Iron & Copper"
}
```

**File:** `messages/it.json` (add entries)

```json
{
  "settings_theme_label": "Tema",
  "settings_theme_system": "Sistema",
  "settings_theme_light": "Pergamena e Ottone",
  "settings_theme_dark": "Ferro e Rame"
}
```

---

## Verification Commands

```bash
# Frontend checks
pnpm format
pnpm lint
pnpm check
pnpm test

# Rust checks
pnpm rust:fmt
pnpm rust:clippy
pnpm rust:test

# Run app
pnpm tauri dev
```

## Testing Checklist

- [ ] Theme persists after app restart
- [ ] System theme detection works
- [ ] Light theme displays correctly
- [ ] Dark theme displays correctly
- [ ] All text passes contrast check
- [ ] Textures disabled on mobile
- [ ] Reduced motion respected
- [ ] Fonts load without FOUT
