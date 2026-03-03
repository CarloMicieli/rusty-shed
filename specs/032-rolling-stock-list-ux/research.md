# Research: Rolling Stock List UX (032)

**Date**: 2026-03-03
**Status**: Complete — all NEEDS CLARIFICATION resolved

---

## Finding 1 — InPlaceEdit component

**Location**: `src/lib/components/InPlaceEdit.svelte`

**Current state**: The component already supports auto-save on blur (`handleBlur → save()`) and Escape-to-cancel. The floating Save/Cancel pill is controlled via a `suppressBlurSave` flag that prevents the blur handler from double-saving when a pill button is clicked. Removing the pill also removes the need for `suppressBlurSave`.

**Decision**: Remove the pill (lines 123–149) and the `suppressBlurSave` flag. Adjust `error` margin from `mt-7` to `mt-1`. No interface change — all four call sites continue to work unchanged.

**Call sites that will be unaffected**:

- `RollingStockCard.svelte` (3 fields: series, roadNumber, livery)
- Any other component using `InPlaceEdit` (no other usages found)

---

## Finding 2 — RollingStockCard empty-field display

**Location**: `src/lib/components/model-details/RollingStockCard.svelte`

**Current state**:

- Series, Road Number: always rendered; `{localSeries || '—'}` in static mode ✅
- Livery: guarded by `{#if editable || rollingStock.livery}` — hidden in static mode if null ❌
- Railway Company: guarded by `{#if editable || rollingStock.railwayCompanyName}` — hidden if null ❌
- Control: rendered only `{#if rollingStock.control}` — hidden if null ❌
- Depot: **not rendered at all** — missing from the card ❌
- Length: not in `OwnedRollingStockView`; out of scope for inline display on card

**Changes needed**:

1. Remove `{#if editable || rollingStock.livery}` guard → always render Livery row
2. Remove `{#if editable || rollingStock.railwayCompanyName}` guard → always render Railway Company row
3. Remove `{#if rollingStock.control}` guard → always render Control row, use `{rollingStock.control ?? '—'}`
4. Add Depot row (requires surfacing `depot` on `OwnedRollingStockView` — see Finding 5)

---

## Finding 3 — RollingStockList empty state and "Add More" button

**Location**: `src/lib/components/model-details/RollingStockList.svelte`

**Current state**: The empty state renders a dashed border box with `model_no_rolling_stock()` message. No "Add More" button exists in either state. The component accepts `editable?: boolean` prop.

**Changes needed**:

1. When `editable` is true: replace empty-state message with an "Add Rolling Stock" CTA button (or card)
2. When `editable` is true and entries exist: show an "+ Add Rolling Stock" secondary button below the list or in a header section
3. Both CTAs open a single `RollingStockCreateDrawer` component
4. On successful creation, the parent must re-fetch rolling stock data; the list itself will trigger an `onRollingStockAdded` callback prop

---

## Finding 4 — Existing "Add Rolling Stock" domain support

**Location**: `src-tauri/src/catalog/domain/railway_model/railway_model.rs:271`

The domain aggregate already has:

```rust
pub fn add_rolling_stock(&mut self, params: RollingStockParams) -> RollingStockId
```

This emits a `RollingStockAdded` domain event. The repository (`sqlite_railway_model_repository.rs`) already handles this event.

**Conclusion**: No domain or repository changes needed. Only a new application use case + Tauri command is required.

---

## Finding 5 — OwnedRollingStockView missing `depot`

**Location**: `src-tauri/src/collecting/domain/owned_rolling_stock_view.rs` (type definition), `src/lib/bindings.ts:3918` (generated type)

`OwnedRollingStockView` does not include `depot`. The `rolling_stocks` table has a `depot` column and the Rust `RollingStock` enum variants include `depot` where applicable (Locomotive, EMU, Railcar). The `OwnedRollingStockView` is built by the `sqlite_railway_model_repository` query that joins rolling_stocks.

**Changes needed**:

1. Add `depot: Option<String>` field to `OwnedRollingStockView` Rust struct
2. Update the SELECT query in the repository to include `rolling_stocks.depot`
3. Re-run `pnpm tauri dev` to regenerate `src/lib/bindings.ts`

---

## Finding 6 — New command: `add_rolling_stock_to_model`

**No existing command** for adding a rolling stock variant to an existing model was found. A new command is required.

**Shape**: `AddRollingStockToModelArgs` (MVP simplified variant):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, validator::Validate, specta::Type)]
pub struct AddRollingStockToModelArgs {
    pub railway_model_id: String,
    pub railway_company_id: String,
    pub category: String,           // "LOCOMOTIVE" | "PASSENGER_CAR" | "FREIGHT_CAR" | ...
    pub series_code: String,        // required, non-empty
    pub road_number: Option<String>,
    pub livery: Option<String>,
    pub depot: Option<String>,
    pub control: Option<String>,    // Control enum string
}
```

The use case maps `category` to a `RollingStockParams` variant with minimal required type-specific fields:

- LOCOMOTIVE → `LocomotiveType::ElectricLocomotive` (default; user can change via Specs drawer)
- ELECTRIC_MULTIPLE_UNIT → `ElectricMultipleUnitType::MotorCar` (default)
- RAILCAR → `RailcarType::MotorCar` (default)
- PASSENGER_CAR / FREIGHT_CAR → no type-specific required field

**Return type**: `Result<RollingStockId, CommandError>`

---

## Finding 7 — New Paraglide message keys needed

Checking existing keys, these are missing for the creation workflow:

| Key                                 | Suggested content (EN)             |
| ----------------------------------- | ---------------------------------- |
| `rolling_stock_create_drawer_title` | "Add Rolling Stock"                |
| `rolling_stock_create_success`      | "Rolling stock added successfully" |
| `rolling_stock_create_error`        | "Failed to add rolling stock"      |
| `rolling_stock_add_cta`             | "Add Rolling Stock"                |
| `rolling_stock_add_more`            | "+ Add Rolling Stock"              |
| `rolling_stock_field_category`      | "Category"                         |

**Existing keys reusable**: `rolling_stock_field_series_code`, `rolling_stock_field_road_number`, `rolling_stock_field_livery`, `rolling_stock_field_depot`, `model_rolling_stock_field_control`, `specs_drawer_cancel`

---

## Finding 8 — Existing tests to update

| File                                                                  | Change needed                                                               |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| `src/lib/components/model-details/__tests__/RollingStockList.test.ts` | Update empty-state tests (no longer just a message, now also a CTA button)  |
| `src/__tests__/components/RollingStockSpecsDrawer.test.ts`            | Likely unaffected (unchanged component)                                     |
| `src/lib/components/model-details/__tests__/RollingStockCard.test.ts` | Add tests for always-visible rows (livery, depot, control with null values) |

New test file needed: `src/__tests__/components/RollingStockCreateDrawer.test.ts`

---

## Summary of resolved NEEDS CLARIFICATION

All items resolved. No open questions remain.

| Topic                    | Resolution                                                                                                 |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- |
| InPlaceEdit pill removal | Remove entirely; blur/Enter already handle save                                                            |
| Control Type inline edit | Control stays read-only on card (enum); edit available via Specs drawer. No `InPlaceEdit` for enum fields. |
| Depot display            | Surface `depot` on `OwnedRollingStockView`; render as `{depot ?? '—'}`                                     |
| Creation drawer scope    | MVP: category + identification fields only; specs via existing drawer after creation                       |
| Backend command          | New `add_rolling_stock_to_model` with simplified args; reuses existing domain method                       |
| Series Code required     | `series_code` validated non-empty at transport boundary; creation fails if empty                           |
