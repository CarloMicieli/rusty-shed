# Svelte 5 UI Expert — Project Memory

## Paraglide i18n Workflow

- **Source files**: `/home/carlo/Projects/rusty-shed/messages/en.json` and `messages/it.json`
- **Generated files**: `src/lib/paraglide/messages/en.js` and `it.js` (auto-compiled, do not edit directly)
- **Compile step**: After editing the JSON source files, run `pnpm prepare` to regenerate the `.js` files.
  Without this step, `pnpm check` will report "Property does not exist" errors for all new keys.
- **Key format**: `snake_case`, feature-prefixed (e.g. `track_purchase_field_*`, `wishlists_sidebar_*`)
- **Import alias**: `import * as m from '$lib/paraglide/messages'` (some older files use `.js` suffix — both work)
- **Parameterised keys**: Use `{param}` in the JSON value, call as `m.key({ param: value })` in Svelte

## Key File Locations

- Messages source: `/home/carlo/Projects/rusty-shed/messages/en.json` + `it.json`
- Paraglide generated: `/home/carlo/Projects/rusty-shed/src/lib/paraglide/messages/`
- Type bindings (Tauri Specta): `/home/carlo/Projects/rusty-shed/src/lib/bindings.ts`
- Component entry points: `src/lib/features/*/` and `src/lib/components/`

## Component Conventions

- Stats/Summary cards: `card gauge-frame` with `ring-1 ring-border/40`
- Content sections: `rounded-lg border border-white/10 bg-black/20 p-4`
- Search inputs: `flex items-center` layout (Icon → Input flex-1 → Clear button)

## Verification Order

1. Edit `messages/en.json` + `messages/it.json`
2. Run `pnpm prepare` (paraglide compile)
3. Edit Svelte files to use `m.*()` calls
4. Run `pnpm check` (0 errors)
5. Run `pnpm lint` (0 warnings)
