import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import { SvelteSet } from 'svelte/reactivity';
import {
  type CollectionItemLite,
  type CreateCollectionItemInput,
  type UpdateCollectionItemInput
} from '$lib/bindings';
import { safeInvoke, getErrorMessage, isRetryableError } from '$lib/services';

export type FilterState = {
  query: string;
  scale: string | null;
  tags: SvelteSet<string>;
};

export const availableScales = scales as { id: string; display: string }[];

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

export class CollectionService {
  #rawItems = $state<CollectionItemLite[]>([]);
  #filters = $state<FilterState>({ query: '', scale: null, tags: new SvelteSet() });
  #isLoading = $state(false);

  availableTags = $derived.by(() => {
    const dynamic = new SvelteSet<string>();
    this.#rawItems.forEach((item) => item.tags?.forEach((tag) => dynamic.add(tag)));
    const combined = new SvelteSet<string>([...Object.keys(FIXED_TAG_META), ...dynamic]);
    return sortAvailableTags([...combined]);
  });

  filteredItems = $derived.by(() => {
    const { query, scale, tags } = this.#filters;
    const q = query.trim().toLowerCase();

    return this.#rawItems.filter((item) => {
      if (scale && item.scale !== scale) return false;
      if (tags.size) {
        const hasTag = item.tags.some((tag) => tags.has(tag));
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

  totalCount = $derived(this.#rawItems.length);

  get rawItems() {
    return this.#rawItems;
  }

  get filters() {
    return this.#filters;
  }

  get isLoading() {
    return this.#isLoading;
  }

  fetchCollection = async (query?: string) => {
    this.#isLoading = true;
    if (query !== undefined) {
      this.#filters.query = query;
    }

    try {
      const result = await safeInvoke<CollectionItemLite[]>('list_collection_items', {
        query: query ?? null
      });

      if (!result.ok) {
        console.error('Failed to fetch collection items:', result.error);
        toastError(randomId(), getErrorMessage(result.error));
        return;
      }

      this.#rawItems = result.data ?? [];
    } finally {
      this.#isLoading = false;
    }
  };

  setQuery = (query: string) => {
    this.#filters.query = query;
  };

  toggleTag = (tag: string) => {
    const next = new SvelteSet(this.#filters.tags);
    if (next.has(tag)) next.delete(tag);
    else next.add(tag);
    this.#filters.tags = next;
  };

  setScale = (scale: string | null) => {
    this.#filters.scale = scale;
  };

  clearFilters = () => {
    this.#filters = { query: '', scale: null, tags: new SvelteSet() };
  };

  createItem = async (input: CreateCollectionItemInput) => {
    const toastId = randomId();
    const snapshot = [...this.#rawItems];
    const tempItem: CollectionItemLite = {
      id: `temp-${toastId}`,
      // eslint-disable-next-line svelte/prefer-svelte-reactivity
      createdAt: new Date().toISOString(),
      description: input.description ?? null,
      tags: input.tags ?? [],
      brand: input.brand,
      catalogNumber: input.catalogNumber,
      title: input.title,
      scale: input.scale,
      powerSystem: input.powerSystem
    };

    this.#rawItems = [...this.#rawItems, tempItem];
    toastLoading(toastId);

    const result = await safeInvoke<CollectionItemLite>('create_collection_item', {
      input: {
        brand: input.brand,
        catalogNumber: input.catalogNumber,
        title: input.title,
        scale: input.scale,
        powerSystem: input.powerSystem,
        description: input.description ?? null,
        tags: input.tags ?? []
      }
    });

    if (!result.ok) {
      console.error('Failed to create collection item:', result.error);
      this.#rawItems = snapshot;
      const retry = isRetryableError(result.error)
        ? () => {
            this.#rawItems = snapshot;
            void this.createItem(input);
          }
        : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return null;
    }

    this.#rawItems = this.#rawItems.map((item) => (item.id === tempItem.id ? result.data : item));
    toastSuccess(toastId);
    return result.data;
  };

  updateItem = async (input: UpdateCollectionItemInput) => {
    const toastId = randomId();
    const snapshot = [...this.#rawItems];
    const prev = this.#rawItems.find((i) => i.id === input.id);
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

    this.#rawItems = this.#rawItems.map((i) => (i.id === input.id ? optimistic : i));
    toastLoading(toastId);

    const result = await safeInvoke<CollectionItemLite>('update_collection_item', { input });

    if (!result.ok) {
      console.error('Failed to update collection item:', result.error);
      this.#rawItems = snapshot;
      const retry = isRetryableError(result.error) ? () => void this.updateItem(input) : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return null;
    }

    this.#rawItems = this.#rawItems.map((i) => (i.id === input.id ? result.data : i));
    toastSuccess(toastId);
    return result.data;
  };

  deleteItem = async (id: string) => {
    const toastId = randomId();
    const snapshot = [...this.#rawItems];

    this.#rawItems = this.#rawItems.filter((i) => i.id !== id);
    toastLoading(toastId);

    const result = await safeInvoke<void>('delete_collection_item', { id });

    if (!result.ok) {
      console.error('Failed to delete collection item:', result.error);
      this.#rawItems = snapshot;
      const retry = isRetryableError(result.error) ? () => void this.deleteItem(id) : undefined;
      toastError(toastId, getErrorMessage(result.error), retry);
      return false;
    }

    toastSuccess(toastId);
    return true;
  };
}

export const collectionService = new CollectionService();
export { tagIcon };
