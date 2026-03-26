import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock @tauri-apps/api/core before any imports
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

// Mock svelte context functions (DepotService uses setContext/getContext)
vi.mock('svelte', async () => {
  const actual = await vi.importActual('svelte');
  return {
    ...actual,
    getContext: vi.fn(),
    setContext: vi.fn()
  };
});

import { DepotService } from '$lib/features/depot/services/DepotService.svelte';
import { invoke } from '@tauri-apps/api/core';
import type { DepotRollingStockView } from '$lib/bindings';

const mockInvoke = vi.mocked(invoke);

// ─────────────────────────────────────────────────────────────
// MOCK HELPERS
// ─────────────────────────────────────────────────────────────

function setupInvokeMock(handlers: Record<string, () => unknown>): void {
  mockInvoke.mockImplementation(async (cmd) => {
    const key = String(cmd);
    if (key in handlers) return handlers[key]() as never;
    throw new Error(`Unmocked command: ${key}`);
  });
}

function makeDepotItem(
  category: DepotRollingStockView['category'],
  overrides: Partial<DepotRollingStockView> = {}
): DepotRollingStockView {
  return {
    id: `item-${Math.random().toString(36).slice(2)}`,
    railwayModelId: 'model-1',
    seriesCode: 'BR101',
    roadNumber: null,
    friendlyName: null,
    depot: null,
    category,
    manufacturerName: 'Märklin',
    productCode: 'P-001',
    control: null,
    livery: null,
    railwayCompanyName: null,
    epoch: null,
    dccAddress: null,
    ...overrides
  };
}

// ─────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────

describe('DepotService', () => {
  let service: DepotService;

  beforeEach(() => {
    vi.resetAllMocks();
    service = new DepotService();
  });

  // ── Initial State ────────────────────────────────────────────

  describe('initial state', () => {
    it('has isLoading=false', () => {
      expect(service.isLoading).toBe(false);
    });

    it('has error=null', () => {
      expect(service.error).toBeNull();
    });

    it('has query=""', () => {
      expect(service.query).toBe('');
    });

    it('has viewMode="table"', () => {
      expect(service.viewMode).toBe('table');
    });

    it('has empty locomotives', () => {
      expect(service.locomotives).toEqual([]);
    });

    it('has empty trains', () => {
      expect(service.trains).toEqual([]);
    });

    it('has empty cars', () => {
      expect(service.cars).toEqual([]);
    });

    it('has totalFiltered=0', () => {
      expect(service.totalFiltered).toBe(0);
    });
  });

  // ── setQuery / clearQuery ───────────────────────────────────

  describe('setQuery()', () => {
    it('updates the query', () => {
      service.setQuery('BR101');
      expect(service.query).toBe('BR101');
    });

    it('replaces existing query', () => {
      service.setQuery('first');
      service.setQuery('second');
      expect(service.query).toBe('second');
    });

    it('accepts empty string', () => {
      service.setQuery('something');
      service.setQuery('');
      expect(service.query).toBe('');
    });
  });

  describe('clearQuery()', () => {
    it('resets query to empty string', () => {
      service.setQuery('some search');
      service.clearQuery();
      expect(service.query).toBe('');
    });

    it('is idempotent when already empty', () => {
      service.clearQuery();
      expect(service.query).toBe('');
    });
  });

  // ── setViewMode ──────────────────────────────────────────────

  describe('setViewMode()', () => {
    it('switches to grid', () => {
      service.setViewMode('grid');
      expect(service.viewMode).toBe('grid');
    });

    it('switches back to table', () => {
      service.setViewMode('grid');
      service.setViewMode('table');
      expect(service.viewMode).toBe('table');
    });
  });

  // ── load() ───────────────────────────────────────────────────

  describe('load()', () => {
    it('sets depot data on success', async () => {
      const depotView = { rollingStocks: [makeDepotItem('LOCOMOTIVE')] };
      setupInvokeMock({ get_depot: () => depotView });

      await service.load();

      expect(service.locomotives).toHaveLength(1);
      expect(service.error).toBeNull();
    });

    it('calls invoke with get_depot command', async () => {
      const depotView = { rollingStocks: [] };
      setupInvokeMock({ get_depot: () => depotView });

      await service.load();

      expect(mockInvoke).toHaveBeenCalledWith('get_depot', {});
    });

    it('resets isLoading to false after success', async () => {
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [] }) });

      await service.load();

      expect(service.isLoading).toBe(false);
    });

    it('sets error when backend returns failure', async () => {
      // safeInvoke catches the invoke throw and returns { ok: false, error: NormalizedError }
      // We simulate this by having invoke throw a Rust CommandError shape
      mockInvoke.mockRejectedValue({ NotFound: 'Depot not found' });

      await service.load();

      expect(service.error).toBe('Depot not found');
      expect(service.isLoading).toBe(false);
    });

    it('sets error message from DatabaseError', async () => {
      mockInvoke.mockRejectedValue({ DatabaseError: 'Connection refused' });

      await service.load();

      expect(service.error).toBe('Connection refused');
    });

    it('resets isLoading to false after failure', async () => {
      mockInvoke.mockRejectedValue({ NotFound: 'Not found' });

      await service.load();

      expect(service.isLoading).toBe(false);
    });

    it('sets error when invoke throws a JavaScript Error', async () => {
      // This exercises the catch(err) path in load()
      // We simulate an unexpected exception by making safeInvoke itself throw
      // (e.g., invoke throws a non-object that can't be normalized gracefully)
      mockInvoke.mockImplementation(() => {
        throw new Error('Unexpected JS error');
      });

      await service.load();

      // safeInvoke wraps the throw into { ok: false, error: NormalizedError }
      // so the service will set #error = getErrorMessage(result.error) = 'Unexpected JS error'
      expect(service.error).toBe('Unexpected JS error');
      expect(service.isLoading).toBe(false);
    });

    it('clears previous error on new load attempt', async () => {
      // First load fails
      mockInvoke.mockRejectedValue({ NotFound: 'Not found' });
      await service.load();
      expect(service.error).not.toBeNull();

      // Second load succeeds
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [] }) });
      await service.load();

      expect(service.error).toBeNull();
    });
  });

  // ── Categorization ───────────────────────────────────────────

  describe('locomotives derived getter', () => {
    it('returns only LOCOMOTIVE items', async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', { id: 'loco-1', seriesCode: 'BR101' }),
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', { id: 'emu-1' }),
        makeDepotItem('PASSENGER_CAR', { id: 'car-1' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });

      await service.load();

      expect(service.locomotives).toHaveLength(1);
      expect(service.locomotives[0].seriesCode).toBe('BR101');
    });

    it('maps group to friendlyName when available', async () => {
      const item = makeDepotItem('LOCOMOTIVE', {
        id: 'loco-1',
        seriesCode: 'BR101',
        friendlyName: 'Pacific Express'
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.locomotives[0].group).toBe('Pacific Express');
    });

    it('falls back to seriesCode when friendlyName is null', async () => {
      const item = makeDepotItem('LOCOMOTIVE', {
        id: 'loco-1',
        seriesCode: 'BR101',
        friendlyName: null
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.locomotives[0].group).toBe('BR101');
    });

    it('maps all relevant fields', async () => {
      const item = makeDepotItem('LOCOMOTIVE', {
        id: 'loco-id',
        railwayModelId: 'model-id',
        seriesCode: 'BR101',
        friendlyName: 'My Loco',
        manufacturerName: 'Märklin',
        productCode: 'P-999',
        roadNumber: '101',
        railwayCompanyName: 'DB',
        livery: 'Red',
        control: 'DCC_FITTED',
        dccAddress: 42
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      const loco = service.locomotives[0];
      expect(loco.id).toBe('loco-id');
      expect(loco.railwayModelId).toBe('model-id');
      expect(loco.seriesCode).toBe('BR101');
      expect(loco.group).toBe('My Loco');
      expect(loco.manufacturer).toBe('Märklin');
      expect(loco.productCode).toBe('P-999');
      expect(loco.roadNumber).toBe('101');
      expect(loco.railwayCompany).toBe('DB');
      expect(loco.livery).toBe('Red');
      expect(loco.control).toBe('DCC_FITTED');
      expect(loco.dccAddress).toBe(42);
    });
  });

  describe('trains derived getter', () => {
    it('returns ELECTRIC_MULTIPLE_UNIT items', async () => {
      const items = [
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', { id: 'emu-1', seriesCode: 'ICE3' }),
        makeDepotItem('LOCOMOTIVE', { id: 'loco-1' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });

      await service.load();

      expect(service.trains).toHaveLength(1);
      expect(service.trains[0].seriesCode).toBe('ICE3');
    });

    it('returns RAILCAR items', async () => {
      const items = [
        makeDepotItem('RAILCAR', { id: 'rail-1', seriesCode: 'VT98' }),
        makeDepotItem('FREIGHT_CAR', { id: 'freight-1' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });

      await service.load();

      expect(service.trains).toHaveLength(1);
      expect(service.trains[0].seriesCode).toBe('VT98');
    });

    it('returns both ELECTRIC_MULTIPLE_UNIT and RAILCAR', async () => {
      const items = [
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', { id: 'emu-1' }),
        makeDepotItem('RAILCAR', { id: 'rail-1' }),
        makeDepotItem('LOCOMOTIVE', { id: 'loco-1' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });

      await service.load();

      expect(service.trains).toHaveLength(2);
    });

    it('maps group to friendlyName when available', async () => {
      const item = makeDepotItem('RAILCAR', {
        seriesCode: 'VT98',
        friendlyName: 'Red Railcar'
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.trains[0].group).toBe('Red Railcar');
    });

    it('falls back to seriesCode when friendlyName is null', async () => {
      const item = makeDepotItem('ELECTRIC_MULTIPLE_UNIT', {
        seriesCode: 'ICE3',
        friendlyName: null
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.trains[0].group).toBe('ICE3');
    });
  });

  describe('cars derived getter', () => {
    it('returns only PASSENGER_CAR and FREIGHT_CAR items', async () => {
      const items = [
        makeDepotItem('PASSENGER_CAR', { id: 'pass-1' }),
        makeDepotItem('FREIGHT_CAR', { id: 'freight-1' }),
        makeDepotItem('LOCOMOTIVE', { id: 'loco-1' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });

      await service.load();

      expect(service.cars).toHaveLength(2);
    });

    it('maps PASSENGER_CAR to category="passenger"', async () => {
      const item = makeDepotItem('PASSENGER_CAR', { id: 'pass-1' });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.cars[0].category).toBe('passenger');
    });

    it('maps FREIGHT_CAR to category="freight"', async () => {
      const item = makeDepotItem('FREIGHT_CAR', { id: 'freight-1' });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.cars[0].category).toBe('freight');
    });

    it('maps type to friendlyName when available', async () => {
      const item = makeDepotItem('PASSENGER_CAR', {
        seriesCode: 'Bm-234',
        friendlyName: 'Dining Car'
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.cars[0].type).toBe('Dining Car');
    });

    it('falls back type to seriesCode when friendlyName is null', async () => {
      const item = makeDepotItem('FREIGHT_CAR', {
        seriesCode: 'Eaos-106',
        friendlyName: null
      });
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.cars[0].type).toBe('Eaos-106');
    });

    it('sets serviceLevel to null', async () => {
      const item = makeDepotItem('PASSENGER_CAR');
      setupInvokeMock({ get_depot: () => ({ rollingStocks: [item] }) });

      await service.load();

      expect(service.cars[0].serviceLevel).toBeNull();
    });
  });

  // ── Filtering ────────────────────────────────────────────────

  describe('filteredLocomotives', () => {
    beforeEach(async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', {
          id: 'loco-1',
          seriesCode: 'BR101',
          roadNumber: '101',
          railwayCompanyName: 'DB',
          livery: 'Red',
          productCode: 'P-001',
          control: 'DCC_FITTED',
          dccAddress: 42
        }),
        makeDepotItem('LOCOMOTIVE', {
          id: 'loco-2',
          seriesCode: 'BR141',
          roadNumber: '141-001',
          railwayCompanyName: 'ÖBB',
          livery: 'Blue',
          productCode: 'P-002',
          control: null,
          dccAddress: null
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();
    });

    it('returns all when query is empty', () => {
      expect(service.filteredLocomotives).toHaveLength(2);
    });

    it('filters by roadNumber (case-insensitive)', () => {
      service.setQuery('141');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].seriesCode).toBe('BR141');
    });

    it('filters by seriesCode', () => {
      service.setQuery('br101');
      expect(service.filteredLocomotives).toHaveLength(1);
    });

    it('filters by railwayCompany', () => {
      service.setQuery('öbb');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].seriesCode).toBe('BR141');
    });

    it('filters by livery', () => {
      service.setQuery('red');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].seriesCode).toBe('BR101');
    });

    it('filters by productCode', () => {
      service.setQuery('p-002');
      expect(service.filteredLocomotives).toHaveLength(1);
    });

    it('filters by control', () => {
      service.setQuery('dcc_fitted');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].seriesCode).toBe('BR101');
    });

    it('filters by dccAddress as string', () => {
      service.setQuery('42');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].seriesCode).toBe('BR101');
    });

    it('returns empty when no match', () => {
      service.setQuery('zzznomatch');
      expect(service.filteredLocomotives).toHaveLength(0);
    });

    it('is case-insensitive', () => {
      service.setQuery('DCC');
      const upper = service.filteredLocomotives.length;
      service.setQuery('dcc');
      const lower = service.filteredLocomotives.length;
      expect(upper).toBe(lower);
    });

    it('restores full list after clearQuery', () => {
      service.setQuery('BR101');
      expect(service.filteredLocomotives).toHaveLength(1);
      service.clearQuery();
      expect(service.filteredLocomotives).toHaveLength(2);
    });

    it('filters by group (friendlyName)', async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', {
          id: 'loco-named',
          seriesCode: 'BR101',
          friendlyName: 'Pacific Express'
        }),
        makeDepotItem('LOCOMOTIVE', {
          id: 'loco-unnamed',
          seriesCode: 'BR141',
          friendlyName: null
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();

      service.setQuery('pacific');
      expect(service.filteredLocomotives).toHaveLength(1);
      expect(service.filteredLocomotives[0].group).toBe('Pacific Express');
    });
  });

  describe('filteredTrains', () => {
    beforeEach(async () => {
      const items = [
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', {
          id: 'emu-1',
          seriesCode: 'ICE3',
          railwayCompanyName: 'DB',
          productCode: 'T-100'
        }),
        makeDepotItem('RAILCAR', {
          id: 'rail-1',
          seriesCode: 'VT98',
          railwayCompanyName: 'DR',
          productCode: 'T-200'
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();
    });

    it('returns all when query is empty', () => {
      expect(service.filteredTrains).toHaveLength(2);
    });

    it('filters by seriesCode', () => {
      service.setQuery('ice');
      expect(service.filteredTrains).toHaveLength(1);
      expect(service.filteredTrains[0].seriesCode).toBe('ICE3');
    });

    it('filters by railwayCompany', () => {
      service.setQuery('dr');
      expect(service.filteredTrains).toHaveLength(1);
      expect(service.filteredTrains[0].seriesCode).toBe('VT98');
    });

    it('returns empty when no match', () => {
      service.setQuery('zzznomatch');
      expect(service.filteredTrains).toHaveLength(0);
    });
  });

  describe('filteredCars', () => {
    beforeEach(async () => {
      const items = [
        makeDepotItem('PASSENGER_CAR', {
          id: 'pass-1',
          seriesCode: 'Bm-234',
          railwayCompanyName: 'SBB',
          productCode: 'C-100'
        }),
        makeDepotItem('FREIGHT_CAR', {
          id: 'freight-1',
          seriesCode: 'Eaos-106',
          railwayCompanyName: 'DB',
          productCode: 'C-200'
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();
    });

    it('returns all when query is empty', () => {
      expect(service.filteredCars).toHaveLength(2);
    });

    it('filters by seriesCode', () => {
      service.setQuery('eaos');
      expect(service.filteredCars).toHaveLength(1);
      expect(service.filteredCars[0].seriesCode).toBe('Eaos-106');
    });

    it('filters by railwayCompany', () => {
      service.setQuery('sbb');
      expect(service.filteredCars).toHaveLength(1);
      expect(service.filteredCars[0].seriesCode).toBe('Bm-234');
    });

    it('filters by type (friendlyName fallback)', async () => {
      const items = [
        makeDepotItem('PASSENGER_CAR', {
          id: 'pass-named',
          seriesCode: 'Bm-234',
          friendlyName: 'Dining Car'
        }),
        makeDepotItem('FREIGHT_CAR', {
          id: 'freight-named',
          seriesCode: 'Eaos-106',
          friendlyName: 'Coal Hopper'
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();

      service.setQuery('dining');
      expect(service.filteredCars).toHaveLength(1);
      expect(service.filteredCars[0].type).toBe('Dining Car');
    });

    it('returns empty when no match', () => {
      service.setQuery('zzznomatch');
      expect(service.filteredCars).toHaveLength(0);
    });
  });

  // ── totalFiltered ────────────────────────────────────────────

  describe('totalFiltered', () => {
    it('is 0 when depot is empty', () => {
      expect(service.totalFiltered).toBe(0);
    });

    it('sums filteredLocomotives + filteredTrains + filteredCars', async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', { id: 'l1', seriesCode: 'BR101' }),
        makeDepotItem('LOCOMOTIVE', { id: 'l2', seriesCode: 'BR141' }),
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', { id: 'e1', seriesCode: 'ICE3' }),
        makeDepotItem('PASSENGER_CAR', { id: 'p1', seriesCode: 'Bm-234' }),
        makeDepotItem('FREIGHT_CAR', { id: 'f1', seriesCode: 'Eaos-106' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();

      expect(service.totalFiltered).toBe(5);
    });

    it('reflects filtered count when query is set', async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', { id: 'l1', seriesCode: 'BR101', railwayCompanyName: 'DB' }),
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', {
          id: 'e1',
          seriesCode: 'ICE3',
          railwayCompanyName: 'DB'
        }),
        makeDepotItem('PASSENGER_CAR', {
          id: 'p1',
          seriesCode: 'Bm-234',
          railwayCompanyName: 'SBB'
        })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();

      service.setQuery('db');

      // One locomotive + one train match 'db'; passenger car does not
      expect(service.totalFiltered).toBe(2);
    });

    it('returns 0 when query matches nothing', async () => {
      const items = [
        makeDepotItem('LOCOMOTIVE', { id: 'l1', seriesCode: 'BR101' }),
        makeDepotItem('ELECTRIC_MULTIPLE_UNIT', { id: 'e1', seriesCode: 'ICE3' })
      ];
      setupInvokeMock({ get_depot: () => ({ rollingStocks: items }) });
      await service.load();

      service.setQuery('zzznomatch');

      expect(service.totalFiltered).toBe(0);
    });
  });
});
