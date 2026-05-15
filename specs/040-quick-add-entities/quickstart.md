# Quickstart: On-the-Fly Entity Quick-Add (040)

**Date**: 2026-05-15  
**Audience**: Developer implementing this feature  
**Prerequisite**: Feature **041-entity-management** must be merged first (provides `create_manufacturer` Tauri command and LOWER() unique indexes on entity name columns).

---

## Prerequisites Check

Before starting:

```bash
# Ensure 041 is merged and bindings are up to date
pnpm specta:generate          # regenerates src/lib/bindings.ts from Rust types
pnpm svelte-check             # no TypeScript errors
pnpm test                     # all tests pass
```

Verify that `commands.createManufacturer` is available in `src/lib/bindings.ts`.

---

## Implementation Order

### Step 1 — Zod Schema

Create `src/lib/schemas/quick-add-form.ts` using the schema defined in [contracts/ipc-commands.md](contracts/ipc-commands.md).

No existing file to modify. No test needed for the schema itself — it is tested indirectly by component tests.

---

### Step 2 — i18n Keys

Add all keys from [contracts/ipc-commands.md → New i18n Keys Required](contracts/ipc-commands.md) to both:
- `messages/en.json`
- `messages/it.json`

Run `pnpm run paraglide:compile` (or the equivalent `prepare` script) to regenerate the compiled message module.

---

### Step 3 — `QuickAddShell.svelte` component

**File**: `src/lib/components/drawer/QuickAddShell.svelte`

This is the visual wrapper for the second drawer. It is **not** a sub-class of `DrawerShell`; it is a self-contained component with:

- Fixed right panel, width `max-w-md`, sitting at `z-[110]`
- Semi-transparent backdrop div at `z-[105]`
- Slide-in animation matching `DrawerShell`
- Props:
  ```typescript
  let {
    open = false,
    title,         // Paraglide message string
    onDismiss,     // () => void
    children,      // Snippet
    footer,        // Snippet
  }: { ... } = $props();
  ```
- When `open` changes to `false`, the dismiss animation plays before the component unmounts (use `$effect` + CSS transition).

**Key markup structure**:
```svelte
<!-- Backdrop / scrim -->
{#if open}
  <div
    class="fixed inset-0 z-[105] bg-black/40"
    onclick={onDismiss}
    role="presentation"
    aria-hidden="true"
  />

  <!-- Panel -->
  <div
    class="fixed inset-y-0 right-0 z-[110] flex w-full max-w-md flex-col bg-background shadow-2xl"
    role="dialog"
    aria-modal="true"
    aria-labelledby="quick-add-title"
  >
    <header>
      <h2 id="quick-add-title">{title}</h2>
      <button onclick={onDismiss} aria-label="Close">…</button>
    </header>

    <div class="flex-1 overflow-y-auto p-4">
      {@render children()}
    </div>

    <footer class="border-t p-4">
      {@render footer()}
    </footer>
  </div>
{/if}
```

---

### Step 4 — `QuickAddEntityForm.svelte` component

**File**: `src/lib/features/quick-add/QuickAddEntityForm.svelte`

A single component that handles all three entity targets (manufacturer, seller, buyer) via a `target` prop.

```typescript
type QuickAddTarget = 'manufacturer' | 'seller' | 'buyer';

let {
  target,                    // QuickAddTarget
  existingNames,             // string[] — for client-side duplicate check
  onSuccess,                 // (entity: Manufacturer | SellerView) => void
  onCancel,                  // () => void
}: { ... } = $props();
```

**Internal state**:
```typescript
let formValues = $state<QuickAddFormValues>({ name: '', websiteUrl: '', countryCode: '' });
let isSaving = $state(false);
let saveError = $state<string | null>(null);

let isDuplicate = $derived(
  existingNames.some(n => n.toLowerCase() === formValues.name.trim().toLowerCase())
);
let canSave = $derived(!isDuplicate && formValues.name.trim().length > 0 && !isSaving);
```

**Save handler**:
```typescript
async function handleSave() {
  isSaving = true;
  saveError = null;

  const result = target === 'manufacturer'
    ? await commands.createManufacturer({ name: formValues.name.trim(), ... })
    : await commands.createSeller({ name: formValues.name.trim(), sellerType: 'SHOP', ... });

  if (result.status === 'ok') {
    onSuccess(result.data);
  } else {
    saveError = m.quick_add_save_failed();
    isSaving = false;
  }
}
```

---

### Step 5 — Add `dimmed` prop to `DrawerShell.svelte`

**File**: `src/lib/components/drawer/DrawerShell.svelte`

Add an optional boolean prop `dimmed` (default `false`). When `true`, apply `opacity-70 pointer-events-none` to the **scrollable content region** only (not the full drawer, so the shell itself remains in the DOM tree and receives no pointer events).

```svelte
let { ..., dimmed = false }: { ..., dimmed?: boolean } = $props();
```

```svelte
<!-- In the scrollable body: -->
<div class={cn('flex-1 overflow-y-auto', dimmed && 'opacity-70 pointer-events-none')}>
  {@render children()}
</div>
```

---

### Step 6 — Wire up AcquisitionItemCard (Manufacturer)

**File**: `src/lib/features/acquisition/components/AcquisitionItemCard.svelte`

1. Add `manufacturers` prop (already passed from parent via `AcquisitionDrawer`).
2. Add `+` icon button immediately inside the `Select.Content` header or as a trailing adornment on the `Select.Trigger`.
3. Lift `quickAddOpen` state to `AcquisitionDrawer.svelte` so the shell can be mounted outside the card.

**State in `AcquisitionDrawer.svelte`**:
```typescript
let quickAddTarget = $state<'manufacturer' | 'seller' | 'buyer' | null>(null);
let quickAddItemUid = $state<string | null>(null); // which item's dropdown to auto-select into

function handleQuickAddSuccess(entity: Manufacturer | SellerView) {
  if (quickAddTarget === 'manufacturer' && quickAddItemUid) {
    manufacturers = [...manufacturers, entity as Manufacturer];
    updateItem(quickAddItemUid, { manufacturerId: (entity as Manufacturer).id });
    toaster.success(m.quick_add_manufacturer_success({ name: entity.name }));
  } else if (quickAddTarget === 'seller') {
    sellers = [...sellers, entity as SellerView];
    sellerId = (entity as SellerView).id;
    toaster.success(m.quick_add_seller_success({ name: entity.name }));
  }
  quickAddTarget = null;
}
```

---

### Step 7 — Wire up AcquisitionBatchFields (Seller & Buyer)

**File**: `src/lib/features/acquisition/components/AcquisitionBatchFields.svelte`

Add `+` button beside the existing `SearchableSelect` for sellers. Emit an `onQuickAdd` event/callback up to the parent drawer to set `quickAddTarget = 'seller'`.

---

### Step 8 — Wire up AddWishlistItemDrawer and AddCollectionItemDrawer

Follow the same pattern as Steps 6–7 for:
- `src/lib/features/wishlists/AddWishlistItemDrawer.svelte`
- `src/lib/features/collection/components/AddCollectionItemDrawer.svelte`

Both already have `manufacturers` loaded; both already use the same `Select.Root` pattern for manufacturer selection.

---

### Step 9 — Regenerate specta bindings & run checks

```bash
pnpm specta:generate      # only if new Rust commands were added by 041
pnpm svelte-check
pnpm lint
pnpm test
```

---

## File Manifest

| Action | File |
|--------|------|
| CREATE | `src/lib/schemas/quick-add-form.ts` |
| CREATE | `src/lib/components/drawer/QuickAddShell.svelte` |
| CREATE | `src/lib/features/quick-add/QuickAddEntityForm.svelte` |
| CREATE | `src/lib/features/quick-add/types.ts` |
| MODIFY | `src/lib/components/drawer/DrawerShell.svelte` (add `dimmed` prop) |
| MODIFY | `src/lib/features/acquisition/AcquisitionDrawer.svelte` |
| MODIFY | `src/lib/features/acquisition/components/AcquisitionItemCard.svelte` |
| MODIFY | `src/lib/features/acquisition/components/AcquisitionBatchFields.svelte` |
| MODIFY | `src/lib/features/wishlists/AddWishlistItemDrawer.svelte` |
| MODIFY | `src/lib/features/collection/components/AddCollectionItemDrawer.svelte` |
| MODIFY | `messages/en.json` |
| MODIFY | `messages/it.json` |

**Rust files (zero changes in 040)** — all write commands are owned by 041.

---

## Key Test Scenarios

Frontend tests live in `src/__tests__/`. Target at minimum:

| Test | File |
|------|------|
| QuickAddEntityForm: save disabled when name empty | `quick-add/QuickAddEntityForm.test.ts` |
| QuickAddEntityForm: save disabled on duplicate name | `quick-add/QuickAddEntityForm.test.ts` |
| QuickAddEntityForm: calls correct command on save | `quick-add/QuickAddEntityForm.test.ts` |
| QuickAddEntityForm: onSuccess called with returned entity | `quick-add/QuickAddEntityForm.test.ts` |
| QuickAddEntityForm: saveError shown on command failure | `quick-add/QuickAddEntityForm.test.ts` |
| AcquisitionDrawer: manufacturer auto-selected after quick-add | `acquisition/AcquisitionDrawer.test.ts` |
| AcquisitionDrawer: parent form data preserved across quick-add session | `acquisition/AcquisitionDrawer.test.ts` |
