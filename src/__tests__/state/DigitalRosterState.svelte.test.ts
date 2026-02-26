import { describe, it, expect, vi, beforeEach } from 'vitest';
import { flushSync } from 'svelte';
import { DigitalRosterState } from '$lib/features/digital-roster/DigitalRosterState.svelte';
import type { DigitalRollingStockView, DigitalSummary } from '$lib/bindings';

// ─── helpers ──────────────────────────────────────────────────────────────

function makeStock(
  dccAddress: number,
  overrides: Partial<DigitalRollingStockView> = {}
): DigitalRollingStockView {
  return {
    dcc_address: dccAddress,
    road_number: `RS-${dccAddress}`,
    series_code: `SERIES-${dccAddress}`,
    description: `Digital stock ${dccAddress}`,
    ...overrides
  } as unknown as DigitalRollingStockView;
}

function makeSummary(): DigitalSummary {
  return {
    totalCount: 5,
    assignedCount: 3,
    availableAddresses: 2
  } as unknown as DigitalSummary;
}

// ─── tests ────────────────────────────────────────────────────────────────

describe('DigitalRosterState', () => {
  it('starts with empty state', () => {
    const state = new DigitalRosterState();
    expect(state.summary).toBeNull();
    expect(state.rollingStocks).toHaveLength(0);
    expect(state.filteredRollingStocks).toHaveLength(0);
    expect(state.filterText).toBe('');
    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
  });

  describe('setSummary', () => {
    it('sets summary', () => {
      const state = new DigitalRosterState();
      const summary = makeSummary();
      flushSync(() => state.setSummary(summary));
      expect(state.summary).toEqual(summary);
    });
  });

  describe('setRollingStocks', () => {
    it('sets stocks array', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(1), makeStock(2)];
      flushSync(() => state.setRollingStocks(stocks));
      expect(state.rollingStocks).toHaveLength(2);
    });
  });

  describe('$derived filteredRollingStocks', () => {
    it('returns all stocks when filterText is empty', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(1), makeStock(22), makeStock(333)];
      flushSync(() => state.setRollingStocks(stocks));
      expect(state.filteredRollingStocks).toHaveLength(3);
    });

    it('filters by DCC address', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(3), makeStock(33), makeStock(100)];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('3');
      });
      // Both 3 and 33 contain "3"
      expect(state.filteredRollingStocks).toHaveLength(2);
    });

    it('filters by road_number (case-insensitive)', () => {
      const state = new DigitalRosterState();
      const stocks = [
        makeStock(1, { road_number: 'BR 50' }),
        makeStock(2, { road_number: 'ICE 3' }),
        makeStock(3, { road_number: 'BR 80' })
      ];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('br');
      });
      expect(state.filteredRollingStocks).toHaveLength(2);
    });

    it('filters by series_code (case-insensitive)', () => {
      const state = new DigitalRosterState();
      const stocks = [
        makeStock(1, { series_code: 'DB-CLASS-50' }),
        makeStock(2, { series_code: 'ÖBB-1116' }),
        makeStock(3, { series_code: 'DB-CLASS-80' })
      ];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('db-class');
      });
      expect(state.filteredRollingStocks).toHaveLength(2);
    });

    it('filters by description (case-insensitive)', () => {
      const state = new DigitalRosterState();
      const stocks = [
        makeStock(1, { description: 'Steam Locomotive' }),
        makeStock(2, { description: 'Diesel Engine' }),
        makeStock(3, { description: 'Steam Railcar' })
      ];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('steam');
      });
      expect(state.filteredRollingStocks).toHaveLength(2);
    });

    it('returns empty array when no stocks match', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(1, { description: 'Diesel Engine' })];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('electric');
      });
      expect(state.filteredRollingStocks).toHaveLength(0);
    });

    it('clears filter when text is reset to empty string', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(1), makeStock(2)];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('99999');
      });
      expect(state.filteredRollingStocks).toHaveLength(0);

      flushSync(() => state.setFilterText(''));
      expect(state.filteredRollingStocks).toHaveLength(2);
    });

    it('handles null road_number without throwing', () => {
      const state = new DigitalRosterState();
      const stocks = [makeStock(1, { road_number: null as unknown as string })];
      flushSync(() => {
        state.setRollingStocks(stocks);
        state.setFilterText('anything');
      });
      // Should not throw; stock won't match
      expect(state.filteredRollingStocks).toHaveLength(0);
    });
  });

  describe('setLoading / setError / clearError', () => {
    it('setLoading updates isLoading flag', () => {
      const state = new DigitalRosterState();
      flushSync(() => state.setLoading(true));
      expect(state.isLoading).toBe(true);

      flushSync(() => state.setLoading(false));
      expect(state.isLoading).toBe(false);
    });

    it('setError records error message', () => {
      const state = new DigitalRosterState();
      flushSync(() => state.setError('Load failed'));
      expect(state.error).toBe('Load failed');
    });

    it('clearError resets error to null', () => {
      const state = new DigitalRosterState();
      flushSync(() => {
        state.setError('Something went wrong');
        state.clearError();
      });
      expect(state.error).toBeNull();
    });
  });
});
