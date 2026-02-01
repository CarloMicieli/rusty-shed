# Data Model: Add Railway Model to Wishlist

**Feature**: 003-add-model-wishlist  
**Date**: 2026-01-30  
**Scope**: Frontend form state and type mappings (backend types already defined)

## Overview

This feature uses existing backend types from `bindings.ts`. The frontend needs to define form state types that map to these backend types on submission.

## Backend Types (from bindings.ts — READ ONLY)

### AddRailwayModelToWishListArgs

```typescript
export type AddRailwayModelToWishListArgs = {
  railwayModel: SimplifiedRailwayModelArgs;
  wishlistId: string;
  priority: WishlistPriority | null;
  status: WishlistStatus | null;
  desiredPriceAmount: bigint | null;
  desiredPriceCurrency: string | null;
  notes: string | null;
  addedDate: string | null;
};
```

### SimplifiedRailwayModelArgs

```typescript
export type SimplifiedRailwayModelArgs = {
  manufacturerId: string;
  productCode: string;
  description: string;
  category: string;
  scale: string;
  epoch: string;
  powerMethod: string;
  rollingStocks: SimplifiedRollingStockArgs[];
};
```

### SimplifiedRollingStockArgs

```typescript
export type SimplifiedRollingStockArgs = {
  railwayCompanyId: string;
  seriesCode: string;
  roadNumber: string | null;
  locomotiveType: string | null;
  category: string;
};
```

### Reference Enums

```typescript
export type WishlistPriority = 'LOW' | 'NORMAL' | 'HIGH';
export type WishlistStatus = 'WANTED' | 'ON_ORDER' | 'PURCHASED' | 'IGNORED';
export type Category =
  | 'LOCOMOTIVES'
  | 'TRAIN_SETS'
  | 'STARTER_SETS'
  | 'FREIGHT_CARS'
  | 'PASSENGER_CARS'
  | 'ELECTRIC_MULTIPLE_UNITS'
  | 'RAILCARS';
export type Scale = 'H0' | 'H0m' | 'H0e' | 'N' | 'TT' | 'Z' | 'G' | 'Scale1' | 'Scale0' | 'Scale00';
export type PowerMethod = 'AC' | 'DC' | 'TRIX_EXPRESS';
```

## Frontend Form State Types (NEW)

### AddRailwayModelFormState

Main form state for the drawer component.

| Field                | Type                      | Required | Default             | Notes                                 |
| -------------------- | ------------------------- | -------- | ------------------- | ------------------------------------- |
| wishlistId           | `string`                  | Yes      | `''` or preselected | From wishlists dropdown               |
| manufacturerId       | `string`                  | Yes      | `''`                | From manufacturers dropdown           |
| productCode          | `string`                  | Yes      | `''`                | User input                            |
| description          | `string`                  | Yes      | `''`                | User input                            |
| category             | `Category \| ''`          | Yes      | `''`                | From static dropdown                  |
| scale                | `Scale \| ''`             | Yes      | `''`                | From static dropdown                  |
| powerMethod          | `PowerMethod \| ''`       | Yes      | `''`                | From static dropdown                  |
| epoch                | `string`                  | Yes      | `''`                | User input (e.g., "III", "IV", "V")   |
| desiredPriceAmount   | `string`                  | No       | `''`                | User input, converted to bigint cents |
| desiredPriceCurrency | `string`                  | No       | `'EUR'`             | Default currency                      |
| priority             | `WishlistPriority`        | No       | `'NORMAL'`          | From dropdown                         |
| notes                | `string`                  | No       | `''`                | User input                            |
| rollingStocks        | `RollingStockFormEntry[]` | No       | `[]`                | Dynamic array                         |

### RollingStockFormEntry

Form state for each rolling stock entry.

| Field            | Type                         | Required | Default | Notes                            |
| ---------------- | ---------------------------- | -------- | ------- | -------------------------------- |
| id               | `string`                     | Yes      | UUID    | Unique key for Svelte each block |
| railwayCompanyId | `string`                     | Yes      | `''`    | From railway companies dropdown  |
| seriesCode       | `string`                     | Yes      | `''`    | User input                       |
| category         | `RollingStockCategory \| ''` | Yes      | `''`    | From static dropdown             |
| roadNumber       | `string`                     | No       | `''`    | User input (optional)            |

### RollingStockCategory (subset of Category for rolling stocks)

```typescript
type RollingStockCategory =
  | 'LOCOMOTIVES'
  | 'FREIGHT_CARS'
  | 'PASSENGER_CARS'
  | 'ELECTRIC_MULTIPLE_UNITS'
  | 'RAILCARS';
```

## Dropdown Data Sources

### Dynamic (loaded from backend)

| Data              | Command                 | Returns            | Cached                   |
| ----------------- | ----------------------- | ------------------ | ------------------------ |
| Wishlists         | `getWishlists()`        | `WishlistView[]`   | Already in WishlistState |
| Manufacturers     | `getManufacturers()`    | `Manufacturer[]`   | Local component state    |
| Railway Companies | `getRailwayCompanies()` | `RailwayCompany[]` | Local component state    |

### Static (TypeScript arrays)

```typescript
export const CATEGORIES: Category[] = [
  'LOCOMOTIVES',
  'TRAIN_SETS',
  'STARTER_SETS',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];

export const SCALES: Scale[] = [
  'H0',
  'H0m',
  'H0e',
  'N',
  'TT',
  'Z',
  'G',
  'Scale1',
  'Scale0',
  'Scale00'
];

export const POWER_METHODS: PowerMethod[] = ['AC', 'DC', 'TRIX_EXPRESS'];

export const PRIORITIES: WishlistPriority[] = ['LOW', 'NORMAL', 'HIGH'];

export const ROLLING_STOCK_CATEGORIES: RollingStockCategory[] = [
  'LOCOMOTIVES',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];
```

## Form → Backend Mapping

### Transformation Function

```typescript
function formToArgs(form: AddRailwayModelFormState): AddRailwayModelToWishListArgs {
  return {
    wishlistId: form.wishlistId,
    railwayModel: {
      manufacturerId: form.manufacturerId,
      productCode: form.productCode,
      description: form.description,
      category: form.category,
      scale: form.scale,
      epoch: form.epoch,
      powerMethod: form.powerMethod,
      rollingStocks: form.rollingStocks.map((rs) => ({
        railwayCompanyId: rs.railwayCompanyId,
        seriesCode: rs.seriesCode,
        roadNumber: rs.roadNumber || null,
        locomotiveType: null, // Not collected in form
        category: rs.category
      }))
    },
    priority: form.priority,
    status: 'WANTED', // Always default per spec
    desiredPriceAmount: form.desiredPriceAmount
      ? BigInt(Math.round(parseFloat(form.desiredPriceAmount) * 100))
      : null,
    desiredPriceCurrency: form.desiredPriceAmount ? form.desiredPriceCurrency : null,
    notes: form.notes || null,
    addedDate: new Date().toISOString().split('T')[0] // YYYY-MM-DD
  };
}
```

## Validation Rules

### Required Fields (block submission if empty)

- `wishlistId`
- `manufacturerId`
- `productCode`
- `description`
- `category`
- `scale`
- `powerMethod`
- `epoch`

### Rolling Stock Validation (per entry)

- `railwayCompanyId` - required
- `seriesCode` - required
- `category` - required

### Computed Validation State

```typescript
const isValid = $derived.by(() => {
  const baseValid =
    form.wishlistId !== '' &&
    form.manufacturerId !== '' &&
    form.productCode.trim() !== '' &&
    form.description.trim() !== '' &&
    form.category !== '' &&
    form.scale !== '' &&
    form.powerMethod !== '' &&
    form.epoch.trim() !== '';

  const rollingStocksValid = form.rollingStocks.every(
    (rs) => rs.railwayCompanyId !== '' && rs.seriesCode.trim() !== '' && rs.category !== ''
  );

  return baseValid && rollingStocksValid;
});
```

## Entity Relationships

```
┌─────────────────────┐
│ AddRailwayModelForm │
├─────────────────────┤
│ wishlistId ─────────┼──► WishlistPreview (from WishlistState)
│ manufacturerId ─────┼──► Manufacturer (loaded on open)
│ category ───────────┼──► Category (static enum)
│ scale ──────────────┼──► Scale (static enum)
│ powerMethod ────────┼──► PowerMethod (static enum)
│ priority ───────────┼──► WishlistPriority (static enum)
│ rollingStocks[] ────┼──┐
└─────────────────────┘  │
                         ▼
               ┌─────────────────────┐
               │ RollingStockEntry   │
               ├─────────────────────┤
               │ railwayCompanyId ───┼──► RailwayCompany (loaded on open)
               │ category ───────────┼──► RollingStockCategory (static)
               └─────────────────────┘
```
