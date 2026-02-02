/**
 * Track Inventory Feature Module
 *
 * Exports services, context functions, and types for track inventory management
 */

export {
  TrackInventoryService,
  setTrackInventoryContext,
  getTrackInventoryContext
} from './services/TrackInventoryService.svelte';

// Re-export types from bindings for convenience
export type {
  TrackInventoryListItem,
  TrackInventoryView,
  TrackInventoryItemView,
  TrackProductView,
  TrackPurchaseView,
  NewTrackInventoryArgs,
  RenameTrackInventoryArgs,
  AddTrackPurchaseArgs
} from '$lib/bindings';
