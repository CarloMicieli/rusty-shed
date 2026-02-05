# shadcn-svelte Components

This directory contains shadcn-svelte UI components for the Rusty Shed application.

## Usage

Components can be imported directly from this directory or through the component index:

```typescript
// Direct import
import { Button } from '$lib/components/shadcn/button';

// Or via component index (once created)
import { Button } from '$lib/components';
```

## Component Organization

Each component follows the shadcn-svelte structure:

- Individual component folders (e.g., `button/`, `card/`, `dialog/`)
- Index files for convenient re-exports
- TypeScript types included

## Steampunk Theme Integration

All shadcn-svelte components are styled using our custom Steampunk theme system:

- Theme CSS variables are defined in `src/lib/themes/steampunk-base.css`
- Light theme: `steampunk-light.css` (Parchment & Brass)
- Dark theme: `steampunk-dark.css` (Iron & Copper)

The theme is automatically applied via the theme store in `src/lib/stores/themeStore.svelte.ts`.

## Adding New Components

To add a new shadcn-svelte component:

1. Use the shadcn-svelte CLI (if available):

   ```bash
   npx shadcn-svelte@latest add [component-name]
   ```

2. Or manually create the component in this directory following the shadcn-svelte patterns

3. Export it from the component index (`src/lib/components/index.ts`)

4. Ensure it respects the Steampunk theme CSS variables

## Customization

Components can be customized via:

- **Props**: Standard Svelte component properties
- **Classes**: Tailwind utility classes for styling overrides
- **Slots**: For content injection and layout customization
- **Theme variables**: Modify `src/lib/themes/` files for global changes

## Best Practices

- Always use TypeScript (`lang="ts"` in script tags)
- Keep components small and focused
- Document exported props with JSDoc
- Test components with Vitest
- Follow Svelte 5 Runes patterns (`$state`, `$derived`, `$props`)
