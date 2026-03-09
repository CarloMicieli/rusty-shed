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
- `RailwayModel` + `RollingStock` types: `src/lib/types/railway-model.ts`

## Component Conventions

- Stats/Summary cards: `card gauge-frame` with `ring-1 ring-border/40`
- Content sections: `rounded-lg border border-white/10 bg-black/20 p-4`
- Search inputs: `flex items-center` layout (Icon → Input flex-1 → Clear button)

## Runes Controller Pattern (`.svelte.ts`)

For components with non-trivial async state (e.g. form state + API calls), extract logic into a
`use*.svelte.ts` file that uses `$state` runes and returns plain objects.

Key rules:
- Pass reactive props as **getter closures** `() => prop`, NOT as direct values — avoids Svelte's
  "reference only captures the initial value" warning (svelte.dev/e/state_referenced_locally).
- Same applies to callbacks from `$props()`: wrap them `() => onCallback?.()` so Svelte can track
  reactivity through the closure.
- Return `{ formState, specLoaded, loadX, saveX }` — callers use `rs.x` notation.
- Example: `src/lib/components/model-details/useRollingStockEditor.svelte.ts`

## `as const` Caution with shadcn-svelte

- Do NOT use `as const` on option arrays passed to shadcn-svelte components (e.g. `BadgePicker`).
  The component expects mutable `Option[]`, and `readonly` arrays will cause TypeScript errors.
- Correct: type-annotate explicitly `const opts: { id: string; label: string }[] = [...]`

## `$derived` Cannot Be Assigned

- `$derived` values are read-only. A pattern like `let x = $derived(...)` followed by `x = newValue`
  inside a save handler is invalid. After a mutation, call `onModelUpdated?.()` to refresh the
  parent prop that feeds the derived.

## Snippet-as-child Pattern for Generic Row Components (SpecRow)

When a component's only job is "label + arbitrary value content", use `Snippet` children:
- `SpecRow.svelte`: `{ label: string; children: Snippet }` — renders `<div><p>label</p>{@render children()}</div>`
- Boolean value display logic (YES/NO badge or dash) can be a local snippet in the parent, called inside SpecRow children.
- This avoids prop-drilling the full `canEdit` + `onSave` chain into the row component.

## Common-Fields Sub-Component Pattern (RollingStockScaleFields)

When 3+ category snippets repeat the same N FormFields verbatim, extract them into a shared sub-component:
- Pass `rs: RollingStockForm`, `errorsFn`, `formLabels`, and variation props (`roadNumberRequired?: boolean`).
- The sub-component uses `bind:value` uniformly; controlled-input (`value=` + `oninput=`) variants in the original are equivalent and safe to consolidate.
- Located at: `src/lib/features/catalogue/components/RollingStockScaleFields.svelte`

## Pre-Existing Baseline Failures (do not fix without permission)

- `src/__tests__/lib/features/collection/modelViewMapper.test.ts`: 8 TypeScript errors
- `src/__tests__/lib/features/maintenance/urgency.test.ts`: 2 date-sensitive test failures
- `src/__tests__/lib/features/collection/FilterState.svelte.test.ts` + `urgency.test.ts`: lint errors

## Verification Order

1. Edit `messages/en.json` + `messages/it.json`
2. Run `pnpm prepare` (paraglide compile)
3. Edit Svelte files to use `m.*()` calls
4. Run `pnpm check` (0 new errors beyond baseline)
5. Run `pnpm lint` (0 new warnings beyond baseline)
6. Run `pnpm test` (same pass/fail as baseline)

## Test Infrastructure

- **Vitest config**: `vite.config.js` — two projects:
  - `client`: `*.svelte.test.ts` — runs in Playwright/Chromium (browser)
  - `server`: `*.test.ts` — runs in Node (no Svelte runtime)
- **Use `.svelte.test.ts`** for anything using `SvelteSet`, `flushSync`, or Svelte component rendering
- **Use `.test.ts`** for pure logic, Zod schemas, service classes
- **Mock pattern for TauriAdapter services**: `vi.mock('$lib/shared/services/TauriAdapter', () => ({ safeInvoke: vi.fn() }))`
- **`flushSync`** from `svelte` is used to synchronously flush reactive state changes in tests

## Drawer Decomposition Pattern

When decomposing a large drawer component, the script-block logic (validation, state, API calls,
transformation helpers) stays in the parent. Only cohesive markup sections move to sub-components.

- Bind form state two-way with `bind:form` on sub-components using `$bindable()`.
- Pass callbacks (`onAddRollingStock`, `onRemoveRollingStock`) — never mutate parent state from child.
- The "line count target" applies primarily to markup lines; dense script blocks legitimately remain.
- Extracted sub-components that represent isolated UI concerns (e.g. a discard-confirmation dialog)
  are minimal and only need `onConfirm`/`onCancel` callback props.

Key wishlists components:
- `AddRailwayModelDrawer.svelte` — drawer shell + state orchestration (355 lines)
- `AddRailwayModelForm.svelte` — form content: wishlist select + base form + wishlist details (149 lines)

Key collection components:
- `AddModelDrawer.svelte` — drawer shell + state orchestration (396 lines)
- `ModelSearchSection.svelte` — RailwayModelBaseForm + rolling stocks + PurchaseSection (84 lines)
- `ModelSelectionCard.svelte` — discard-changes confirmation dialog (30 lines)

## Bindings Type Gotchas

- `Length` = `{ Millimeters: string } | { Inches: string } | ...` — string values, not numbers/BigInt
- `Metadata` fields are snake_case: `created_at`, `updated_at` (not camelCase)
- `DigitalSetup` fields: `interface: DccInterface`, `dcc_address: number`, `installed_decoder_id: DecoderId`
- `OwnedRollingStockView.rollingStockId` = catalog ID (used to match against `RollingStockView` entries)
