# Component Contract: AddModelDrawer

**Feature**: 002-add-model-collection  
**Date**: 2026-01-30  
**Location**: `src/lib/features/collection/components/AddModelDrawer.svelte`

## Overview

A slide-in drawer component for adding a new railway model to the user's collection. Replaces the existing simplified `ItemDrawer.svelte` with comprehensive form fields matching the spec requirements.

---

## Props Interface

```typescript
interface AddModelDrawerProps {
  /**
   * Controls drawer visibility.
   * When true, drawer slides in from right.
   */
  open: boolean;

  /**
   * Callback when drawer requests close.
   * Called on: overlay click, Escape key, Cancel button.
   */
  onClose: () => void;

  /**
   * Callback when model is successfully added.
   * Called after successful Tauri command invocation.
   */
  onSuccess: () => void;
}
```

## Usage Example

```svelte
<script lang="ts">
  import AddModelDrawer from './components/AddModelDrawer.svelte';

  let showDrawer = $state(false);

  function handleSuccess() {
    showDrawer = false;
    // Refresh collection list
  }
</script>

<button onclick={() => (showDrawer = true)}>Add railway model</button>

<AddModelDrawer open={showDrawer} onClose={() => (showDrawer = false)} onSuccess={handleSuccess} />
```

---

## Internal State

```typescript
// Form state (see data-model.md for full types)
let form = $state<AddModelFormState>(createDefaultFormState());

// Reference data (loaded on open)
let manufacturers = $state<Manufacturer[]>([]);
let railwayCompanies = $state<RailwayCompany[]>([]);
let sellers = $state<SellerView[]>([]);

// UI state
let isSubmitting = $state(false);
let isLoadingData = $state(false);
let validationErrors = $state<ValidationErrors>({});
let showPurchaseSection = $state(false);
```

---

## Behaviors

### Open Drawer

1. Reset form to default state
2. Fetch reference data (manufacturers, railway companies, sellers)
3. Focus first input field

### Add Rolling Stock

1. Generate unique `uid` for new entry
2. Append to `form.rollingStocks` array
3. Focus railway company field of new entry

### Remove Rolling Stock

1. Remove entry by `uid`
2. If last entry, show validation message (at least 1 required)

### Submit Form

1. Validate all required fields
2. If invalid, show errors and abort
3. Transform form state to `AddRailwayModelToCollectionArgs`
4. Call `commands.addRailwayModelToCollection(args)`
5. On success: call `onSuccess()`, show toast
6. On error: show toast with error message

### Close Drawer

1. If form has changes, show confirmation dialog
2. If confirmed or no changes, call `onClose()`
3. Reset form state

---

## Validation Display

| Field State                             | Visual Indicator                |
| --------------------------------------- | ------------------------------- |
| Valid                                   | Default border                  |
| Invalid (after blur)                    | Red border, error message below |
| Required & empty (after submit attempt) | Red border, error message below |

---

## Accessibility

- Dialog uses `role="dialog"` and `aria-modal="true"`
- Focus trapped within drawer when open
- Escape key closes drawer
- All form fields have associated labels
- Error messages linked to fields via `aria-describedby`
- Submit button disabled during submission

---

## Paraglide Message Keys

```json
{
  "add_model_title": "Add railway model",
  "add_model_subtitle": "Add a new model to your collection",

  "add_model_section_model": "Railway Model",
  "add_model_section_rolling_stock": "Rolling Stocks",
  "add_model_section_purchase": "Purchase Information",

  "add_model_manufacturer": "Manufacturer",
  "add_model_product_code": "Product Code",
  "add_model_description": "Description",
  "add_model_category": "Category",
  "add_model_scale": "Scale",
  "add_model_power_method": "Power Method",
  "add_model_epoch": "Epoch",

  "add_model_add_rolling_stock": "Add rolling stock",
  "add_model_remove_rolling_stock": "Remove",
  "add_model_railway_company": "Railway Company",
  "add_model_series_code": "Series Code",
  "add_model_road_number": "Road Number",
  "add_model_rs_category": "Category",

  "add_model_seller": "Seller",
  "add_model_price": "Purchase Price",
  "add_model_currency": "Currency",
  "add_model_purchase_condition": "Purchase Condition",
  "add_model_model_condition": "Model Condition",
  "add_model_box_condition": "Box Condition",
  "add_model_notes": "Notes",

  "add_model_cancel": "Cancel",
  "add_model_submit": "Add to Collection",
  "add_model_submitting": "Adding...",

  "add_model_success": "Railway model added to collection",
  "add_model_error": "Failed to add railway model",

  "add_model_validation_manufacturer": "Please select a manufacturer",
  "add_model_validation_product_code": "Product code is required",
  "add_model_validation_description": "Description is required",
  "add_model_validation_category": "Please select a category",
  "add_model_validation_scale": "Please select a scale",
  "add_model_validation_power": "Please select a power method",
  "add_model_validation_epoch": "Please select an epoch",
  "add_model_validation_rs_required": "At least one rolling stock is required",
  "add_model_validation_rs_company": "Please select a railway company",
  "add_model_validation_rs_series": "Series code is required",
  "add_model_validation_rs_category": "Please select a category",

  "add_model_discard_title": "Discard changes?",
  "add_model_discard_message": "You have unsaved changes. Are you sure you want to close?",
  "add_model_discard_confirm": "Discard",
  "add_model_discard_cancel": "Keep editing"
}
```

---

## Component Hierarchy

```
AddModelDrawer.svelte
│
├── Drawer Container (fixed positioning, overlay)
│   │
│   ├── Header
│   │   ├── Title (add_model_title)
│   │   └── Close button (X icon)
│   │
│   ├── Content (scrollable)
│   │   │
│   │   ├── Railway Model Section
│   │   │   ├── Manufacturer select
│   │   │   ├── Product code input
│   │   │   ├── Description input
│   │   │   ├── Category select
│   │   │   ├── Scale select
│   │   │   ├── Power method select
│   │   │   └── Epoch select
│   │   │
│   │   ├── Rolling Stocks Section
│   │   │   ├── Section header + Add button
│   │   │   └── RollingStockEntry[] (map)
│   │   │       └── RollingStockEntry.svelte
│   │   │           ├── Railway company select
│   │   │           ├── Series code input
│   │   │           ├── Category select
│   │   │           ├── Road number input
│   │   │           └── Remove button
│   │   │
│   │   └── Purchase Section (collapsible)
│   │       └── PurchaseSection.svelte
│   │           ├── Seller select
│   │           ├── Price input + Currency select
│   │           ├── Purchase condition select
│   │           ├── Model condition select
│   │           ├── Box condition select
│   │           └── Notes textarea
│   │
│   └── Footer
│       ├── Cancel button
│       └── Submit button
│
└── DiscardDialog (conditional)
    └── Confirmation modal for unsaved changes
```

---

## Dependencies

### Components

- `RollingStockEntry.svelte` - Sub-component for rolling stock row
- `PurchaseSection.svelte` - Sub-component for purchase info

### Tauri Commands

- `commands.getManufacturers()` - Load manufacturer list
- `commands.getRailwayCompanies()` - Load railway company list
- `commands.getSellers()` - Load seller list
- `commands.addRailwayModelToCollection(args)` - Submit form

### Utilities

- `toaster` from `$lib/toaster` - Toast notifications
- `safeInvoke` from `$lib/services` - Safe Tauri command wrapper

### Constants

- `scales` from `$lib/data/constants/scales.json`
- `epochs` from `$lib/data/constants/epochs.json`
- `categories` from `$lib/data/constants/categories.json`
- `powerMethods` from `$lib/data/constants/powerMethods.json`

---

## Sub-Component: RollingStockEntry

**Location**: `src/lib/features/collection/components/RollingStockEntry.svelte`

```typescript
interface RollingStockEntryProps {
  /** Entry data bound two-way */
  entry: RollingStockFormEntry;

  /** Available railway companies for dropdown */
  railwayCompanies: RailwayCompany[];

  /** Whether remove button is enabled (disabled if only entry) */
  canRemove: boolean;

  /** Callback to remove this entry */
  onRemove: () => void;

  /** Validation errors for this entry */
  errors?: RollingStockValidationError;
}
```

---

## Sub-Component: PurchaseSection

**Location**: `src/lib/features/collection/components/PurchaseSection.svelte`

```typescript
interface PurchaseSectionProps {
  /** Purchase state bound two-way */
  purchase: PurchaseFormState;

  /** Available sellers for dropdown */
  sellers: SellerView[];

  /** Whether section is expanded */
  expanded: boolean;

  /** Toggle expanded state */
  onToggle: () => void;
}
```
