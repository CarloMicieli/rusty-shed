import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import { safeInvoke, getErrorMessage } from '$lib/services';
import type { DashboardSummary, QueryCriteria } from '$lib/bindings';
import { financeState } from '$lib/state/finance.svelte';

function resolveMaintenanceDue(
  totals: DashboardSummary['totals'] | Record<string, unknown> | null | undefined
): number {
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
  get budgetData() {
    return financeState.data;
  }
  get isLoading() {
    return this.#isLoading;
  }
  get error() {
    return this.#error;
  }

  // 3. Derived Logic (equivalent to Svelte 4 derived stores)
  hasMaintenance = $derived(resolveMaintenanceDue(this.#data?.totals) > 0);
  recentItemsCount = $derived(this.#data?.recentItems?.length ?? 0);

  /**
   * Loads dashboard data and handles state transitions
   */
  async load(criteria: QueryCriteria | null = null) {
    // Prevent double-loading if already in progress
    if (this.#isLoading) return;

    this.#isLoading = true;
    this.#error = null;

    console.debug('Invoking get_dashboard_summary with criteria:', criteria);
    const result = await safeInvoke<DashboardSummary>('get_dashboard_summary', { criteria });

    if (result.ok) {
      this.#data = result.data;
    } else {
      const errorMsg = getErrorMessage(result.error);
      console.error('Dashboard Store Error:', errorMsg, { raw: result.error, result });
      this.#error = 'dashboard_load_failed';
      toastError(errorMsg);
    }

    this.#isLoading = false;
  }

  /**
   * Loads budget dashboard data
   */
  async loadBudget() {
    await financeState.ensureLoaded();
  }

  /**
   * Public alias for load to be used in UI retry buttons
   */
  async retry() {
    this.#data = null; // Clear old data on manual retry to trigger skeletons
    await this.load();
    await financeState.refresh();
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
