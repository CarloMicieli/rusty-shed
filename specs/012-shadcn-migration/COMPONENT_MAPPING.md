# Skeleton to shadcn-svelte Component Mapping

**Feature**: 012-shadcn-migration  
**Created**: 2026-02-05  
**Status**: In Progress

## Component Audit Summary

### Skeleton UI Classes Found

- **Buttons**: `btn`, `btn-sm`, `btn-icon`, `btn-icon-sm`
- **Variants**: `variant-filled-*`, `variant-soft-*`, `variant-ghost-*`, `variant-glass-*`
- **Cards**: `card`
- **Badges**: `badge`, `badge-icon`
- **Alerts**: `alert`

### Components Already Migrated

- ✅ **Accordion** - Custom implementation using Svelte 5 context API
- ✅ **Toast** - Custom ToastProvider with top-right positioning
- ✅ **Modal** - Custom modal store with Skeleton-compatible API

## Mapping Table

| Skeleton Component       | shadcn-svelte Equivalent     | Status     | Notes                                   |
| ------------------------ | ---------------------------- | ---------- | --------------------------------------- |
| **Layout & Structure**   |
| `card` class             | Tailwind utilities           | ⏳ Pending | Use bg-card, border, rounded-lg, shadow |
| `variant-filled-surface` | bg-surface/bg-card           | ⏳ Pending | Map to theme variables                  |
| `variant-soft-*`         | bg-{color}/10                | ⏳ Pending | Soft backgrounds with opacity           |
| `variant-ghost-*`        | hover:bg-{color}/10          | ⏳ Pending | Transparent with hover                  |
| **Buttons**              |
| `btn` class              | `<Button>` component         | ⏳ Pending | Need to create Button component         |
| `btn-sm`                 | Button size="sm"             | ⏳ Pending | Size variant                            |
| `btn-icon`               | Button variant="ghost"       | ⏳ Pending | Icon button variant                     |
| `variant-filled-primary` | Button variant="default"     | ⏳ Pending | Primary button                          |
| `variant-ghost-surface`  | Button variant="ghost"       | ⏳ Pending | Ghost button                            |
| `variant-ghost-error`    | Button variant="destructive" | ⏳ Pending | Destructive action                      |
| **Badges**               |
| `badge` class            | `<Badge>` component          | ⏳ Pending | Need to create Badge component          |
| `variant-soft-*`         | Badge variant="secondary"    | ⏳ Pending | Soft badge variant                      |
| `variant-filled-*`       | Badge variant="default"      | ⏳ Pending | Filled badge variant                    |
| **Forms**                |
| FormField                | shadcn Form                  | ✅ Done    | Already has FormField.svelte            |
| Input fields             | shadcn Input                 | ⏳ Pending | Need Input component                    |
| Select/Dropdown          | shadcn Select                | ⏳ Pending | Need Select component                   |
| **Dialogs & Overlays**   |
| Modal (store)            | Dialog component             | ✅ Done    | Modal store exists                      |
| Drawer                   | Sheet component              | ⏳ Pending | Need Sheet component                    |
| **Alerts**               |
| `alert` class            | Alert component              | ⏳ Pending | Need Alert component                    |
| **Data Display**         |
| Table elements           | Table component              | ⏳ Pending | Need Table component                    |

## Priority Migration Order

### Phase 1: Core UI Primitives (Current)

1. **Button Component** - Most widely used (50+ instances)
2. **Badge Component** - Tag/status display (30+ instances)
3. **Card Styling** - Layout containers (20+ instances)

### Phase 2: Form Components

4. Input
5. Select/Dropdown
6. Textarea
7. Checkbox/Radio

### Phase 3: Complex Components

8. Dialog/Sheet (for drawers)
9. Alert
10. Table

### Phase 4: Navigation & Misc

11. Navigation components (if any Skeleton-specific)
12. Remaining edge cases

## Implementation Strategy

### 1. Create shadcn Components

Install or create the following shadcn-svelte components:

- Button
- Badge
- Input
- Select
- Textarea
- Checkbox
- Radio
- Dialog
- Sheet
- Alert
- Table

### 2. Create Variant Mapping Utilities

Create helper utilities to map Skeleton variant classes to shadcn equivalents:

```typescript
// src/lib/utils/variant-map.ts
export function mapVariantToButton(variant: string): ButtonVariant {
  if (variant.includes('filled-primary')) return 'default';
  if (variant.includes('ghost')) return 'ghost';
  if (variant.includes('error') || variant.includes('destructive')) return 'destructive';
  // ... more mappings
}
```

### 3. Gradual Migration

- Replace components file by file
- Test each component thoroughly
- Maintain visual parity with Steampunk theme

## Steampunk Theme Integration

All shadcn-svelte components will respect the Steampunk theme via CSS variables:

- `--color-primary-*` → Brass/Copper colors
- `--color-surface-*` → Background surfaces
- `--color-accent-*` → Furnace orange highlights
- Custom variant classes preserved via Tailwind

## Testing Checklist per Component

- [ ] Visual appearance matches original
- [ ] Interactions work (hover, focus, active states)
- [ ] Accessibility (ARIA labels, keyboard navigation)
- [ ] Responsive design (mobile, tablet, desktop)
- [ ] Dark/Light mode switching
- [ ] Steampunk theme variables applied
