import { setContext, getContext } from 'svelte';
import { toaster } from '$lib/toaster';
import { safeInvoke, getErrorMessage } from '$lib/services';
import type { DashboardSummary, QueryCriteria, BudgetDashboardSummary } from '$lib/bindings';

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
  #budgetData = $state<BudgetDashboardSummary | null>(null);
  #isLoading = $state(false);
  #error = $state<string | null>(null);
  #budgetLoadInFlight: Promise<void> | null = null;

  // 2. Getters (providing read-only reactive access)
  get data() {
    return this.#data;
  }
  get budgetData() {
    return this.#budgetData;
  }
  get isLoading() {
    return this.#isLoading;
  }
  get error() {
    return this.#error;
  }

  // 3. Derived Logic (equivalent to Svelte 4 derived stores)
  hasMaintenance = $derived(
    (this.#data?.totals?.maintenanceDue ??
      ((this.#data?.totals as Record<string, unknown>)['maintenance_due'] as number | undefined) ??
      0) > 0
  );
  recentItemsCount = $derived(this.#data?.recentItems.length ?? 0);

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
    if (this.#budgetLoadInFlight) {
      await this.#budgetLoadInFlight;
      return;
    }

    this.#budgetLoadInFlight = (async () => {
      console.debug('Invoking get_budget_dashboard');
      const result = await safeInvoke<BudgetDashboardSummary>('get_budget_dashboard');

      if (result.ok) {
        this.#budgetData = result.data;
      } else {
        const errorMsg = getErrorMessage(result.error);
        console.warn('Budget Dashboard Error:', errorMsg, { raw: result.error });
        // Don't show error toast for budget - it's optional data
        this.#budgetData = null;
      }
    })();

    try {
      await this.#budgetLoadInFlight;
    } finally {
      this.#budgetLoadInFlight = null;
    }
  }

  /**
   * Public alias for load to be used in UI retry buttons
   */
  async retry() {
    this.#data = null; // Clear old data on manual retry to trigger skeletons
    await this.load();
    await this.loadBudget();
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
