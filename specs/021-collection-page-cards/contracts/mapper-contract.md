# Mapper Contract: Card Data Transformation

**Feature**: 021-collection-page-cards
**Date**: 2026-02-12
**Phase**: Phase 1 - Design & Contracts

## Overview

This document defines the function signatures and contracts for the card data mapper utility module.

**Module**: `src/lib/features/collection/utils/cardDataMapper.ts`

---

## Public API

### collectionItemToCardData

**Purpose**: Transform CollectionItemView to RailwayModelCardData for RailwayModelPreviewCard component.

**Signature**:

```typescript
function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData;
```

**Parameters**:

- `item`: CollectionItemView - Source data from backend collection query

**Returns**: RailwayModelCardData - Transformed data for RailwayModelPreviewCard component

**Contract**:

- ✅ MUST return a valid RailwayModelCardData object
- ✅ MUST handle all nullable fields gracefully (no exceptions)
- ✅ MUST set missing fields (powerMethod, photoUrl) to `null`
- ✅ MUST map `description` → `series`
- ✅ MUST map `epoch` → `era`
- ✅ MUST derive `category` using `mapCategory()` helper
- ✅ MUST derive `digitalFeatures` using `extractDigitalFeatures()` helper
- ✅ MUST set `unitCount` to `null` for single-unit models
- ✅ MUST extract `purchaseDate` from discriminated union using `extractPurchaseDate()` helper

**Example**:

```typescript
import { collectionItemToCardData } from '$lib/features/collection/utils/cardDataMapper';

const item: CollectionItemView = { /* ... from backend ... */ };
const cardData = collectionItemToCardData(item);

// cardData is now ready for RailwayModelPreviewCard
<RailwayModelPreviewCard model={cardData} onDelete={handleDelete} />
```

---

### mapCategory

**Purpose**: Map backend Category enum to frontend ModelCategory type with locomotive subtype refinement.

**Signature**:

```typescript
function mapCategory(
  category: Category | null,
  rollingStocks: OwnedRollingStockView[]
): ModelCategory;
```

**Parameters**:

- `category`: Category | null - Backend category enum value
- `rollingStocks`: OwnedRollingStockView[] - Array of rolling stock units for subtype lookup

**Returns**: ModelCategory - Frontend category type for icon selection

**Contract**:

- ✅ MUST return a valid `ModelCategory` value
- ✅ MUST return `'Unknown'` for `null` input
- ✅ MUST refine `'LOCOMOTIVES'` using `locomotiveType` from rolling stock
- ✅ MUST default to `'SteamLocomotive'` if `locomotiveType` unavailable
- ✅ MUST handle empty `rollingStocks` array gracefully
- ✅ MUST return `'Unknown'` for unmapped category values

**Mapping**:
| Input Category | Output ModelCategory | Notes |
|---|---|---|
| `null` | `'Unknown'` | Null handling |
| `'FREIGHT_CARS'` | `'FreightCar'` | Direct mapping |
| `'PASSENGER_CARS'` | `'PassengerCar'` | Direct mapping |
| `'TRAIN_SETS'` | `'TrainSet'` | Direct mapping |
| `'STARTER_SETS'` | `'TrainSet'` | Treat as train set |
| `'ELECTRIC_MULTIPLE_UNITS'` | `'TrainSet'` | Multi-unit electric |
| `'RAILCARS'` | `'Railcar'` | Direct mapping |
| `'LOCOMOTIVES'` + `'STEAM_LOCOMOTIVE'` | `'SteamLocomotive'` | Subtype refinement |
| `'LOCOMOTIVES'` + `'DIESEL_LOCOMOTIVE'` | `'DieselLocomotive'` | Subtype refinement |
| `'LOCOMOTIVES'` + `'ELECTRIC_LOCOMOTIVE'` | `'ElectricLocomotive'` | Subtype refinement |
| `'LOCOMOTIVES'` + `null`/missing | `'SteamLocomotive'` | Fallback |
| Unmapped value | `'Unknown'` | Safe fallback |

**Example**:

```typescript
// Freight car - direct mapping
mapCategory('FREIGHT_CARS', [])
// => 'FreightCar'

// Locomotive with subtype - refinement
mapCategory('LOCOMOTIVES', [
  { locomotive: { locomotiveType: 'STEAM_LOCOMOTIVE' }, ... }
])
// => 'SteamLocomotive'

// Locomotive without subtype - fallback
mapCategory('LOCOMOTIVES', [])
// => 'SteamLocomotive'
```

---

### extractDigitalFeatures

**Purpose**: Extract digital features from rolling stock control and technical specifications.

**Signature**:

```typescript
function extractDigitalFeatures(rollingStocks: OwnedRollingStockView[]): DigitalFeature[];
```

**Parameters**:

- `rollingStocks`: OwnedRollingStockView[] - Array of rolling stock units to analyze

**Returns**: DigitalFeature[] - Array of unique digital feature strings

**Contract**:

- ✅ MUST return array of unique `DigitalFeature` values (no duplicates)
- ✅ MUST aggregate features across all rolling stock units
- ✅ MUST skip freight/passenger cars (no motor = no digital features)
- ✅ MUST handle null/undefined fields gracefully
- ✅ MUST detect `'Sound'` when `control === 'DCC_SOUND'`
- ✅ MUST detect `'DCC'` when `control !== 'NO_DCC' && control !== null` OR `digital !== null`
- ✅ MUST detect `'Light'` when `technical_specifications?.lights === 'YES'`
- ✅ MUST NOT include `'Smoke'` (not currently supported in schema)
- ✅ MUST return empty array `[]` for empty input or non-motor rolling stock

**Detection Rules**:

| Feature   | Condition                                                        | Data Source                                                  |
| --------- | ---------------------------------------------------------------- | ------------------------------------------------------------ |
| `'Sound'` | `control === 'DCC_SOUND'`                                        | `rollingStocks[].control`                                    |
| `'DCC'`   | `control !== 'NO_DCC' && control !== null` OR `digital !== null` | `rollingStocks[].control` or `.digital`                      |
| `'Light'` | `technical_specifications?.lights === 'YES'`                     | `rollingStocks[].locomotive.technical_specifications.lights` |
| `'Smoke'` | ❌ Not available                                                 | N/A                                                          |

**Example**:

```typescript
// DCC with sound locomotive
extractDigitalFeatures([
  {
    control: 'DCC_SOUND',
    digital: { interface: 'PLUX_22', dcc_address: 3 },
    locomotive: {
      locomotiveType: 'STEAM_LOCOMOTIVE',
      technical_specifications: { lights: 'YES', interior_lights: 'NO' }
    }
  }
])
// => ['Sound', 'DCC', 'Light']

// Non-DCC freight car
extractDigitalFeatures([
  { control: 'NO_DCC', digital: null, freightCar: {} }
])
// => []

// Multiple units with mixed features (deduplication)
extractDigitalFeatures([
  { control: 'DCC_FITTED', locomotive: { ... } },
  { control: 'DCC_FITTED', locomotive: { ... } }
])
// => ['DCC'] (no duplicates)
```

---

### extractPurchaseDate

**Purpose**: Extract purchase date from discriminated union PurchaseInfo type.

**Signature**:

```typescript
function extractPurchaseDate(purchaseInfo: PurchaseInfo | null): string | null;
```

**Parameters**:

- `purchaseInfo`: PurchaseInfo | null - Purchase information discriminated union

**Returns**: string | null - ISO 8601 date string (YYYY-MM-DD) or null

**Contract**:

- ✅ MUST return `null` for `null` input
- ✅ MUST extract `purchaseDate` from `purchased` type
- ✅ MUST extract `purchaseDate` from `preOrdered` type (if present)
- ✅ MUST return `null` for `sold` type (don't show purchase date for sold items)
- ✅ MUST return ISO 8601 formatted date string (YYYY-MM-DD)
- ✅ MUST handle missing optional fields gracefully

**Behavior by Type**:
| PurchaseInfo.type | Return Value | Rationale |
|---|---|---|
| `null` | `null` | No purchase info available |
| `'purchased'` | `data.purchaseDate` | Extract from purchased data |
| `'preOrdered'` | `data.purchaseDate ?? null` | Extract if available |
| `'sold'` | `null` | Don't show purchase date for sold items |

**Example**:

```typescript
// Purchased item
extractPurchaseDate({
  type: 'purchased',
  data: { purchaseDate: '2024-03-15', retailer: 'ModelTrains.com', price: null }
});
// => '2024-03-15'

// Pre-ordered item with purchase date
extractPurchaseDate({
  type: 'preOrdered',
  data: { expectedDate: '2024-06-01', purchaseDate: '2024-03-01', retailer: null }
});
// => '2024-03-01'

// Sold item
extractPurchaseDate({
  type: 'sold',
  data: { soldDate: '2023-12-01', buyer: 'John Doe', price: null }
});
// => null

// No purchase info
extractPurchaseDate(null);
// => null
```

---

## Error Handling

**Philosophy**: Graceful degradation - never throw exceptions

All functions MUST handle invalid/missing data by:

1. Using optional chaining (`?.`) for nullable property access
2. Using nullish coalescing (`??`) for fallback values
3. Returning safe defaults (`null`, `[]`, `'Unknown'`) instead of throwing

**Examples**:

```typescript
// ✅ Good: Safe navigation
const roadNumber = rollingStocks[0]?.roadNumber ?? null;

// ❌ Bad: Unsafe access (may throw)
const roadNumber = rollingStocks[0].roadNumber;

// ✅ Good: Fallback for missing category
if (!category) return 'Unknown';

// ❌ Bad: Throw exception
if (!category) throw new Error('Category required');
```

---

## Performance Characteristics

| Function                   | Time Complexity | Space Complexity | Notes                                          |
| -------------------------- | --------------- | ---------------- | ---------------------------------------------- |
| `collectionItemToCardData` | O(n)            | O(n)             | n = rolling stock count                        |
| `mapCategory`              | O(n)            | O(1)             | n = rolling stock count (find operation)       |
| `extractDigitalFeatures`   | O(n)            | O(1)             | n = rolling stock count, Set for deduplication |
| `extractPurchaseDate`      | O(1)            | O(1)             | Simple discriminated union access              |

**Typical Usage**:

- Collection rendering: 100 models × 3 rolling stock avg = 300 units processed
- Estimated total transformation time: <5ms (negligible)

**Optimization**: Not required for current scale. Consider memoization if profiling reveals performance issues.

---

## Testing Requirements

### Unit Test Coverage

**File**: `tests/unit/features/collection/cardDataMapper.test.ts`

**Required Test Cases**:

1. **collectionItemToCardData**
   - ✅ All direct field mappings
   - ✅ Semantic mappings (description → series, epoch → era)
   - ✅ Null handling for all nullable fields
   - ✅ Missing fields return null (powerMethod, photoUrl)
   - ✅ Unit count null for single unit, number for multiple
   - ✅ Calls helper functions correctly

2. **mapCategory**
   - ✅ All direct mappings (each Category enum value)
   - ✅ Locomotive subtype refinement (all 3 types)
   - ✅ Locomotive fallback (missing/null subtype)
   - ✅ Null input handling
   - ✅ Unmapped value handling
   - ✅ Empty rolling stock array

3. **extractDigitalFeatures**
   - ✅ Sound detection (DCC_SOUND)
   - ✅ DCC detection (via control and via digital)
   - ✅ Light detection (YES, NO, NOT_APPLICABLE)
   - ✅ Empty rolling stock array
   - ✅ Freight/passenger cars (no features)
   - ✅ Multiple units with deduplication
   - ✅ Mixed rolling stock types

4. **extractPurchaseDate**
   - ✅ Purchased type with date
   - ✅ PreOrdered type with date
   - ✅ PreOrdered type without date
   - ✅ Sold type returns null
   - ✅ Null purchaseInfo

**Minimum Coverage**: 90% line coverage, 100% branch coverage for mapper functions

---

## Usage Example

**Complete Integration Example**:

```typescript
// In CollectionDashboard.svelte
import { collectionItemToCardData } from '$lib/features/collection/utils/cardDataMapper';
import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';

const collectionService = getCollectionContext();
const filteredItems = $derived(collectionService.filteredItems);

function handleCardClick(item: CollectionItemView) {
  goto(`/models/${item.railwayModel.railwayModelId}`);
}

function handleDelete(modelId: string) {
  ui.requestDelete(modelId);
}
```

```svelte
{#each filteredItems as item (item.id)}
  <RailwayModelPreviewCard
    model={collectionItemToCardData(item)}
    onDelete={() => handleDelete(item.id)}
    onclick={() => handleCardClick(item)}
  />
{/each}
```

---

## Versioning

**Version**: 1.0.0
**Status**: ✅ APPROVED - Ready for implementation
**Date**: 2026-02-12

**Breaking Changes**: None (new module)

**Future Enhancements**:

1. Add `powerMethod` support when backend DTO extended
2. Add `photoUrl` support when image storage implemented
3. Add `Smoke` feature detection when schema extended
