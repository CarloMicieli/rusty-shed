import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { Wishlist, WishlistItem } from '$lib/bindings';

export type WishlistPreviewLite = {
  id: string;
  name: string;
  notes: string | null;
  is_default: boolean;
  count: number;
  updated_at: string;
  total_value: Record<string, number>;
};

export type WishlistStateSnapshot = {
  wishlists: WishlistPreviewLite[];
  itemsByWishlist: Record<string, WishlistItem[]>;
  activeWishlistId: string | null;
};

// Helper to invoke Tauri commands without relying on generated bindings being up-to-date.
async function invokeCommand<T>(cmd: string, payload?: Record<string, unknown>): Promise<T> {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke(cmd, payload ?? {}) as Promise<T>;
}

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

function toastError(id: string, retry?: () => void) {
  toaster.error({
    id,
    title: m.collection_toast_error(),
    duration: 5000,
    action: retry
      ? {
          label: m.collection_toast_retry(),
          onClick: retry
        }
      : undefined
  });
}

class WishlistService {
  #wishlists = $state<WishlistPreviewLite[]>([]);
  #itemsByWishlist = $state<Record<string, WishlistItem[]>>({});
  #activeWishlistId = $state<string | null>(null);
  #isLoading = $state(false);

  #snapshot: WishlistStateSnapshot | null = null;

  #defaultWishlist = $derived.by(() => this.#wishlists.find((w) => w.is_default) ?? null);

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
    this.#snapshot = {
      wishlists: structuredClone(this.#wishlists),
      itemsByWishlist: structuredClone(this.#itemsByWishlist),
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
      const response = await invokeCommand<WishlistPreviewLite[]>('get_wishlists');
      this.#wishlists = response ?? [];

      const defaultList = this.#wishlists.find((w) => w.is_default);
      if (!this.#activeWishlistId && defaultList) {
        this.#activeWishlistId = defaultList.id;
      }
    } catch (e) {
      console.error(e);
      toastError(randomId());
    } finally {
      this.#isLoading = false;
    }
  }

  async loadWishlistItems(wishlistId: string) {
    try {
      const response = await invokeCommand<
        Wishlist | { status: 'ok'; data: Wishlist | null } | null
      >('get_wishlist_by_id', { id: wishlistId });

      let wishlist: Wishlist | null = null;
      if (response && typeof response === 'object' && 'status' in response) {
        wishlist = response.status === 'ok' ? response.data : null;
      } else {
        wishlist = response as Wishlist | null;
      }

      this.#itemsByWishlist = {
        ...this.#itemsByWishlist,
        [wishlistId]: wishlist?.items ?? []
      };
    } catch (e) {
      console.error(e);
      toastError(randomId());
    }
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
    const optimistic: WishlistPreviewLite = {
      id: tempId,
      name,
      notes: null,
      is_default: isDefault,
      count: 0,
      updated_at: new Date().toISOString(),
      total_value: {}
    };

    const cleared = isDefault
      ? this.#wishlists.map((w) => ({ ...w, is_default: false }))
      : this.#wishlists;
    this.#wishlists = [...cleared, optimistic];
    if (isDefault) this.#activeWishlistId = tempId;
    toastLoading(toastId);

    try {
      const created = await invokeCommand<WishlistPreviewLite>('create_wishlist', {
        input: { name, notes: null, is_default: isDefault }
      });

      this.#wishlists = this.#wishlists.map((w) => (w.id === tempId ? created : w));
      if (created.is_default) this.#activeWishlistId = created.id;
      toastSuccess(toastId);
      return created;
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.createWishlist(name, isDefault);
      });
      return null;
    }
  }

  async renameWishlist(id: string, name: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    this.#wishlists = this.#wishlists.map((w) => (w.id === id ? { ...w, name } : w));
    toastLoading(toastId);

    try {
      await invokeCommand('rename_wishlist', { input: { id, name } });
      toastSuccess(toastId);
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.renameWishlist(id, name);
      });
    }
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

    try {
      await invokeCommand('delete_wishlist', { id });
      toastSuccess(toastId);
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.deleteWishlist(id);
      });
    }
  }

  async setDefaultWishlist(id: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    this.#wishlists = this.#wishlists.map((w) => ({ ...w, is_default: w.id === id }));
    this.#activeWishlistId = id;
    toastLoading(toastId);

    try {
      await invokeCommand('set_default_wishlist', { id });
      toastSuccess(toastId);
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.setDefaultWishlist(id);
      });
    }
  }

  async addItem(wishlistId: string, modelId: string) {
    const toastId = randomId();
    this.#captureSnapshot();
    const optimistic: WishlistItem = {
      id: `temp-${toastId}`,
      railway_model_id: modelId,
      priority: 'NORMAL',
      status: 'WANTED',
      added_date: new Date().toISOString().slice(0, 10),
      removed_date: null,
      notes: null,
      desired_price: null,
      purchased_price: null
    } as WishlistItem;

    const bucket = this.#itemsByWishlist[wishlistId] ?? [];
    this.#itemsByWishlist = { ...this.#itemsByWishlist, [wishlistId]: [...bucket, optimistic] };
    this.#wishlists = this.#wishlists.map((w) =>
      w.id === wishlistId ? { ...w, count: w.count + 1 } : w
    );
    toastLoading(toastId);

    try {
      const created = await invokeCommand<WishlistItem>('add_to_wishlist', {
        input: { wishlist_id: wishlistId, railway_model_id: modelId }
      });

      const current = this.#itemsByWishlist[wishlistId] ?? [];
      this.#itemsByWishlist = {
        ...this.#itemsByWishlist,
        [wishlistId]: current.map((item) => (item.id === optimistic.id ? created : item))
      };
      toastSuccess(toastId);
      return created;
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.addItem(wishlistId, modelId);
      });
      return null;
    }
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
      w.id === wishlistId ? { ...w, count: Math.max(0, w.count - 1) } : w
    );
    toastLoading(toastId);

    try {
      await invokeCommand('remove_from_wishlist', { item_id: itemId });
      toastSuccess(toastId);
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.removeItem(wishlistId, itemId);
      });
    }
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
      if (w.id === fromWishlistId) return { ...w, count: Math.max(0, w.count - 1) };
      if (w.id === toWishlistId) return { ...w, count: w.count + 1 };
      return w;
    });
    toastLoading(toastId);

    try {
      await invokeCommand('move_item_to_list', {
        input: { item_id: itemId, destination_wishlist_id: toWishlistId }
      });
      toastSuccess(toastId);
    } catch (e) {
      console.error(e);
      this.revertSnapshot();
      toastError(toastId, () => {
        this.revertSnapshot();
        void this.moveItemToList(itemId, fromWishlistId, toWishlistId);
      });
    }
  }
}

export const wishlistService = new WishlistService();
export { invokeCommand };
