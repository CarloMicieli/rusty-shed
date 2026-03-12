# Contract: Frontend — AcquisitionDrawer

**Feature module**: `src/lib/features/acquisition/`
**Consumes**: `commands.recordAcquisition(args: RecordAcquisitionArgs)` from `$lib/bindings`

---

## Component Tree

```
AcquisitionDrawer.svelte              ← root; owns all state
├── [overlay div]                     ← backdrop, onclick → handleCloseRequest
├── [panel div]                       ← fixed right, translate-x animation
│   ├── AcquisitionHeader.svelte      ← sticky; seller, date, batch defaults
│   ├── [scrollable div]              ← flex-1 overflow-y-auto
│   │   └── AcquisitionItemCard.svelte  ← one per items[i]
│   └── AcquisitionFooter.svelte      ← sticky; "Add Another Item" + "Finalize"
└── [discard dialog]                  ← shown when closing with unsaved data
```

---

## AcquisitionDrawer.svelte

### Props

```typescript
interface Props {
  open: boolean;
  onClose: () => void;
  onSuccess: () => void;
}
```

### State

```typescript
let form = $state<AcquisitionFormState>(createDefaultFormState());
let sellers = $state<SellerView[]>([]);
let manufacturers = $state<Manufacturer[]>([]);
let isSubmitting = $state(false);
let isLoadingData = $state(false);
let touched = $state(false);
let showDiscardDialog = $state(false);
let validationErrors = $state<AcquisitionValidationErrors>({});
```

### Derived

```typescript
let hasChanges = $derived(
  form.sellerId !== null ||
    form.items.length > 1 ||
    form.items.some(
      (item) =>
        item.manufacturerId !== null ||
        item.productCode.trim() !== '' ||
        item.description.trim() !== ''
    )
);

let currency = $derived(settingsState.settings.currency ?? 'EUR');
```

### Key Handlers

```typescript
// On open: reset form + load reference data
async function handleOpen() {
  form = createDefaultFormState();
  touched = false;
  validationErrors = {};
  isLoadingData = true;
  const [mfgResult, sellerResult] = await Promise.all([
    commands.getManufacturers(),
    commands.getSellers()
  ]);
  manufacturers = mfgResult.status === 'ok' ? mfgResult.data : [];
  sellers = sellerResult.status === 'ok' ? sellerResult.data : [];
  isLoadingData = false;
}

// Add item (pre-fills batch defaults)
function handleAddItem() {
  form.items = [...form.items, createDefaultItem(form.batchDefaults)];
  // scroll last card into view after DOM update
}

// Duplicate item (copy all fields, clear productCode)
function handleDuplicate(uid: string) {
  const source = form.items.find((i) => i.uid === uid)!;
  const clone: AcquisitionItemEntry = {
    ...source,
    uid: crypto.randomUUID(),
    productCode: ''
  };
  const idx = form.items.findIndex((i) => i.uid === uid);
  form.items = [...form.items.slice(0, idx + 1), clone, ...form.items.slice(idx + 1)];
}

// Remove item (guard: keep at least 1)
function handleRemove(uid: string) {
  if (form.items.length <= 1) return;
  form.items = form.items.filter((i) => i.uid !== uid);
}

// Batch default change propagates to items that still match the old default
function handleBatchDefaultChange(field: 'scale' | 'powerMethod', value: string | null) {
  const old = form.batchDefaults[field];
  form.batchDefaults = { ...form.batchDefaults, [field]: value };
  form.items = form.items.map((item) => (item[field] === old ? { ...item, [field]: value } : item));
}

// Close with unsaved data guard
function handleCloseRequest() {
  if (hasChanges) {
    showDiscardDialog = true;
  } else {
    onClose();
  }
}

// Submit
async function handleFinalize() {
  touched = true;
  const errors = validateForm(form);
  validationErrors = errors;
  if (hasErrors(errors)) return;

  isSubmitting = true;
  const args = toRecordAcquisitionArgs(form, currency);
  const result = await commands.recordAcquisition(args);
  isSubmitting = false;

  if (result.status === 'ok') {
    onSuccess();
  } else {
    // show toast error; drawer stays open
    toaster.error(m.acquisition_error_finalize());
  }
}
```

### `toRecordAcquisitionArgs` conversion

```typescript
function toRecordAcquisitionArgs(
  form: AcquisitionFormState,
  currency: string
): RecordAcquisitionArgs {
  return {
    sellerId: form.sellerId,
    purchaseDate: form.purchaseDate,
    items: form.items.map((item) => ({
      manufacturerId: item.manufacturerId!,
      productCode: item.productCode,
      description: item.description,
      category: item.category!,
      scale: item.scale ?? '',
      epoch: item.epoch ?? '',
      powerMethod: item.powerMethod ?? '',
      priceAmount:
        item.priceAmount != null
          ? (Math.round(item.priceAmount * 100) as unknown as bigint)
          : BigInt(0),
      priceCurrency: currency
    }))
  };
}
```

---

## AcquisitionHeader.svelte

### Props

```typescript
interface Props {
  sellerId: string | null;
  onSellerChange: (id: string | null) => void;
  purchaseDate: string;
  onDateChange: (date: string) => void;
  batchDefaults: BatchDefaults;
  onBatchDefaultChange: (field: 'scale' | 'powerMethod', value: string | null) => void;
  sellers: SellerView[];
}
```

**Fields rendered**:

- Seller: `<select>` dropdown from `sellers` (matches existing PurchaseSection pattern)
- Date: `DatePickerField` component (max = today)
- Scale: dropdown (Scale enum values)
- Power Method: dropdown (PowerMethod enum values)

---

## AcquisitionItemCard.svelte

### Props

```typescript
interface Props {
  item: AcquisitionItemEntry;
  index: number;
  manufacturers: Manufacturer[];
  currency: string;
  errors: AcquisitionItemErrors;
  canRemove: boolean;
  onUpdate: (uid: string, patch: Partial<AcquisitionItemEntry>) => void;
  onDuplicate: (uid: string) => void;
  onRemove: (uid: string) => void;
}
```

**Fields rendered** (in order):

1. Manufacturer — `<select>` from `manufacturers`
2. Product Code — `<input type="text">`
3. Description — `<input type="text">`
4. Category — `<select>` from Category enum
5. Scale — `<select>` (can override batch default)
6. Epoch — `<select>` or text input
7. Power Method — `<select>` (can override batch default)
8. Price — `CurrencyInput` showing `currency` symbol

**Actions** (card top-right icon row):

- Copy icon → `onDuplicate(item.uid)`
- Trash icon → `onRemove(item.uid)` (hidden if `!canRemove`)

---

## AcquisitionFooter.svelte

### Props

```typescript
interface Props {
  isSubmitting: boolean;
  isLoadingData: boolean;
  onAddItem: () => void;
  onFinalize: () => void;
}
```

**Buttons**:

- "Add Another Item" (variant="outline") → `onAddItem()`
- "Finalize Purchase" (variant="default") → `onFinalize()`, disabled while `isSubmitting || isLoadingData`

---

## Validation Error Types

```typescript
interface AcquisitionItemErrors {
  manufacturerId?: string;
  productCode?: string;
  category?: string;
}

interface AcquisitionValidationErrors {
  general?: string; // "add at least one item"
  items?: AcquisitionItemErrors[];
}
```

---

## Dashboard Integration

File: `src/routes/dashboard/+page.svelte`

```svelte
<!-- Add state -->
let showAcquisitionDrawer = $state(false);

<!-- Replace "Add Railway Model" action -->
<!-- Before: goto(resolve('/catalogue/new-model')) -->
<!-- After: showAcquisitionDrawer = true -->

<!-- Add drawer at bottom of page -->
<AcquisitionDrawer
  open={showAcquisitionDrawer}
  onClose={() => (showAcquisitionDrawer = false)}
  onSuccess={() => {
    showAcquisitionDrawer = false;
    dashboard.refresh(); // reload Recent Acquisitions
  }}
/>
```

---

## Global Shortcut (frontend listener)

File: `src/routes/+layout.svelte` (or `dashboard/+page.svelte` if scoped)

```typescript
import { listen } from '@tauri-apps/api/event';
import { onMount } from 'svelte';

onMount(() => {
  const unlisten = listen('open-acquisition-drawer', () => {
    showAcquisitionDrawer = true;
  });
  return () => unlisten.then((fn) => fn());
});
```

**Note**: If placed in `+layout.svelte`, `showAcquisitionDrawer` must be exposed via context so any
page can open the drawer. Preferred scope: layout-level so Ctrl+N works from any route.

---

## Paraglide Keys Required

New keys to add to `messages/en.json` (and all other locales):

| Key                                   | Suggested value                            |
| ------------------------------------- | ------------------------------------------ |
| `acquisition_drawer_title`            | `"New Acquisition"`                        |
| `acquisition_drawer_subtitle`         | `"Record a purchase"`                      |
| `acquisition_seller_label`            | `"Seller"`                                 |
| `acquisition_date_label`              | `"Purchase Date"`                          |
| `acquisition_batch_scale_label`       | `"Default Scale"`                          |
| `acquisition_batch_power_label`       | `"Default Power Method"`                   |
| `acquisition_item_manufacturer_label` | `"Manufacturer"`                           |
| `acquisition_item_product_code_label` | `"Product Code"`                           |
| `acquisition_item_description_label`  | `"Description"`                            |
| `acquisition_item_category_label`     | `"Category"`                               |
| `acquisition_item_epoch_label`        | `"Epoch"`                                  |
| `acquisition_item_scale_label`        | `"Scale"`                                  |
| `acquisition_item_power_label`        | `"Power Method"`                           |
| `acquisition_item_price_label`        | `"Price"`                                  |
| `acquisition_add_item_button`         | `"Add Another Item"`                       |
| `acquisition_finalize_button`         | `"Finalize Purchase"`                      |
| `acquisition_finalizing_button`       | `"Saving…"`                                |
| `acquisition_cancel_button`           | `"Cancel"`                                 |
| `acquisition_discard_title`           | `"Discard acquisition?"`                   |
| `acquisition_discard_description`     | `"You have unsaved items. Discard them?"`  |
| `acquisition_discard_confirm`         | `"Discard"`                                |
| `acquisition_discard_cancel`          | `"Keep editing"`                           |
| `acquisition_error_finalize`          | `"Failed to save acquisition. Try again."` |
| `acquisition_validation_empty_items`  | `"Add at least one item before saving."`   |
| `acquisition_validation_manufacturer` | `"Manufacturer is required"`               |
| `acquisition_validation_product_code` | `"Product code is required"`               |
| `acquisition_validation_category`     | `"Category is required"`                   |
| `acquisition_toast_success`           | `"Acquisition saved successfully"`         |
| `dashboard_action_new_acquisition`    | `"New Acquisition"`                        |
