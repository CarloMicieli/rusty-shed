# Quickstart: Rolling Stock List UX (032)

**Date**: 2026-03-03

This guide describes the implementation order and key steps for a developer picking up this feature.

---

## Prerequisites

- Branch `032-rolling-stock-list-ux` is checked out (created by `/speckit.specify`)
- `pnpm install` has been run
- `pnpm tauri dev` runs successfully on the baseline

---

## Implementation order

Work in this order to minimise blocked dependencies:

### Step 1 — Add `depot` to `OwnedRollingStockView` (Rust + bindings)

**Why first**: Every frontend change that touches the Depot field depends on the generated binding.

1. Open `src-tauri/src/collecting/domain/owned_rolling_stock_view.rs`
2. Add `pub depot: Option<String>` field
3. Open `src-tauri/src/collecting/infrastructure/` (or wherever the SELECT query for `OwnedRollingStockView` lives — look for `rolling_stocks.series`, `rolling_stocks.road_number` in the query) and add `rs.depot` or `rolling_stocks.depot`
4. Run `pnpm tauri dev` to regenerate `src/lib/bindings.ts`
5. Confirm `depot: string | null` appears on `OwnedRollingStockView` in bindings.ts

---

### Step 2 — New Rust use case + command (Backend)

1. Create `src-tauri/src/catalog/application/add_rolling_stock_to_model.rs`
   - `AddRollingStockToModelInput` struct
   - Use case struct + `execute()` method (load model → build params → `add_rolling_stock` → save)
2. Add `AddRollingStockToModelArgs` (in the commands file or a new `args` module)
   - Must derive `Debug, Clone, Serialize, Deserialize, Validate, specta::Type`
3. Add `#[tauri::command] #[specta::specta] pub async fn add_rolling_stock_to_model(...)` in `catalogue_commands.rs`
4. Register the command in the Tauri builder (`.invoke_handler(...)` or the specta builder call)
5. Run `cargo clippy -- -D warnings` to verify no issues
6. Run `pnpm tauri dev` to regenerate bindings — confirm `addRollingStockToModel` appears

---

### Step 3 — Strip Save/Cancel pill from `InPlaceEdit` (Frontend)

1. Open `src/lib/components/InPlaceEdit.svelte`
2. Delete the `<!-- Floating Save/Cancel pill -->` `<div>` block (lines 123–149)
3. Delete `let suppressBlurSave = false;` and the two `onmousedown` handlers
4. Change `{#if error}` margin from `mt-7` to `mt-1`
5. Run `pnpm check` and `pnpm lint`
6. Run the existing `InPlaceEdit` tests (update assertions for removed buttons)

---

### Step 4 — Update `RollingStockCard` (Frontend)

1. Add local `depot` state and sync from `rollingStock.depot`
2. Remove the `{#if editable || rollingStock.livery}` guard → always render Livery
3. Remove the `{#if editable || rollingStock.railwayCompanyName}` guard → always render Railway Company
4. Remove the `{#if rollingStock.control}` guard → render as `{rollingStock.control ?? '—'}`
5. Add a Depot row:
   - Static: `{localDepot || '—'}`
   - Editable: `<InPlaceEdit value={localDepot} ... onSave={(v) => saveIdentificationField('depot', v)} />`
6. Update `saveIdentificationField` to pass `depot` correctly (the `UpdateRollingStockIdentificationArgs` already has a `depot` field — it was being hard-coded to `null`)
7. Run tests, update assertions for newly visible rows

---

### Step 5 — Add Paraglide message keys

Add the following keys to `messages/en.json` (and other locale files if applicable):

```json
"rolling_stock_create_drawer_title": "Add Rolling Stock",
"rolling_stock_create_success": "Rolling stock added successfully",
"rolling_stock_create_error": "Failed to add rolling stock",
"rolling_stock_add_cta": "Add Rolling Stock",
"rolling_stock_add_more": "+ Add Rolling Stock",
"rolling_stock_field_category": "Category"
```

Run `pnpm prepare` (or `pnpm build`) to compile Paraglide and generate the message modules.

---

### Step 6 — Create `RollingStockCreateDrawer` component

**File**: `src/lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte`

**Props**:

```typescript
interface Props {
  open: boolean;
  railwayModelId: RailwayModelId;
  onCreated?: (id: RollingStockId) => void;
  onClose: () => void;
}
```

**Form fields** (in order):

1. Railway Company (BadgePicker, loaded via `commands.getRailwayCompanies()`)
2. Category (select, maps to `RollingStockCategory`)
3. Series Code (text input, required)
4. Road Number (text input, optional)
5. Livery (text input, optional)
6. Depot (text input, optional)
7. Control Type (select, reuse `controlOptions` from `RollingStockSpecsDrawer`)

**Behaviour**: mirrors `RollingStockSpecsDrawer` pattern (backdrop, Escape key, footer Save/Cancel buttons, discard dialog if dirty). On save calls `commands.addRollingStockToModel(...)`.

---

### Step 7 — Update `RollingStockList`

1. Add `onRollingStockAdded?: () => void` callback prop
2. Add `createDrawerOpen = $state(false)` and import `RollingStockCreateDrawer`
3. Empty state (when `editable`): replace plain message with CTA button that sets `createDrawerOpen = true`
4. Populated state (when `editable`): add `<button>` below the list that sets `createDrawerOpen = true`
5. Wire `RollingStockCreateDrawer` with `onCreated={() => { onRollingStockAdded?.(); createDrawerOpen = false; }}`
6. Update tests

---

### Step 8 — Verification checklist

```bash
pnpm lint          # ESLint + Svelte lint
pnpm check         # svelte-check + TypeScript
pnpm test          # Vitest
pnpm run rust:clippy  # cargo clippy -D warnings
pnpm run rust:test    # cargo test
```

All must pass with zero errors/warnings before the feature is considered complete.

---

## Key file paths

| File                                                                             | Change type                |
| -------------------------------------------------------------------------------- | -------------------------- |
| `src-tauri/src/collecting/domain/owned_rolling_stock_view.rs`                    | Add `depot` field          |
| `src-tauri/src/collecting/infrastructure/` (query)                               | Add `depot` to SELECT      |
| `src-tauri/src/catalog/application/add_rolling_stock_to_model.rs`                | NEW                        |
| `src-tauri/src/commands/catalogue_commands.rs`                                   | Add new command            |
| `src/lib/components/InPlaceEdit.svelte`                                          | Remove Save/Cancel pill    |
| `src/lib/components/model-details/RollingStockCard.svelte`                       | Unconditional rows + depot |
| `src/lib/components/model-details/RollingStockList.svelte`                       | CTA + Add More + drawer    |
| `src/lib/features/rolling-stock-edit/components/RollingStockCreateDrawer.svelte` | NEW                        |
| `messages/en.json` (and other locales)                                           | 6 new keys                 |
