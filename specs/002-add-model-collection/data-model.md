# Data Model: Add Railway Model to Collection

**Feature**: 002-add-model-collection  
**Date**: 2026-01-30

## Overview

This document defines the form state types and data transformations for the "Add Railway Model to Collection" feature.

---

## Form State Types

### AddModelFormState

The main form state managing all inputs for adding a railway model to the collection.

```typescript
/**
 * Form state for adding a railway model to collection.
 * Managed via Svelte 5 $state rune in the drawer component.
 */
interface AddModelFormState {
  // Railway Model fields
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: string | null;
  scale: string | null;
  powerMethod: string | null;
  epoch: string | null;

  // Rolling stocks (dynamic list)
  rollingStocks: RollingStockFormEntry[];

  // Purchase info (optional section)
  purchase: PurchaseFormState;
}
```

### RollingStockFormEntry

Individual rolling stock entry within the railway model.

```typescript
/**
 * Form state for a single rolling stock entry.
 * Supports add/remove operations in the UI.
 */
interface RollingStockFormEntry {
  /** Client-side unique ID for list keying */
  uid: string;

  /** Railway company operating this rolling stock */
  railwayCompanyId: string | null;

  /** Series/class code (e.g., "Re 4/4", "TEE") */
  seriesCode: string;

  /** Rolling stock category (locomotive, passenger_car, etc.) */
  category: string | null;

  /** Road/running number (optional) */
  roadNumber: string;

  /**
   * Locomotive type if category is 'locomotive' (e.g., STEAM, DIESEL, ELECTRIC).
   * Required when category is locomotive; auto-hidden for other categories.
   * Maps to SimplifiedRollingStockArgs.locomotiveType in backend.
   */
  locomotiveType: string | null;
}
```

### PurchaseFormState

Optional purchase information section.

```typescript
/**
 * Form state for optional purchase information.
 * All fields are optional except when the user wants to record a purchase.
 */
interface PurchaseFormState {
  /** Seller from whom the model was purchased */
  sellerId: string | null;

  /** Purchase price amount (user input as decimal string) */
  priceAmount: string;

  /** Currency code (default: user preference or "EUR") */
  priceCurrency: string;

  /** Purchase condition (NEW, PRE_OWNED) */
  purchaseCondition: PurchaseCondition | null;

  /** Physical/mechanical condition of the model */
  modelCondition: ModelCondition | null;

  /** Condition of the original box */
  boxCondition: BoxCondition | null;

  /** Free-form notes */
  notes: string;

  /** Date of purchase (YYYY-MM-DD) */
  purchaseDate: string;
}
```

---

## Condition Type Enums (from bindings.ts)

```typescript
type PurchaseCondition = 'NEW' | 'PRE_OWNED';

type ModelCondition =
  | 'MINT' // Brand new, factory fresh
  | 'NEAR_MINT' // Almost like new
  | 'EXCELLENT' // Clean, very light use
  | 'VERY_GOOD' // Minor wear from use
  | 'GOOD' // Visible wear, small scratches
  | 'FAIR' // Significant wear, missing small parts
  | 'POOR' // Heavy damage, non-functional
  | 'FOR_PARTS'; // Only useful for salvaging

type BoxCondition =
  | 'ORIGINAL_MINT' // Crisp, no tears, no shelf wear
  | 'ORIGINAL_GOOD' // Some corner scuffing or minor creases
  | 'ORIGINAL_WORN' // Significant tears, tape repairs
  | 'REPLACEMENT_BOX' // Not original, but suitable storage
  | 'NO_BOX'; // Loose model, no packaging
```

---

## Reference Data Types

### Manufacturer (from backend)

```typescript
// From bindings.ts
interface Manufacturer {
  id: ManufacturerId; // "trn:manufacturer:{slug}"
  name: string;
  country: string | null;
}
```

### RailwayCompany (from backend)

```typescript
// From bindings.ts
interface RailwayCompany {
  id: RailwayCompanyId; // "trn:railway-company:{slug}"
  name: string;
  country: string | null;
}
```

### SellerView (from backend)

```typescript
// From bindings.ts
interface SellerView {
  id: SellerId; // "trn:seller:{slug}"
  name: string;
  sellerType: SellerType;
}
```

### Static Constants

```typescript
// From src/lib/data/constants/*.json

type ScaleOption = { id: string; display: string };
// Examples: { id: "H0", display: "H0 (1:87)" }

type EpochOption = { id: string; display: string };
// Examples: { id: "III", display: "III" }, { id: "III/IV", display: "III/IV" }

type PowerMethodOption = { id: string; display: string };
// Examples: { id: "DC", display: "DC" }, { id: "AC", display: "AC" }

type CategoryOption = { id: string; labelKey: string };
// Examples: { id: "LOCOMOTIVES", labelKey: "constants_categories_locomotives" }
```

---

## Validation Rules

### Required Field Validation

```typescript
interface ValidationErrors {
  manufacturerId?: string;
  productCode?: string;
  description?: string;
  category?: string;
  scale?: string;
  powerMethod?: string;
  epoch?: string;
  rollingStocks?: string; // "At least one rolling stock required"
  rollingStockErrors?: RollingStockValidationError[];
  priceAmount?: string;
}

interface RollingStockValidationError {
  uid: string;
  railwayCompanyId?: string;
  seriesCode?: string;
  category?: string;
}
```

### Validation Logic (Svelte 5 $derived)

```typescript
const isModelValid = $derived(
  form.manufacturerId !== null &&
    form.productCode.trim().length > 0 &&
    form.description.trim().length > 0 &&
    form.category !== null &&
    form.scale !== null &&
    form.powerMethod !== null &&
    form.epoch !== null
);

const areRollingStocksValid = $derived(
  form.rollingStocks.length > 0 &&
    form.rollingStocks.every(
      (rs) =>
        rs.railwayCompanyId !== null && rs.seriesCode.trim().length > 0 && rs.category !== null
    )
);

const isFormValid = $derived(isModelValid && areRollingStocksValid);
```

---

## Data Transformation

### Form State → Command Args

Transform function to convert form state to backend command arguments.

```typescript
function toAddRailwayModelArgs(form: AddModelFormState): AddRailwayModelToCollectionArgs {
  const today = new Date().toISOString().split('T')[0]; // YYYY-MM-DD

  return {
    railwayModel: {
      manufacturerId: form.manufacturerId!,
      productCode: form.productCode.trim(),
      description: form.description.trim(),
      category: form.category!,
      scale: form.scale!,
      epoch: form.epoch!,
      powerMethod: form.powerMethod!,
      rollingStocks: form.rollingStocks.map((rs) => ({
        railwayCompanyId: rs.railwayCompanyId!,
        seriesCode: rs.seriesCode.trim(),
        roadNumber: rs.roadNumber.trim() || null,
        locomotiveType: rs.locomotiveType,
        category: rs.category!
      }))
    },
    priceAmount: parsePriceToCents(form.purchase.priceAmount),
    priceCurrency: form.purchase.priceCurrency || 'EUR',
    sellerId: form.purchase.sellerId,
    addedDate: today,
    purchaseDate: form.purchase.purchaseDate || today,
    purchaseCondition: form.purchase.purchaseCondition,
    modelCondition: form.purchase.modelCondition,
    boxCondition: form.purchase.boxCondition,
    notes: form.purchase.notes.trim() || null
  };
}

/**
 * Convert decimal string (e.g., "123.45") to cents (bigint).
 */
function parsePriceToCents(priceString: string): bigint {
  if (!priceString.trim()) return BigInt(0);
  const parsed = parseFloat(priceString);
  if (isNaN(parsed) || parsed < 0) return BigInt(0);
  return BigInt(Math.round(parsed * 100));
}
```

---

## Default Values

### Initial Form State

```typescript
function createDefaultFormState(): AddModelFormState {
  return {
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    scale: 'H0', // Most common scale
    powerMethod: 'DC', // Most common power method
    epoch: null,
    rollingStocks: [createDefaultRollingStock()], // Start with one
    purchase: createDefaultPurchaseState()
  };
}

function createDefaultRollingStock(): RollingStockFormEntry {
  return {
    uid: crypto.randomUUID(),
    railwayCompanyId: null,
    seriesCode: '',
    category: null,
    roadNumber: '',
    locomotiveType: null
  };
}

function createDefaultPurchaseState(): PurchaseFormState {
  return {
    sellerId: null,
    priceAmount: '',
    priceCurrency: 'EUR',
    purchaseCondition: null,
    modelCondition: null,
    boxCondition: null,
    notes: '',
    purchaseDate: new Date().toISOString().split('T')[0]
  };
}
```

---

## State Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                        AddModelDrawer                            │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Railway Model Section                                      │  │
│  │  [Manufacturer ▼] [Product Code] [Description]            │  │
│  │  [Category ▼] [Scale ▼] [Power ▼] [Epoch ▼]              │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Rolling Stocks Section                         [+ Add]    │  │
│  │  ┌────────────────────────────────────────────────────┐  │  │
│  │  │ RollingStockEntry #1                          [×]  │  │  │
│  │  │  [Railway Company ▼] [Series Code] [Category ▼]    │  │  │
│  │  │  [Road Number (optional)]                          │  │  │
│  │  └────────────────────────────────────────────────────┘  │  │
│  │  ┌────────────────────────────────────────────────────┐  │  │
│  │  │ RollingStockEntry #2                          [×]  │  │  │
│  │  │  ...                                               │  │  │
│  │  └────────────────────────────────────────────────────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │ Purchase Information (collapsible)               [▼]      │  │
│  │  [Seller ▼] [Price] [Currency ▼]                         │  │
│  │  [Purchase Condition ▼] [Model Condition ▼]              │  │
│  │  [Box Condition ▼]                                        │  │
│  │  [Notes                                              ]    │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    [Cancel]  [Add to Collection]          │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Entity Relationships

```
AddRailwayModelToCollectionArgs
├── railwayModel: SimplifiedRailwayModelArgs
│   ├── manufacturerId ──────► Manufacturer (via getManufacturers)
│   ├── productCode (string)
│   ├── description (string)
│   ├── category ────────────► Category constant
│   ├── scale ───────────────► Scale constant
│   ├── epoch ───────────────► Epoch constant
│   ├── powerMethod ─────────► PowerMethod constant
│   └── rollingStocks[]
│       ├── railwayCompanyId ► RailwayCompany (via getRailwayCompanies)
│       ├── seriesCode (string)
│       ├── roadNumber (string | null)
│       ├── locomotiveType (string | null)
│       └── category ────────► RollingStockCategory constant
│
├── sellerId ────────────────► Seller (via getSellers)
├── priceAmount (bigint cents)
├── priceCurrency (string)
├── purchaseCondition ───────► PurchaseCondition enum
├── modelCondition ──────────► ModelCondition enum
├── boxCondition ────────────► BoxCondition enum
├── notes (string | null)
├── addedDate (string YYYY-MM-DD)
└── purchaseDate (string YYYY-MM-DD)
```
