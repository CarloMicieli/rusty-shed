import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import { SvelteSet } from 'svelte/reactivity';
import type { CollectionView, AddRailwayModelToCollectionArgs } from '$lib/bindings';
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
 * CollectionState manages the collection feature state and operations.
 *
 * Currently only supports fetching the collection.
 * CRUD operations will be added when backend commands are implemented.
 */
export class CollectionState {
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
      if (scale && item.railwayModel.scale !== scale) return false;
      // Tag filtering will be added when backend supports it
      if (q) {
        const manufacturer =
          typeof item.railwayModel.manufacturer === 'object'
            ? ((item.railwayModel.manufacturer as { name?: string }).name ?? '')
            : item.railwayModel.manufacturer;
        const haystack =
          `${manufacturer} ${item.railwayModel.productCode} ${item.railwayModel.description}`.toLowerCase();
        if (!haystack.includes(q)) return false;
      }
      return true;
    });
  });

  totalCount = $derived(this.#collection?.items.length ?? 0);

  get summary() {
    return this.#collection?.summary;
  }

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

  fetchCollection = async (_query?: string) => {
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

  /**
   * Add a railway model to the collection.
   * @param args - The railway model data and purchase information
   * @returns true if successful, false otherwise
   */
  addRailwayModel = async (args: AddRailwayModelToCollectionArgs): Promise<boolean> => {
    const result = await safeInvoke('add_railway_model_to_collection', { args });

    if (result.ok) {
      toaster.success({
        id: randomId(),
        title: m.add_model_success(),
        duration: 3000
      });
      await this.fetchCollection();
      return true;
    }

    toastError(randomId(), getErrorMessage(result.error));
    return false;
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
  // Placeholder CRUD methods (no-op) so UI can call them safely until backend is implemented
  createItem = async (input: unknown) => {
    console.warn('createItem not implemented yet', input);
    return null;
  };

  updateItem = async (input: unknown) => {
    console.warn('updateItem not implemented yet', input);
    return null;
  };

  deleteItem = async (id: string) => {
    console.warn('deleteItem not implemented yet', id);
    return null;
  };
}

const COLLECTION_CONTEXT_KEY = Symbol('collection-context');

export function createCollectionState() {
  return new CollectionState();
}

export function setCollectionContext(state: CollectionState) {
  setContext(COLLECTION_CONTEXT_KEY, state);
}

export function getCollectionContext(): CollectionState {
  const state = getContext<CollectionState>(COLLECTION_CONTEXT_KEY);
  if (!state) {
    throw new Error(
      'CollectionContext not provided. Ensure component is within a CollectionContext provider.'
    );
  }
  return state;
}

export { tagIcon };
