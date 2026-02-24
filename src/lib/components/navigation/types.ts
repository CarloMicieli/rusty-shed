import type { SvelteComponent } from 'svelte';

/**
 * Represents a single navigation feature
 */
export interface NavigationItem {
  /**
   * Unique identifier for the navigation item
   * @example 'home', 'collection', 'finance'
   */
  id: string;

  /**
   * Paraglide message function that returns the localized label
   * @example () => m.app_home()
   */
  label: () => string;

  /**
   * lucide-svelte icon component
   * @example LayoutDashboard, TrainFront, Wallet
   */
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  icon: typeof SvelteComponent<any>;

  /**
   * Route path for navigation
   * @example '/dashboard', '/collection'
   */
  href: string;

  /**
   * Determines if item appears in mobile bottom bar (true) or More menu (false)
   */
  isPrimary: boolean;

  /**
   * Optional: Badge count to display (e.g., wishlist count)
   */
  badgeCount?: number;

  /**
   * Optional: Use prefix matching for active state (for routes with subroutes)
   * @default false (uses exact match)
   * @example true for '/railway-tracks' to match '/my-tracks/all', '/my-tracks/n-scale', etc.
   */
  usePrefixMatch?: boolean;

  /**
   * Optional: Additional route prefixes that should mark this item as active.
   * Useful when a feature has sub-routes at a different path (e.g. /collection/{id}).
   * @example ['/collection'] for the 'collection' nav item to stay active on detail pages
   */
  additionalPrefixes?: string[];
}

/**
 * Props for the MoreMenu component
 */
export interface MoreMenuProps {
  /**
   * Controls visibility of the bottom sheet
   */
  open: boolean;

  /**
   * Callback when sheet should close (backdrop tap, ESC key, or navigation)
   */
  onClose: () => void;

  /**
   * Secondary navigation items to display
   */
  items: NavigationItem[];
}

/**
 * Props for SidebarNavigation component
 */
export interface SidebarNavigationProps {
  /**
   * Optional CSS class to apply to the sidebar root
   */
  class?: string;
}

/**
 * Props for BottomNavigation component
 */
export interface BottomNavigationProps {
  /**
   * Optional CSS class to apply to the navigation root
   */
  class?: string;
}
