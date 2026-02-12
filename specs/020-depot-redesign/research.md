# Research Report: Depot Page Redesign

**Date**: 2026-02-12
**Feature**: 020-depot-redesign
**Status**: Complete

## Executive Summary

All research questions resolved. Key findings:

- ✅ Epoch field is available in database but NOT currently in DepotRollingStockView
- ✅ Ownership and soft-delete filtering already implemented correctly
- ✅ RollingStockCategory enum supports new 4-category structure
- ✅ shadcn-svelte Accordion component is available and well-documented in codebase
- 🔧 Changes required: Add epoch to depot view, reorganize UI categories, implement Accordion

---

## Section 1: Depot Query Analysis

### Question

Does the current depot query (`GetDepot` use case) include epoch data in the result?

### Answer

**NO** - The current depot query does NOT include epoch data in `DepotRollingStockView`.

### Evidence

**Current DepotRollingStockView fields** (`src-tauri/src/collecting/domain/depot_view.rs`, lines 24-57):

- id, series_code, road_number, friendly_name, depot, category
- manufacturer_name, product_code, control, livery, railway_company_name
- ❌ **epoch field is missing**

**Mapper function** (`src-tauri/src/collecting/infrastructure/mappers.rs`, lines 259-294):

```rust
Ok(crate::collecting::domain::DepotRollingStockView {
    id: owned.id.clone(),
    series_code: collection_item.railway_model.product_code.clone(),
    road_number: owned.road_number.clone(),
    // ... other fields ...
    railway_company_name: owned.railway_company_name.clone(),
    // ❌ epoch not included
})
```

### Decision

**Add epoch field to DepotRollingStockView** and update mapper to include it.

---

## Section 2: Epoch Data Availability

### Question

Is epoch stored at the collection item level or railway model level?

### Answer

**Railway Model Level** - Epoch is stored in the `railway_models` table and joined into collection item queries.

### Evidence

**Database Schema** (`src-tauri/src/collecting/infrastructure/entities.rs`, line 55):

```rust
pub struct CollectionItemRow {
    // ...
    pub epoch: Epoch,  // ✅ Available from railway_models join
}
```

**SQL Query** (`src-tauri/src/collecting/infrastructure/database.rs`, lines 56-76):

```sql
SELECT
    ...
    rm.epoch,  -- Line 69: Fetched from railway_models table
FROM collection_items ci
JOIN railway_models rm ON rm.id = ci.railway_model_id
WHERE ci.removed_date IS NULL  -- Soft-delete filter
```

**Test Confirmation** (`database.rs`, line 294):

```rust
assert_eq!(collection_item_row.epoch, "IV".into());
```

### Data Flow

1. `GetDepot` use case → `find_depot_view()`
2. `find_depot_view()` → `get_collection_items()` (already fetches epoch via railway_models join)
3. `CollectionMapper::row_to_collection_item()` → maps epoch to `CollectionItemView.railway_model.epoch`
4. 🔧 **Needs change**: `collection_item_owned_to_depot()` currently discards epoch

### Decision

**No SQL changes needed** - Epoch is already in the data pipeline. Only mapper update required.

---

## Section 3: shadcn-svelte Accordion Component

### Question

How to implement collapsible sections with sticky headers using shadcn-svelte Accordion?

### Answer

shadcn-svelte provides a fully-featured Accordion component built on bits-ui primitives with animations and accessibility.

### Component Structure

**Location**: `/home/carlo/Projects/rusty-shed/src/lib/components/ui/accordion/`

- `accordion.svelte` - Root wrapper
- `accordion-item.svelte` - Individual section container
- `accordion-trigger.svelte` - Clickable header
- `accordion-content.svelte` - Expandable content with animations

### API Reference

```svelte
<Accordion.Root type="multiple" bind:value={openSections}>
  <Accordion.Item value="unique-id" class="rounded-lg border">
    <Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2">
      <div class="flex items-center gap-2">
        <Icon class="h-4 w-4" />
        <span>Section Title</span>
        <Badge variant="secondary">{count}</Badge>
      </div>
    </Accordion.Trigger>

    <Accordion.Content class="px-3 pt-1 pb-4">
      <!-- Content here -->
    </Accordion.Content>
  </Accordion.Item>
</Accordion.Root>
```

### Sticky Headers Implementation

```svelte
<Accordion.Trigger
  class="bg-surface-800/95 sticky top-0 z-10 flex w-full items-center justify-between px-3 py-2 backdrop-blur-sm"
>
  <h3 class="h4 mb-0">Sticky Header</h3>
</Accordion.Trigger>
```

**Required classes**:

- `sticky top-0 z-10` - Positioning
- `bg-surface-800/95` - Semi-transparent background
- `backdrop-blur-sm` - Optional glass morphism effect

### Count Badge Pattern

```svelte
<Accordion.Trigger class="flex w-full items-center justify-between px-3 py-2">
  <div class="flex items-center gap-2">
    <TrainFront class="h-4 w-4 text-muted-foreground" />
    <span>Locomotives</span>
    <Badge variant="secondary">{locomotives.length}</Badge>
  </div>
</Accordion.Trigger>
```

### Performance for Large Lists

**Best Practices**:

1. Use `type="multiple"` to allow independent expansion
2. Start with key sections open: `let openSections = $state(['locomotives', 'railcarsEmuDmu'])`
3. Use stable keys in `{#each}`: `{#each items as item (item.id)}`
4. Avoid rendering all content upfront - let accordion manage visibility

### Built-in Features

- ✅ Auto-rotating chevron icon
- ✅ Smooth expand/collapse animations (`animate-accordion-up/down`)
- ✅ Keyboard navigation (arrow keys, Enter/Space)
- ✅ Accessibility (ARIA attributes, focus management)
- ✅ Data attributes for styling (`data-state="open/closed"`)

### Existing Usage Examples in Codebase

**Multi-section form** (`CreateRailwayModel.svelte`, lines 162-312):

- Multiple independently expandable sections
- Count badges for dynamic content

**Nested accordion** (`RollingStockSection.svelte`, lines 118-135):

- Technical details within rolling stock items
- Unique IDs using index: `value={`technical-${index}-passenger`}`

**Lazy-loading** (`HistoricalArchive.svelte`, lines 53-89):

- Load data on expand
- Loading indicators in trigger

### Decision

**Use Accordion.Root with type="multiple"** for independent category expansion. Implement sticky headers with `sticky top-0 z-10` classes. Add count badges to each trigger showing item counts.

---

## Section 4: Category Mapping Logic

### Question

How to reorganize existing categories (Locomotives, Trains, Cars) into new structure?

### Answer

Current 3-category UI maps cleanly to new 4-category structure using existing `RollingStockCategory` enum.

### RollingStockCategory Enum Values

**File**: `src-tauri/src/catalog/domain/railway_model/category.rs` (lines 73-93)

```rust
pub enum RollingStockCategory {
    Locomotive,           // Steam, diesel, electric traction units
    FreightCar,          // Goods transport vehicles
    PassengerCar,        // Passenger transport coaches
    ElectricMultipleUnit, // Self-propelled EMU trains
    Railcar,             // Lightweight self-propelled units
}
```

### Current vs. New Category Mapping

| Enum Value             | Current UI Category | New UI Category        | Notes                 |
| ---------------------- | ------------------- | ---------------------- | --------------------- |
| `Locomotive`           | Locomotives         | **Locomotives**        | Unchanged             |
| `ElectricMultipleUnit` | Trains              | **Railcars & EMU/DMU** | Self-propelled units  |
| `Railcar`              | Trains              | **Railcars & EMU/DMU** | Includes DMU (diesel) |
| `PassengerCar`         | Cars                | **Passenger Cars**     | Split from Cars       |
| `FreightCar`           | Cars                | **Freight Cars**       | Split from Cars       |

### Current Frontend Logic

**File**: `src/lib/features/depot/DepotState.svelte.ts` (lines 29-48)

```typescript
// Current 3 categories
const locomotives = $derived(allItems.filter((item) => item.category === 'LOCOMOTIVE'));

const trains = $derived(
  allItems.filter(
    (item) => item.category === 'ELECTRIC_MULTIPLE_UNIT' || item.category === 'RAILCAR'
  )
);

const cars = $derived(
  allItems.filter((item) => item.category === 'PASSENGER_CAR' || item.category === 'FREIGHT_CAR')
);
```

### New Frontend Logic (Required Changes)

```typescript
// New 4 categories
const locomotives = $derived(allItems.filter((item) => item.category === 'LOCOMOTIVE'));

const railcarsAndEMU = $derived(
  // Renamed from "trains"
  allItems.filter(
    (item) => item.category === 'ELECTRIC_MULTIPLE_UNIT' || item.category === 'RAILCAR'
  )
);

const passengerCars = $derived(
  // NEW: Split from "cars"
  allItems.filter((item) => item.category === 'PASSENGER_CAR')
);

const freightCars = $derived(
  // NEW: Split from "cars"
  allItems.filter((item) => item.category === 'FREIGHT_CAR')
);
```

### i18n Message Keys Required

**New keys needed** (add to `/messages/en.json`):

- `depot_railcars_and_emu_title` - "Railcars & EMU/DMU"
- `depot_passenger_cars_title` - "Passenger Cars"
- `depot_freight_cars_title` - "Freight Cars"
- `depot_empty_railcars_and_emu` - Empty state message
- `depot_empty_passenger_cars` - Empty state message
- `depot_empty_freight_cars` - Empty state message

**Reuse existing**:

- `depot_locomotives_title` ✓
- `depot_empty_locomotives` ✓

### Icon Recommendations

- **Locomotives**: `TrainFront` (existing) ✓
- **Railcars & EMU/DMU**: `TramFront` (existing for trains) ✓
- **Passenger Cars**: `Users` or `Armchair` (new)
- **Freight Cars**: `Box` (existing) ✓

### Edge Cases

1. **All enum values mapped**: ✅ Complete coverage, no unmapped categories
2. **Empty categories**: Each section hides when filtered to 0 items (existing behavior)
3. **Search filtering**: Works across all 4 categories (no change to filter logic)
4. **Type discriminants**: Existing `Car` type has `category: 'passenger' | 'freight'` which aligns with split

### Files Requiring Changes

1. **DepotState.svelte.ts**: Update derived getters (4 categories + 4 filtered)
2. **+page.svelte**: Render 4 Accordion items instead of 3
3. **Paraglide messages**: Add 6 new i18n keys
4. **depot-data.ts**: Potentially update type definitions

### Decision

**Split "Cars" category into "Passenger Cars" and "Freight Cars"**. Keep "Locomotives" unchanged. Rename "Trains" to "Railcars & EMU/DMU" for clarity. All enum values have clear mappings.

---

## Section 5: Ownership Filtering Verification

### Question

Does the current depot query already filter by ownership and exclude soft-deleted items?

### Answer

**YES** - Both filters are correctly implemented in the existing depot query.

### Ownership Filtering Evidence

**SQL Query** (`src-tauri/src/collecting/infrastructure/database.rs`, lines 134-172):

```sql
FROM owned_rolling_stocks AS ors
JOIN collection_items AS ci ON ci.id = ors.collection_item_id
WHERE ci.collection_id = ?1  -- ✅ Filters by specific collection (ownership)
```

Only owned rolling stocks linked to the user's collection are returned.

### Soft-Delete Filtering Evidence

**get_owned_rolling_stocks()** (line 163):

```sql
AND ci.removed_date IS NULL  -- ✅ Excludes soft-deleted items
```

**get_collection_items()** (line 76):

```sql
AND ci.removed_date IS NULL  -- ✅ Same filter for collection items
```

### Test Confirmation

**Test**: "it_should_persist_removed_date_and_update_summary" (`repositories.rs`, lines 925-935)

- Confirms removed items are excluded from depot view
- Verifies summary counts exclude soft-deleted items

### Functional Requirements Compliance

- **FR-013** (only owned rolling stock): ✅ **SATISFIED** via `ci.collection_id = ?1` filter
- **FR-014** (exclude removed items): ✅ **SATISFIED** via `removed_date IS NULL` filter
- **FR-015** (show all duplicates): ✅ **SATISFIED** - No deduplication logic in query

### Decision

**No changes needed for ownership/soft-delete filtering**. Requirements FR-013, FR-014, and FR-015 are already implemented correctly in the existing depot query.

---

## Summary of Changes Required

| Component                          | Change                                                | Complexity | Priority |
| ---------------------------------- | ----------------------------------------------------- | ---------- | -------- |
| **Backend: depot_view.rs**         | Add `pub epoch: Option<Epoch>` field                  | Low        | High     |
| **Backend: mappers.rs**            | Include epoch in mapper function                      | Low        | High     |
| **Frontend: DepotState.svelte.ts** | Split categories from 3 to 4                          | Medium     | High     |
| **Frontend: +page.svelte**         | Replace grid toggle with Accordion, render 4 sections | Medium     | High     |
| **Frontend: Paraglide**            | Add 6 new i18n message keys                           | Low        | High     |
| **Frontend: Components**           | Remove grid view components (LocomotiveCard, etc.)    | Low        | Medium   |
| **SQL/Query Logic**                | No changes needed                                     | N/A        | N/A      |
| **Filtering Logic**                | No changes needed (already correct)                   | N/A        | N/A      |

---

## Alternatives Considered

### Alternative 1: Keep 3 Categories

**Rejected** - Spec explicitly requires 4 categories for better organization.

### Alternative 2: Create New Backend Endpoint

**Rejected** - Existing `GetDepot` use case can be extended with epoch field. No need for new endpoint.

### Alternative 3: Client-Side Epoch Fetching

**Rejected** - Epoch is already in database query results. Adding to DepotRollingStockView is simpler than separate fetch.

### Alternative 4: Use Tabs Instead of Accordion

**Rejected** - Accordion allows viewing multiple categories simultaneously, better for browsing large collections.

---

## Risk Assessment

| Risk                              | Likelihood | Impact | Mitigation                                                  |
| --------------------------------- | ---------- | ------ | ----------------------------------------------------------- |
| Breaking TypeScript types         | Low        | Medium | Regenerate specta bindings after Rust changes               |
| Missing i18n translations         | Medium     | Low    | Add all required keys before UI changes                     |
| Performance with 500+ items       | Low        | Medium | Test with large dataset, implement virtualization if needed |
| Accordion animation lag           | Low        | Low    | Use built-in animations, avoid custom transitions           |
| User confusion (category changes) | Medium     | Low    | Clear category labels, migration guide if needed            |

---

## Next Steps

Proceed to **Phase 1: Design & Contracts** with all research questions resolved.
