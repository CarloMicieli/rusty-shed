// Export feature module barrel
export { SearchService, setSearchContext, getSearchContext } from './SearchService.svelte.js';
export {
  searchRailwayModelIds,
  resolveRailwayModels,
  fetchRailwayModelImagePath
} from './ModelSearchService';
export { default as SearchResultCard } from './components/SearchResultCard.svelte';
export { default as SearchEmptyState } from './components/SearchEmptyState.svelte';
