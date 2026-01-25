import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import { safeInvoke, getErrorMessage } from '$lib/services';
import type { DashboardSummary, QueryCriteria } from '$lib/bindings';

function toastError(message?: string) {
  toaster.error({
    id: 'dashboard-error',
    title: message || 'Dashboard failed to load',
    description: 'Please check your connection or try again.',
    duration: 4000
  });
}

/**
 * DashboardState loads dashboard data and handles state transitions
 */
export class DashboardState {
  // 1. Reactive State
  #data = $state<DashboardSummary | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);

  // 2. Getters (providing read-only reactive access)
  get data() {
    return this.#data;
  }
  get isLoading() {
    return this.#isLoading;
  }
  get error() {
    return this.#error;
  }

  // 3. Derived Logic (equivalent to Svelte 4 derived stores)
  hasMaintenance = $derived((this.#data?.totals?.maintenance_due ?? 0) > 0);
  recentItemsCount = $derived(this.#data?.recentItems.length ?? 0);

  /**
   * Loads dashboard data and handles state transitions
   */
  async load(criteria: QueryCriteria | null = null) {
    // Prevent double-loading if already in progress
    if (this.#isLoading) return;

    this.#isLoading = true;
    this.#error = null;

    const result = await safeInvoke<DashboardSummary>('get_dashboard_summary', { criteria });

    if (result.ok) {
      this.#data = result.data;
    } else {
      console.error('Dashboard Store Error:', result.error);
      this.#error = 'dashboard_load_failed';

      const errorMsg = getErrorMessage(result.error);
      toastError(errorMsg);
    }

    this.#isLoading = false;
  }

  /**
   * Public alias for load to be used in UI retry buttons
   */
  async retry() {
    this.#data = null; // Clear old data on manual retry to trigger skeletons
    await this.load();
  }
}

const DASHBOARD_CONTEXT_KEY = Symbol('dashboard-context');

export function createDashboardState() {
  return new DashboardState();
}

export function setDashboardContext(state: DashboardState) {
  setContext(DASHBOARD_CONTEXT_KEY, state);
}

export function getDashboardContext(): DashboardState {
  const state = getContext<DashboardState>(DASHBOARD_CONTEXT_KEY);
  if (!state) {
    throw new Error(
      'DashboardContext not provided. Ensure component is within a DashboardContext provider.'
    );
  }
  return state;
}
