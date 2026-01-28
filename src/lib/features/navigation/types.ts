/**
 * Navigation types.
 */

export interface NavigationItem {
  /**
   * Unique identifier for the navigation item.
   */
  id: string;

  /**
   * Display label (i18n key or text).
   */
  label: string;

  /**
   * Icon name (from icon library).
   */
  icon: string;

  /**
   * Route path.
   */
  href: string;

  /**
   * Whether this item is active.
   */
  active?: boolean;

  /**
   * Badge count (optional).
   */
  badge?: number;
}

export interface NavigationConfig {
  /**
   * Primary navigation items.
   */
  items: NavigationItem[];

  /**
   * Whether navigation is collapsed (for sidebar).
   */
  collapsed?: boolean;
}
