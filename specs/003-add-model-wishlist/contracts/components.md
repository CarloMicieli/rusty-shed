# Component Contracts: Add Railway Model to Wishlist

**Feature**: 003-add-model-wishlist  
**Date**: 2026-01-30

## AddRailwayModelDrawer.svelte

### Props Interface

```typescript
interface AddRailwayModelDrawerProps {
  /** Whether the drawer is open */
  open: boolean;

  /** Pre-selected wishlist ID (optional, for contextual opening) */
  preselectedWishlistId?: string | null;

  /** Available wishlists for the dropdown */
  wishlists: WishlistPreview[];

  /** Callback when drawer requests close */
  onClose: () => void;

  /** Callback on successful submission */
  onSuccess: () => void;
}
```

### Events

| Event       | Trigger                                           | Payload |
| ----------- | ------------------------------------------------- | ------- |
| `onClose`   | User clicks backdrop, Escape key, or close button | none    |
| `onSuccess` | Backend command completes successfully            | none    |

### State Management

- Internal `$state` for form fields
- Internal `$state` for dropdown data (manufacturers, railway companies)
- Internal `$state` for loading/submitting states
- `$derived` for validation state

### Accessibility

- `role="dialog"` on drawer container
- `aria-modal="true"`
- Focus trap within drawer
- Escape key closes drawer

---

## RollingStockEntry.svelte

### Props Interface

```typescript
interface RollingStockEntryProps {
  /** Unique identifier for this entry */
  id: string;

  /** Current entry data */
  entry: RollingStockFormEntry;

  /** Available railway companies for dropdown */
  railwayCompanies: RailwayCompany[];

  /** Index in the list (for display) */
  index: number;

  /** Callback when entry data changes */
  onChange: (id: string, field: keyof RollingStockFormEntry, value: string) => void;

  /** Callback to remove this entry */
  onRemove: (id: string) => void;
}
```

### Events

| Event      | Trigger                 | Payload              |
| ---------- | ----------------------- | -------------------- |
| `onChange` | Any field value changes | `(id, field, value)` |
| `onRemove` | Remove button clicked   | `(id)`               |

---

## WishlistState.svelte.ts Extensions

### New Method

```typescript
class WishlistState {
  // ... existing methods ...

  /**
   * Add a new railway model to a wishlist.
   * Creates the railway model in the catalog and adds it to the specified wishlist.
   *
   * @param args - The complete arguments for the command
   * @returns Promise<boolean> - true on success, false on failure
   */
  async addRailwayModelToWishlist(args: AddRailwayModelToWishListArgs): Promise<boolean>;
}
```

### Implementation Contract

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

  // Refresh the active wishlist items
  if (this.#activeWishlistId === args.wishlistId) {
    await this.loadWishlistItems(args.wishlistId);
  }

  // Refresh wishlist previews (counts may have changed)
  await this.fetchWishlists();

  toastSuccess(toastId);
  return true;
}
```

---

## WishlistsDashboard.svelte Changes

### New State

```typescript
// Add to useWishlistUI or inline state
let showAddModelDrawer = $state(false);

function openAddModelDrawer() {
  showAddModelDrawer = true;
}

function closeAddModelDrawer() {
  showAddModelDrawer = false;
}

async function handleAddModelSuccess() {
  closeAddModelDrawer();
  // Refresh handled by WishlistState method
}
```

### Template Changes

- Add "Add railway model" button in header section
- Add `<AddRailwayModelDrawer>` component at bottom of template

---

## WishlistHeader.svelte Changes

### Updated Props Interface

```typescript
interface WishlistHeaderProps {
  wishlist: WishlistPreview | null;
  onRename?: (name: string) => void;
  onSetDefault?: () => void;
  onAddModel?: () => void; // NEW
}
```

### Template Changes

- Add "Add railway model" button next to "Set as Default" button
