/**
 * Wishlist Service - Manages wishlist state and operations.
 *
 * This service provides:
 * - Wishlist CRUD operations
 * - Wishlist item management
 * - Optimistic updates with rollback
 * - Error handling with retry logic
 */

import { setContext, getContext } from 'svelte';
import { SvelteDate } from 'svelte/reactivity';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { getErrorMessage, isRetryableError } from '$lib/shared/domain/errors';
import type { WishlistView, WishlistItem, WishlistPreview } from '$lib/bindings';

// ─────────────────────────────────────────────────────────────
// TYPES
// ─────────────────────────────────────────────────────────────

export type { WishlistPreview as WishlistPreviewLite };

export type WishlistStateSnapshot = {
  wishlists: WishlistPreview[];
  itemsByWishlist: Record<string, WishlistItem[]>;
  activeWishlistId: string | null;
};

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('wishlist-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class WishlistService {
  // Private reactive state
  #wishlists = $state<WishlistPreview[]>([]);
  #itemsByWishlist = $state<Record<string, WishlistItem[]>>({});
  #activeWishlistId = $state<string | null>(null);
  #isLoading = $state(false);
  #snapshot: WishlistStateSnapshot | null = null;

  // Public readonly getters (defensive encapsulation)
  get wishlists(): WishlistPreview[] {
    return this.#wishlists;
  }

  get itemsByWishlist(): Record<string, WishlistItem[]> {
    return this.#itemsByWishlist;
  }

  get activeWishlistId(): string | null {
    return this.#activeWishlistId;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  // Derived state
  defaultWishlist = $derived.by(() => {
    return this.#wishlists.find((w) => w.isDefault) ?? null;
  });

  activeWishlist = $derived.by(() => {
    if (!this.#activeWishlistId) return null;
    return this.#wishlists.find((w) => w.id === this.#activeWishlistId) ?? null;
  });

  wishlistItems = $derived.by(() => {
    if (!this.#activeWishlistId) return [];
    return this.#itemsByWishlist[this.#activeWishlistId] ?? [];
  });

  // ─────────────────────────────────────────────────────────────
  // PRIVATE HELPERS
  // ─────────────────────────────────────────────────────────────

  #captureSnapshot(): void {
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

  #revertSnapshot(): void {
    if (!this.#snapshot) return;
    this.#wishlists = this.#snapshot.wishlists;
    this.#itemsByWishlist = this.#snapshot.itemsByWishlist;
    this.#activeWishlistId = this.#snapshot.activeWishlistId;
  }

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Fetch all wishlists from the backend.
   */
  async fetchWishlists(): Promise<void> {
    this.#isLoading = true;
    try {
      const result = await safeInvoke<WishlistView[]>('get_wishlists');

      if (!result.ok) {
        console.error('Failed to fetch wishlists:', result.error);
        toaster.error({
          id: crypto.randomUUID(),
          title: getErrorMessage(result.error),
          duration: 5000
        });
        return;
      }

      this.#wishlists = (result.data ?? []) as unknown as WishlistPreview[];

      // Auto-select default wishlist if none selected
      const defaultList = this.#wishlists.find((w) => w.isDefault);
      if (!this.#activeWishlistId && defaultList) {
        this.#activeWishlistId = defaultList.id;
      }
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Load items for a specific wishlist.
   *
   * @param wishlistId - The wishlist ID
   */
  async loadWishlistItems(wishlistId: string): Promise<void> {
    const result = await safeInvoke<WishlistView | null>('get_wishlist_by_id', { id: wishlistId });

    if (!result.ok) {
      console.error('Failed to load wishlist items:', result.error);
      toaster.error({
        id: crypto.randomUUID(),
        title: getErrorMessage(result.error),
        duration: 5000
      });
      return;
    }

    // Map WishlistItemView to WishlistItem (cast via unknown for type compatibility)
    const items = (result.data?.items ?? []) as unknown as WishlistItem[];

    this.#itemsByWishlist = {
      ...this.#itemsByWishlist,
      [wishlistId]: items
    };
  }

  /**
   * Select a wishlist and load its items.
   *
   * @param id - The wishlist ID to select
   */
  async selectWishlist(id: string): Promise<void> {
    this.#activeWishlistId = id;
    if (!this.#itemsByWishlist[id]) {
      await this.loadWishlistItems(id);
    }
  }

  /**
   * Create a new wishlist.
   *
   * @param name - The wishlist name
   * @param isDefault - Whether this should be the default wishlist
   * @returns The created wishlist or null if failed
   */
  async createWishlist(name: string, isDefault = false): Promise<WishlistPreview | null> {
    this.#captureSnapshot();

    // Optimistic update
    const tempId = `temp-${crypto.randomUUID()}`;
    const optimistic: WishlistPreview = {
      id: tempId,
      name,
      notes: null,
      isDefault: isDefault,
      count: 0n,
      updatedAt: new SvelteDate().toISOString(),
      totalValue: {}
    };

    const cleared = isDefault
      ? this.#wishlists.map((w) => ({ ...w, isDefault: false }))
      : this.#wishlists;
    this.#wishlists = [...cleared, optimistic];
    if (isDefault) this.#activeWishlistId = tempId;

    const toastId = toaster.loading(m.collection_toast_loading(), { duration: 4000 });

    const result = await safeInvoke<WishlistPreview>('create_wishlist', {
      input: { name, notes: null, isDefault }
    });

    if (!result.ok) {
      console.error('Failed to create wishlist:', result.error);
      this.#revertSnapshot();
      toaster.dismiss(toastId);
      const retry = isRetryableError(result.error)
        ? {
            label: m.collection_toast_retry(),
            onClick: () => {
              this.#revertSnapshot();
              void this.createWishlist(name, isDefault);
            }
          }
        : undefined;
      toaster.error(getErrorMessage(result.error), {
        duration: 5000,
        action: retry
      });
      return null;
    }

    // Replace optimistic with real data
    this.#wishlists = this.#wishlists.map((w) => (w.id === tempId ? result.data : w));
    if (result.data.isDefault) this.#activeWishlistId = result.data.id;
    toaster.dismiss(toastId);
    toaster.success(m.collection_toast_success(), { duration: 2000 });
    return result.data;
  }

  /**
   * Rename a wishlist.
   *
   * @param id - The wishlist ID
   * @param name - The new name
   */
  async renameWishlist(id: string, name: string): Promise<void> {
    this.#captureSnapshot();

    // Optimistic update
    this.#wishlists = this.#wishlists.map((w) => (w.id === id ? { ...w, name } : w));
    const toastId = toaster.loading(m.collection_toast_loading(), { duration: 4000 });

    const result = await safeInvoke('rename_wishlist', { input: { wishlistId: id, name } });

    if (!result.ok) {
      console.error('Failed to rename wishlist:', result.error);
      this.#revertSnapshot();
      toaster.dismiss(toastId);
      const retry = isRetryableError(result.error)
        ? {
            label: m.collection_toast_retry(),
            onClick: () => {
              this.#revertSnapshot();
              void this.renameWishlist(id, name);
            }
          }
        : undefined;
      toaster.error(getErrorMessage(result.error), {
        duration: 5000,
        action: retry
      });
      return;
    }

    toaster.dismiss(toastId);
    toaster.success(m.collection_toast_success(), { duration: 2000 });
  }

  /**
   * Delete a wishlist.
   *
   * @param id - The wishlist ID to delete
   */
  async deleteWishlist(id: string): Promise<void> {
    this.#captureSnapshot();

    // Optimistic update
    this.#wishlists = this.#wishlists.filter((w) => w.id !== id);
    const nextItems = { ...this.#itemsByWishlist };
    delete nextItems[id];
    this.#itemsByWishlist = nextItems;
    if (this.#activeWishlistId === id) this.#activeWishlistId = null;

    const toastId = toaster.loading(m.collection_toast_loading(), { duration: 4000 });

    const result = await safeInvoke('delete_wishlist', { id });

    if (!result.ok) {
      console.error('Failed to delete wishlist:', result.error);
      this.#revertSnapshot();
      toaster.dismiss(toastId);
      const retry = isRetryableError(result.error)
        ? {
            label: m.collection_toast_retry(),
            onClick: () => {
              this.#revertSnapshot();
              void this.deleteWishlist(id);
            }
          }
        : undefined;
      toaster.error(getErrorMessage(result.error), {
        duration: 5000,
        action: retry
      });
      return;
    }

    toaster.dismiss(toastId);
    toaster.success(m.collection_toast_success(), { duration: 2000 });
  }

  /**
   * Refresh a specific wishlist (reload from backend).
   *
   * @param id - The wishlist ID to refresh
   */
  async refreshWishlist(id: string): Promise<void> {
    await this.loadWishlistItems(id);
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the WishlistService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setWishlistService(service?: WishlistService): WishlistService {
  const instance = service ?? new WishlistService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the WishlistService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getWishlistService(): WishlistService {
  const service = getContext<WishlistService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'WishlistService not found in context. Did you call setWishlistService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use setWishlistService() instead
 */
export function createWishlistState(): WishlistService {
  console.warn('createWishlistState is deprecated. Use setWishlistService() instead.');
  return new WishlistService();
}

/**
 * @deprecated Use setWishlistService() instead
 */
export function setWishlistContext(state: WishlistService): void {
  console.warn('setWishlistContext is deprecated. Use setWishlistService() instead.');
  setContext(SERVICE_KEY, state);
}

/**
 * @deprecated Use getWishlistService() instead
 */
export function getWishlistContext(): WishlistService {
  console.warn('getWishlistContext is deprecated. Use getWishlistService() instead.');
  return getWishlistService();
}
