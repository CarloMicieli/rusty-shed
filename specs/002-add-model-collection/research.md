# Research: Add Railway Model to Collection

**Feature**: 002-add-model-collection  
**Date**: 2026-01-30

## Overview

This document captures research findings for implementing the enhanced "Add Railway Model to Collection" feature. All technical decisions are documented with rationale and alternatives considered.

---

## R1: Reference Data Sources

### Decision

Use a combination of static JSON constants for enumerated values and Tauri commands for dynamic entity data.

### Findings

| Data Type                    | Source                                               | Load Strategy                            |
| ---------------------------- | ---------------------------------------------------- | ---------------------------------------- |
| **Manufacturers**            | `commands.getManufacturers()`                        | Fetch on drawer open, cache in component |
| **Railway Companies**        | `commands.getRailwayCompanies()`                     | Fetch on drawer open, cache in component |
| **Sellers**                  | `commands.getSellers()`                              | Fetch on drawer open, cache in component |
| **Scales**                   | `src/lib/data/constants/scales.json`                 | Static import                            |
| **Epochs**                   | `src/lib/data/constants/epochs.json`                 | Static import                            |
| **Power Methods**            | `src/lib/data/constants/powerMethods.json`           | Static import                            |
| **Categories**               | `src/lib/data/constants/categories.json`             | Static import                            |
| **Rolling Stock Categories** | `src/lib/data/constants/rollingStockCategories.json` | Static import                            |

### Static Data Structures

**scales.json** - 10 entries:

```json
[
  { "id": "H0", "display": "H0 (1:87)" },
  { "id": "N", "display": "N (1:160)" },
  ...
]
```

**epochs.json** - 12 entries including transition epochs:

```json
[
  { "id": "I", "display": "I" },
  { "id": "I/II", "display": "I/II" },
  ...
]
```

**powerMethods.json** - 3 entries:

```json
[
  { "id": "AC", "display": "AC" },
  { "id": "DC", "display": "DC" },
  { "id": "TRIX_EXPRESS", "display": "TRIX Express" }
]
```

**categories.json** - 7 entries with Paraglide label keys:

```json
[
  { "id": "LOCOMOTIVES", "labelKey": "constants_categories_locomotives" },
  { "id": "TRAIN_SETS", "labelKey": "constants_categories_train_sets" },
  ...
]
```

### Rationale

- Static constants avoid IPC overhead for enumerated values that rarely change
- Dynamic entities (manufacturers, railway companies, sellers) are user-managed and must be fresh
- Categories use Paraglide label keys for i18n support

### Alternatives Considered

1. **Fetch all data via Tauri commands**: Rejected - unnecessary IPC for static enums
2. **Hardcode in components**: Rejected - violates DRY, harder to maintain
3. **Single reference data command**: Rejected - over-engineering, current approach works

---

## R2: Backend Command Analysis

### Decision

Use the existing `addRailwayModelToCollection` command without modifications.

### Findings

The command signature from `bindings.ts`:

```typescript
async addRailwayModelToCollection(
  args: AddRailwayModelToCollectionArgs
): Promise<Result<null, CommandError>>
```

**AddRailwayModelToCollectionArgs** structure:

```typescript
{
  railwayModel: SimplifiedRailwayModelArgs;
  priceAmount: bigint; // cents
  priceCurrency: string; // e.g., "EUR"
  sellerId: string | null;
  addedDate: string; // YYYY-MM-DD
  purchaseDate: string; // YYYY-MM-DD
  purchaseCondition: string | null;
  modelCondition: string | null;
  boxCondition: string | null;
  notes: string | null;
}
```

**SimplifiedRailwayModelArgs** structure:

```typescript
{
  manufacturerId: string;
  productCode: string;
  description: string;
  category: string;
  scale: string;
  epoch: string;
  powerMethod: string;
  rollingStocks: SimplifiedRollingStockArgs[];
}
```

**SimplifiedRollingStockArgs** structure:

```typescript
{
  railwayCompanyId: string;
  seriesCode: string;
  roadNumber: string | null;
  locomotiveType: string | null;
  category: string;
}
```

### Mapping to Spec Requirements

| Spec Requirement               | Args Field                         | Status |
| ------------------------------ | ---------------------------------- | ------ |
| Manufacturer dropdown          | `railwayModel.manufacturerId`      | ✅     |
| Product code                   | `railwayModel.productCode`         | ✅     |
| Short description              | `railwayModel.description`         | ✅     |
| Category dropdown              | `railwayModel.category`            | ✅     |
| Scale dropdown                 | `railwayModel.scale`               | ✅     |
| Power method dropdown          | `railwayModel.powerMethod`         | ✅     |
| Epoch                          | `railwayModel.epoch`               | ✅     |
| Rolling stock: railway company | `rollingStocks[].railwayCompanyId` | ✅     |
| Rolling stock: series code     | `rollingStocks[].seriesCode`       | ✅     |
| Rolling stock: category        | `rollingStocks[].category`         | ✅     |
| Rolling stock: road number     | `rollingStocks[].roadNumber`       | ✅     |
| Seller dropdown                | `sellerId`                         | ✅     |
| Purchase price                 | `priceAmount` + `priceCurrency`    | ✅     |
| Purchase condition             | `purchaseCondition`                | ✅     |
| Model condition                | `modelCondition`                   | ✅     |
| Box condition                  | `boxCondition`                     | ✅     |
| Notes                          | `notes`                            | ✅     |

### Rationale

All spec requirements are covered by existing command. No backend changes needed.

### Alternatives Considered

1. **New dedicated command**: Rejected - existing command is sufficient
2. **Separate create + add commands**: Rejected - existing flow handles both atomically

---

## R3: Condition Types

### Decision

Use string-based conditions matching backend expectations.

### Findings

From `bindings.ts`:

**PurchaseCondition**:

```typescript
export type PurchaseCondition = 'NEW' | 'PRE_OWNED';
```

**ModelCondition** - Need to verify exact type (search bindings):

- Found in `CollectionItemView.modelCondition: ModelCondition | null`
- Type definition needed

**BoxCondition** - Need to verify exact type:

- Found in `CollectionItemView.boxCondition: BoxCondition | null`
- Type definition needed

### Action Required

Create static constants for conditions or use the typed enums from bindings.

---

## R4: Form Validation Strategy

### Decision

Use client-side validation with Svelte 5 `$derived` for reactive feedback.

### Validation Rules

| Field                          | Rule                        | Message Key                             |
| ------------------------------ | --------------------------- | --------------------------------------- |
| Manufacturer                   | Required                    | `add_model_manufacturer_required`       |
| Product Code                   | Required, non-empty         | `add_model_product_code_required`       |
| Description                    | Required, non-empty         | `add_model_description_required`        |
| Category                       | Required                    | `add_model_category_required`           |
| Scale                          | Required                    | `add_model_scale_required`              |
| Power Method                   | Required                    | `add_model_power_method_required`       |
| Epoch                          | Required                    | `add_model_epoch_required`              |
| Rolling Stocks                 | At least 1 required         | `add_model_rolling_stock_required`      |
| Rolling Stock: Railway Company | Required per entry          | `add_model_rs_railway_company_required` |
| Rolling Stock: Series Code     | Required per entry          | `add_model_rs_series_code_required`     |
| Rolling Stock: Category        | Required per entry          | `add_model_rs_category_required`        |
| Purchase Price                 | Positive number if provided | `add_model_price_invalid`               |

### Rationale

- Real-time validation improves UX
- Backend still validates (domain logic in Rust)
- Frontend validation is for hints only

### Alternatives Considered

1. **Submit-time only validation**: Rejected - poor UX
2. **Form library (like Formsnap)**: Rejected - Svelte 5 runes sufficient

---

## R5: Drawer Component Pattern

### Decision

Create a new drawer component following the existing `ItemDrawer.svelte` pattern but with expanded fields.

### Findings

Current `ItemDrawer.svelte` characteristics:

- Uses fixed positioning with `fixed inset-0 z-50`
- Slides in from right with `justify-end`
- Background overlay with `bg-black/40`
- Max width `max-w-xl`
- Escape key and overlay click to close
- Form state managed internally with `$state`

### Component Structure

```svelte
AddModelDrawer.svelte ├── Header (title, close button) ├── Railway Model Section │ ├── Manufacturer
select │ ├── Product code input │ ├── Description input │ ├── Category select │ ├── Scale select │
├── Power method select │ └── Epoch select ├── Rolling Stocks Section │ ├── Add button │ └──
RollingStockEntry[] (dynamic list) ├── Purchase Info Section (collapsible) │ ├── Seller select │ ├──
Price input + currency │ ├── Purchase condition select │ ├── Model condition select │ ├── Box
condition select │ └── Notes textarea └── Footer (Cancel, Save buttons)
```

### Rationale

Extending existing pattern ensures UI consistency. New sub-components (RollingStockEntry, PurchaseSection) improve maintainability.

---

## R6: State Management Pattern

### Decision

Extend `CollectionState.svelte.ts` with a new method to handle the add operation.

### Findings

Current `CollectionState` class provides:

- `fetchCollection()` - loads collection data
- `createItem()` / `updateItem()` / `deleteItem()` - placeholder CRUD
- Filter state management

### Implementation Approach

Add new method to `CollectionState`:

```typescript
addRailwayModel = async (args: AddRailwayModelToCollectionArgs): Promise<boolean> => {
  const result = await safeInvoke('add_railway_model_to_collection', { args });
  if (result.ok) {
    await this.fetchCollection(); // Refresh list
    return true;
  }
  toastError(randomId(), getErrorMessage(result.error));
  return false;
};
```

### Rationale

Centralizes collection operations in the state class, maintains consistency with existing patterns.

---

## Summary

All research questions resolved. Key findings:

1. **No backend changes needed** - existing command covers all requirements
2. **Reference data available** - static JSON + Tauri commands
3. **Form validation** - Svelte 5 `$derived` for reactive hints
4. **Component pattern** - extend existing drawer with sub-components
5. **State management** - extend CollectionState with new method

Ready to proceed to Phase 1: Design.
