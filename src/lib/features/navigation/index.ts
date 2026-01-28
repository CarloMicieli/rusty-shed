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

// Components
export { default as SidebarNavigation } from './components/SidebarNavigation.svelte';
export { default as BottomNavigation } from './components/BottomNavigation.svelte';

// Types
export type { NavigationItem, NavigationConfig } from './types';
