---
name: svelte
description: Use this skill when writing Svelte components, working with Tailwind CSS, shadcn-svelte, SvelteKit routing, state management with Svelte 5 Runes, or when the user asks about frontend UI, components, styling, or mentions Svelte/SvelteKit. Apply when working in src/ directory or *.svelte/*.ts files.
version: 1.0.0
---

# Svelte 5 + Tailwind + shadcn-svelte Standards

This skill provides comprehensive guidelines for writing modern, reactive Svelte 5 applications with Tailwind CSS and shadcn-svelte components.

## When This Skill Applies

Use this skill when:

- Writing or modifying Svelte components in `src/` directory
- Building UI components, layouts, or pages
- Working with Svelte 5 Runes (`$state`, `$derived`, `$props`)
- Styling with Tailwind CSS or shadcn-svelte
- Implementing SvelteKit routing, forms, or server-side logic
- User mentions: Svelte, components, UI, frontend, styling, Tailwind, shadcn

## Project Context

- **Frontend Root**: `src/` contains all Svelte/UI code
- **Framework**: Svelte 5 with Runes API + SvelteKit
- **Styling**: Tailwind CSS 4 + shadcn-svelte component library
- **Architecture**: Feature-first structure in `src/lib/features/`
- **i18n**: Paraglide-JS for internationalization (no hardcoded strings)

## Mandatory Workflow

Before marking any Svelte task as complete:

1. **PLAN**: Detail component state, props, and UI structure
2. **IMPLEMENT**: Write component with TypeScript and proper typing
3. **DOCUMENT**: Add JSDoc for exported props and functions
4. **FORMAT**: Run `pnpm format`
5. **LINT**: Run `pnpm lint`
6. **TYPE CHECK**: Run `pnpm check`
7. **TEST**: Run `pnpm test` (add tests for new components)
8. **VERIFY**: Ensure all commands pass before completion

## Core Principles

### Svelte 5 Runes First

- **Always** use Svelte 5 Runes syntax for reactive state
- Use `$state()` for mutable reactive state
- Use `$derived()` for computed values
- Use `$props()` for component props with TypeScript types
- Use `$effect()` for side effects (sparingly)

### TypeScript Always

- Use `lang="ts"` in all `<script>` tags
- Define prop types with TypeScript interfaces
- Use type inference where possible
- Export types for reusable components

### Component Organization

- Keep components small and focused (single responsibility)
- Extract complex logic into `.svelte.ts` files
- Use feature-first structure: `src/lib/features/<feature>/`
- Each feature should contain: components, stores, API/Tauri logic

### No Hardcoded Strings

- **ALWAYS** use Paraglide-JS for user-facing text
- Import messages: `import * as m from '$lib/paraglide/messages'`
- Use message functions: `{m.welcomeMessage()}`
- Never hardcode strings in components or error messages

## Svelte 5 Patterns

### State Management

```svelte
<script lang="ts">
  // Reactive state
  let count = $state(0);

  // Derived state
  let doubled = $derived(count * 2);

  // Props with types
  let { title, items = [] }: { title: string; items?: Item[] } = $props();

  // Effects (use sparingly)
  $effect(() => {
    console.log('Count changed:', count);
  });
</script>
```

### Component Props

```svelte
<script lang="ts">
  interface Props {
    title: string;
    items: Item[];
    optional?: boolean;
  }

  let { title, items, optional = false }: Props = $props();
</script>
```

### Event Handlers

```svelte
<script lang="ts">
  function handleClick(event: MouseEvent) {
    // Handle event
  }

  // Or inline
  <button onclick={() => count++}>Increment</button>;
</script>
```

### Snippets (Reusable Markup)

```svelte
<script lang="ts">
  // Define reusable snippets
  {#snippet card(title: string, content: string)}
    <div class="card">
      <h3>{title}</h3>
      <p>{content}</p>
    </div>
  {/snippet}
</script>

{@render card('Title', 'Content')}
```

## Tailwind CSS Guidelines

### Utility-First Approach

- Use Tailwind utility classes for all styling
- Avoid custom CSS unless absolutely necessary
- Use Tailwind's design tokens (colors, spacing, typography)
- Leverage responsive modifiers: `sm:`, `md:`, `lg:`, `xl:`
- Use dark mode: `dark:` prefix

### Component Composition

```svelte
<div class="flex items-center gap-4 rounded-lg bg-white p-6 shadow-md dark:bg-gray-900">
  <img src={avatar} alt="" class="h-12 w-12 rounded-full" />
  <div class="flex-1">
    <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
      {name}
    </h3>
    <p class="text-sm text-gray-600 dark:text-gray-400">{role}</p>
  </div>
</div>
```

### Responsive Design

```svelte
<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
  <!-- Responsive grid -->
</div>
```

## shadcn-svelte Integration

### Using shadcn Components

- Import shadcn components from `$lib/components/ui/`
- Follow shadcn's composition patterns
- Customize via Tailwind classes, not CSS overrides
- Use shadcn primitives for accessibility

### Example Usage

```svelte
<script lang="ts">
  import { Button } from '$lib/components/ui/button';
  import { Card } from '$lib/components/ui/card';
  import * as m from '$lib/paraglide/messages';
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>{m.cardTitle()}</Card.Title>
  </Card.Header>
  <Card.Content>
    {m.cardContent()}
  </Card.Content>
  <Card.Footer>
    <Button onclick={handleAction}>{m.submitButton()}</Button>
  </Card.Footer>
</Card.Root>
```

## SvelteKit Patterns

### Page Structure

```svelte
<!-- +page.svelte -->
<script lang="ts">
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<h1>{data.title}</h1>
```

### Loading Data

```ts
// +page.ts or +page.server.ts
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch }) => {
  return {
    title: 'Page Title',
    items: await fetchItems()
  };
};
```

### Form Actions

```ts
// +page.server.ts
import type { Actions } from './$types';

export const actions: Actions = {
  default: async ({ request }) => {
    const data = await request.formData();
    // Handle form submission
    return { success: true };
  }
};
```

## Tauri Integration

### Invoking Tauri Commands

- Abstract Tauri calls into service/repository layers
- Keep components clean and framework-agnostic
- Handle errors gracefully

```ts
// src/lib/features/auth/authService.ts
import { invoke } from '@tauri-apps/api/core';

export async function login(email: string, password: string) {
  try {
    return await invoke<User>('login', { email, password });
  } catch (error) {
    // Handle error
    throw new Error('Login failed');
  }
}
```

```svelte
<!-- Component using service -->
<script lang="ts">
  import { login } from './authService';
  import * as m from '$lib/paraglide/messages';

  let email = $state('');
  let password = $state('');

  async function handleLogin() {
    try {
      await login(email, password);
    } catch (error) {
      // Show error message
    }
  }
</script>
```

## Feature-First Architecture

### Directory Structure

```
src/lib/features/
├── auth/
│   ├── components/
│   │   ├── LoginForm.svelte
│   │   └── RegisterForm.svelte
│   ├── stores/
│   │   └── authStore.svelte.ts
│   ├── services/
│   │   └── authService.ts
│   └── index.ts (exports)
├── dashboard/
│   ├── components/
│   ├── DashboardState.svelte.ts
│   └── index.ts
```

### Exports

```ts
// src/lib/features/auth/index.ts
export { default as LoginForm } from './components/LoginForm.svelte';
export { authStore } from './stores/authStore.svelte.ts';
export * from './services/authService';
```

## State Management

### Local Component State

```svelte
<script lang="ts">
  let count = $state(0);
  let items = $state<Item[]>([]);
</script>
```

### Shared State (.svelte.ts files)

```ts
// dashboardState.svelte.ts
export class DashboardState {
  summary = $state<Summary | null>(null);
  loading = $state(false);

  get hasData() {
    return $derived(this.summary !== null);
  }

  async loadSummary() {
    this.loading = true;
    try {
      this.summary = await invoke('get_dashboard_summary');
    } finally {
      this.loading = false;
    }
  }
}

// Create singleton instance
export const dashboardState = new DashboardState();
```

### Using Shared State

```svelte
<script lang="ts">
  import { dashboardState } from './dashboardState.svelte';
</script>

{#if dashboardState.loading}
  <Loading />
{:else if dashboardState.hasData}
  <Summary data={dashboardState.summary} />
{/if}
```

## Patterns to Follow

- Use Svelte 5 Runes (`$state`, `$derived`, `$props`)
- TypeScript for all logic
- Paraglide-JS for all text
- Tailwind utilities for styling
- shadcn-svelte for UI primitives
- Feature-first organization
- Abstract Tauri calls into services
- Keep components small and focused

## Patterns to Avoid

- **Don't** use old Svelte 3/4 reactivity (`$:`)
- **Don't** hardcode strings (use Paraglide-JS)
- **Don't** write custom CSS (use Tailwind)
- **Don't** put business logic in components (use services/stores)
- **Don't** invoke Tauri commands directly in components
- **Don't** create deeply nested components (flatten when possible)
- **Don't** use `any` types (use proper TypeScript typing)

## Testing

- Write unit tests with Vitest for logic
- Write component tests for complex components
- Write E2E tests with Playwright for critical flows
- Test accessibility with screen readers
- Test responsive behavior

## Accessibility

- Use semantic HTML elements
- Add proper ARIA labels where needed
- Ensure keyboard navigation works
- Test with screen readers
- Use shadcn-svelte components (built with accessibility)

## Performance

- Use `{#key}` blocks to force re-renders when needed
- Lazy load heavy components: `import('./Heavy.svelte')`
- Use `$derived` instead of recomputing in templates
- Avoid unnecessary effects
- Keep bundle size small

## Quality Checklist

Before submitting code:

- [ ] **Runes**: Uses Svelte 5 Runes syntax
- [ ] **TypeScript**: All scripts use `lang="ts"` with proper types
- [ ] **i18n**: No hardcoded strings (Paraglide-JS everywhere)
- [ ] **Styling**: Uses Tailwind utilities, no custom CSS
- [ ] **Components**: Small, focused, single responsibility
- [ ] **Logic**: Complex logic extracted to `.svelte.ts` files
- [ ] **Services**: Tauri calls abstracted into services
- [ ] **Tests**: Component tests added
- [ ] **Tooling**: Passes `pnpm format`, `pnpm lint`, `pnpm check`
- [ ] **Accessibility**: Semantic HTML, ARIA labels, keyboard navigation

## Resources

- [Svelte 5 Runes Documentation](https://svelte.dev/docs/svelte/$state)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [shadcn-svelte Documentation](https://www.shadcn-svelte.com/)
- [Paraglide-JS Documentation](https://inlang.com/m/gerre34r/library-inlang-paraglideJs)
