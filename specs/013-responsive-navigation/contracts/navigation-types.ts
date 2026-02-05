/**
 * Type definitions for the Responsive Navigation System
 *
 * This file contains all TypeScript interfaces and types used across
 * navigation components (SidebarNavigation, BottomNavigation, MoreMenu).
 *
 * @module navigation/types
 */

import type { ComponentType } from 'svelte';

/**
 * Represents a single navigation feature in the application.
 * Used across desktop sidebar, mobile bottom bar, and More menu contexts.
 */
export interface NavigationItem {
  /**
   * Unique identifier for the navigation item.
   * Used for key props and debugging.
   *
   * @example 'home', 'collection', 'finance', 'wishlists'
   */
  id: string;

  /**
   * Paraglide message function that returns the localized label.
   * Function signature ensures reactive updates when locale changes.
   *
   * @example () => m.app_home()
   * @returns Localized string for the navigation label
   */
  label: () => string;

  /**
   * lucide-svelte icon component to display.
   * Must be a Svelte component type from lucide-svelte.
   *
   * @example LayoutDashboard, TrainFront, Wallet, Heart
   */
  icon: ComponentType;

  /**
   * Route path for navigation (SvelteKit route).
   * Must be an absolute path starting with '/'.
   *
   * @example '/my-dashboard', '/my-collection', '/my-budget'
   */
  href: string;

  /**
   * Determines if item appears in mobile bottom bar (primary) or More menu (secondary).
   * - true: Item appears in mobile 5-slot bottom bar
   * - false: Item appears in More menu (secondary features)
   *
   * Desktop sidebar always shows all items regardless of this flag.
   */
  isPrimary: boolean;

  /**
   * Optional badge count to display next to the label.
   * Used for features that have countable items (e.g., wishlist count).
   *
   * @example 5 (for 5 items in wishlist)
   * @default undefined (no badge shown)
   */
  badgeCount?: number;

  /**
   * Optional flag to use prefix matching for active state detection.
   * - true: Active if current route starts with `href`
   * - false: Active only if current route exactly matches `href`
   *
   * Useful for features with subroutes (e.g., /my-tracks, /my-tracks/all, /my-tracks/n-scale).
   *
   * @default false (uses exact match)
   */
  usePrefixMatch?: boolean;
}

/**
 * Props for the MoreMenu component (bottom sheet for secondary features).
 */
export interface MoreMenuProps {
  /**
   * Controls the visibility of the More menu.
   * When true, the bottom sheet is open.
   */
  open: boolean;

  /**
   * Callback function to close the More menu.
   * Called when user taps backdrop, presses ESC, or selects a navigation item.
   */
  onClose: () => void;

  /**
   * List of secondary navigation items to display in the More menu.
   * Should be filtered from NAVIGATION_ITEMS where isPrimary === false.
   */
  items: NavigationItem[];
}

/**
 * Props for a reusable NavigationItem component (optional).
 * Can be used to extract common rendering logic for navigation links.
 */
export interface NavigationItemProps {
  /**
   * The navigation item data to render.
   */
  item: NavigationItem;

  /**
   * Whether this item is currently active (matches current route).
   */
  isActive: boolean;

  /**
   * Optional click handler for custom behavior (e.g., close menu on click).
   */
  onClick?: () => void;

  /**
   * Optional CSS class name for custom styling.
   */
  class?: string;
}

/**
 * Helper type for active state detection function.
 */
export type IsActiveFunction = (item: NavigationItem, pathname: string) => boolean;

/**
 * Type for the centralized navigation configuration.
 */
export type NavigationConfig = NavigationItem[];
