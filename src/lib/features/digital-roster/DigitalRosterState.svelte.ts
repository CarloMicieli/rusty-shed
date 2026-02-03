import type { DigitalRollingStockView, DigitalSummary } from '$lib/bindings';
import { getContext, setContext } from 'svelte';
import type { DigitalRosterController } from './DigitalRosterController.svelte';

const DIGITAL_ROSTER_CONTEXT_KEY = Symbol('digitalRosterContext');

/**
 * Reactive state for the Digital Roster feature using Svelte 5 runes
 */
export class DigitalRosterState {
  summary = $state<DigitalSummary | null>(null);
  rollingStocks = $state<DigitalRollingStockView[]>([]);
  filteredRollingStocks = $derived(this.applyFilter());
  filterText = $state('');
  isLoading = $state(false);
  error = $state<string | null>(null);

  private applyFilter(): DigitalRollingStockView[] {
    if (!this.filterText.trim()) {
      return this.rollingStocks;
    }

    const searchTerm = this.filterText.toLowerCase();
    return this.rollingStocks.filter(
      (rs) =>
        rs.dcc_address.toString().includes(searchTerm) ||
        rs.road_number?.toLowerCase().includes(searchTerm) ||
        rs.series_code?.toLowerCase().includes(searchTerm) ||
        rs.description?.toLowerCase().includes(searchTerm)
    );
  }

  setFilterText(text: string) {
    this.filterText = text;
  }

  setSummary(summary: DigitalSummary) {
    this.summary = summary;
  }

  setRollingStocks(stocks: DigitalRollingStockView[]) {
    this.rollingStocks = stocks;
  }

  setLoading(loading: boolean) {
    this.isLoading = loading;
  }

  setError(error: string | null) {
    this.error = error;
  }

  clearError() {
    this.error = null;
  }
}

/**
 * Set the digital roster controller in the Svelte context
 */
export function setDigitalRosterContext(controller: DigitalRosterController) {
  setContext(DIGITAL_ROSTER_CONTEXT_KEY, controller);
}

/**
 * Get the digital roster controller from the Svelte context
 */
export function getDigitalRosterContext(): DigitalRosterController {
  const context = getContext<DigitalRosterController>(DIGITAL_ROSTER_CONTEXT_KEY);
  if (!context) {
    throw new Error(
      'Digital roster context not found. Did you forget to call setDigitalRosterContext?'
    );
  }
  return context;
}
