# Data Model: Collection Page Card Integration

**Feature**: 021-collection-page-cards
**Date**: 2026-02-12
**Phase**: Phase 1 - Design & Contracts

## Overview

This document defines the data structures and transformation logic required to integrate RailwayModelCard and RailwayModelPreviewCard components into the collection page.

---

## Core Interfaces

### RailwayModelCardData (Target Interface)

**Source**: `src/lib/components/RailwayModelPreviewCard.svelte` (lines 34-61)

**Purpose**: Props interface for RailwayModelPreviewCard component

```typescript
interface RailwayModelCardData {
  /** Unique identifier for the model */
  id: string;

  /** Manufacturer name (e.g., "A.C.M.E.", "Märklin") */
  manufacturer: string | null;

  /** Manufacturer's product code (e.g., "1236", "H0-2047") */
  productCode: string | null;

  /** Series designation (e.g., "Class 140", "BR 01") */
  series: string | null;

  /** Model category for classification */
  category: ModelCategory;

  /** Road number / identification marking (e.g., "50 80 26-81 517-7") */
  roadNumber: string | null;

  /** Model scale (e.g., "H0", "N", "TT", "Z") */
  scale: string | null;

  /** Power method (e.g., "DC", "AC", "DCC") */
  powerMethod: string | null;

  /** Historical era classification (e.g., "III", "IV", "V") */
  era: string | null;

  /** Purchase date (ISO 8601 format: "YYYY-MM-DD") */
  purchaseDate: string | null;

  /** URL to model photo/image */
  photoUrl: string | null;

  /** Number of units in the set (e.g., 3 for a 3-car train set) */
  unitCount: number | null;

  /** Digital features available on the model */
  digitalFeatures: DigitalFeature[];
}

type DigitalFeature = 'Sound' | 'DCC' | 'Smoke' | 'Light';

type ModelCategory =
  | 'SteamLocomotive'
  | 'ElectricLocomotive'
  | 'DieselLocomotive'
  | 'Wagon'
  | 'PassengerCar'
  | 'FreightCar'
  | 'Railcar'
  | 'TrainSet'
  | 'Unknown';
```

### CollectionItemView (Source Interface)

**Source**: TypeScript bindings from Rust backend

**Purpose**: Data structure returned by collection queries

```typescript
interface CollectionItemView {
  /** Unique identifier for the collection item */
  id: string;

  /** Date when item was added to collection (ISO 8601) */
  addedDate: string;

  /** Optional user notes about this item */
  notes: string | null;

  /** Nested railway model information */
  railwayModel: CollectionRailwayModel;

  /** Purchase information (discriminated union) */
  purchaseInfo: PurchaseInfo | null;

  /** Array of owned rolling stock units */
  rollingStocks: OwnedRollingStockView[];
}

interface CollectionRailwayModel {
  /** Railway model ID (TRN format) */
  railwayModelId: string;

  /** Manufacturer name */
  manufacturer: string | null;

  /** Product code */
  productCode: string | null;

  /** Model scale */
  scale: string;

  /** Historical epoch/era */
  epoch: string;

  /** Model description (used as series) */
  description: string;

  /** Model category enum */
  category: Category;
}

type Category =
  | 'LOCOMOTIVES'
  | 'TRAIN_SETS'
  | 'STARTER_SETS'
  | 'FREIGHT_CARS'
  | 'PASSENGER_CARS'
  | 'ELECTRIC_MULTIPLE_UNITS'
  | 'RAILCARS';

// Discriminated union for purchase info
type PurchaseInfo =
  | { type: 'purchased'; data: PurchasedData }
  | { type: 'sold'; data: SoldData }
  | { type: 'preOrdered'; data: PreOrderedData };

interface PurchasedData {
  purchaseDate: string; // ISO 8601
  retailer: string | null;
  price: Money | null;
}

interface OwnedRollingStockView {
  /** Rolling stock ID */
  id: string;

  /** Road number */
  roadNumber: string | null;

  /** Control type */
  control: Control | null;

  /** Digital setup information */
  digital: DigitalSetup | null;

  /** Discriminated union for rolling stock type */
  locomotive?: LocomotiveData;
  electricMultipleUnit?: ElectricMultipleUnitData;
  railcar?: RailcarData;
  freightCar?: FreightCarData;
  passengerCar?: PassengerCarData;
}

type Control = 'DCC_READY' | 'DCC_FITTED' | 'DCC_SOUND' | 'NO_DCC';

interface DigitalSetup {
  interface: DccInterface;
  dcc_address: number;
  installed_decoder_id: string | null;
}

type DccInterface =
  | 'NEM_651'
  | 'NEM_652'
  | 'NEM_654'
  | 'PLUX_8'
  | 'PLUX_12'
  | 'PLUX_16'
  | 'PLUX_22'
  | 'NEXT_18'
  | 'NEXT_18_S'
  | 'MTC_21';

interface LocomotiveData {
  locomotiveType: 'STEAM_LOCOMOTIVE' | 'DIESEL_LOCOMOTIVE' | 'ELECTRIC_LOCOMOTIVE';
  technical_specifications: TechnicalSpecifications | null;
}

interface TechnicalSpecifications {
  lights: FeatureFlag | null;
  interior_lights: FeatureFlag | null;
}

type FeatureFlag = 'YES' | 'NO' | 'NOT_APPLICABLE';
```

---

## Transformation Logic

### Main Transformation Function

**Function**: `collectionItemToCardData`

**Signature**:

```typescript
function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData;
```

**Field Mapping**:

| Target Field      | Source Path                               | Transformation             | Notes                |
| ----------------- | ----------------------------------------- | -------------------------- | -------------------- |
| `id`              | `railwayModel.railwayModelId`             | Direct copy                | String               |
| `manufacturer`    | `railwayModel.manufacturer`               | Direct copy                | Nullable             |
| `productCode`     | `railwayModel.productCode`                | Direct copy                | Nullable             |
| `series`          | `railwayModel.description`                | Direct copy                | Semantic mapping     |
| `category`        | `railwayModel.category` + `rollingStocks` | `mapCategory()`            | See Category Mapping |
| `roadNumber`      | `rollingStocks[0]?.roadNumber`            | First unit                 | Nullable             |
| `scale`           | `railwayModel.scale`                      | Direct copy                | String               |
| `powerMethod`     | N/A                                       | `null`                     | Not available        |
| `era`             | `railwayModel.epoch`                      | Direct copy                | String               |
| `purchaseDate`    | `purchaseInfo?.data?.purchaseDate`        | Conditional extract        | Union type           |
| `photoUrl`        | N/A                                       | `null`                     | Not available        |
| `unitCount`       | `rollingStocks.length`                    | Count if > 1               | Nullable             |
| `digitalFeatures` | `rollingStocks[]`                         | `extractDigitalFeatures()` | See Digital Features |

**Implementation**:

```typescript
export function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData {
  const { railwayModel, purchaseInfo, rollingStocks } = item;

  return {
    id: railwayModel.railwayModelId,
    manufacturer: railwayModel.manufacturer,
    productCode: railwayModel.productCode,
    series: railwayModel.description, // Map description → series
    category: mapCategory(railwayModel.category, rollingStocks),
    roadNumber: rollingStocks[0]?.roadNumber ?? null,
    scale: railwayModel.scale,
    powerMethod: null, // Not available in CollectionRailwayModel
    era: railwayModel.epoch,
    purchaseDate: extractPurchaseDate(purchaseInfo),
    photoUrl: null, // Not available in current schema
    unitCount: rollingStocks.length > 1 ? rollingStocks.length : null,
    digitalFeatures: extractDigitalFeatures(rollingStocks)
  };
}
```

---

### Category Mapping

**Function**: `mapCategory`

**Signature**:

```typescript
function mapCategory(
  category: Category | null,
  rollingStocks: OwnedRollingStockView[]
): ModelCategory;
```

**Mapping Rules**:

```typescript
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
      // Refine based on locomotive type subtype
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
          return 'SteamLocomotive'; // Safe fallback
      }
    }

    default:
      return 'Unknown';
  }
}
```

**Edge Cases**:

- `null` category → `'Unknown'`
- `LOCOMOTIVES` without rolling stock → `'SteamLocomotive'` (fallback)
- Unmapped category values → `'Unknown'`

---

### Digital Features Extraction

**Function**: `extractDigitalFeatures`

**Signature**:

```typescript
function extractDigitalFeatures(rollingStocks: OwnedRollingStockView[]): DigitalFeature[];
```

**Extraction Rules**:

| Feature   | Condition                                                        | Data Source                                           |
| --------- | ---------------------------------------------------------------- | ----------------------------------------------------- |
| `'Sound'` | `control === 'DCC_SOUND'`                                        | `rollingStocks[].control`                             |
| `'DCC'`   | `control !== 'NO_DCC' && control !== null` OR `digital !== null` | `rollingStocks[].control` or `.digital`               |
| `'Light'` | `technical_specifications?.lights === 'YES'`                     | `rollingStocks[].locomotive.technical_specifications` |
| `'Smoke'` | ❌ Not available                                                 | N/A                                                   |

**Implementation**:

```typescript
export function extractDigitalFeatures(rollingStocks: OwnedRollingStockView[]): DigitalFeature[] {
  const features = new Set<DigitalFeature>();

  for (const unit of rollingStocks) {
    // Only motor-enabled types have these features
    const motorData = unit.locomotive || unit.electricMultipleUnit || unit.railcar || null;

    if (!motorData) continue; // Skip freight/passenger cars

    // Sound detection
    if (unit.control === 'DCC_SOUND') {
      features.add('Sound');
    }

    // DCC detection
    const hasDcc = (unit.control && unit.control !== 'NO_DCC') || unit.digital !== null;

    if (hasDcc) {
      features.add('DCC');
    }

    // Light detection (locomotive-specific)
    if ('technical_specifications' in motorData) {
      const lights = motorData.technical_specifications?.lights;
      if (lights === 'YES') {
        features.add('Light');
      }
    }
  }

  return Array.from(features);
}
```

**Edge Cases**:

- Empty rolling stock array → `[]`
- Freight/passenger cars only → `[]`
- Null/undefined control → skip DCC/Sound detection
- Multiple units → union of all features (Set deduplication)

---

### Purchase Date Extraction

**Function**: `extractPurchaseDate`

**Signature**:

```typescript
function extractPurchaseDate(purchaseInfo: PurchaseInfo | null): string | null;
```

**Implementation**:

```typescript
export function extractPurchaseDate(purchaseInfo: PurchaseInfo | null): string | null {
  if (!purchaseInfo) return null;

  // Handle discriminated union
  switch (purchaseInfo.type) {
    case 'purchased':
      return purchaseInfo.data.purchaseDate;
    case 'preOrdered':
      return purchaseInfo.data.purchaseDate ?? null;
    case 'sold':
      return null; // Sold items don't show purchase date on cards
    default:
      return null;
  }
}
```

**Edge Cases**:

- `null` purchaseInfo → `null`
- `type: 'sold'` → `null` (don't show purchase date for sold items)
- `type: 'preOrdered'` with null date → `null`

---

## Null/Undefined Handling Strategy

### Principles

1. **Use nullish coalescing (`??`)** for optional field access
2. **Use optional chaining (`?.`)** for nested property access
3. **Prefer `null` over `undefined`** for API boundaries (JSON compatibility)
4. **Validate at transformation boundary** - ensure output type safety

### Examples

```typescript
// ✅ Good: Safe navigation with nullish coalescing
const roadNumber = item.rollingStocks[0]?.roadNumber ?? null;

// ✅ Good: Optional chaining for deeply nested fields
const purchaseDate = item.purchaseInfo?.data?.purchaseDate ?? null;

// ✅ Good: Explicit null for missing data
const powerMethod = null; // Not available in source data

// ❌ Bad: Unsafe access
const roadNumber = item.rollingStocks[0].roadNumber; // May throw if array empty

// ❌ Bad: Mixed null/undefined
const powerMethod = undefined; // Use null for consistency
```

---

## Validation Rules

### Input Validation

None required - TypeScript types enforce structure at compile time.

### Output Validation

All fields in `RailwayModelCardData` must conform to expected types:

- `id`: Non-empty string (guaranteed by database)
- `category`: Valid `ModelCategory` enum value
- `digitalFeatures`: Array of valid `DigitalFeature` values (no duplicates)
- `unitCount`: Null or integer > 1
- `purchaseDate`: Null or ISO 8601 string (YYYY-MM-DD)

**Assertions**:

```typescript
// Assert unit count is meaningful
if (rollingStocks.length === 1) {
  unitCount = null; // Don't show "1 unit" badge
}

// Assert digital features are unique
const features = new Set<DigitalFeature>(); // Automatic deduplication
return Array.from(features);
```

---

## Performance Considerations

### Complexity Analysis

| Operation                   | Complexity                         | Impact                |
| --------------------------- | ---------------------------------- | --------------------- |
| Direct field copy           | O(1)                               | Negligible            |
| Category mapping            | O(n) where n = rolling stock count | Low (typical n = 1-5) |
| Digital features extraction | O(n) where n = rolling stock count | Low (typical n = 1-5) |
| Purchase date extraction    | O(1)                               | Negligible            |

**Typical collection rendering**:

- 100 models × ~3 rolling stock each = 300 units processed
- Estimated transformation time: <5ms (negligible)

### Optimization Notes

- No optimization needed for current scale
- If profiling reveals issues, consider:
  - Memoizing transformed data
  - Virtual scrolling for large collections (already implemented)
  - Lazy loading of rolling stock details

---

## Testing Strategy

### Unit Tests

**File**: `tests/unit/features/collection/cardDataMapper.test.ts`

**Test Cases**:

1. **Field Mapping**
   - ✅ Direct field copies (id, manufacturer, productCode, scale, era)
   - ✅ Semantic mapping (description → series, epoch → era)
   - ✅ Null handling (manufacturer, productCode, roadNumber)
   - ✅ Missing fields (powerMethod, photoUrl return null)

2. **Category Mapping**
   - ✅ Direct mappings (FREIGHT_CARS → FreightCar, etc.)
   - ✅ Locomotive subtype refinement (STEAM_LOCOMOTIVE → SteamLocomotive)
   - ✅ Fallback behavior (null → Unknown, missing subtype → SteamLocomotive)
   - ✅ All category enum values covered

3. **Digital Features Extraction**
   - ✅ Sound detection (control === 'DCC_SOUND')
   - ✅ DCC detection (control !== 'NO_DCC', digital !== null)
   - ✅ Light detection (technical_specifications.lights === 'YES')
   - ✅ Empty rolling stock array
   - ✅ Freight/passenger cars (no features)
   - ✅ Deduplication across multiple units

4. **Purchase Date Extraction**
   - ✅ Purchased type with date
   - ✅ PreOrdered type with date
   - ✅ Sold type (returns null)
   - ✅ Null purchaseInfo

5. **Edge Cases**
   - ✅ Empty rolling stock array
   - ✅ Single unit (unitCount = null)
   - ✅ Multiple units (unitCount = length)
   - ✅ All nullable fields set to null

### Integration Tests

**File**: `tests/unit/features/collection/CollectionDashboard.test.ts`

**Test Cases**:

1. ✅ RailwayModelPreviewCard receives correct props
2. ✅ Click handler navigates to correct route
3. ✅ Delete handler opens confirmation dialog
4. ✅ Digital feature badges render correctly
5. ✅ Category placeholders display when no photoUrl
6. ✅ Unit count badge appears for multi-unit models

---

## Migration Notes

### Breaking Changes

None - this is a new transformation utility.

### Backward Compatibility

- Existing ItemCard component can coexist during transition
- Gradual migration: replace ItemCard → RailwayModelPreviewCard per page

### Future Enhancements

1. **Add powerMethod to backend DTO**
   - Modify `CollectionRailwayModel` to include `powerMethod` field
   - Update mapper to use real value instead of `null`

2. **Implement image storage**
   - Add `photoUrl` or `imagePath` to collection views
   - Update mapper to use real image URLs

3. **Add Smoke feature support**
   - Add `technical_smoke_generator` field to rolling_stocks table
   - Update `TechnicalSpecifications` to include smoke flag
   - Update `extractDigitalFeatures()` to detect smoke

---

## Approval Status

**Reviewer**: Auto (Design Phase)
**Date**: 2026-02-12
**Status**: ✅ **APPROVED** - Ready for implementation

**Next Steps**: Proceed to contracts generation and quickstart guide
