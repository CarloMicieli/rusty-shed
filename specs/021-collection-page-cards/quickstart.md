# Quickstart Guide: Collection Page Card Integration

**Feature**: 021-collection-page-cards
**Date**: 2026-02-12
**Phase**: Phase 1 - Design & Contracts

## Overview

This guide helps developers integrate the RailwayModelCard and RailwayModelPreviewCard components into the collection page by replacing the existing ItemCard component.

**Target Audience**: Frontend developers working on the Rusty Shed collection feature

**Prerequisites**:

- ✅ RailwayModelCard component implemented
- ✅ RailwayModelPreviewCard component implemented
- ✅ Familiarity with Svelte 5 runes and TypeScript
- ✅ Understanding of collection data structure (CollectionItemView)

---

## Quick Start (TL;DR)

```bash
# 1. Create the mapper utility
touch src/lib/features/collection/utils/cardDataMapper.ts

# 2. Implement transformation functions (see Implementation section)

# 3. Update CollectionDashboard.svelte
#    - Import cardDataMapper and RailwayModelPreviewCard
#    - Replace <ItemCard> with <RailwayModelPreviewCard>
#    - Transform data: collectionItemToCardData(item)

# 4. Write tests
touch tests/unit/features/collection/cardDataMapper.test.ts

# 5. Run quality gates
pnpm lint
pnpm check
pnpm test
```

---

## Step-by-Step Implementation

### Step 1: Create the Data Mapper Utility

**File**: `src/lib/features/collection/utils/cardDataMapper.ts`

Create the utility module with transformation functions:

```typescript
import type {
  CollectionItemView,
  OwnedRollingStockView,
  Category,
  Control,
  FeatureFlag,
  PurchaseInfo
} from '$lib/bindings';

import type {
  RailwayModelCardData,
  ModelCategory,
  DigitalFeature
} from '$lib/components/RailwayModelPreviewCard.svelte';

/**
 * Transform CollectionItemView to RailwayModelCardData
 */
export function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData {
  const { railwayModel, purchaseInfo, rollingStocks } = item;

  return {
    id: railwayModel.railwayModelId,
    manufacturer: railwayModel.manufacturer,
    productCode: railwayModel.productCode,
    series: railwayModel.description,
    category: mapCategory(railwayModel.category, rollingStocks),
    roadNumber: rollingStocks[0]?.roadNumber ?? null,
    scale: railwayModel.scale,
    powerMethod: null, // Not available
    era: railwayModel.epoch,
    purchaseDate: extractPurchaseDate(purchaseInfo),
    photoUrl: null, // Not available
    unitCount: rollingStocks.length > 1 ? rollingStocks.length : null,
    digitalFeatures: extractDigitalFeatures(rollingStocks)
  };
}

/**
 * Map backend Category to frontend ModelCategory
 */
export function mapCategory(
  category: Category | null,
  rollingStocks: OwnedRollingStockView[]
): ModelCategory {
  if (!category) return 'Unknown';

  switch (category) {
    case 'FREIGHT_CARS':
      return 'FreightCar';
    case 'PASSENGER_CARS':
      return 'PassengerCar';
    case 'TRAIN_SETS':
    case 'STARTER_SETS':
    case 'ELECTRIC_MULTIPLE_UNITS':
      return 'TrainSet';
    case 'RAILCARS':
      return 'Railcar';
    case 'LOCOMOTIVES': {
      const locomotiveUnit = rollingStocks.find((rs) => rs.locomotive);
      const locType = locomotiveUnit?.locomotive?.locomotiveType;

      switch (locType) {
        case 'STEAM_LOCOMOTIVE':
          return 'SteamLocomotive';
        case 'DIESEL_LOCOMOTIVE':
          return 'DieselLocomotive';
        case 'ELECTRIC_LOCOMOTIVE':
          return 'ElectricLocomotive';
        default:
          return 'SteamLocomotive'; // Fallback
      }
    }
    default:
      return 'Unknown';
  }
}

/**
 * Extract digital features from rolling stock
 */
export function extractDigitalFeatures(rollingStocks: OwnedRollingStockView[]): DigitalFeature[] {
  const features = new Set<DigitalFeature>();

  for (const unit of rollingStocks) {
    const motorData = unit.locomotive || unit.electricMultipleUnit || unit.railcar || null;

    if (!motorData) continue;

    // Sound detection
    if (unit.control === 'DCC_SOUND') {
      features.add('Sound');
    }

    // DCC detection
    const hasDcc = (unit.control && unit.control !== 'NO_DCC') || unit.digital !== null;

    if (hasDcc) {
      features.add('DCC');
    }

    // Light detection
    if ('technical_specifications' in motorData) {
      const lights = motorData.technical_specifications?.lights;
      if (lights === 'YES') {
        features.add('Light');
      }
    }
  }

  return Array.from(features);
}

/**
 * Extract purchase date from discriminated union
 */
export function extractPurchaseDate(purchaseInfo: PurchaseInfo | null): string | null {
  if (!purchaseInfo) return null;

  switch (purchaseInfo.type) {
    case 'purchased':
      return purchaseInfo.data.purchaseDate;
    case 'preOrdered':
      return purchaseInfo.data.purchaseDate ?? null;
    case 'sold':
      return null;
    default:
      return null;
  }
}
```

---

### Step 2: Update CollectionDashboard Component

**File**: `src/lib/features/collection/CollectionDashboard.svelte`

#### 2a. Add Imports

Replace the ItemCard import with RailwayModelPreviewCard:

```typescript
// Remove: import ItemCard from './components/ItemCard.svelte';

// Add:
import RailwayModelPreviewCard from '$lib/components/RailwayModelPreviewCard.svelte';
import { collectionItemToCardData } from './utils/cardDataMapper';
```

#### 2b. Update Card Rendering

Find the grid rendering section (around line 250-255) and replace:

```svelte
<!-- OLD CODE - Remove this -->
<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  {#each filteredItems as item (item.id)}
    <ItemCard {item} onDelete={ui.requestDelete} onClick={handleCardClick} />
  {/each}
</div>
```

With:

```svelte
<!-- NEW CODE -->
<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
  {#each filteredItems as item (item.id)}
    <RailwayModelPreviewCard
      model={collectionItemToCardData(item)}
      onDelete={() => ui.requestDelete(item.id)}
      onclick={() => handleCardClick(item)}
    />
  {/each}
</div>
```

**Key Changes**:

- ✅ Transform data with `collectionItemToCardData(item)`
- ✅ Pass `model` prop (not `item`)
- ✅ Wrap `onDelete` callback to pass `item.id`
- ✅ Use `onclick` (lowercase) instead of `onClick`

---

### Step 3: Write Unit Tests

**File**: `tests/unit/features/collection/cardDataMapper.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import {
  collectionItemToCardData,
  mapCategory,
  extractDigitalFeatures,
  extractPurchaseDate
} from '$lib/features/collection/utils/cardDataMapper';

describe('cardDataMapper', () => {
  describe('collectionItemToCardData', () => {
    it('should map all direct fields correctly', () => {
      const item = {
        id: 'coll-123',
        railwayModel: {
          railwayModelId: 'trn:railway-model:marklin:3000',
          manufacturer: 'Märklin',
          productCode: '3000',
          scale: 'H0',
          epoch: 'III',
          description: 'BR 89.0',
          category: 'LOCOMOTIVES'
        },
        rollingStocks: [
          {
            id: 'rs-1',
            roadNumber: '89 006',
            control: 'DCC_SOUND',
            digital: { interface: 'PLUX_22', dcc_address: 3 },
            locomotive: {
              locomotiveType: 'STEAM_LOCOMOTIVE',
              technical_specifications: {
                lights: 'YES',
                interior_lights: 'NO'
              }
            }
          }
        ],
        purchaseInfo: {
          type: 'purchased',
          data: { purchaseDate: '2024-03-15', retailer: null, price: null }
        },
        addedDate: '2024-03-16',
        notes: null
      };

      const result = collectionItemToCardData(item);

      expect(result).toEqual({
        id: 'trn:railway-model:marklin:3000',
        manufacturer: 'Märklin',
        productCode: '3000',
        series: 'BR 89.0',
        category: 'SteamLocomotive',
        roadNumber: '89 006',
        scale: 'H0',
        powerMethod: null,
        era: 'III',
        purchaseDate: '2024-03-15',
        photoUrl: null,
        unitCount: null,
        digitalFeatures: ['Sound', 'DCC', 'Light']
      });
    });
  });

  describe('mapCategory', () => {
    it('should map FREIGHT_CARS to FreightCar', () => {
      expect(mapCategory('FREIGHT_CARS', [])).toBe('FreightCar');
    });

    it('should refine LOCOMOTIVES with locomotive type', () => {
      const rollingStocks = [
        {
          locomotive: { locomotiveType: 'STEAM_LOCOMOTIVE' }
        }
      ];
      expect(mapCategory('LOCOMOTIVES', rollingStocks)).toBe('SteamLocomotive');
    });

    it('should default to SteamLocomotive if type missing', () => {
      expect(mapCategory('LOCOMOTIVES', [])).toBe('SteamLocomotive');
    });

    it('should return Unknown for null category', () => {
      expect(mapCategory(null, [])).toBe('Unknown');
    });
  });

  describe('extractDigitalFeatures', () => {
    it('should detect Sound from DCC_SOUND control', () => {
      const rollingStocks = [
        {
          control: 'DCC_SOUND',
          digital: null,
          locomotive: { locomotiveType: 'STEAM_LOCOMOTIVE' }
        }
      ];
      expect(extractDigitalFeatures(rollingStocks)).toContain('Sound');
    });

    it('should detect DCC from control', () => {
      const rollingStocks = [
        {
          control: 'DCC_FITTED',
          digital: null,
          locomotive: { locomotiveType: 'DIESEL_LOCOMOTIVE' }
        }
      ];
      expect(extractDigitalFeatures(rollingStocks)).toContain('DCC');
    });

    it('should detect Light from technical specs', () => {
      const rollingStocks = [
        {
          control: null,
          digital: null,
          locomotive: {
            locomotiveType: 'ELECTRIC_LOCOMOTIVE',
            technical_specifications: { lights: 'YES', interior_lights: 'NO' }
          }
        }
      ];
      expect(extractDigitalFeatures(rollingStocks)).toContain('Light');
    });

    it('should return empty array for freight cars', () => {
      const rollingStocks = [{ control: null, digital: null, freightCar: {} }];
      expect(extractDigitalFeatures(rollingStocks)).toEqual([]);
    });

    it('should deduplicate features across units', () => {
      const rollingStocks = [
        { control: 'DCC_FITTED', locomotive: { locomotiveType: 'STEAM' } },
        { control: 'DCC_FITTED', locomotive: { locomotiveType: 'STEAM' } }
      ];
      const features = extractDigitalFeatures(rollingStocks);
      expect(features.filter((f) => f === 'DCC')).toHaveLength(1);
    });
  });

  describe('extractPurchaseDate', () => {
    it('should extract date from purchased type', () => {
      const purchaseInfo = {
        type: 'purchased',
        data: { purchaseDate: '2024-03-15', retailer: null, price: null }
      };
      expect(extractPurchaseDate(purchaseInfo)).toBe('2024-03-15');
    });

    it('should return null for sold type', () => {
      const purchaseInfo = {
        type: 'sold',
        data: { soldDate: '2023-12-01', buyer: null, price: null }
      };
      expect(extractPurchaseDate(purchaseInfo)).toBe(null);
    });

    it('should return null for null input', () => {
      expect(extractPurchaseDate(null)).toBe(null);
    });
  });
});
```

---

### Step 4: Run Quality Gates

Before committing, run all quality checks:

```bash
# Format code
pnpm format

# Lint check
pnpm lint

# TypeScript check
pnpm check

# Run tests
pnpm test

# Coverage check
pnpm test:coverage
```

**Expected Results**:

- ✅ No lint errors
- ✅ No TypeScript errors
- ✅ All tests pass
- ✅ Coverage ≥ 90% for mapper utility

---

## Testing Strategy

### Unit Tests

**Focus**: cardDataMapper utility functions

**Coverage Target**: 90%+ line coverage, 100% branch coverage

**Test Categories**:

1. Field mapping tests (direct, semantic, null handling)
2. Category mapping tests (all enum values, subtype refinement)
3. Digital features extraction tests (all control types, deduplication)
4. Purchase date extraction tests (all union types)
5. Edge case tests (empty arrays, null values, missing fields)

### Component Integration Tests

**Focus**: RailwayModelPreviewCard rendering in CollectionDashboard

**Test Cases**:

1. ✅ Card renders with correct props
2. ✅ Click handler navigates to model detail page
3. ✅ Delete button opens confirmation dialog
4. ✅ Digital feature badges appear correctly
5. ✅ Category placeholder displays when no photo
6. ✅ Unit count badge shows for multi-unit models

### Manual Testing Checklist

After implementation, manually verify:

- [ ] Collection page displays models with new preview cards
- [ ] Thumbnails display or show category-specific placeholders
- [ ] Metadata badges (scale, era, power method) visible
- [ ] Digital feature overlays (Sound, DCC icons) appear correctly
- [ ] Unit count badge shows on multi-unit models
- [ ] Road numbers display and truncate with expand toggle
- [ ] Clicking card navigates to `/models/[id]` detail page
- [ ] Delete button opens confirmation dialog
- [ ] Hover effects work smoothly
- [ ] Responsive layout works (mobile, tablet, desktop)
- [ ] Filter and search functionality still works
- [ ] Empty state and no-results state still display
- [ ] Loading skeletons work during data fetch

---

## Common Pitfalls & Troubleshooting

### Issue 1: TypeScript Import Errors

**Problem**: Cannot import types from RailwayModelPreviewCard.svelte

**Solution**: Export types explicitly:

```typescript
// In RailwayModelPreviewCard.svelte
export type { RailwayModelCardData, ModelCategory, DigitalFeature };
```

### Issue 2: Null/Undefined Errors

**Problem**: Runtime errors accessing nested properties

**Solution**: Use optional chaining and nullish coalescing:

```typescript
// ✅ Good
const roadNumber = rollingStocks[0]?.roadNumber ?? null;

// ❌ Bad
const roadNumber = rollingStocks[0].roadNumber || null;
```

### Issue 3: Category Icon Not Showing

**Problem**: Placeholder icon not displaying when photoUrl is null

**Solution**: Ensure `category` is a valid `ModelCategory` value:

```typescript
// Check mapCategory returns valid enum value
console.log(mapCategory(item.railwayModel.category, item.rollingStocks));
// Should be one of: 'SteamLocomotive', 'FreightCar', etc.
```

### Issue 4: Digital Features Not Appearing

**Problem**: DCC/Sound badges not rendering on cards

**Solution**: Verify rolling stock has correct data structure:

```typescript
// Check extractDigitalFeatures output
console.log(extractDigitalFeatures(item.rollingStocks));
// Should be array like: ['Sound', 'DCC', 'Light']
```

### Issue 5: Unit Count Always Null

**Problem**: Multi-unit models not showing unit count badge

**Solution**: Check logic for unit count calculation:

```typescript
// Only show unit count if > 1
unitCount: rollingStocks.length > 1 ? rollingStocks.length : null;
```

### Issue 6: Click Handler Not Working

**Problem**: Clicking card doesn't navigate

**Solution**: Use lowercase `onclick` (Svelte 5):

```svelte
<!-- ✅ Correct (Svelte 5) -->
<RailwayModelPreviewCard onclick={() => handleCardClick(item)} />

<!-- ❌ Wrong (Svelte 4 syntax) -->
<RailwayModelPreviewCard onClick={() => handleCardClick(item)} />
```

---

## Performance Considerations

### Transformation Overhead

- **Expected**: <1ms per model transformation
- **Typical load**: 100 models = ~100ms total transformation time
- **Impact**: Negligible (rendering is the bottleneck, not transformation)

### Optimization Tips

1. **Virtual Scrolling**: Already implemented in collection view
2. **Memoization**: Not needed unless profiling shows issues
3. **Lazy Loading**: Consider for large collections (1000+ models)

**Profiling Command**:

```typescript
console.time('transform');
const cardData = collectionItemToCardData(item);
console.timeEnd('transform');
// Should be <1ms
```

---

## Rollback Plan

If integration causes issues, you can temporarily rollback:

1. **Revert CollectionDashboard changes**:
   - Restore ItemCard import
   - Restore old grid rendering code
   - Remove cardDataMapper import

2. **Keep new code for future use**:
   - Leave cardDataMapper.ts in place
   - Leave tests in place
   - Document issues for resolution

**Rollback does NOT require**:

- Database migrations (no schema changes)
- Backend changes (no API changes)
- Rebuild (only frontend changes)

---

## Next Steps

After successful integration:

1. ✅ Deprecate ItemCard component (mark as deprecated in code comments)
2. ✅ Document new pattern in project docs
3. ✅ Consider extending to other pages (if applicable)
4. ✅ Add powerMethod and photoUrl support when backend ready

---

## Support & Resources

**Documentation**:

- [Feature Specification](./spec.md)
- [Research Report](./research.md)
- [Data Model](./data-model.md)
- [Mapper Contract](./contracts/mapper-contract.md)

**Related Components**:

- RailwayModelPreviewCard: `src/lib/components/RailwayModelPreviewCard.svelte`
- RailwayModelCard: `src/lib/components/RailwayModelCard.svelte`
- CollectionDashboard: `src/lib/features/collection/CollectionDashboard.svelte`

**Questions?** Refer to the research report for design decisions and rationale.
