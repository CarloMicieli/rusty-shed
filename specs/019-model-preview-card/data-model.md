# Data Model: Railway Model Preview Card Component

**Date**: 2026-02-11
**Feature**: Railway Model Preview Card Component
**Purpose**: Define prop interface and data structures for the component

## Component Props Interface

### Primary Props

```typescript
interface RailwayModelPreviewCardProps {
  /**
   * The railway model data to display in the card
   */
  model: RailwayModelCardData;

  /**
   * Optional callback invoked when user confirms deletion
   * If not provided, delete button will not be rendered
   */
  onDelete?: (modelId: string) => void;

  /**
   * Optional CSS class to apply to the card root element
   */
  class?: string;
}
```

### RailwayModelCardData Interface

```typescript
interface RailwayModelCardData {
  /**
   * Unique identifier for the model
   */
  id: string;

  /**
   * Manufacturer name (e.g., "A.C.M.E.", "Märklin")
   * Required field - displays "Unknown" if not provided
   */
  manufacturer: string | null;

  /**
   * Manufacturer's product code (e.g., "1236", "H0-2047")
   */
  productCode: string | null;

  /**
   * Series designation (e.g., "Class 140", "BR 01")
   */
  series: string | null;

  /**
   * Model category for classification
   */
  category: ModelCategory;

  /**
   * Road number / identification marking (e.g., "50 80 26-81 517-7")
   * Truncated with ellipsis if exceeds 25 characters
   * Displays "---" if not provided
   */
  roadNumber: string | null;

  /**
   * Model scale (e.g., "H0", "N", "TT", "Z")
   */
  scale: string | null;

  /**
   * Power method (e.g., "DC", "AC", "DCC")
   */
  powerMethod: string | null;

  /**
   * Historical era classification (e.g., "III", "IV", "V")
   */
  era: string | null;

  /**
   * Purchase date (ISO 8601 format: "YYYY-MM-DD")
   */
  purchaseDate: string | null;

  /**
   * URL to model photo/image
   * If not provided, category-specific placeholder is shown
   */
  photoUrl: string | null;

  /**
   * Number of units in the set (e.g., 3 for a 3-car train set)
   * If > 1, displays unit count badge on thumbnail
   */
  unitCount: number | null;

  /**
   * Digital features available on the model
   * Used to display overlay icons (sound, DCC, etc.)
   */
  digitalFeatures: DigitalFeature[];
}
```

### Supporting Types

```typescript
/**
 * Model categories for placeholder icon selection
 */
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

/**
 * Digital features for overlay badges
 */
type DigitalFeature =
  | 'Sound' // Speaker icon overlay
  | 'DCC' // Digital Command Control (bolt icon)
  | 'Smoke' // Smoke generator
  | 'Light'; // Interior/exterior lighting
```

## Data Validation Rules

### Required Fields

- `id`: Must be non-empty string
- `category`: Must be valid ModelCategory enum value

### Optional Fields with Defaults

- `manufacturer`: Defaults to "Unknown" in display
- `roadNumber`: Defaults to "---" in display
- All other null fields: Omit from display (badges, metadata)

### Field Constraints

- `roadNumber`: If length > 25 characters, truncate to 22 chars + "..." and provide tooltip
- `unitCount`: Only display badge if value > 1
- `digitalFeatures`: Only display overlay if array is non-empty
- `photoUrl`: If null, show category-specific placeholder icon

## Derived State

The component computes the following derived values from props:

```typescript
// Truncation detection
const shouldTruncateRoadNumber = $derived(model.roadNumber && model.roadNumber.length > 25);

// Display values
const displayRoadNumber = $derived(
  model.roadNumber
    ? shouldTruncateRoadNumber
      ? model.roadNumber.substring(0, 22) + '...'
      : model.roadNumber
    : '---'
);

const displayManufacturer = $derived(model.manufacturer || 'Unknown');

// Placeholder icon selection
const placeholderIcon = $derived(() => {
  switch (model.category) {
    case 'SteamLocomotive':
      return Train;
    case 'ElectricLocomotive':
    case 'DieselLocomotive':
      return Zap;
    case 'Wagon':
    case 'FreightCar':
      return Box;
    case 'PassengerCar':
      return Users;
    case 'Railcar':
    case 'TrainSet':
      return Layers;
    default:
      return Train; // Generic fallback
  }
});

// Badge visibility flags
const showUnitCountBadge = $derived(model.unitCount && model.unitCount > 1);

const hasDigitalFeatures = $derived(model.digitalFeatures.length > 0);
```

## Component Events

### onDelete Event

```typescript
/**
 * Emitted when user confirms deletion in the confirmation dialog
 * Parent component is responsible for handling the actual deletion logic
 *
 * @param modelId - ID of the model to delete
 */
type DeleteHandler = (modelId: string) => void;
```

**Flow**:

1. User clicks trash button
2. AlertDialog opens with confirmation message
3. If user clicks "Delete", `onDelete(model.id)` is called
4. If user clicks "Cancel" or closes dialog, no event is emitted
5. Parent component handles state update and backend deletion

## Styling Contracts

### Tailwind Classes Used

**Container**:

- `@container` - Enables container queries for responsive behavior
- `card` - shadcn-svelte card base styles
- `gauge-frame` - Project-specific card frame styling
- `ring-1 ring-border/40` - Subtle border per style guide

**Layout**:

- `grid @lg:grid-cols-[auto_1fr] @sm:grid-cols-1` - Responsive grid
- `aspect-video` - 16:9 aspect ratio for thumbnail
- `gap-4` - Consistent spacing

**Typography**:

- `font-mono` - Monospaced font for road number identification plate
- `text-sm`, `text-xs` - Size hierarchy

**Badges**:

- Badge component from shadcn-svelte with `variant="secondary"`

### CSS Custom Properties

**Expected design tokens** (from Skeleton UI 4.x):

- `--color-surface-200` - Placeholder background
- `--color-surface-500` - Icon color
- `--color-border` - Card border
- `--font-mono` - Monospaced font family

## Internationalization (i18n)

### Required Message Keys

```typescript
// messages/en/components.json
{
  "components_deleteConfirmTitle": "Delete Model?",
  "components_deleteConfirmMessage": "Are you sure you want to delete {model}? This action cannot be undone.",
  "components_unknownManufacturer": "Unknown",
  "components_noRoadNumber": "---",
  "components_purchaseDate": "PURCHASED"
}

// messages/en/common.json
{
  "common_delete": "Delete",
  "common_cancel": "Cancel"
}
```

**Usage**:

```typescript
import * as m from '$lib/paraglide/messages';

const deleteConfirmMessage = m.components_deleteConfirmMessage({
  model: model.series || 'this model'
});
```

## Testing Considerations

### Test Data Fixtures

**Complete model**:

```typescript
const completeModel: RailwayModelCardData = {
  id: 'test-001',
  manufacturer: 'Märklin',
  productCode: '37586',
  series: 'Class 66',
  category: 'DieselLocomotive',
  roadNumber: '66 001',
  scale: 'H0',
  powerMethod: 'DCC',
  era: 'VI',
  purchaseDate: '2024-06-15',
  photoUrl: 'https://example.com/photo.jpg',
  unitCount: 1,
  digitalFeatures: ['Sound', 'DCC']
};
```

**Minimal model** (edge case):

```typescript
const minimalModel: RailwayModelCardData = {
  id: 'test-002',
  manufacturer: null,
  productCode: null,
  series: null,
  category: 'Unknown',
  roadNumber: null,
  scale: null,
  powerMethod: null,
  era: null,
  purchaseDate: null,
  photoUrl: null,
  unitCount: null,
  digitalFeatures: []
};
```

**Long road number** (truncation test):

```typescript
const longRoadNumberModel: RailwayModelCardData = {
  ...completeModel,
  roadNumber: '12 34 56 78 90 12 34 56 78 90 123' // 35 characters
};
```

## Summary

The component accepts a single `model` prop of type `RailwayModelCardData` with comprehensive railway model metadata. All fields except `id` and `category` are optional with graceful fallbacks. The component emits an `onDelete` event for parent handling. No internal state management is required - component is fully controlled by props.
