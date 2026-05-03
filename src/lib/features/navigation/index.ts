/**
 * Navigation Feature - Public API
 *
 * This module exports the public interface for the navigation feature.
 */

// Services
export {
  NavigationService,
  setNavigationService,
  getNavigationService
} from './services/NavigationService.svelte';

// Types
export type { NavigationItem, NavigationConfig } from './types';
