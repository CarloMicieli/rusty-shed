import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { Wishlist, WishlistItem } from '$lib/bindings';
import { safeInvoke, getErrorMessage, isRetryableError } from '$lib/services';

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
			const result = await safeInvoke<WishlistPreviewLite[]>('get_wishlists');

			if (!result.ok) {
				console.error('Failed to fetch wishlists:', result.error);
				toastError(randomId(), getErrorMessage(result.error));
				return;
			}

			this.#wishlists = result.data ?? [];

			const defaultList = this.#wishlists.find((w) => w.is_default);
			if (!this.#activeWishlistId && defaultList) {
				this.#activeWishlistId = defaultList.id;
			}
		} finally {
			this.#isLoading = false;
		}
	}

	async loadWishlistItems(wishlistId: string) {
		const result = await safeInvoke<Wishlist | null>('get_wishlist_by_id', { id: wishlistId });

		if (!result.ok) {
			console.error('Failed to load wishlist items:', result.error);
			toastError(randomId(), getErrorMessage(result.error));
			return;
		}

		this.#itemsByWishlist = {
			...this.#itemsByWishlist,
			[wishlistId]: result.data?.items ?? []
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

    const result = await safeInvoke<WishlistPreviewLite>('create_wishlist', {
      input: { name, notes: null, is_default: isDefault }
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
    if (result.data.is_default) this.#activeWishlistId = result.data.id;
    toastSuccess(toastId);
    return result.data;
  }

  async renameWishlist(id: string, name: string) {
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
    this.#wishlists = this.#wishlists.map((w) => ({ ...w, is_default: w.id === id }));
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

    const result = await safeInvoke<WishlistItem>('add_to_wishlist', {
      input: { wishlist_id: wishlistId, railway_model_id: modelId }
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
      [wishlistId]: current.map((item) => (item.id === optimistic.id ? result.data : item))
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
      w.id === wishlistId ? { ...w, count: Math.max(0, w.count - 1) } : w
    );
    toastLoading(toastId);

    const result = await safeInvoke('remove_from_wishlist', { item_id: itemId });

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
      if (w.id === fromWishlistId) return { ...w, count: Math.max(0, w.count - 1) };
      if (w.id === toWishlistId) return { ...w, count: w.count + 1 };
      return w;
    });
    toastLoading(toastId);

    const result = await safeInvoke('move_item_to_list', {
      input: { item_id: itemId, destination_wishlist_id: toWishlistId }
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
}

export const wishlistService = new WishlistService();
