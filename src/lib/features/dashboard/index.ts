/**
 * Dashboard Feature - Public API
 *
 * This module exports the public interface for the dashboard feature.
 */

// Services
export {
  DashboardService,
  setDashboardService,
  getDashboardService,
  // Legacy exports (deprecated)
  createDashboardState,
  setDashboardContext,
  getDashboardContext
} from './services/DashboardService.svelte';

// Components
export { default as DashboardCharts } from './components/DashboardCharts.svelte';
export { default as StatsCard } from './components/StatsCard.svelte';
export { default as RecentItemCard } from './components/RecentItemCard.svelte';
export { default as DepotView } from './components/DepotView.svelte';
export { default as DepotTable } from './components/DepotTable.svelte';
export { default as DepotListCard } from './components/DepotListCard.svelte';
export { default as ModelCard } from './components/ModelCard.svelte';
export { default as PurchaseGroupCard } from './components/PurchaseGroupCard.svelte';
