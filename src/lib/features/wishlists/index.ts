/**
 * Wishlists Feature - Public API
 *
 * This module exports the public interface for the wishlists feature.
 */

// Services
export {
  WishlistService,
  setWishlistService,
  getWishlistService,
  type WishlistPreviewLite,
  type WishlistStateSnapshot,
  // Legacy exports (deprecated)
  createWishlistState,
  setWishlistContext,
  getWishlistContext
} from './services/WishlistService.svelte';

// Components
export { default as WishlistsDashboard } from './WishlistsDashboard.svelte';
export { default as AddWishlistItemModal } from '../../components/AddWishlistItemModal.svelte';
