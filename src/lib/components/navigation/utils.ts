import type { NavigationItem } from './types';

/**
 * Determines if a navigation item is currently active based on the given pathname
 * @param item - Navigation item to check
 * @param pathname - Current route pathname (from $page.url.pathname)
 * @returns true if the item is active
 */
export function isActive(item: NavigationItem, pathname: string): boolean {
  if (item.usePrefixMatch) {
    return pathname.startsWith(item.href);
  }
  return pathname === item.href;
}

/**
 * Determines if the More button should show active state
 * The More button is active if any secondary feature is currently active
 * @param secondaryItems - List of secondary navigation items
 * @param pathname - Current route pathname
 * @returns true if any secondary feature is active
 */
export function isMoreButtonActive(secondaryItems: NavigationItem[], pathname: string): boolean {
  return secondaryItems.some((item) => isActive(item, pathname));
}
