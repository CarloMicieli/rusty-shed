/**
 * Collection Service - Manages collection state and operations.
 *
 * This service provides:
 * - Collection data fetching and caching
 * - Filtering and search
 * - Tag management
 * - CRUD operations (when backend supports them)
 */

import { setContext, getContext } from 'svelte';
import { SvelteSet } from 'svelte/reactivity';
import { toaster } from '$lib/toaster';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { getErrorMessage } from '$lib/shared/domain/errors';
import { FIXED_TAG_META, sortAvailableTags, tagIcon } from '$lib/config/tags';
import scales from '$lib/data/constants/scales.json';
import type { CollectionView } from '$lib/bindings';
import { createEmptyFilterState, type FilterState } from '../domain/FilterState';

export const availableScales = scales as { id: string; display: string }[];

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('collection-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class CollectionService {
  // Private reactive state
  #collection = $state<CollectionView | null>(null);
  #filters = $state<FilterState>(createEmptyFilterState());
  #isLoading = $state(false);

  // Public readonly getters (defensive encapsulation)
  get collection(): CollectionView | null {
    return this.#collection;
  }

  get summary() {
    return this.#collection?.summary;
  }

  get rawItems() {
    return this.#collection?.items ?? [];
  }

  get filters(): FilterState {
    return this.#filters;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  // Derived state
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
      // Scale filter
      if (scale && item.railwayModel.scale !== scale) {
        return false;
      }

      // Tag filtering (to be implemented when backend supports it)
      // if (this.#filters.tags.size > 0) {
      //   const itemTags = new Set(item.tags || []);
      //   const hasAllTags = [...this.#filters.tags].every(tag => itemTags.has(tag));
      //   if (!hasAllTags) return false;
      // }

      // Text query filter
      if (q) {
        const manufacturer =
          typeof item.railwayModel.manufacturer === 'object'
            ? ((item.railwayModel.manufacturer as { name?: string }).name ?? '')
            : item.railwayModel.manufacturer;
        const haystack =
          `${manufacturer} ${item.railwayModel.productCode} ${item.railwayModel.description}`.toLowerCase();
        if (!haystack.includes(q)) {
          return false;
        }
      }

      return true;
    });
  });

  totalCount = $derived(this.#collection?.items.length ?? 0);
  filteredCount = $derived(this.filteredItems.length);

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Fetch the entire collection from the backend.
   *
   * @param _query - Optional query parameter (reserved for future use)
   */
  async fetchCollection(_query?: string): Promise<void> {
    this.#isLoading = true;

    try {
      const result = await safeInvoke<CollectionView>('get_collection');

      if (!result.ok) {
        console.error('Failed to fetch collection:', result.error);
        toaster.error({
          id: crypto.randomUUID(),
          title: getErrorMessage(result.error),
          duration: 5000
        });
        return;
      }

      this.#collection = result.data;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Set the text query filter.
   *
   * @param query - The search query
   */
  setQuery(query: string): void {
    this.#filters.query = query;
  }

  /**
   * Toggle a tag filter on/off.
   *
   * @param tag - The tag to toggle
   */
  toggleTag(tag: string): void {
    const next = new SvelteSet(this.#filters.tags);
    if (next.has(tag)) {
      next.delete(tag);
    } else {
      next.add(tag);
    }
    this.#filters.tags = next;
  }

  /**
   * Set the scale filter.
   *
   * @param scale - The scale to filter by (null = all scales)
   */
  setScale(scale: string | null): void {
    this.#filters.scale = scale;
  }

  /**
   * Clear all active filters.
   */
  clearFilters(): void {
    this.#filters = createEmptyFilterState();
  }

  // ─────────────────────────────────────────────────────────────
  // CRUD OPERATIONS (Placeholder - to be implemented)
  // ─────────────────────────────────────────────────────────────

  /**
   * Create a new collection item.
   *
   * @param input - The item data
   * @returns The created item ID or null if failed
   */
  async createItem(input: unknown): Promise<string | null> {
    console.warn('CollectionService.createItem not implemented yet', input);
    return null;
  }

  /**
   * Update an existing collection item.
   *
   * @param input - The updated item data
   * @returns True if successful
   */
  async updateItem(input: unknown): Promise<boolean> {
    console.warn('CollectionService.updateItem not implemented yet', input);
    return false;
  }

  /**
   * Delete a collection item by ID.
   *
   * @param id - The item ID to delete
   * @returns True if successful
   */
  async deleteItem(id: string): Promise<boolean> {
    console.warn('CollectionService.deleteItem not implemented yet', id);
    return false;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the CollectionService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setCollectionService(service?: CollectionService): CollectionService {
  const instance = service ?? new CollectionService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the CollectionService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getCollectionService(): CollectionService {
  const service = getContext<CollectionService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'CollectionService not found in context. Did you call setCollectionService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use setCollectionService() instead
 */
export function createCollectionState(): CollectionService {
  console.warn('createCollectionState is deprecated. Use setCollectionService() instead.');
  return new CollectionService();
}

/**
 * @deprecated Use setCollectionService() instead
 */
export function setCollectionContext(state: CollectionService): void {
  console.warn('setCollectionContext is deprecated. Use setCollectionService() instead.');
  setContext(SERVICE_KEY, state);
}

/**
 * @deprecated Use getCollectionService() instead
 */
export function getCollectionContext(): CollectionService {
  console.warn('getCollectionContext is deprecated. Use getCollectionService() instead.');
  return getCollectionService();
}

// Re-export tagIcon utility
export { tagIcon };
