/**
 * Navigation Service - Manages navigation state and active routes.
 *
 * This service provides:
 * - Active route tracking
 * - Navigation item management
 * - Sidebar collapse state
 */

import { setContext, getContext } from 'svelte';
import type { NavigationItem, NavigationConfig } from '../types';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('navigation-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class NavigationService {
  // Private reactive state
  #activeRoute = $state<string>('/');
  #isCollapsed = $state(false);
  #items = $state<NavigationItem[]>([]);

  // Public readonly getters (defensive encapsulation)
  get activeRoute(): string {
    return this.#activeRoute;
  }

  get isCollapsed(): boolean {
    return this.#isCollapsed;
  }

  get items(): NavigationItem[] {
    return this.#items;
  }

  // Derived state
  activeItem = $derived.by(() => {
    return this.#items.find((item) => item.href === this.#activeRoute);
  });

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Set the active route.
   *
   * @param route - The route path
   */
  setActiveRoute(route: string): void {
    this.#activeRoute = route;
  }

  /**
   * Toggle sidebar collapsed state.
   */
  toggleCollapsed(): void {
    this.#isCollapsed = !this.#isCollapsed;
  }

  /**
   * Set sidebar collapsed state.
   *
   * @param collapsed - Whether the sidebar should be collapsed
   */
  setCollapsed(collapsed: boolean): void {
    this.#isCollapsed = collapsed;
  }

  /**
   * Initialize navigation items.
   *
   * @param items - Array of navigation items
   */
  setItems(items: NavigationItem[]): void {
    this.#items = items;
  }

  /**
   * Check if a route is active.
   *
   * @param route - The route to check
   * @returns True if the route is active
   */
  isActive(route: string): boolean {
    return this.#activeRoute === route;
  }

  /**
   * Navigate to a route (updates active state).
   *
   * @param route - The route to navigate to
   */
  navigateTo(route: string): void {
    this.#activeRoute = route;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the NavigationService in the current context.
 *
 * @param config - Optional initial configuration
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setNavigationService(
  config?: Partial<NavigationConfig>,
  service?: NavigationService
): NavigationService {
  const instance = service ?? new NavigationService();

  if (config?.items) {
    instance.setItems(config.items);
  }

  if (config?.collapsed !== undefined) {
    instance.setCollapsed(config.collapsed);
  }

  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the NavigationService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getNavigationService(): NavigationService {
  const service = getContext<NavigationService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'NavigationService not found in context. Did you call setNavigationService() in a parent component?'
    );
  }
  return service;
}
