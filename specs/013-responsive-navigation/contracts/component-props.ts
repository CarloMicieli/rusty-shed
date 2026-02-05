/**
 * Component Prop Interfaces - Responsive Navigation System
 *
 * This file documents the expected props for each component in the navigation system.
 * These are reference contracts for implementation; actual Svelte components may use
 * `$props()` or traditional `export let` syntax.
 *
 * @module navigation/component-props
 */

import type { NavigationItem } from './navigation-types';

/**
 * Props for SidebarNavigation.svelte
 *
 * Desktop-only navigation sidebar that displays all 9 features in a vertical list.
 * Visible at md breakpoint (≥768px) and above.
 */
export interface SidebarNavigationProps {
  /**
   * Optional CSS class name for the sidebar container.
   * Allows customization of sidebar styles from parent components.
   *
   * @default undefined
   */
  class?: string;

  /**
   * Optional flag to control sidebar visibility programmatically.
   * Overrides default responsive behavior (hidden on mobile, visible on desktop).
   *
   * Use case: Feature flags, A/B testing, or custom sidebar toggle behavior.
   *
   * @default undefined (uses responsive CSS: hidden lg:flex)
   */
  isVisible?: boolean;
}

/**
 * Props for BottomNavigation.svelte
 *
 * Mobile-only bottom navigation bar with 5 slots:
 * - 4 primary features (Home, Collection, Finance, Wishlists)
 * - 1 More button (opens secondary features menu)
 *
 * Visible below md breakpoint (<768px).
 */
export interface BottomNavigationProps {
  /**
   * Optional CSS class name for the bottom bar container.
   * Allows customization of bottom bar styles from parent components.
   *
   * @default undefined
   */
  class?: string;

  /**
   * Optional flag to control bottom bar visibility programmatically.
   * Overrides default responsive behavior (visible on mobile, hidden on desktop).
   *
   * Use case: Feature flags, A/B testing, or custom navigation patterns.
   *
   * @default undefined (uses responsive CSS: lg:hidden)
   */
  isVisible?: boolean;
}

/**
 * Props for MoreMenu.svelte
 *
 * Bottom sheet/drawer component that displays secondary navigation features.
 * Opened by tapping the "More" button in the mobile bottom navigation bar.
 */
export interface MoreMenuProps {
  /**
   * Controls the visibility of the More menu.
   * When true, the bottom sheet is open and visible.
   *
   * @required
   */
  open: boolean;

  /**
   * Callback function invoked when the menu should close.
   * Called in these scenarios:
   * - User taps a secondary navigation item (after navigation)
   * - User taps the backdrop overlay
   * - User presses ESC key
   * - User swipes down on the sheet (if gesture enabled)
   *
   * @required
   * @example () => { moreMenuOpen = false; }
   */
  onClose: () => void;

  /**
   * List of secondary navigation items to display in the menu.
   * Should be filtered from NAVIGATION_ITEMS where isPrimary === false.
   *
   * Expected items (4 total):
   * - Maintenance
   * - Depot
   * - Digital (DCC)
   * - Railway Tracks
   *
   * @required
   */
  items: NavigationItem[];

  /**
   * Optional CSS class name for the sheet content container.
   * Allows customization of menu styles.
   *
   * @default undefined
   */
  class?: string;
}

/**
 * Props for NavigationItem.svelte (optional reusable component)
 *
 * Reusable component for rendering a single navigation link.
 * Can be used in SidebarNavigation, BottomNavigation, and MoreMenu
 * to reduce code duplication.
 *
 * Note: This component is optional. Teams may choose to inline navigation
 * item rendering in each context for more flexibility.
 */
export interface NavigationItemProps {
  /**
   * The navigation item data to render.
   * Contains all information needed to render the link: label, icon, href, etc.
   *
   * @required
   */
  item: NavigationItem;

  /**
   * Whether this item is currently active (matches current route).
   * Used to apply active state styling.
   *
   * @required
   * @example isActive(item, $page.url.pathname)
   */
  isActive: boolean;

  /**
   * Optional click handler for custom behavior.
   *
   * Use cases:
   * - Close More menu after navigation
   * - Track analytics events
   * - Custom navigation logic
   *
   * @default undefined
   * @example () => { closeMoreMenu(); }
   */
  onClick?: () => void;

  /**
   * Rendering variant for different contexts.
   * - 'sidebar': Desktop sidebar item (horizontal layout with icon + label)
   * - 'bottom-bar': Mobile bottom bar item (vertical layout, compact)
   * - 'menu': More menu item (horizontal layout, full width)
   *
   * @default 'sidebar'
   */
  variant?: 'sidebar' | 'bottom-bar' | 'menu';

  /**
   * Optional CSS class name for the link element.
   * Allows custom styling per context.
   *
   * @default undefined
   */
  class?: string;
}

/**
 * Internal component state (not props, but documented for reference)
 */

/**
 * State for BottomNavigation.svelte
 */
export interface BottomNavigationState {
  /**
   * Controls whether the More menu is open.
   * Managed by Svelte 5 $state rune.
   */
  moreMenuOpen: boolean;
}

/**
 * State for SidebarNavigation.svelte
 *
 * Note: SidebarNavigation is stateless. All state is derived from:
 * - $page.url.pathname (active route detection)
 * - NAVIGATION_ITEMS config (navigation data)
 * - Paraglide messages (localized labels)
 */
export interface SidebarNavigationState {
  // No local state; fully derived
}

/**
 * State for MoreMenu.svelte
 *
 * Note: MoreMenu is a controlled component. State is managed by parent (BottomNavigation).
 */
export interface MoreMenuState {
  // No local state; controlled by parent via `open` prop
}

/**
 * Type guard to check if a NavigationItem is primary
 */
export function isPrimaryItem(item: NavigationItem): boolean {
  return item.isPrimary === true;
}

/**
 * Type guard to check if a NavigationItem is secondary
 */
export function isSecondaryItem(item: NavigationItem): boolean {
  return item.isPrimary === false;
}
