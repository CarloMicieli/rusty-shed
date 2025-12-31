import { toaster } from '$lib/toaster';
import { safeInvoke, getErrorMessage } from '$lib/services';

// Types remain the same as they are static definitions
export type DashboardTotals = {
  collection_items: number;
  wishlists: number;
  maintenance_due: number;
  total_value?: { amount: number; currency: string } | null;
};

export type DashboardRecentItem = {
  id: string;
  title: string;
  subtitle?: string | null;
};

export type DashboardDepotEntry = {
  id: string;
  manufacturer?: string | null;
  productCode?: string | null;
  category?: string | null;
  scale?: string | null;
  railwayCompany?: string | null;
  description?: string | null;
};

export type DashboardSummary = {
  totals: DashboardTotals;
  recent_items: DashboardRecentItem[];
  depot_items: DashboardDepotEntry[];
};

function toastError(message?: string) {
  toaster.error({
    id: 'dashboard-error',
    title: message || 'Dashboard failed to load',
    description: 'Please check your connection or try again.',
    duration: 4000
  });
}

/**
 * Loads dashboard data and handles state transitions
 */
class DashboardStore {
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
  recentItemsCount = $derived(this.#data?.recent_items.length ?? 0);

  /**
   * Loads dashboard data and handles state transitions
   */
  async load() {
    // Prevent double-loading if already in progress
    if (this.#isLoading) return;

    this.#isLoading = true;
    this.#error = null;

    const result = await safeInvoke<DashboardSummary>('dashboard_summary');

    if (result.ok) {
      // Data is already in snake_case from bindings
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

// Export a single instance to be used across the app
export const dashboardStore = new DashboardStore();
