---
name: Svelte UI Standards
description: This file describes the Svelte UI code style and workflow standards for the project.
applyTo: '**/*.{svelte,ts}'
---

# Svelte UI Standards

## Scope

Apply these rules to all files within the `src/` directory.

## Workflow Requirements

1. **Plan:** Detail the component state and props before implementation.
2. **Testing:** Add Vitest or Playwright tests for new components.
3. **Document:** Use JSDoc for exported props and functions.
4. **Prettify:** Run `pnpm format`.
5. **Lint:** Run `pnpm lint`.
6. **Type Check:** Run `pnpm check`.
7. **Verify:** Ensure all tests pass with `pnpm test`.

## Technical Preferences

- Use Svelte 5 Runes (`$state`, `$derived`, `$props`) where applicable.
- Use TypeScript for all `<script>` tags (`lang="ts"`).
- Keep components small; extract logic into `.svelte.ts` files if complex.
