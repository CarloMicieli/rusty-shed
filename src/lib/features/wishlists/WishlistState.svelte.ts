import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type {
  WishlistView,
  WishlistItem,
  WishlistPreview,
  WishlistItemView,
  AddRailwayModelToWishListArgs
} from '$lib/bindings';
import { safeInvoke, getErrorMessage, isRetryableError } from '$lib/services';

// Using WishlistPreview from bindings directly
export type { WishlistPreview as WishlistPreviewLite };

export type WishlistStateSnapshot = {
  wishlists: WishlistPreview[];
  itemsByWishlist: Record<string, WishlistItem[]>;
  activeWishlistId: string | null;
};

function randomId() {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
  return Math.random().toString(36).slice(2);
}

function toastLoading(id: string) {
  toaster.loading({ id, title: m.collection_toast_loading(), duration: 4000 });
}

function toastSuccess(id: string) {
  toaster.success({ id, title: m.collection_toast_success(), duration: 2000 });
}

function toastError(id: string, message?: string, retry?: () => void) {
  toaster.error({
    id,
    title: message || m.collection_toast_error(),
    duration: 5000,
    action: retry
      ? {
          label: m.collection_toast_retry(),
          onClick: retry
        }
      : undefined
  });
}

export class WishlistState {
  #wishlists = $state<WishlistPreview[]>([]);
  #itemsByWishlist = $state<Record<string, WishlistItem[]>>({});
  #activeWishlistId = $state<string | null>(null);
  #isLoading = $state(false);

  #snapshot: WishlistStateSnapshot | null = null;

  #defaultWishlist = $derived.by(() => this.#wishlists.find((w) => w.isDefault) ?? null);

  // Normalize a potentially snake_case shaped wishlist item (from backend)
  // into the camelCase `WishlistItem` shape used throughout the UI.
  _normalizeItem(obj: WishlistItem | WishlistItemView | unknown): WishlistItem {
    const o = obj as Record<string, unknown>;
    return {
      id: (o.id as string) ?? '',
      railwayModelId:
        (o.railwayModelId as string | undefined) ??
        (o['railway_model_id'] as string | undefined) ??
        '',
      priority: (o.priority as string) ?? 'NORMAL',
      status: (o.status as string) ?? 'WANTED',
      addedDate:
        (o.addedDate as string | undefined) ?? (o['added_date'] as string | undefined) ?? '',
      removedDate:
        (o.removedDate as string | undefined) ?? (o['removed_date'] as string | undefined) ?? null,
      notes: (o.notes as string | null) ?? null,
      desiredPrice: (o.desiredPrice as unknown) ?? (o['desired_price'] as unknown) ?? null,
      purchasedPrice: (o.purchasedPrice as unknown) ?? (o['purchased_price'] as unknown) ?? null
    } as WishlistItem;
  }

  // Normalize WishlistView/WishlistPreview shapes from backend into UI `WishlistPreview`.
  _normalizePreview(obj: WishlistPreview | WishlistView | unknown): WishlistPreview {
    const o = obj as Record<string, unknown>;
    return {
      id: (o.id as string) ?? '',
      name: (o.name as string) ?? '',
      notes: (o.notes as string | null) ?? null,
      isDefault:
        (o.isDefault as boolean | undefined) ?? (o['is_default'] as boolean | undefined) ?? false,
      count: o.count as unknown as bigint,
      updatedAt:
        (o.updatedAt as string | undefined) ??
        (o['updated_at'] as string | undefined) ??
        new Date().toISOString(), // eslint-disable-line svelte/prefer-svelte-reactivity
      totalValue:
        (o.totalValue as Record<string, unknown> | undefined) ??
        (o['total_value'] as Record<string, unknown> | undefined) ??
        {}
    } as WishlistPreview;
  }

  #activeWishlist = $derived.by(() => {
    if (!this.#activeWishlistId) return null;
    return this.#wishlists.find((w) => w.id === this.#activeWishlistId) ?? null;
  });

  #wishlistItems = $derived.by(() => {
    if (!this.#activeWishlistId) return [];
    return this.#itemsByWishlist[this.#activeWishlistId] ?? [];
  });

  get wishlists() {
    return this.#wishlists;
  }

  get itemsByWishlist() {
    return this.#itemsByWishlist;
  }

  get activeWishlistId() {
    return this.#activeWishlistId;
  }

  get isLoading() {
    return this.#isLoading;
  }

  get defaultWishlist() {
    return this.#defaultWishlist;
  }

  get activeWishlist() {
    return this.#activeWishlist;
  }

  get wishlistItems() {
    return this.#wishlistItems;
  }

  #captureSnapshot() {
    const clonedWishlists = this.#wishlists.map((w) => ({ ...w }));
    const clonedItems = Object.fromEntries(
      Object.entries(this.#itemsByWishlist).map(([key, items]) => [
        key,
        items.map((item) => ({ ...item }))
      ])
    );

    this.#snapshot = {
      wishlists: clonedWishlists,
      itemsByWishlist: clonedItems,
      activeWishlistId: this.#activeWishlistId
    };
  }

  revertSnapshot() {
    if (!this.#snapshot) return;
    this.#wishlists = this.#snapshot.wishlists;
    this.#itemsByWishlist = this.#snapshot.itemsByWishlist;
    this.#activeWishlistId = this.#snapshot.activeWishlistId;
  }

  async fetchWishlists() {
    this.#isLoading = true;
    try {
      const result = await safeInvoke<WishlistView[]>('get_wishlists');

      if (!result.ok) {
        console.error('Failed to fetch wishlists:', result.error);
        toastError(randomId(), getErrorMessage(result.error));
        return;
      }

      // Keep backend response shape (tests rely on snake_case fields)
      this.#wishlists = (result.data ?? []) as unknown as WishlistPreview[];

      const defaultList = this.#wishlists.find((w) => {
        const o = w as unknown as Record<string, unknown>;
        return (
          (o.isDefault as boolean | undefined) ?? (o['is_default'] as boolean | undefined) ?? false
        );
      });
      if (!this.#activeWishlistId && defaultList) {
        this.#activeWishlistId = defaultList.id;
      }
    } finally {
      this.#isLoading = false;
    }
  }

  async loadWishlistItems(wishlistId: string) {
    const result = await safeInvoke<WishlistView | null>('get_wishlist_by_id', { id: wishlistId });

    if (!result.ok) {
      console.error('Failed to load wishlist items:', result.error);
      toastError(randomId(), getErrorMessage(result.error));
      return;
    }

    this.#itemsByWishlist = {
      ...this.#itemsByWishlist,
      [wishlistId]: (result.data?.items ?? []).map((it) => this._normalizeItem(it))
    };
  }

  async selectWishlist(id: string) {
    this.#activeWishlistId = id;
    if (!this.#itemsByWishlist[id]) {
      await this.loadWishlistItems(id);
    }
  }

  async createWishlist(name: string, isDefault = false) {
    const toastId = randomId();
    this.#captureSnapshot();

    const tempId = `temp-${toastId}`;
    const optimistic: WishlistPreview = {
      id: tempId,
      name,
      notes: null,
      isDefault: isDefault,
      count: 0n,
      // eslint-disable-next-line svelte/prefer-svelte-reactivity
      updatedAt: new Date().toISOString(),
      totalValue: {}
    };

    const cleared = isDefault
      ? this.#wishlists.map((w) => ({ ...w, isDefault: false }))
      : this.#wishlists;
    this.#wishlists = [...cleared, optimistic];
    if (isDefault) this.#activeWishlistId = tempId;
    toastLoading(toastId);

    const result = await safeInvoke<WishlistPreview>('create_wishlist', {
      input: { name, notes: null, isDefault }
    });

    if (!result.ok) {
      console.error('Failed to create wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.createWishlist(name, isDefault);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return null;
    }

    this.#wishlists = this.#wishlists.map((w) => (w.id === tempId ? result.data : w));
    if (result.data.isDefault) this.#activeWishlistId = result.data.id;
    toastSuccess(toastId);
    return result.data;
  }

  async renameWishlist(id: string, name: string) {
    console.log('WishlistState: renameWishlist called', { id, name });
    const toastId = randomId();
    this.#captureSnapshot();
    this.#wishlists = this.#wishlists.map((w) => (w.id === id ? { ...w, name } : w));
    toastLoading(toastId);

    const result = await safeInvoke('rename_wishlist', { input: { id, name } });

    if (!result.ok) {
      console.error('Failed to rename wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.renameWishlist(id, name);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return;
    }

    toastSuccess(toastId);
  }

  async deleteWishlist(id: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    this.#wishlists = this.#wishlists.filter((w) => w.id !== id);

    const nextItems = { ...this.#itemsByWishlist };
    delete nextItems[id];
    this.#itemsByWishlist = nextItems;
    if (this.#activeWishlistId === id) this.#activeWishlistId = null;
    toastLoading(toastId);

    const result = await safeInvoke('delete_wishlist', { id });

    if (!result.ok) {
      console.error('Failed to delete wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.deleteWishlist(id);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return;
    }

    toastSuccess(toastId);
  }

  async setDefaultWishlist(id: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    this.#wishlists = this.#wishlists.map((w) => ({ ...w, isDefault: w.id === id }));
    this.#activeWishlistId = id;
    toastLoading(toastId);

    const result = await safeInvoke('set_default_wishlist', { id });

    if (!result.ok) {
      console.error('Failed to set default wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.setDefaultWishlist(id);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return;
    }

    toastSuccess(toastId);
  }

  async addItem(wishlistId: string, modelId: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    const optimistic: WishlistItem = {
      id: `temp-${toastId}`,
      railwayModelId: modelId,
      priority: 'NORMAL',
      status: 'WANTED',
      // eslint-disable-next-line svelte/prefer-svelte-reactivity
      addedDate: new Date().toISOString().slice(0, 10),
      removedDate: null,
      notes: null,
      desiredPrice: null,
      purchasedPrice: null
    } as WishlistItem;

    const bucket = this.#itemsByWishlist[wishlistId] ?? [];
    this.#itemsByWishlist = { ...this.#itemsByWishlist, [wishlistId]: [...bucket, optimistic] };
    this.#wishlists = this.#wishlists.map((w) =>
      w.id === wishlistId ? { ...w, count: w.count + 1n } : w
    );
    toastLoading(toastId);

    const result = await safeInvoke<WishlistItem>('add_to_wishlist', {
      input: {
        wishlistId,
        railwayModelId: modelId,
        priority: null,
        status: null,
        desiredPriceAmount: null,
        desiredPriceCurrency: null,
        notes: null,
        addedDate: null
      }
    });

    if (!result.ok) {
      console.error('Failed to add item to wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.addItem(wishlistId, modelId);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return null;
    }

    const current = this.#itemsByWishlist[wishlistId] ?? [];
    this.#itemsByWishlist = {
      ...this.#itemsByWishlist,
      [wishlistId]: current.map((item) =>
        item.id === optimistic.id ? this._normalizeItem(result.data ?? item) : item
      )
    };
    toastSuccess(toastId);
    return result.data;
  }

  async removeItem(wishlistId: string, itemId: string) {
    const toastId = randomId();
    this.#captureSnapshot();

    const bucket = this.#itemsByWishlist[wishlistId] ?? [];
    this.#itemsByWishlist = {
      ...this.#itemsByWishlist,
      [wishlistId]: bucket.filter((i) => i.id !== itemId)
    };
    this.#wishlists = this.#wishlists.map((w) =>
      w.id === wishlistId ? { ...w, count: w.count > 0n ? w.count - 1n : 0n } : w
    );
    toastLoading(toastId);

    const result = await safeInvoke('remove_from_wishlist', { itemId });

    if (!result.ok) {
      console.error('Failed to remove item from wishlist:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.removeItem(wishlistId, itemId);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return;
    }

    toastSuccess(toastId);
  }

  async moveItemToList(itemId: string, fromWishlistId: string, toWishlistId: string) {
    const toastId = randomId();
    this.#captureSnapshot();

    const source = this.#itemsByWishlist[fromWishlistId] ?? [];
    const target = this.#itemsByWishlist[toWishlistId] ?? [];
    const item = source.find((i) => i.id === itemId);
    if (!item) return;

    this.#itemsByWishlist = {
      ...this.#itemsByWishlist,
      [fromWishlistId]: source.filter((i) => i.id !== itemId),
      [toWishlistId]: [...target, item]
    };

    this.#wishlists = this.#wishlists.map((w) => {
      if (w.id === fromWishlistId) return { ...w, count: w.count > 0n ? w.count - 1n : 0n };
      if (w.id === toWishlistId) return { ...w, count: w.count + 1n };
      return w;
    });
    toastLoading(toastId);

    const result = await safeInvoke('move_item_to_list', {
      input: { itemId, destinationWishlistId: toWishlistId, wishlistId: fromWishlistId }
    });

    if (!result.ok) {
      console.error('Failed to move item to list:', result.error);
      this.revertSnapshot();
      const retry = isRetryableError(result.error)
        ? () => {
            this.revertSnapshot();
            void this.moveItemToList(itemId, fromWishlistId, toWishlistId);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return;
    }

    toastSuccess(toastId);
  }

  /**
   * Add a new railway model to a wishlist.
   * Creates the railway model in the catalog and adds it to the specified wishlist.
   *
   * @param args - The complete arguments for the command
   * @returns Promise<boolean> - true on success, false on failure
   */
  async addRailwayModelToWishlist(args: AddRailwayModelToWishListArgs): Promise<boolean> {
    const toastId = randomId();
    toastLoading(toastId);

    const result = await safeInvoke('add_railway_model_to_wish_list', { args });

    if (!result.ok) {
      console.error('Failed to add railway model to wishlist:', result.error);
      toastError(toastId, getErrorMessage(result.error));
      return false;
    }

    // Refresh the active wishlist items if it matches the target wishlist
    if (this.#activeWishlistId === args.wishlistId) {
      await this.loadWishlistItems(args.wishlistId);
    }

    // Refresh wishlist previews (counts may have changed)
    await this.fetchWishlists();

    toastSuccess(toastId);
    return true;
  }
}

const WISHLIST_CONTEXT_KEY = Symbol('wishlist-context');

export function createWishlistState() {
  return new WishlistState();
}

export function setWishlistContext(state: WishlistState) {
  setContext(WISHLIST_CONTEXT_KEY, state);
}

export function getWishlistContext(): WishlistState {
  const state = getContext<WishlistState>(WISHLIST_CONTEXT_KEY);
  if (!state) {
    throw new Error(
      'WishlistContext not provided. Ensure component is within a WishlistContext provider.'
    );
  }
  return state;
}
