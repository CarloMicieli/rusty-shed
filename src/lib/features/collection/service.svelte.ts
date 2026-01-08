import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import { SvelteSet } from 'svelte/reactivity';
import type { CollectionView } from '$lib/bindings';
import { safeInvoke, getErrorMessage } from '$lib/services';

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

function toastError(id: string, message?: string) {
  toaster.error({
    id,
    title: message || m.collection_toast_error(),
    duration: 5000
  });
}

/**
 * Read-only CollectionService
 *
 * Currently only supports fetching the collection.
 * CRUD operations will be added when backend commands are implemented.
 */
export class CollectionService {
  #collection = $state<CollectionView | null>(null);
  #filters = $state<FilterState>({ query: '', scale: null, tags: new SvelteSet() });
  #isLoading = $state(false);

  availableTags = $derived.by(() => {
    const dynamic = new SvelteSet<string>();
    // Note: CollectionItemView doesn't have tags field in bindings
    // This will need to be updated when the backend adds tag support
    const combined = new SvelteSet<string>([...Object.keys(FIXED_TAG_META), ...dynamic]);
    return sortAvailableTags([...combined]);
  });

  filteredItems = $derived.by(() => {
    const items = this.#collection?.items ?? [];
    const { query, scale } = this.#filters;
    const q = query.trim().toLowerCase();

    return items.filter((item) => {
      if (scale && item.railway_model.scale !== scale) return false;
      // Tag filtering will be added when backend supports it
      if (q) {
        const haystack =
          `${item.railway_model.manufacturer} ${item.railway_model.product_code} ${item.railway_model.description}`.toLowerCase();
        if (!haystack.includes(q)) return false;
      }
      return true;
    });
  });

  totalCount = $derived(this.#collection?.items.length ?? 0);

  get collection() {
    return this.#collection;
  }

  get rawItems() {
    return this.#collection?.items ?? [];
  }

  get filters() {
    return this.#filters;
  }

  get isLoading() {
    return this.#isLoading;
  }

  fetchCollection = async () => {
    this.#isLoading = true;

    try {
      const result = await safeInvoke<CollectionView>('get_collection');

      if (!result.ok) {
        console.error('Failed to fetch collection:', result.error);
        toastError(randomId(), getErrorMessage(result.error));
        return;
      }

      this.#collection = result.data;
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

  // CRUD operations commented out - will be implemented when backend commands are available
  /*
  createItem = async (input: CreateCollectionItemInput) => {
    // TODO: Implement when add_collection_item command is available
  };

  updateItem = async (input: UpdateCollectionItemInput) => {
    // TODO: Implement when update_collection_item command is available
  };

  deleteItem = async (id: string) => {
    // TODO: Implement when delete_collection_item command is available
  };
  */
}

export const collectionService = new CollectionService();
export { tagIcon };
