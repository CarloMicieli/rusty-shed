import { derived, get, writable } from 'svelte/store';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import {
  commands,
  type CollectionItemLite,
  type CreateCollectionItemInput,
  type UpdateCollectionItemInput
} from '$lib/bindings';

export type FilterState = {
  query: string;
  scale: string | null;
  tags: Set<string>;
};

const rawItems = writable<CollectionItemLite[]>([]);
const filters = writable<FilterState>({ query: '', scale: null, tags: new Set() });
const isLoading = writable(false);

const availableTags = derived(rawItems, ($items) => {
  const dynamic = new Set<string>();
  $items.forEach((item) => item.tags?.forEach((tag) => dynamic.add(tag)));
  const combined = new Set<string>([...Object.keys(FIXED_TAG_META), ...dynamic]);
  return sortAvailableTags([...combined]);
});

const filteredItems = derived([rawItems, filters], ([$items, $filters]) => {
  const q = $filters.query.trim().toLowerCase();
  return $items.filter((item) => {
    if ($filters.scale && item.scale !== $filters.scale) return false;
    if ($filters.tags.size) {
      const hasTag = item.tags.some((tag) => $filters.tags.has(tag));
      if (!hasTag) return false;
    }
    if (q) {
      const haystack =
        `${item.brand} ${item.catalogNumber} ${item.title} ${item.description ?? ''} ${item.tags.join(' ')}`.toLowerCase();
      if (!haystack.includes(q)) return false;
    }
    return true;
  });
});

const totalCount = derived(rawItems, ($items) => $items.length);

function randomId() {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
  return Math.random().toString(36).slice(2);
}

function toastLoading(id: string) {
  toaster.loading({
    id,
    title: m.collection_toast_loading(),
    duration: 4000
  });
}

function toastSuccess(id: string) {
  toaster.success({
    id,
    title: m.collection_toast_success(),
    duration: 2000
  });
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

export const availableScales = scales as { id: string; display: string }[];

async function fetchCollection(query?: string) {
  isLoading.set(true);
  if (query !== undefined) {
    filters.update((f) => ({ ...f, query }));
  }
  try {
    const response = await commands.listCollectionItems(query ?? undefined);
    if (response.status === 'ok') {
      rawItems.set(response.data ?? []);
    } else {
      toastError(randomId());
    }
  } catch (e) {
    console.error(e);
    toastError(randomId());
  } finally {
    isLoading.set(false);
  }
}

function setQuery(query: string) {
  filters.update((f) => ({ ...f, query }));
}

function toggleTag(tag: string) {
  filters.update((f) => {
    const next = new Set(f.tags);
    if (next.has(tag)) next.delete(tag);
    else next.add(tag);
    return { ...f, tags: next };
  });
}

function setScale(scale: string | null) {
  filters.update((f) => ({ ...f, scale }));
}

function clearFilters() {
  filters.set({ query: '', scale: null, tags: new Set() });
}

async function createItem(input: CreateCollectionItemInput) {
  const toastId = randomId();
  const snapshot = get(rawItems);
  const tempItem: CollectionItemLite = {
    id: `temp-${toastId}`,
    createdAt: Date.now(),
    description: input.description ?? null,
    tags: input.tags ?? [],
    brand: input.brand,
    catalogNumber: input.catalogNumber,
    title: input.title,
    scale: input.scale,
    powerSystem: input.powerSystem
  };

  rawItems.update((items) => [...items, tempItem]);
  toastLoading(toastId);

  try {
    const response = await commands.createCollectionItem({
      brand: input.brand,
      catalogNumber: input.catalogNumber,
      title: input.title,
      scale: input.scale,
      powerSystem: input.powerSystem,
      description: input.description ?? null,
      tags: input.tags ?? []
    });
    if (response.status === 'ok') {
      const created = response.data;
      rawItems.update((items) => items.map((item) => (item.id === tempItem.id ? created : item)));
      toastSuccess(toastId);
      return created;
    }
    throw response.error;
  } catch (e) {
    console.error(e);
    rawItems.set(snapshot);
    toastError(toastId, () => {
      rawItems.set(snapshot);
      void createItem(input);
    });
    return null;
  }
}

async function updateItem(input: UpdateCollectionItemInput) {
  const toastId = randomId();
  const snapshot = get(rawItems);
  const prev = snapshot.find((i) => i.id === input.id);
  if (!prev) return null;

  const optimistic: CollectionItemLite = {
    ...prev,
    brand: input.brand,
    catalogNumber: input.catalogNumber,
    title: input.title,
    scale: input.scale,
    powerSystem: input.powerSystem,
    description: input.description ?? null,
    tags: input.tags ?? []
  };

  rawItems.update((items) => items.map((i) => (i.id === input.id ? optimistic : i)));
  toastLoading(toastId);

  try {
    const response = await commands.updateCollectionItem(input);
    if (response.status === 'ok') {
      rawItems.update((items) => items.map((i) => (i.id === input.id ? response.data : i)));
      toastSuccess(toastId);
      return response.data;
    }
    throw response.error;
  } catch (e) {
    console.error(e);
    rawItems.set(snapshot);
    toastError(toastId, () => void updateItem(input));
    return null;
  }
}

async function deleteItem(id: string) {
  const toastId = randomId();
  const snapshot = get(rawItems);
  rawItems.update((items) => items.filter((i) => i.id !== id));
  toastLoading(toastId);

  try {
    const response = await commands.deleteCollectionItem(id);
    if (response.status === 'ok') {
      toastSuccess(toastId);
      return true;
    }
    throw response.error;
  } catch (e) {
    console.error(e);
    rawItems.set(snapshot);
    toastError(toastId, () => void deleteItem(id));
    return false;
  }
}

export const collectionStore = {
  rawItems,
  filters,
  filteredItems,
  availableTags,
  totalCount,
  isLoading,
  fetchCollection,
  createItem,
  updateItem,
  deleteItem,
  setQuery,
  toggleTag,
  setScale,
  clearFilters,
  tagIcon
};
