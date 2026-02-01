# Quickstart: Add Railway Model to Collection

**Feature**: 002-add-model-collection  
**Date**: 2026-01-30

## Implementation Order

This guide provides the step-by-step implementation order for the feature. Follow these steps in sequence.

---

## Phase 1: Foundation (Messages & State)

### Step 1.1: Add Paraglide Message Keys

**File**: `messages/en.json`

Add all message keys from the component contract. See [contracts/AddModelDrawer.contract.md](contracts/AddModelDrawer.contract.md) for the full list.

**Verification**: `pnpm paraglide-js compile`

### Step 1.2: Extend CollectionState

**File**: `src/lib/features/collection/CollectionState.svelte.ts`

Add method to handle adding railway model to collection:

```typescript
addRailwayModel = async (args: AddRailwayModelToCollectionArgs): Promise<boolean> => {
  const result = await safeInvoke('add_railway_model_to_collection', { args });
  if (result.ok) {
    toaster.success({
      id: randomId(),
      title: m.add_model_success(),
      duration: 3000
    });
    await this.fetchCollection();
    return true;
  }
  toastError(randomId(), getErrorMessage(result.error));
  return false;
};
```

**Verification**: TypeScript compiles (`pnpm check`)

---

## Phase 2: Sub-Components

### Step 2.1: Create RollingStockEntry Component

**File**: `src/lib/features/collection/components/RollingStockEntry.svelte`

Create the reusable rolling stock entry row component:

```svelte
<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { X } from 'lucide-svelte';
  import type { RailwayCompany } from '$lib/bindings';
  import rollingStockCategories from '$lib/data/constants/rollingStockCategories.json';

  // Props interface - see contract
</script>

<!-- Rolling stock row: railway company, series, category, road number, remove -->
```

**Verification**: Component renders without errors in isolation

### Step 2.2: Create PurchaseSection Component

**File**: `src/lib/features/collection/components/PurchaseSection.svelte`

Create the collapsible purchase information section:

```svelte
<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { ChevronDown, ChevronRight } from 'lucide-svelte';
  import type { SellerView } from '$lib/bindings';

  // Props interface - see contract
</script>

<!-- Collapsible section: seller, price, conditions, notes -->
```

**Verification**: Component renders and expands/collapses correctly

---

## Phase 3: Main Drawer Component

### Step 3.1: Create AddModelDrawer Component

**File**: `src/lib/features/collection/components/AddModelDrawer.svelte`

Implement the main drawer following the contract:

1. Set up form state with `$state` rune
2. Load reference data on open
3. Implement validation with `$derived`
4. Add form sections (railway model, rolling stocks, purchase)
5. Implement submit handler
6. Add close confirmation for unsaved changes

**Verification**:

- `pnpm check` passes
- Drawer opens/closes correctly
- Form validation works

### Step 3.2: Wire Up Form Submission

Implement the submit handler:

```typescript
async function handleSubmit() {
  if (!isFormValid) {
    markAllTouched();
    return;
  }

  isSubmitting = true;
  try {
    const args = toAddRailwayModelArgs(form);
    const success = await collectionService.addRailwayModel(args);
    if (success) {
      onSuccess();
    }
  } finally {
    isSubmitting = false;
  }
}
```

**Verification**: Form submission creates model in collection

---

## Phase 4: Integration

### Step 4.1: Update CollectionDashboard

**File**: `src/lib/features/collection/CollectionDashboard.svelte`

Replace `ItemDrawer` with `AddModelDrawer`:

1. Import new component
2. Update drawer trigger to use new component
3. Remove old `ItemDrawer` import (or keep for editing if needed)

```diff
- import ItemDrawer from './components/ItemDrawer.svelte';
+ import AddModelDrawer from './components/AddModelDrawer.svelte';

- <ItemDrawer {open} {editing} {availableScales} {onClose} {onSubmit} />
+ <AddModelDrawer
+   open={ui.showDrawer}
+   onClose={ui.closeDrawer}
+   onSuccess={handleAddSuccess}
+ />
```

**Verification**: "Add railway model" button opens new drawer

### Step 4.2: Update Add Button Text

Update the button label to match spec:

```svelte
<button class="variant-filled-primary btn" onclick={ui.startCreate}>
  <Plus size={16} />
  <span>{m.collection_add_model()}</span>
</button>
```

**Verification**: Button shows correct label

---

## Phase 5: Testing

### Step 5.1: Add Unit Tests

**File**: `src/__tests__/lib/features/collection/AddModelForm.test.ts`

Test cases:

1. Form state initializes with defaults
2. Validation detects missing required fields
3. Adding/removing rolling stocks updates state
4. `toAddRailwayModelArgs` transforms correctly
5. Price parsing handles edge cases

### Step 5.2: Manual Testing Checklist

- [ ] Drawer opens from "Add railway model" button
- [ ] All dropdowns populate with data
- [ ] Can add multiple rolling stocks
- [ ] Can remove rolling stocks (but not last one)
- [ ] Validation shows errors for empty required fields
- [ ] Form submits successfully with valid data
- [ ] New model appears in collection after submit
- [ ] Close confirmation appears when form has changes
- [ ] Escape key and overlay click close drawer

---

## Phase 6: Cleanup & Polish

### Step 6.1: Add Italian Translations

**File**: `messages/it.json`

Add Italian translations for all new message keys.

### Step 6.2: Accessibility Review

- [ ] All form fields have labels
- [ ] Error messages are announced
- [ ] Focus management is correct
- [ ] Keyboard navigation works

### Step 6.3: Final Verification

```bash
pnpm format
pnpm lint
pnpm check
pnpm test
```

---

## File Checklist

| File                                                              | Action   | Status |
| ----------------------------------------------------------------- | -------- | ------ |
| `messages/en.json`                                                | ADD keys | ⬜     |
| `messages/it.json`                                                | ADD keys | ⬜     |
| `src/lib/features/collection/CollectionState.svelte.ts`           | MODIFY   | ⬜     |
| `src/lib/features/collection/components/RollingStockEntry.svelte` | CREATE   | ⬜     |
| `src/lib/features/collection/components/PurchaseSection.svelte`   | CREATE   | ⬜     |
| `src/lib/features/collection/components/AddModelDrawer.svelte`    | CREATE   | ⬜     |
| `src/lib/features/collection/CollectionDashboard.svelte`          | MODIFY   | ⬜     |
| `src/__tests__/lib/features/collection/AddModelForm.test.ts`      | CREATE   | ⬜     |

---

## Commands Reference

```bash
# Development
pnpm dev                    # Start dev server
pnpm tauri dev              # Start Tauri dev

# Validation
pnpm format                 # Format code
pnpm lint                   # Lint code
pnpm check                  # TypeScript check
pnpm test                   # Run tests

# Paraglide
pnpm paraglide-js compile   # Compile messages
```
