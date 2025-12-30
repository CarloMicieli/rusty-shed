import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import { SvelteSet } from 'svelte/reactivity';
import {
  commands,
  type CollectionItemLite,
  type CreateCollectionItemInput,
  type UpdateCollectionItemInput
} from '$lib/bindings';

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

class CollectionStore {
  rawItems = $state<CollectionItemLite[]>([]);
  filters = $state<FilterState>({ query: '', scale: null, tags: new SvelteSet() });
  isLoading = $state(false);

  availableTags = $derived.by(() => {
    const dynamic = new SvelteSet<string>();
    this.rawItems.forEach((item) => item.tags?.forEach((tag) => dynamic.add(tag)));
    const combined = new SvelteSet<string>([...Object.keys(FIXED_TAG_META), ...dynamic]);
    return sortAvailableTags([...combined]);
  });

  filteredItems = $derived.by(() => {
    const { query, scale, tags } = this.filters;
    const q = query.trim().toLowerCase();

    return this.rawItems.filter((item) => {
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

  totalCount = $derived(this.rawItems.length);

  fetchCollection = async (query?: string) => {
    this.isLoading = true;
    if (query !== undefined) {
      this.filters.query = query;
    }

    try {
      const response = await commands.listCollectionItems(query ?? null);
      if (response.status === 'ok') {
        this.rawItems = response.data ?? [];
      } else {
        toastError(randomId());
      }
    } catch (e) {
      console.error(e);
      toastError(randomId());
    } finally {
      this.isLoading = false;
    }
  };

  setQuery = (query: string) => {
    this.filters.query = query;
  };

  toggleTag = (tag: string) => {
    const next = new SvelteSet(this.filters.tags);
    if (next.has(tag)) next.delete(tag);
    else next.add(tag);
    this.filters.tags = next;
  };

  setScale = (scale: string | null) => {
    this.filters.scale = scale;
  };

  clearFilters = () => {
    this.filters = { query: '', scale: null, tags: new SvelteSet() };
  };

  createItem = async (input: CreateCollectionItemInput) => {
    const toastId = randomId();
    const snapshot = [...this.rawItems];
    const tempItem: CollectionItemLite = {
      id: `temp-${toastId}`,
      createdAt: new Date().toISOString(),
      description: input.description ?? null,
      tags: input.tags ?? [],
      brand: input.brand,
      catalogNumber: input.catalogNumber,
      title: input.title,
      scale: input.scale,
      powerSystem: input.powerSystem
    };

    this.rawItems = [...this.rawItems, tempItem];
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
        this.rawItems = this.rawItems.map((item) => (item.id === tempItem.id ? created : item));
        toastSuccess(toastId);
        return created;
      }

      throw response.error;
    } catch (e) {
      console.error(e);
      this.rawItems = snapshot;
      toastError(toastId, () => {
        this.rawItems = snapshot;
        void this.createItem(input);
      });
      return null;
    }
  };

  updateItem = async (input: UpdateCollectionItemInput) => {
    const toastId = randomId();
    const snapshot = [...this.rawItems];
    const prev = this.rawItems.find((i) => i.id === input.id);
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

    this.rawItems = this.rawItems.map((i) => (i.id === input.id ? optimistic : i));
    toastLoading(toastId);

    try {
      const response = await commands.updateCollectionItem(input);
      if (response.status === 'ok') {
        this.rawItems = this.rawItems.map((i) => (i.id === input.id ? response.data : i));
        toastSuccess(toastId);
        return response.data;
      }

      throw response.error;
    } catch (e) {
      console.error(e);
      this.rawItems = snapshot;
      toastError(toastId, () => void this.updateItem(input));
      return null;
    }
  };

  deleteItem = async (id: string) => {
    const toastId = randomId();
    const snapshot = [...this.rawItems];

    this.rawItems = this.rawItems.filter((i) => i.id !== id);
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
      this.rawItems = snapshot;
      toastError(toastId, () => void this.deleteItem(id));
      return false;
    }
  };
}

export const collectionStore = new CollectionStore();
export { tagIcon };
