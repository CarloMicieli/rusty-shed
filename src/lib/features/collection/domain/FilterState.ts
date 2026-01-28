/**
 * Collection domain types and models.
 */

import { SvelteSet } from 'svelte/reactivity';

/**
 * Filter state for collection items.
 */
export interface FilterState {
  /**
   * Text query for searching items.
   */
  query: string;

  /**
   * Selected scale filter (null = all scales).
   */
  scale: string | null;

  /**
   * Selected tag filters.
   */
  tags: SvelteSet<string>;
}

/**
 * Create an empty filter state.
 */
export function createEmptyFilterState(): FilterState {
  return {
    query: '',
    scale: null,
    tags: new SvelteSet()
  };
}

/**
 * Check if filters are active (any non-default values).
 */
export function hasActiveFilters(filters: FilterState): boolean {
  return filters.query.trim() !== '' || filters.scale !== null || filters.tags.size > 0;
}
