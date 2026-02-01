# Quickstart: Add Railway Model to Wishlist

**Feature**: 003-add-model-wishlist  
**Date**: 2026-01-30  
**Time Estimate**: 4-6 hours

## Prerequisites

- [x] Rust backend implementation complete (`addRailwayModelToWishList` command)
- [x] Feature specification approved
- [x] On branch `003-add-model-wishlist`

## Implementation Order

### Phase 1: Foundation (1 hour)

1. **Add i18n message keys**

   ```bash
   # Edit messages/en.json and messages/it.json
   # Add all keys from contracts/messages.md
   pnpm prepare  # Regenerate Paraglide types
   ```

2. **Add static data arrays**
   Create or update `src/lib/features/wishlists/constants.ts`:

   ```typescript
   import type { Category, Scale, PowerMethod, WishlistPriority } from '$lib/bindings';

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
   ```

### Phase 2: Service Layer (30 min)

3. **Extend WishlistState.svelte.ts**
   Add the `addRailwayModelToWishlist` method:

   ```typescript
   async addRailwayModelToWishlist(
     args: AddRailwayModelToWishListArgs
   ): Promise<boolean> {
     const toastId = randomId();
     toastLoading(toastId);

     const result = await safeInvoke('add_railway_model_to_wish_list', { args });

     if (!result.ok) {
       toastError(toastId, getErrorMessage(result.error));
       return false;
     }

     if (this.#activeWishlistId === args.wishlistId) {
       await this.loadWishlistItems(args.wishlistId);
     }
     await this.fetchWishlists();

     toastSuccess(toastId);
     return true;
   }
   ```

### Phase 3: Components (2-3 hours)

4. **Create RollingStockEntry.svelte**
   Location: `src/lib/features/wishlists/components/RollingStockEntry.svelte`
   - Small form section with 4 fields
   - Railway company dropdown, series code input, category dropdown, road number input
   - Remove button

5. **Create AddRailwayModelDrawer.svelte**
   Location: `src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte`
   - Mirror `ItemDrawer.svelte` structure from collection
   - Load manufacturers and railway companies on mount
   - Form with all fields from data-model.md
   - Rolling stocks section with add/remove
   - Validation with `$derived`
   - Submit calls `wishlistService.addRailwayModelToWishlist()`

### Phase 4: Integration (1 hour)

6. **Update WishlistHeader.svelte**
   - Add `onAddModel` prop
   - Add "Add railway model" button with Plus icon

7. **Update WishlistsDashboard.svelte**
   - Add drawer open/close state
   - Add handler functions
   - Import and render `AddRailwayModelDrawer`
   - Pass `preselectedWishlistId` as `activeWishlistId`

### Phase 5: Testing & Verification (1 hour)

8. **Add component tests**
   Location: `src/__tests__/lib/features/wishlists/AddRailwayModelDrawer.test.ts`
   - Test form validation
   - Test rolling stock add/remove
   - Test form reset on close

9. **Manual verification**

   ```bash
   pnpm dev
   # Navigate to /my-wishlists
   # Test: Add railway model from header
   # Test: Add railway model with pre-selected wishlist
   # Test: Add/remove rolling stocks
   # Test: Form validation
   # Test: Successful submission
   ```

10. **Run verification suite**
    ```bash
    pnpm format
    pnpm lint
    pnpm check
    pnpm test
    ```

## File Checklist

| Status | File                                                                 | Action                     |
| ------ | -------------------------------------------------------------------- | -------------------------- |
| ☐      | `messages/en.json`                                                   | Add 40+ new keys           |
| ☐      | `messages/it.json`                                                   | Add 40+ new keys (Italian) |
| ☐      | `src/lib/features/wishlists/constants.ts`                            | Create with static arrays  |
| ☐      | `src/lib/features/wishlists/WishlistState.svelte.ts`                 | Add method                 |
| ☐      | `src/lib/features/wishlists/components/RollingStockEntry.svelte`     | Create                     |
| ☐      | `src/lib/features/wishlists/components/AddRailwayModelDrawer.svelte` | Create                     |
| ☐      | `src/lib/features/wishlists/components/WishlistHeader.svelte`        | Modify                     |
| ☐      | `src/lib/features/wishlists/WishlistsDashboard.svelte`               | Modify                     |
| ☐      | `src/__tests__/lib/features/wishlists/AddRailwayModelDrawer.test.ts` | Create                     |

## Key Patterns to Follow

### Drawer Pattern (from ItemDrawer.svelte)

```svelte
{#if open}
  <div
    class="fixed inset-0 z-50 flex justify-end bg-black/40"
    role="presentation"
    tabindex="-1"
    onclick={handleClose}
    onkeydown={(e) => e.key === 'Escape' && handleClose()}
  >
    <div
      class="h-full w-full max-w-xl overflow-y-auto border-l
                border-surface-700/60 bg-surface-900 p-6 shadow-2xl"
      role="dialog"
      aria-modal="true"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Content -->
    </div>
  </div>
{/if}
```

### Form Validation Pattern

```typescript
const isValid = $derived.by(() => {
  return (
    form.wishlistId !== '' &&
    form.manufacturerId !== '' &&
    form.productCode.trim() !== '' &&
    // ... other required fields
    form.rollingStocks.every(
      (rs) => rs.railwayCompanyId !== '' && rs.seriesCode.trim() !== '' && rs.category !== ''
    )
  );
});
```

### Dynamic Array Pattern (rolling stocks)

```typescript
let rollingStocks = $state<RollingStockFormEntry[]>([]);

function addRollingStock() {
  rollingStocks = [
    ...rollingStocks,
    {
      id: crypto.randomUUID(),
      railwayCompanyId: '',
      seriesCode: '',
      category: '',
      roadNumber: ''
    }
  ];
}

function removeRollingStock(id: string) {
  rollingStocks = rollingStocks.filter((rs) => rs.id !== id);
}
```

## Common Pitfalls

1. **Don't forget to run `pnpm prepare`** after adding i18n keys
2. **Use `BigInt()` for price amounts** when calling backend
3. **Always provide `addedDate`** in YYYY-MM-DD format
4. **Reset form state** when drawer closes (use `$effect`)
5. **Load dropdown data** when drawer opens, not on page mount

## Verification Commands

```bash
# Format code
pnpm format

# Lint check
pnpm lint

# Type check
pnpm check

# Run tests
pnpm test

# Run dev server
pnpm dev
```

## Success Criteria

- [ ] User can open drawer from "My Wishlists" page
- [ ] User can select wishlist from dropdown
- [ ] All required fields have validation
- [ ] User can add/remove rolling stocks
- [ ] Form submits successfully
- [ ] Wishlist updates after submission
- [ ] All tests pass
- [ ] No lint/type errors
