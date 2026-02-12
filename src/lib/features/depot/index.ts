/**
 * Depot Feature - Public API
 *
 * This module exports the public interface for the depot feature.
 */

// Services
export {
  DepotService,
  setDepotService,
  getDepotService,
  // Legacy exports (deprecated)
  createDepotState,
  setDepotContext,
  getDepotContext
} from './services/DepotService.svelte';

// Components (depot-specific components for the depot page)
export { default as DepotTable } from './components/DepotTable.svelte';
