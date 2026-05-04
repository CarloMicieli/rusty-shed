import { browser } from '$app/environment';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import type { BudgetDashboardSummary } from '$lib/bindings';

class FinanceState {
  data = $state<BudgetDashboardSummary | null>(null);
  loading = $state(false);
  error = $state<string | null>(null);
  hasFetched = $state(false);

  #inFlight: Promise<BudgetDashboardSummary | null> | null = null;
  #unlisten: UnlistenFn | null = null;

  async ensureLoaded(): Promise<BudgetDashboardSummary | null> {
    return this.#load(false);
  }

  async refresh(): Promise<BudgetDashboardSummary | null> {
    return this.#load(true);
  }

  async startListening(): Promise<void> {
    if (!browser || this.#unlisten) return;

    try {
      this.#unlisten = await listen('finance-data-changed', () => {
        void this.refresh();
      });
    } catch {
      // Ignore listener setup errors outside Tauri runtime (tests/browser preview).
    }
  }

  stopListening(): void {
    if (!this.#unlisten) return;
    this.#unlisten();
    this.#unlisten = null;
  }

  async #load(forceRefresh: boolean): Promise<BudgetDashboardSummary | null> {
    if (!forceRefresh && this.hasFetched) {
      return this.data;
    }

    if (this.#inFlight) {
      return this.#inFlight;
    }

    this.loading = true;
    this.error = null;

    this.#inFlight = (async () => {
      try {
        console.debug('Invoking get_budget_dashboard');
        const summary = await invoke<BudgetDashboardSummary>('get_budget_dashboard');
        this.data = summary;
        this.hasFetched = true;
        return summary;
      } catch (error) {
        this.error = error instanceof Error ? error.message : String(error);
        return null;
      } finally {
        this.loading = false;
      }
    })();

    try {
      return await this.#inFlight;
    } finally {
      this.#inFlight = null;
    }
  }
}

export const financeState = new FinanceState();
