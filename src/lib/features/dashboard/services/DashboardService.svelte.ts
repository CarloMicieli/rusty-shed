/**
 * Dashboard Service - Manages dashboard data and state.
 *
 * This service provides:
 * - Dashboard summary data fetching
 * - Query criteria handling
 * - Error handling with toast notifications
 * - Retry logic
 */

import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { getErrorMessage } from '$lib/shared/domain/errors';
import type { DashboardSummary, QueryCriteria } from '$lib/bindings';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('dashboard-service');

function resolveMaintenanceDue(totals: DashboardSummary['totals'] | Record<string, unknown> | null | undefined): number {
  if (!totals) return 0;

  if (typeof totals.maintenanceDue === 'number') {
    return totals.maintenanceDue;
  }

  if ('maintenance_due' in totals) {
    const fallback = (totals as Record<string, unknown>)['maintenance_due'];
    return typeof fallback === 'number' ? fallback : 0;
  }

  return 0;
}

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class DashboardService {
  // Private reactive state
  #data = $state<DashboardSummary | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);

  // Public readonly getters (defensive encapsulation)
  get data(): DashboardSummary | null {
    return this.#data;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  get error(): string | null {
    return this.#error;
  }

  // Derived state
  hasMaintenance = $derived(
    resolveMaintenanceDue(this.#data?.totals) > 0
  );

  recentItemsCount = $derived(this.#data?.recentItems?.length ?? 0);

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Load dashboard summary data.
   *
   * @param criteria - Optional query criteria for filtering
   */
  async load(criteria: QueryCriteria | null = null): Promise<void> {
    // Prevent double-loading if already in progress
    if (this.#isLoading) return;

    this.#isLoading = true;
    this.#error = null;

    const result = await safeInvoke<DashboardSummary>('get_dashboard_summary', { criteria });

    if (result.ok) {
      this.#data = result.data;
    } else {
      console.error('Dashboard Service Error:', result.error);
      this.#error = 'dashboard_load_failed';

      const errorMsg = getErrorMessage(result.error);
      toaster.error({
        id: 'dashboard-error',
        title: errorMsg || 'Dashboard failed to load',
        duration: 4000
      });
    }

    this.#isLoading = false;
  }

  /**
   * Retry loading dashboard data (clears existing data first).
   */
  async retry(): Promise<void> {
    this.#data = null; // Clear old data on manual retry to trigger skeletons
    await this.load();
  }

  /**
   * Clear dashboard data.
   */
  clear(): void {
    this.#data = null;
    this.#error = null;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the DashboardService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setDashboardService(service?: DashboardService): DashboardService {
  const instance = service ?? new DashboardService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the DashboardService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getDashboardService(): DashboardService {
  const service = getContext<DashboardService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'DashboardService not found in context. Did you call setDashboardService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use setDashboardService() instead
 */
export function createDashboardState(): DashboardService {
  console.warn('createDashboardState is deprecated. Use setDashboardService() instead.');
  return new DashboardService();
}

/**
 * @deprecated Use setDashboardService() instead
 */
export function setDashboardContext(state: DashboardService): void {
  console.warn('setDashboardContext is deprecated. Use setDashboardService() instead.');
  setContext(SERVICE_KEY, state);
}

/**
 * @deprecated Use getDashboardService() instead
 */
export function getDashboardContext(): DashboardService {
  console.warn('getDashboardContext is deprecated. Use getDashboardService() instead.');
  return getDashboardService();
}
