import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import scales from '$lib/data/constants/scales.json';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import { SvelteSet } from 'svelte/reactivity';
import type {
  CollectionView,
  AddRailwayModelToCollectionArgs,
  ReceivePreorderArgs
} from '$lib/bindings';
import { commands } from '$lib/bindings';
import { safeCommand, getErrorMessage } from '$lib/services';
import { collectionStore } from '$lib/state/collection.svelte';

export type StatusFilter = 'active' | 'preordered' | 'sold' | 'all';

export type FilterState = {
  query: string;
  /** @deprecated use `scales` multi-select instead */
  scale: string | null;
  scales: SvelteSet<string>;
  companies: SvelteSet<string>;
  categories: SvelteSet<string>;
  epochs: SvelteSet<string>;
  tags: SvelteSet<string>;
  status: StatusFilter;
};

export const availableScales = scales as { id: string; display: string }[];

function randomId() {
  if (typeof crypto !== 'undefined' && crypto.randomUUID) return crypto.randomUUID();
  return Math.random().toString(36).slice(2);
}

function toastError(id: string, message?: string) {
  // Note: Sonner API uses the first parameter as message, not an object
  // The id parameter is kept for backward compatibility but not used
  toaster.error(message || m.collection_toast_error(), { duration: 5000 });
}

/**
 * CollectionState manages the collection feature state and operations.
 *
 * Currently only supports fetching the collection.
 * CRUD operations will be added when backend commands are implemented.
 */
export class CollectionState {
  #collection = $state<CollectionView | null>(null);
  #filters = $state<FilterState>({
    query: '',
    scale: null,
    scales: new SvelteSet(),
    companies: new SvelteSet(),
    categories: new SvelteSet(),
    epochs: new SvelteSet(),
    tags: new SvelteSet(),
    status: 'active'
  });
  #isLoading = $state(false);

  availableTags = $derived.by(() => {
    const dynamic = new SvelteSet<string>();
    // Note: CollectionItemView doesn't have tags field in bindings
    // This will need to be updated when the backend adds tag support
    const combined = new SvelteSet<string>([...Object.keys(FIXED_TAG_META), ...dynamic]);
    return sortAvailableTags([...combined]);
  });

  /** Unique Scale values present in the current collection, sorted alphabetically */
  availableScaleIds = $derived.by((): string[] => {
    const items = this.#collection?.items ?? [];
    const seen = new SvelteSet<string>();
    for (const item of items) {
      if (item.railwayModel.scale) seen.add(item.railwayModel.scale);
    }
    return [...seen].sort();
  });

  /** Unique railway company names present in the current collection, sorted alphabetically */
  availableCompanies = $derived.by((): string[] => {
    const items = this.#collection?.items ?? [];
    const seen = new SvelteSet<string>();
    for (const item of items) {
      for (const rs of item.rollingStocks) {
        if (rs.railwayCompanyName) seen.add(rs.railwayCompanyName);
      }
    }
    return [...seen].sort();
  });

  /** Unique Category values present in the current collection, sorted alphabetically */
  availableCategories = $derived.by((): string[] => {
    const items = this.#collection?.items ?? [];
    const seen = new SvelteSet<string>();
    for (const item of items) {
      if (item.railwayModel.category) seen.add(item.railwayModel.category);
    }
    return [...seen].sort();
  });

  /** Unique Epoch values present in the current collection, sorted alphabetically */
  availableEpochs = $derived.by((): string[] => {
    const items = this.#collection?.items ?? [];
    const seen = new SvelteSet<string>();
    for (const item of items) {
      if (item.railwayModel.epoch) seen.add(item.railwayModel.epoch);
    }
    return [...seen].sort();
  });

  /** True when at least one filter dimension is active */
  get hasActiveFilters(): boolean {
    const f = this.#filters;
    return (
      f.query !== '' ||
      f.scales.size > 0 ||
      f.companies.size > 0 ||
      f.categories.size > 0 ||
      f.epochs.size > 0 ||
      f.tags.size > 0 ||
      f.status !== 'active'
    );
  }

  filteredItems = $derived.by(() => {
    const items = this.#collection?.items ?? [];
    const { query, scales, companies, categories, epochs, status } = this.#filters;
    const q = query.trim().toLowerCase();

    return items.filter((item) => {
      // Status filter
      const isPreorder = item.purchaseInfo?.kind === 'preOrdered';
      const isSold = item.removedDate !== null && item.purchaseInfo?.kind === 'sold';
      const isActive = item.removedDate === null && !isPreorder;

      if (status === 'active' && !isActive) return false;
      if (status === 'preordered' && !isPreorder) return false;
      if (status === 'sold' && !isSold) return false;
      // 'all' passes every item through

      if (scales.size > 0 && !scales.has(item.railwayModel.scale)) return false;
      if (categories.size > 0 && !categories.has(item.railwayModel.category)) return false;
      if (epochs.size > 0 && !epochs.has(item.railwayModel.epoch)) return false;
      if (companies.size > 0) {
        const itemCompanies = item.rollingStocks
          .map((rs) => rs.railwayCompanyName)
          .filter((c): c is string => c !== null);
        if (!itemCompanies.some((c) => companies.has(c))) return false;
      }
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
    if (this.#collection !== null) return; // cache hit — avoid redundant IPC on back-navigation
    await this.forceRefresh();
  };

  forceRefresh = async () => {
    this.#isLoading = true;

    try {
      const result = await safeCommand(commands.getCollection());

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
    // Legacy single-select: sync to multi-select `scales` set
    if (scale === null) {
      this.#filters.scales = new SvelteSet();
    } else {
      this.#filters.scales = new SvelteSet([scale]);
    }
    this.#filters.scale = scale;
  };

  toggleScale = (scale: string) => {
    const next = new SvelteSet(this.#filters.scales);
    if (next.has(scale)) next.delete(scale);
    else next.add(scale);
    this.#filters.scales = next;
    // Keep legacy field in sync (null = all, first value otherwise)
    this.#filters.scale = next.size === 0 ? null : [...next][0];
  };

  toggleCompany = (company: string) => {
    const next = new SvelteSet(this.#filters.companies);
    if (next.has(company)) next.delete(company);
    else next.add(company);
    this.#filters.companies = next;
  };

  toggleCategory = (category: string) => {
    const next = new SvelteSet(this.#filters.categories);
    if (next.has(category)) next.delete(category);
    else next.add(category);
    this.#filters.categories = next;
  };

  toggleEpoch = (epoch: string) => {
    const next = new SvelteSet(this.#filters.epochs);
    if (next.has(epoch)) next.delete(epoch);
    else next.add(epoch);
    this.#filters.epochs = next;
  };

  clearFilters = () => {
    this.#filters = {
      query: '',
      scale: null,
      scales: new SvelteSet(),
      companies: new SvelteSet(),
      categories: new SvelteSet(),
      epochs: new SvelteSet(),
      tags: new SvelteSet(),
      status: 'active'
    };
  };

  setStatus = (status: StatusFilter) => {
    this.#filters.status = status;
  };

  /**
   * Add a railway model to the collection.
   * @param args - The railway model data and purchase information
   * @returns true if successful, false otherwise
   */
  addRailwayModel = async (args: AddRailwayModelToCollectionArgs): Promise<boolean> => {
    const result = await safeCommand(commands.addRailwayModelToCollection(args));

    if (result.ok) {
      toaster.success(m.add_model_success(), { duration: 3000 });
      await this.forceRefresh();
      void collectionStore.refresh();
      return true;
    }

    toastError(randomId(), getErrorMessage(result.error));
    return false;
  };

  /**
   * Mark a preorder item as received.
   * @returns true if successful, false otherwise
   */
  receivePreorder = async (args: ReceivePreorderArgs): Promise<boolean> => {
    const result = await safeCommand(commands.receivePreorder(args));

    if (result.ok) {
      toaster.success(m.collection_item_receive_action(), { duration: 3000 });
      await this.forceRefresh();
      void collectionStore.refresh();
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
    try {
      // Find the item to get its category
      const item = this.#collection?.items.find((i) => i.id === id);
      if (!item) {
        toaster.error('Item not found');
        return null;
      }

      // Use today's date as removed date
      // eslint-disable-next-line svelte/prefer-svelte-reactivity
      const removedDate = new Date().toISOString().split('T')[0];

      const result = await safeCommand(
        commands.removeCollectionItem({
          collectionItemId: id,
          category: item.railwayModel.category,
          removedDate
        })
      );

      if (!result.ok) {
        const errorMessage = getErrorMessage(result.error);
        toaster.error(`Failed to remove item: ${errorMessage}`);
        return null;
      }

      // Refresh collection after successful deletion
      await this.forceRefresh();
      void collectionStore.refresh();
      toaster.success('Item removed from collection');
      return result.data;
    } catch (error) {
      console.error('Error removing collection item:', error);
      toaster.error('Failed to remove item');
      return null;
    }
  };
}

/** Singleton instance — import this directly instead of using setContext/getContext. */
export const collectionState = new CollectionState();

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
