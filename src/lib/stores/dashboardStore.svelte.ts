import { toaster } from '$lib/toaster';

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

/**
 * Internal helper for Tauri/Backend invocation
 */
async function invokeDashboardSummary(): Promise<DashboardSummary> {
  try {
    const bindings = await import('$lib/bindings');
    if (bindings?.commands?.dashboardSummary) {
      const result = await bindings.commands.dashboardSummary();
      if (result.status === 'ok') {
        // Convert camelCase bindings to snake_case store types
        const data = result.data;
        return {
          totals: {
            collection_items: data.totals.collectionItems,
            wishlists: data.totals.wishlists,
            maintenance_due: data.totals.maintenanceDue,
            total_value: data.totals.totalValue
              ? {
                  amount: Number(data.totals.totalValue.amount),
                  currency: data.totals.totalValue.currency
                }
              : null
          },
          recent_items: data.recentItems,
          depot_items: data.depotItems
        };
      }
      throw new Error('Failed to fetch dashboard summary');
    }
  } catch {
    /* fallthrough */
  }

  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<DashboardSummary>('dashboard_summary');
}

/**
 * Svelte 5 Rune-based Dashboard Store
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

    try {
      const summary = await invokeDashboardSummary();
      this.#data = summary;
    } catch (e) {
      console.error('Dashboard Store Error:', e);
      this.#error = 'dashboard_load_failed';

      toaster.error({
        id: 'dashboard-load',
        title: 'Dashboard failed to load',
        description: 'Please check your connection or try again.',
        duration: 4000
      });
    } finally {
      this.#isLoading = false;
    }
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
