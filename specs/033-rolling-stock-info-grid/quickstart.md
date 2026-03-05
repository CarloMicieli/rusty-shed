# Quickstart: Rolling Stock Information Grid

**Feature**: `033-rolling-stock-info-grid`
**Date**: 2026-03-05

---

## Prerequisites

- Node.js + pnpm installed (`pnpm --version`)
- Rust toolchain installed (`rustup show`)
- Working branch: `git checkout 033-rolling-stock-info-grid`

---

## Development Loop

```bash
# Start the Tauri dev server (hot-reload frontend + backend)
pnpm tauri dev
```

Navigate to any Railway Model Card that has rolling stock entries. The card should show the full 5-row information grid once the feature is implemented.

---

## Files to Change

| File                                                       | Change Type | Description                                                 |
| ---------------------------------------------------------- | ----------- | ----------------------------------------------------------- |
| `src/lib/components/InPlaceBooleanEdit.svelte`             | **CREATE**  | New 3-state FeatureFlag toggle primitive                    |
| `src/lib/components/model-details/RollingStockCard.svelte` | **MODIFY**  | Restructure layout to 5-row grid + add 7 new field bindings |
| `messages/en.json`                                         | **MODIFY**  | Add ~5 new `rolling_stock_field_*` message keys             |

No Rust files require changes.

---

## Step-by-Step Implementation

### Step 1 — Add Message Keys

In `messages/en.json`, add:

```json
{
  "rolling_stock_field_interior_lights": "Interior Lights",
  "rolling_stock_field_lights": "Lights",
  "rolling_stock_field_control_type": "Control Type",
  "rolling_stock_field_length": "Length",
  "rolling_stock_field_series": "Series"
}
```

Then regenerate Paraglide messages:

```bash
pnpm run paraglide:compile
# or it runs automatically on pnpm tauri dev
```

### Step 2 — Create `InPlaceBooleanEdit.svelte`

Create `src/lib/components/InPlaceBooleanEdit.svelte` with:

- **Props**: `value: 'YES' | 'NO' | null`, `placeholder?: string`, `onSave: (v: 'YES' | 'NO' | null) => Promise<void>`
- **View mode**: Compact chip (green ✓ "Yes" / muted "No" / italic "—")
- **Edit mode**: Three inline buttons (— / Yes / No), saves immediately on click
- **Error/saving states**: Consistent with `InPlaceSelectEdit` visual language

See [data-model.md](data-model.md#inplacebooleanedits-component-contract) for full spec.

### Step 3 — Update `RollingStockCard.svelte`

1. **Add local state** for 7 new fields:

   ```typescript
   let localFlywheelFitted = $state<'YES' | 'NO' | null>(null);
   let localBodyShell = $state<string | null>(null);
   let localChassis = $state<string | null>(null);
   let localInteriorLights = $state<'YES' | 'NO' | null>(null);
   let localLights = $state<'YES' | 'NO' | null>(null);
   let localCouplingSocket = $state<string | null>(null);
   let localCloseCouplers = $state<boolean | null>(null);
   let localDigitalShunting = $state<boolean | null>(null);
   ```

2. **Extract values from prop** in the existing `$effect` / sync logic:

   ```typescript
   localFlywheelFitted =
     ts?.flywheel_fitted === 'YES' ? 'YES' : ts?.flywheel_fitted === 'NO' ? 'NO' : null;
   // ... similar for others (see data-model.md extraction)
   ```

3. **Add `saveSpecField` handler** that calls `updateRollingStockSpecifications`:
   See [contracts/tauri-commands.md](contracts/tauri-commands.md) for the full call pattern.

4. **Replace the `<dl>` layout** with a `<div class="grid grid-cols-3 gap-x-4 gap-y-3">` containing 15 field cells following the field registry in [data-model.md](data-model.md#field-registry-information-grid).

5. **Add `InPlaceBooleanEdit`** for FeatureFlag fields and `InPlaceSelectEdit` for enum material/coupling fields.

### Step 4 — Verify

```bash
# TypeScript + Svelte type checking
pnpm check

# Linting
pnpm lint

# Tests
pnpm test

# Format
pnpm format
```

---

## Testing the Feature

### Manual Test Checklist

1. **Grid visible**: Open a rolling stock card → all 15 fields show with labels in correct row/column
2. **Empty placeholder**: Fields with null values show italic placeholder (e.g., "Add Depot")
3. **Text edit**: Click Depot → input appears → type value → blur → value saved → shows new value
4. **Enum edit (Body Shell)**: Click Body Shell → dropdown shows "—/Plastic/Metal Die-Cast" → select → saved
5. **Boolean edit (Flywheel)**: Click Flywheel → picker shows "—/Yes/No" → select → saved
6. **Escape cancels**: Start editing any field → press Escape → no save, reverts to previous
7. **Save indicator**: During save, visual feedback (spinner/pulse) is shown
8. **Error handling**: Simulate failure (offline) → error shown inline, value reverts
9. **Multi-entry consistency**: Card with 3 entries → all show same grid structure

### Automated Tests

- **Unit test** for `InPlaceBooleanEdit`:
  - Renders "—" when value is null
  - Renders "Yes" chip when value is 'YES'
  - Clicking a choice calls `onSave` with correct value
  - Escape key cancels without calling `onSave`
  - Shows error when `onSave` rejects

- **Integration test** (if test harness supports component trees):
  - `RollingStockCard` with mock `commands` renders all 15 fields
  - Editing a FeatureFlag field triggers `updateRollingStockSpecifications` with correct args

---

## Troubleshooting

| Problem                                                     | Solution                                                                                             |
| ----------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Missing Paraglide key error                                 | Run `pnpm run paraglide:compile` and restart dev server                                              |
| TypeScript error on `closeCouplers` type                    | Ensure `featureFlagToBool` helper converts `'YES'/'NO'/null` → `boolean/null` before passing to args |
| Grid not aligning in 3 columns                              | Confirm `grid-cols-3` class is on the container, not on individual cells                             |
| `updateRollingStockSpecifications` returns validation error | Check `seriesCode` is never empty when sending the command                                           |
