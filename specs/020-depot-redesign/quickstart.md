# Quickstart Guide: Depot Page Redesign

**Feature**: 020-depot-redesign
**Date**: 2026-02-12

This guide provides step-by-step instructions for implementing the depot page redesign.

---

## Prerequisites

- ✅ Branch `020-depot-redesign` checked out
- ✅ Dependencies installed (`pnpm install`)
- ✅ Research and design documentation reviewed

---

## Development Workflow

### 1. Start Development Environment

```bash
# Terminal 1: Start Tauri development server
pnpm tauri dev

# Terminal 2: Run tests in watch mode (optional)
pnpm test --watch
```

**Expected**:

- Tauri app opens with current depot page (3 categories, grid/table toggle)
- Hot reload enabled for frontend changes
- Backend changes require rebuild

---

### 2. Code Quality Tools

Run before committing:

```bash
# Frontend linting and type checking
pnpm lint        # ESLint checks
pnpm check       # TypeScript/Svelte type checking
pnpm format      # Prettier formatting

# Backend linting and formatting
cargo fmt        # Format Rust code
cargo clippy     # Lint Rust code (must pass with no warnings)
cargo test       # Run Rust tests
```

**Pre-commit checklist**:

- [ ] `pnpm lint` passes
- [ ] `pnpm check` passes
- [ ] `cargo clippy` passes (no warnings)
- [ ] `cargo test` passes
- [ ] Manual UI testing complete

---

## Implementation Steps

### Phase 1: Backend Changes (Add Epoch Field)

#### Step 1.1: Update DepotRollingStockView

**File**: `src-tauri/src/collecting/domain/depot_view.rs`

```rust
// Line ~57: Add epoch field
pub struct DepotRollingStockView {
    // ... existing fields ...
    pub railway_company_name: Option<String>,
    /// Epoch/era (e.g., "IV", "III/IV", "Vm")
    pub epoch: Option<Epoch>,  // ADD THIS LINE
}
```

**Import required**:

```rust
use crate::catalog::domain::railway_model::Epoch;
```

#### Step 1.2: Update Mapper Function

**File**: `src-tauri/src/collecting/infrastructure/mappers.rs`

```rust
// Line ~290: Add epoch to struct initialization
Ok(crate::collecting::domain::DepotRollingStockView {
    // ... existing fields ...
    railway_company_name: owned.railway_company_name.clone(),
    epoch: Some(collection_item.railway_model.epoch.clone()),  // ADD THIS LINE
})
```

#### Step 1.3: Rebuild Backend

```bash
cd src-tauri
cargo build
```

**Expected**:

- Compilation succeeds
- TypeScript bindings auto-regenerated with epoch field
- No breaking changes to existing code

#### Step 1.4: Verify TypeScript Types

**Check**: Auto-generated bindings should now include:

```typescript
export interface DepotRollingStockView {
  // ... existing fields ...
  epoch: string | null; // ← Should appear
}
```

**Location**: Check generated bindings in `src/bindings/` or similar

---

### Phase 2: Frontend Changes (Category Reorganization)

#### Step 2.1: Update i18n Messages

**File**: `messages/en.json` (or appropriate Paraglide message file)

Add these message keys:

```json
{
  "depot_railcars_and_emu_title": "Railcars & EMU/DMU",
  "depot_passenger_cars_title": "Passenger Cars",
  "depot_freight_cars_title": "Freight Cars",
  "depot_empty_railcars_and_emu": "No railcars or multiple units in your depot",
  "depot_empty_passenger_cars": "No passenger cars in your depot",
  "depot_empty_freight_cars": "No freight cars in your depot"
}
```

**Rebuild messages**:

```bash
pnpm run paraglide:compile  # Or equivalent command
```

#### Step 2.2: Update DepotState (Category Logic)

**File**: `src/lib/features/depot/DepotState.svelte.ts`

**Current code** (lines 29-48):

```typescript
const locomotives = $derived(/* ... */);
const trains = $derived(/* ... */);
const cars = $derived(/* ... */);
```

**Replace with**:

```typescript
// Category 1: Locomotives (unchanged)
const locomotives = $derived(allItems.filter((item) => item.category === 'LOCOMOTIVE'));

// Category 2: Railcars & EMU/DMU (renamed from "trains")
const railcarsEmuDmu = $derived(
  allItems.filter(
    (item) => item.category === 'ELECTRIC_MULTIPLE_UNIT' || item.category === 'RAILCAR'
  )
);

// Category 3: Passenger Cars (NEW: split from "cars")
const passengerCars = $derived(allItems.filter((item) => item.category === 'PASSENGER_CAR'));

// Category 4: Freight Cars (NEW: split from "cars")
const freightCars = $derived(allItems.filter((item) => item.category === 'FREIGHT_CAR'));
```

**Update filtered derivations** (lines 50-66):

```typescript
const filteredLocomotives = $derived(locomotives.filter((item) => filterMatch(item, query)));

const filteredRailcarsEmuDmu = $derived(railcarsEmuDmu.filter((item) => filterMatch(item, query)));

const filteredPassengerCars = $derived(passengerCars.filter((item) => filterMatch(item, query)));

const filteredFreightCars = $derived(freightCars.filter((item) => filterMatch(item, query)));

const totalFiltered = $derived(
  filteredLocomotives.length +
    filteredRailcarsEmuDmu.length +
    filteredPassengerCars.length +
    filteredFreightCars.length
);
```

**Update return object**:

```typescript
return {
  // ... other properties ...
  locomotives,
  railcarsEmuDmu, // Changed from "trains"
  passengerCars, // NEW
  freightCars, // NEW
  filteredLocomotives,
  filteredRailcarsEmuDmu, // Changed from "filteredTrains"
  filteredPassengerCars, // NEW
  filteredFreightCars, // NEW
  totalFiltered
  // ... other properties ...
};
```

#### Step 2.3: Update Depot Page Component

**File**: `src/routes/my-depot/+page.svelte`

**Import Accordion component** (line ~13):

```svelte
import * as Accordion from '$lib/components/ui/accordion';
```

**Remove grid view toggle** (lines 64-85):

```svelte
<!-- DELETE THIS SECTION -->
<div class="flex items-center gap-1 rounded-lg border border-border bg-card p-1">
  <button class="btn-icon btn-icon-sm">...</button>
  <button class="btn-icon btn-icon-sm">...</button>
</div>
```

**Update derived state variables** (lines 40-47):

```svelte
<script lang="ts">
  const filteredLocomotives = $derived(depot.filteredLocomotives);
  const filteredRailcarsEmuDmu = $derived(depot.filteredRailcarsEmuDmu); // Changed
  const filteredPassengerCars = $derived(depot.filteredPassengerCars); // NEW
  const filteredFreightCars = $derived(depot.filteredFreightCars); // NEW
  const totalFiltered = $derived(depot.totalFiltered);

  // Remove viewMode related code
</script>
```

**Replace rendering section** (lines 139-201):

```svelte
{:else}
  <Accordion.Root type="multiple" value={['locomotives', 'railcarsEmuDmu', 'passengerCars', 'freightCars']}>
    <!-- Category 1: Locomotives -->
    {#if filteredLocomotives.length > 0}
      <Accordion.Item value="locomotives" class="rounded-lg border border-white/10 bg-black/20 mb-4">
        <Accordion.Trigger class="sticky top-[var(--header-offset)] z-10 flex w-full items-center justify-between px-4 py-3 bg-surface-900/95 backdrop-blur-sm rounded-t-lg">
          <div class="flex items-center gap-3">
            <TrainFront size={20} class="text-primary" />
            <h3 class="text-lg font-semibold">{m.depot_locomotives_title()}</h3>
            <Badge variant="secondary">{filteredLocomotives.length}</Badge>
          </div>
        </Accordion.Trigger>
        <Accordion.Content class="px-0 pt-0">
          <DepotTable items={filteredLocomotives} />
        </Accordion.Content>
      </Accordion.Item>
    {/if}

    <!-- Category 2: Railcars & EMU/DMU -->
    {#if filteredRailcarsEmuDmu.length > 0}
      <Accordion.Item value="railcarsEmuDmu" class="rounded-lg border border-white/10 bg-black/20 mb-4">
        <Accordion.Trigger class="sticky top-[var(--header-offset)] z-10 flex w-full items-center justify-between px-4 py-3 bg-surface-900/95 backdrop-blur-sm">
          <div class="flex items-center gap-3">
            <TramFront size={20} class="text-primary" />
            <h3 class="text-lg font-semibold">{m.depot_railcars_and_emu_title()}</h3>
            <Badge variant="secondary">{filteredRailcarsEmuDmu.length}</Badge>
          </div>
        </Accordion.Trigger>
        <Accordion.Content class="px-0 pt-0">
          <DepotTable items={filteredRailcarsEmuDmu} />
        </Accordion.Content>
      </Accordion.Item>
    {/if}

    <!-- Category 3: Passenger Cars -->
    {#if filteredPassengerCars.length > 0}
      <Accordion.Item value="passengerCars" class="rounded-lg border border-white/10 bg-black/20 mb-4">
        <Accordion.Trigger class="sticky top-[var(--header-offset)] z-10 flex w-full items-center justify-between px-4 py-3 bg-surface-900/95 backdrop-blur-sm">
          <div class="flex items-center gap-3">
            <Users size={20} class="text-primary" />
            <h3 class="text-lg font-semibold">{m.depot_passenger_cars_title()}</h3>
            <Badge variant="secondary">{filteredPassengerCars.length}</Badge>
          </div>
        </Accordion.Trigger>
        <Accordion.Content class="px-0 pt-0">
          <DepotTable items={filteredPassengerCars} />
        </Accordion.Content>
      </Accordion.Item>
    {/if}

    <!-- Category 4: Freight Cars -->
    {#if filteredFreightCars.length > 0}
      <Accordion.Item value="freightCars" class="rounded-lg border border-white/10 bg-black/20 mb-4">
        <Accordion.Trigger class="sticky top-[var(--header-offset)] z-10 flex w-full items-center justify-between px-4 py-3 bg-surface-900/95 backdrop-blur-sm rounded-t-lg">
          <div class="flex items-center gap-3">
            <BoxIcon size={20} class="text-primary" />
            <h3 class="text-lg font-semibold">{m.depot_freight_cars_title()}</h3>
            <Badge variant="secondary">{filteredFreightCars.length}</Badge>
          </div>
        </Accordion.Trigger>
        <Accordion.Content class="px-0 pt-0">
          <DepotTable items={filteredFreightCars} />
        </Accordion.Content>
      </Accordion.Item>
    {/if}
  </Accordion.Root>
{/if}
```

**Add imports for new icons**:

```svelte
import {Users} from 'lucide-svelte';
```

#### Step 2.4: Update DepotTable (Add Era Column)

**File**: `src/lib/features/depot/components/DepotTable.svelte`

**Add Era column header** (after Product Code column):

```svelte
<th class="px-3 py-2 text-left text-sm font-medium text-muted-foreground">
  {m.depot_era()}
</th>
```

**Add Era column data** (in table row):

```svelte
<td class="px-3 py-2 text-sm text-muted-foreground">
  {item.epoch ?? '-'}
</td>
```

**Add i18n message**:

```json
{
  "depot_era": "Era"
}
```

#### Step 2.5: Remove Grid View Components

**Delete these files**:

- `src/lib/features/depot/components/LocomotiveCard.svelte`
- `src/lib/features/depot/components/TrainCard.svelte`
- `src/lib/features/depot/components/CarCard.svelte`

**Delete or refactor**:

- `src/lib/features/depot/components/DepotSection.svelte` (if only used for grid view)

```bash
rm src/lib/features/depot/components/LocomotiveCard.svelte
rm src/lib/features/depot/components/TrainCard.svelte
rm src/lib/features/depot/components/CarCard.svelte
# Review DepotSection.svelte - remove if not needed
```

---

## Testing the Depot Redesign

### Manual Testing Checklist

**Test Data Setup**:

1. Ensure collection has rolling stock in all 4 categories:
   - At least 2 locomotives
   - At least 1 EMU or Railcar
   - At least 2 passenger cars
   - At least 2 freight cars
2. Create duplicate models (same road number)
3. Add models with and without epoch data

**UI Tests**:

- [ ] Open `/my-depot` route
- [ ] Verify 4 accordion sections appear (if items exist in each category)
- [ ] Verify count badges show correct numbers
- [ ] Expand/collapse each section independently
- [ ] Verify sticky headers stay visible when scrolling
- [ ] Verify Era column appears in table
- [ ] Test search filtering:
  - [ ] Type partial road number (e.g., "103") → items filter
  - [ ] Type manufacturer (e.g., "Roco") → items filter across categories
  - [ ] Clear search → all items reappear
- [ ] Verify empty states:
  - [ ] Search with no matches → "No results" message
  - [ ] Category with no items → section hidden
- [ ] Test ownership filtering:
  - [ ] Only owned items appear
  - [ ] Soft-deleted items do not appear
- [ ] Test duplicates:
  - [ ] Multiple items with same road number all appear

**Performance Tests**:

- [ ] Add 100+ items to collection
- [ ] Verify search debounce (150ms delay)
- [ ] Verify smooth scrolling with sticky headers
- [ ] Verify accordion animations are smooth

**Accessibility Tests**:

- [ ] Tab navigation works through accordions
- [ ] Enter/Space toggles accordion sections
- [ ] Screen reader announces section state (open/closed)

---

## Running Automated Tests

### Frontend Tests

```bash
# Run all tests
pnpm test

# Run specific test file
pnpm test depot-state.test.ts

# Run with coverage
pnpm test:coverage
```

**Expected tests to pass**:

- Category grouping logic (4 categories)
- Search filtering across categories
- Duplicate handling (all shown)
- Empty state handling

### Backend Tests

```bash
# Run all Rust tests
cd src-tauri
cargo test

# Run specific test
cargo test depot

# Run with output
cargo test -- --nocapture
```

**Expected tests to pass**:

- Depot query includes epoch field
- Ownership filtering works
- Soft-delete filtering works
- Mapper includes epoch in result

---

## Troubleshooting

### Issue: TypeScript errors after backend changes

**Solution**:

```bash
cd src-tauri
cargo build  # Regenerates TypeScript bindings
cd ..
pnpm check   # Verify types are correct
```

### Issue: i18n messages not found

**Solution**:

```bash
pnpm run paraglide:compile
# Or restart dev server
```

### Issue: Accordion not collapsing/expanding

**Check**:

- `value` prop on `Accordion.Item` is unique
- `type="multiple"` allows independent expansion
- No CSS conflicts with `data-state` attributes

### Issue: Sticky headers not working

**Check**:

- `sticky top-[var(--header-offset)] z-10` classes applied to Trigger
- Parent container doesn't have `overflow: hidden`
- CSS variable `--header-offset` is defined

### Issue: Era column shows "undefined"

**Check**:

- Backend rebuild completed (`cargo build`)
- Mapper includes `epoch: Some(...)` line
- Frontend uses `item.epoch ?? '-'` for null handling

---

## Key Files Reference

### Backend

| File                                                 | Purpose                 |
| ---------------------------------------------------- | ----------------------- |
| `src-tauri/src/collecting/domain/depot_view.rs`      | Add epoch field         |
| `src-tauri/src/collecting/infrastructure/mappers.rs` | Include epoch in mapper |

### Frontend

| File                                                  | Purpose                                        |
| ----------------------------------------------------- | ---------------------------------------------- |
| `src/routes/my-depot/+page.svelte`                    | Replace grid with Accordion, render 4 sections |
| `src/lib/features/depot/DepotState.svelte.ts`         | Category grouping logic (4 categories)         |
| `src/lib/features/depot/components/DepotTable.svelte` | Add Era column                                 |
| `messages/en.json`                                    | Add i18n message keys                          |

---

## Next Steps After Implementation

1. Run full test suite: `pnpm test && cargo test`
2. Run linters: `pnpm lint && cargo clippy`
3. Manual UI testing with checklist above
4. Create commit following conventional commits format
5. Open pull request with description referencing spec
6. Request review from maintainers

---

**Questions?** Refer to:

- [spec.md](./spec.md) - Feature requirements
- [plan.md](./plan.md) - Implementation plan
- [research.md](./research.md) - Technical research
- [data-model.md](./data-model.md) - Data model changes
