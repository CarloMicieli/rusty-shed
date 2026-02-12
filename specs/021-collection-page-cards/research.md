# Research Report: Collection Page Card Integration

**Feature**: 021-collection-page-cards
**Date**: 2026-02-12
**Phase**: Phase 0 - Research & Discovery

## Executive Summary

This research phase investigated four key technical questions required to integrate RailwayModelCard and RailwayModelPreviewCard components into the collection page. All research tasks have been completed with actionable findings.

**Key Findings**:

1. Data mapping from CollectionItemView to RailwayModelCardData is feasible with known workarounds for missing fields
2. Digital features (Sound, DCC, Light) can be extracted from rolling stock control and technical specifications
3. Category mapping has clear rules from backend Category enum to frontend ModelCategory type
4. Detail view should continue using the existing dedicated route pattern (/models/[id])

## Research Task 1: Data Mapping Strategy

### Question

How to map CollectionItemView to RailwayModelCardData interface?

### Findings

#### Complete Mapping Table

| RailwayModelCardData Field | CollectionItemView Source             | Transformation Logic               | Implementation Notes                    |
| -------------------------- | ------------------------------------- | ---------------------------------- | --------------------------------------- |
| `id`                       | `railwayModel.railwayModelId`         | Direct mapping                     | String identifier                       |
| `manufacturer`             | `railwayModel.manufacturer`           | Direct mapping (nullable)          | May be null                             |
| `productCode`              | `railwayModel.productCode`            | Direct mapping (nullable)          | May be null                             |
| `series`                   | `railwayModel.description`            | Use description field              | Backend uses "description" not "series" |
| `category`                 | `railwayModel.category`               | Transform Category → ModelCategory | See Research Task 3                     |
| `roadNumber`               | `rollingStocks[0]?.roadNumber`        | Extract from first rolling stock   | May be null                             |
| `scale`                    | `railwayModel.scale`                  | Direct mapping                     | Scale enum value                        |
| `powerMethod`              | **NOT AVAILABLE**                     | Fallback to null                   | Not in CollectionRailwayModel           |
| `era`                      | `railwayModel.epoch`                  | Direct mapping                     | Backend uses "epoch" field              |
| `purchaseDate`             | `purchaseInfo?.data?.purchaseDate`    | Extract from union type            | Discriminated union handling            |
| `photoUrl`                 | **NOT AVAILABLE**                     | Fallback to null                   | No image field in current schema        |
| `unitCount`                | `rollingStocks.length`                | Count array length if > 1          | Null if single unit                     |
| `digitalFeatures`          | `rollingStocks[].digital` + `control` | Extract per Research Task 2        | Array aggregation                       |

#### Missing Fields Strategy

**Field: `powerMethod`**

- **Status**: Not available in CollectionRailwayModel
- **Workaround**: Set to `null`, component handles gracefully
- **Future**: Consider adding to backend DTO or fetching full RailwayModelView

**Field: `photoUrl`**

- **Status**: No image storage in collection views
- **Workaround**: Set to `null`, component displays category-specific placeholder icon
- **Future**: Implement image storage/retrieval system

**Field: `series`**

- **Status**: Backend uses `description` field instead
- **Decision**: Map `railwayModel.description` → `series`
- **Rationale**: Semantic overlap; description often contains series information

#### Null Safety Strategy

All optional fields must be handled with TypeScript's strict null checking:

```typescript
// Safe navigation for nested optional fields
const purchaseDate = item.purchaseInfo?.data?.purchaseDate ?? null;

// Safe array access for rolling stock
const roadNumber = item.rollingStocks?.[0]?.roadNumber ?? null;

// Fallback for missing fields
const powerMethod = null; // Not available in CollectionItemView
const photoUrl = null; // Not available in current schema
```

### Decision

**Approved**: Create `cardDataMapper.ts` utility module with transformation function that:

1. Maps all available fields directly
2. Extracts nested/array-based fields safely
3. Sets missing fields to `null`
4. Handles discriminated union types (purchaseInfo)
5. Aggregates digital features from rolling stock array

---

## Research Task 2: Digital Features Extraction

### Question

How to extract digital features (Sound, DCC, Smoke, Light) from model data?

### Findings

#### Available Fields in Rolling Stock Data

**Motor-enabled rolling stock types** (Locomotive, ElectricMultipleUnit, Railcar) provide:

- `control`: Control enum (`'DCC_READY'`, `'DCC_FITTED'`, `'DCC_SOUND'`, `'NO_DCC'`)
- `dcc_interface`: DccInterface enum (connector types: NEM_651, PLUX_8, NEXT_18, etc.)
- `technical_specifications.lights`: FeatureFlag (`'YES'`, `'NO'`, `'NOT_APPLICABLE'`)
- `technical_specifications.interior_lights`: FeatureFlag

**Non-motor types** (FreightCar, PassengerCar) do not have these fields.

#### Extraction Rules

| DigitalFeature | Extraction Rule                                                        | Data Source                                       |
| -------------- | ---------------------------------------------------------------------- | ------------------------------------------------- |
| **Sound**      | `control === 'DCC_SOUND'`                                              | `rollingStocks[].control`                         |
| **DCC**        | `control !== 'NO_DCC' && control !== null` OR `dcc_interface !== null` | `rollingStocks[].control` or `dcc_interface`      |
| **Light**      | `technical_specifications?.lights === 'YES'`                           | `rollingStocks[].technical_specifications.lights` |
| **Smoke**      | **Not available**                                                      | No field in current schema                        |

#### Implementation Algorithm

```typescript
function extractDigitalFeatures(rollingStock: RollingStockView[]): DigitalFeature[] {
  const features = new Set<DigitalFeature>();

  for (const unit of rollingStock) {
    // Extract motor-enabled rolling stock data
    const data = unit.locomotive || unit.electricMultipleUnit || unit.railcar || null;
    if (!data) continue; // Skip freight/passenger cars

    // Sound detection
    if (data.control === 'DCC_SOUND') {
      features.add('Sound');
    }

    // DCC detection
    const hasDcc = (data.control && data.control !== 'NO_DCC') || data.dcc_interface !== null;
    if (hasDcc) {
      features.add('DCC');
    }

    // Light detection
    if (data.technical_specifications?.lights === 'YES') {
      features.add('Light');
    }
  }

  return Array.from(features);
}
```

#### Edge Cases

1. **Multiple rolling stock units**: Aggregate features across all units (union of features)
2. **Null/undefined fields**: Safe navigation with optional chaining
3. **FreightCar/PassengerCar**: No control/DCC fields → return empty array
4. **DCC_READY without dcc_interface**: Still counts as DCC capability
5. **Smoke feature**: Currently unsupported; skip in implementation

### Decision

**Approved**: Implement `extractDigitalFeatures()` function in cardDataMapper utility that:

1. Iterates over all rolling stock units
2. Filters for motor-enabled types only
3. Aggregates unique features using Set
4. Returns array of DigitalFeature strings
5. Handles null safety throughout

---

## Research Task 3: Category Classification

### Question

How to map model categories to ModelCategory enum for placeholder icons?

### Findings

#### Backend Category Values

Source: `src-tauri/src/catalog/domain/railway_model/category.rs`

```rust
pub enum Category {
    Locomotives,
    TrainSets,
    StarterSets,
    FreightCars,
    PassengerCars,
    ElectricMultipleUnits,
    Railcars,
}
```

Database storage: SCREAMING_SNAKE_CASE (e.g., `LOCOMOTIVES`, `FREIGHT_CARS`)

#### Frontend ModelCategory Type

Source: `src/lib/components/RailwayModelPreviewCard.svelte` (lines 11-20)

```typescript
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

#### Category Mapping Table

| Backend Category          | Frontend ModelCategory      | Icon   | Reasoning                                      |
| ------------------------- | --------------------------- | ------ | ---------------------------------------------- |
| `LOCOMOTIVES`             | `SteamLocomotive` (default) | Train  | Requires locomotive_type subtype for precision |
| `TRAIN_SETS`              | `TrainSet`                  | Layers | Multi-unit combined sets                       |
| `STARTER_SETS`            | `TrainSet`                  | Layers | Treat as train set                             |
| `FREIGHT_CARS`            | `FreightCar`                | Box    | Cargo vehicles                                 |
| `PASSENGER_CARS`          | `PassengerCar`              | Users  | People transport                               |
| `ELECTRIC_MULTIPLE_UNITS` | `TrainSet`                  | Layers | Multi-unit electric trains                     |
| `RAILCARS`                | `Railcar`                   | Layers | Self-propelled passenger vehicles              |
| `null` / unmapped         | `Unknown`                   | Train  | Fallback                                       |

#### Locomotive Type Refinement

For `LOCOMOTIVES` category, the backend provides `locomotive_type` in rolling stock data:

```
STEAM_LOCOMOTIVE → SteamLocomotive (Train icon)
DIESEL_LOCOMOTIVE → DieselLocomotive (Zap icon)
ELECTRIC_LOCOMOTIVE → ElectricLocomotive (Zap icon)
```

**Storage location**: `rolling_stocks.locomotive_type` (separate from railway_models.category)

#### Implementation Strategy

```typescript
function mapCategory(category: Category | null, rollingStock: RollingStockView[]): ModelCategory {
  // Handle null/missing category
  if (!category) return 'Unknown';

  // Direct mappings
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

    // Locomotives require subtype lookup
    case 'LOCOMOTIVES': {
      const locomotiveUnit = rollingStock.find((rs) => rs.locomotive);
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

### Decision

**Approved**: Implement `mapCategory()` function in cardDataMapper utility that:

1. Handles null/undefined gracefully → 'Unknown'
2. Maps direct category values
3. Refines LOCOMOTIVES using locomotive_type subtype
4. Defaults to SteamLocomotive if subtype unavailable
5. Returns 'Unknown' for unmapped values

---

## Research Task 4: Detail View Pattern

### Question

Should detailed view use modal dialog or navigate to dedicated route?

### Findings

#### Current Implementation

**File**: `src/lib/features/collection/CollectionDashboard.svelte` (lines 143-145)

```typescript
function handleCardClick(item: CollectionItemView) {
  goto(`/models/${item.railwayModel.railwayModelId}`);
}
```

**Status**: The app currently navigates to a dedicated route `/models/[...modelId]`.

#### Existing /models/[modelId] Route

**File**: `src/routes/models/[...modelId]/+page.svelte`

**Features**:

- Full-screen dedicated page with comprehensive model details
- Displays RailwayModelView with tabbed rolling stock information
- Image upload functionality (drag-and-drop + file browser)
- Expandable rolling stock rows with detailed specifications
- Native back button support and browser history integration
- Deep linking capability (bookmarking, sharing URLs)
- Independent loading and error states
- Mobile-responsive design

#### Modal Patterns in the App

The application uses modals/dialogs for specific purposes:

1. **DeleteModal**: Destructive confirmation dialogs (centered overlay)
2. **AddModelDrawer**: Form-based data entry (right-sliding drawer)
3. **AlertDialog**: Quick confirmations in RailwayModelPreviewCard

**Pattern observation**: Modals are used for:

- Confirmations (delete, destructive actions)
- Forms (add/edit operations)
- Quick interactions

Routes are used for:

- Primary content views
- Detailed information display
- Complex multi-section layouts

#### Comparison Matrix

| Criterion       | Modal Dialog                       | Dedicated Route              |
| --------------- | ---------------------------------- | ---------------------------- |
| Screen space    | Limited                            | Full screen                  |
| Back button     | Custom handler needed              | Native browser support ✅    |
| Deep linking    | Not supported                      | Fully supported ✅           |
| Mobile UX       | Cramped                            | Responsive ✅                |
| Implementation  | New development                    | Already implemented ✅       |
| Content volume  | Poor for complex content           | Ideal for detailed views ✅  |
| Image upload UX | Constrained                        | Full drag-drop area ✅       |
| App consistency | Inconsistent with existing pattern | Matches existing behavior ✅ |

### Decision

**Approved**: **Keep the existing dedicated route pattern** (/models/[modelId])

**Rationale**:

1. ✅ **Already implemented and working** - The route exists and functions properly
2. ✅ **Content complexity** - RailwayModelCard displays extensive information (specs, tabs, rolling stock, image upload) requiring full screen space
3. ✅ **Consistent with app architecture** - Application uses SvelteKit routing for primary content navigation
4. ✅ **Better UX** - Native back button, deep linking, bookmarking, mobile-responsive
5. ✅ **Accessibility** - Simpler focus management, natural screen reader navigation
6. ✅ **User expectations** - Users expect "view details" to open a full view, not an overlay
7. ✅ **Existing modal patterns** - App reserves modals for confirmations and forms, not primary content

**Implementation Impact**: No changes needed to detail view navigation. Focus integration efforts on:

- Replacing ItemCard with RailwayModelPreviewCard in collection grid
- Ensuring data mapping utility correctly transforms CollectionItemView
- Preserving existing click handler (goto `/models/[id]`)

---

## Consolidated Implementation Plan

### Data Transformation Utility

**File**: `src/lib/features/collection/utils/cardDataMapper.ts`

**Exports**:

```typescript
// Main transformation function
export function collectionItemToCardData(item: CollectionItemView): RailwayModelCardData;

// Helper functions
export function extractDigitalFeatures(rollingStock: RollingStockView[]): DigitalFeature[];

export function mapCategory(
  category: Category | null,
  rollingStock: RollingStockView[]
): ModelCategory;
```

**Test Coverage**: `tests/unit/features/collection/cardDataMapper.test.ts`

- Test all field mappings
- Test null/undefined handling
- Test digital feature extraction for all control types
- Test category mapping including locomotive subtypes
- Test edge cases (empty rolling stock, null category, etc.)

### Component Integration

**File**: `src/lib/features/collection/CollectionDashboard.svelte`

**Changes**:

1. Import RailwayModelPreviewCard and cardDataMapper
2. Replace `<ItemCard>` with `<RailwayModelPreviewCard>` in line 253
3. Transform data using `collectionItemToCardData(item)`
4. Pass transformed data and callbacks to RailwayModelPreviewCard
5. Preserve existing click handler (handleCardClick with goto)
6. Preserve existing delete handler (ui.requestDelete)

**Preservation**:

- Keep existing navigation: `goto(/models/${id})`
- Keep DeleteModal for delete confirmations
- Keep FilterPanel, AddModelDrawer, loading states, empty states

---

## Open Questions & Future Work

### Missing Data Fields

1. **powerMethod**: Not available in CollectionRailwayModel
   - **Impact**: Medium - useful metadata for filtering/display
   - **Solution**: Consider extending backend DTO or accepting null

2. **photoUrl**: No image storage in collection views
   - **Impact**: High - affects visual presentation
   - **Solution**: Implement image storage system or fetch from RailwayModelView

3. **Smoke feature**: No field in current schema
   - **Impact**: Low - rarely used feature
   - **Solution**: Add `technical_smoke_generator` field in future migration

### Performance Considerations

- **Data transformation overhead**: Minimal - simple field mappings
- **Digital feature extraction**: O(n) where n = rolling stock count (typically 1-5 units)
- **Recommendation**: No optimization needed unless profiling shows issues

### Testing Strategy

1. **Unit tests**: cardDataMapper utility (all transformations)
2. **Component tests**: RailwayModelPreviewCard integration in CollectionDashboard
3. **Integration tests**: Full click-to-detail-view flow
4. **Regression tests**: Ensure existing collection features work (filter, search, delete)
5. **Visual regression**: Screenshot comparison before/after integration

---

## Approval Status

| Research Task               | Status      | Reviewer              | Date       |
| --------------------------- | ----------- | --------------------- | ---------- |
| Data Mapping Strategy       | ✅ Approved | Auto (Research Agent) | 2026-02-12 |
| Digital Features Extraction | ✅ Approved | Auto (Research Agent) | 2026-02-12 |
| Category Classification     | ✅ Approved | Auto (Research Agent) | 2026-02-12 |
| Detail View Pattern         | ✅ Approved | Auto (Research Agent) | 2026-02-12 |

**Overall Status**: ✅ **RESEARCH PHASE COMPLETE**

**Next Phase**: Proceed to Phase 1 - Design & Contracts (data-model.md, contracts/, quickstart.md)
