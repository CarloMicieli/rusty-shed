import { derived, get, writable } from 'svelte/store';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import type { Wishlist, WishlistItem } from '$lib/bindings';

type WishlistPreviewLite = {
  id: string;
  name: string;
  notes: string | null;
  is_default: boolean;
  count: number;
  updated_at: string;
  total_value: Record<string, number>;
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

export type WishlistStateSnapshot = {
  wishlists: WishlistPreviewLite[];
  itemsByWishlist: Record<string, WishlistItem[]>;
  activeWishlistId: string | null;
};
const wishlists = writable<WishlistPreviewLite[]>([]);
const itemsByWishlist = writable<Record<string, WishlistItem[]>>({});
const activeWishlistId = writable<string | null>(null);
const isLoading = writable(false);
let previousState: WishlistStateSnapshot | null = null;

const defaultWishlist = derived(wishlists, ($lists) => $lists.find((w) => w.is_default) ?? null);

const activeWishlist = derived([wishlists, activeWishlistId], ([$lists, $activeId]) => {
  if (!$activeId) return null;
  return $lists.find((w) => w.id === $activeId) ?? null;
});

const wishlistItems = derived([itemsByWishlist, activeWishlistId], ([$items, $activeId]) => {
  if (!$activeId) return [];
  return $items[$activeId] ?? [];
});

function captureSnapshot() {
  previousState = {
    wishlists: structuredClone(get(wishlists)),
    itemsByWishlist: structuredClone(get(itemsByWishlist)),
    activeWishlistId: get(activeWishlistId)
  };
}

function revertSnapshot() {
  if (!previousState) return;
  wishlists.set(previousState.wishlists);
  itemsByWishlist.set(previousState.itemsByWishlist);
  activeWishlistId.set(previousState.activeWishlistId);
}

async function fetchWishlists() {
  isLoading.set(true);
  try {
    const response = await invokeCommand<WishlistPreviewLite[]>('get_wishlists');
    wishlists.set(response ?? []);

    const currentActive = get(activeWishlistId);
    const defaultList = response.find((w) => w.is_default);
    if (!currentActive && defaultList) {
      activeWishlistId.set(defaultList.id);
    }
  } catch (e) {
    console.error(e);
    toastError(randomId());
  } finally {
    isLoading.set(false);
  }
}

async function loadWishlistItems(wishlistId: string) {
  try {
    const response = await invokeCommand<Wishlist | { status: 'ok'; data: Wishlist | null } | null>(
      'get_wishlist_by_id',
      {
        id: wishlistId
      }
    );

    // Handle both Specta result shape or direct wishlist
    let wishlist: Wishlist | null = null;
    if (response && typeof response === 'object' && 'status' in response) {
      wishlist = response.status === 'ok' ? response.data : null;
    } else {
      wishlist = response as Wishlist | null;
    }
    itemsByWishlist.update((map) => ({ ...map, [wishlistId]: wishlist?.items ?? [] }));
  } catch (e) {
    console.error(e);
    toastError(randomId());
  }
}

async function selectWishlist(id: string) {
  activeWishlistId.set(id);
  const map = get(itemsByWishlist);
  if (!map[id]) {
    await loadWishlistItems(id);
  }
}

async function createWishlist(name: string, isDefault = false) {
  const toastId = randomId();
  captureSnapshot();

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

  wishlists.update((list) => {
    const cleared = isDefault ? list.map((w) => ({ ...w, is_default: false })) : list;
    return [...cleared, optimistic];
  });
  if (isDefault) activeWishlistId.set(tempId);
  toastLoading(toastId);

  try {
    const created = await invokeCommand<WishlistPreviewLite>('create_wishlist', {
      input: { name, notes: null, is_default: isDefault }
    });

    wishlists.update((list) => list.map((w) => (w.id === tempId ? created : w)));
    if (created.is_default) activeWishlistId.set(created.id);
    toastSuccess(toastId);
    return created;
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void createWishlist(name, isDefault);
    });
    return null;
  }
}

async function renameWishlist(id: string, name: string) {
  const toastId = randomId();
  captureSnapshot();
  wishlists.update((list) => list.map((w) => (w.id === id ? { ...w, name } : w)));
  toastLoading(toastId);

  try {
    await invokeCommand('rename_wishlist', { input: { id, name } });
    toastSuccess(toastId);
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void renameWishlist(id, name);
    });
  }
}

async function deleteWishlist(id: string) {
  const toastId = randomId();
  captureSnapshot();
  wishlists.update((list) => list.filter((w) => w.id !== id));
  itemsByWishlist.update((map) => {
    const next = { ...map };
    delete next[id];
    return next;
  });
  if (get(activeWishlistId) === id) activeWishlistId.set(null);
  toastLoading(toastId);

  try {
    await invokeCommand('delete_wishlist', { id });
    toastSuccess(toastId);
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void deleteWishlist(id);
    });
  }
}

async function setDefaultWishlist(id: string) {
  const toastId = randomId();
  captureSnapshot();
  wishlists.update((list) => list.map((w) => ({ ...w, is_default: w.id === id })));
  activeWishlistId.set(id);
  toastLoading(toastId);

  try {
    await invokeCommand('set_default_wishlist', { id });
    toastSuccess(toastId);
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void setDefaultWishlist(id);
    });
  }
}

async function addItem(wishlistId: string, modelId: string) {
  const toastId = randomId();
  captureSnapshot();
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

  itemsByWishlist.update((map) => {
    const bucket = map[wishlistId] ?? [];
    return { ...map, [wishlistId]: [...bucket, optimistic] };
  });
  wishlists.update((list) =>
    list.map((w) => (w.id === wishlistId ? { ...w, count: w.count + 1 } : w))
  );
  toastLoading(toastId);

  try {
    const created = await invokeCommand<WishlistItem>('add_to_wishlist', {
      input: { wishlist_id: wishlistId, railway_model_id: modelId }
    });
    itemsByWishlist.update((map) => {
      const bucket = map[wishlistId] ?? [];
      return {
        ...map,
        [wishlistId]: bucket.map((item) => (item.id === optimistic.id ? created : item))
      };
    });
    toastSuccess(toastId);
    return created;
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void addItem(wishlistId, modelId);
    });
    return null;
  }
}

async function removeItem(wishlistId: string, itemId: string) {
  const toastId = randomId();
  captureSnapshot();
  itemsByWishlist.update((map) => {
    const bucket = map[wishlistId] ?? [];
    return { ...map, [wishlistId]: bucket.filter((i) => i.id !== itemId) };
  });
  wishlists.update((list) =>
    list.map((w) => (w.id === wishlistId ? { ...w, count: Math.max(0, w.count - 1) } : w))
  );
  toastLoading(toastId);

  try {
    await invokeCommand('remove_from_wishlist', { item_id: itemId });
    toastSuccess(toastId);
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void removeItem(wishlistId, itemId);
    });
  }
}

async function moveItemToList(itemId: string, fromWishlistId: string, toWishlistId: string) {
  const toastId = randomId();
  captureSnapshot();

  itemsByWishlist.update((map) => {
    const source = map[fromWishlistId] ?? [];
    const target = map[toWishlistId] ?? [];
    const item = source.find((i) => i.id === itemId);
    if (!item) return map;
    return {
      ...map,
      [fromWishlistId]: source.filter((i) => i.id !== itemId),
      [toWishlistId]: [...target, item]
    };
  });
  wishlists.update((list) =>
    list.map((w) => {
      if (w.id === fromWishlistId) return { ...w, count: Math.max(0, w.count - 1) };
      if (w.id === toWishlistId) return { ...w, count: w.count + 1 };
      return w;
    })
  );
  toastLoading(toastId);

  try {
    await invokeCommand('move_item_to_list', {
      input: { item_id: itemId, destination_wishlist_id: toWishlistId }
    });
    toastSuccess(toastId);
  } catch (e) {
    console.error(e);
    revertSnapshot();
    toastError(toastId, () => {
      revertSnapshot();
      void moveItemToList(itemId, fromWishlistId, toWishlistId);
    });
  }
}

export const wishlistStore = {
  wishlists,
  itemsByWishlist,
  activeWishlistId,
  activeWishlist,
  defaultWishlist,
  wishlistItems,
  isLoading,
  fetchWishlists,
  selectWishlist,
  loadWishlistItems,
  createWishlist,
  renameWishlist,
  deleteWishlist,
  setDefaultWishlist,
  addItem,
  removeItem,
  moveItemToList,
  revertSnapshot
};
