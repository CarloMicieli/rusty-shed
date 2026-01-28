/**
 * Collection Feature - Public API
 *
 * This module exports the public interface for the collection feature.
 */

// Services
export {
  CollectionService,
  setCollectionService,
  getCollectionService,
  availableScales,
  tagIcon,
  // Legacy exports (deprecated)
  createCollectionState,
  setCollectionContext,
  getCollectionContext
} from './services/CollectionService.svelte';

// Components
export { default as CollectionDashboard } from './CollectionDashboard.svelte';

// Domain
export { createEmptyFilterState, hasActiveFilters, type FilterState } from './domain/FilterState';
