import { beforeEach, describe, expect, it, vi } from 'vitest';

const safeInvokeMock = vi.hoisted(() => vi.fn());
vi.mock('$lib/services', () => ({
  safeInvoke: safeInvokeMock,
  getErrorMessage: (error: { message: string }) => error.message
}));

import { createDepotState } from '$lib/features/depot/DepotState.svelte';
import type { DepotView, DepotRollingStockView } from '$lib/bindings';

function makeItem(
  category: DepotRollingStockView['category'],
  overrides: Partial<DepotRollingStockView> = {}
): DepotRollingStockView {
  return {
    id: `item-${category}`,
    railwayModelId: 'model-1',
    seriesCode: 'SERIES-1',
    series: null,
    roadNumber: '001',
    friendlyName: null,
    depot: null,
    category,
    manufacturerName: 'Roco',
    productCode: 'PC-001',
    control: 'DCC_FITTED',
    livery: 'Green',
    railwayCompanyName: 'FS',
    epoch: 'VI',
    dccAddress: 42,
    ...overrides
  };
}

function makeDepot(items: DepotRollingStockView[]): DepotView {
  return { rollingStocks: items } as DepotView;
}

describe('DepotState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('starts empty and uses table view', () => {
    const state = createDepotState();

    expect(state.isLoading).toBe(false);
    expect(state.error).toBeNull();
    expect(state.query).toBe('');
    expect(state.viewMode).toBe('table');
    expect(state.totalFiltered).toBe(0);
  });

  it('loads and categorizes locomotives, trainsets, and cars', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: true,
      data: makeDepot([
        makeItem('LOCOMOTIVE', { roadNumber: 'Loco 101' }),
        makeItem('ELECTRIC_MULTIPLE_UNIT', { friendlyName: 'ALe 803' }),
        makeItem('RAILCAR', { seriesCode: 'Railcar' }),
        makeItem('PASSENGER_CAR', { friendlyName: 'Gran Comfort' }),
        makeItem('FREIGHT_CAR', { roadNumber: 'F-200' })
      ])
    });
    const state = createDepotState();

    await state.load();

    expect(state.locomotives).toHaveLength(1);
    expect(state.railcarsEmuDmu).toHaveLength(2);
    expect(state.passengerCars).toHaveLength(1);
    expect(state.freightCars).toHaveLength(1);
    expect(state.totalFiltered).toBe(5);
    expect(state.isLoading).toBe(false);
  });

  it('maps category labels and friendly names for display', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: true,
      data: makeDepot([
        makeItem('LOCOMOTIVE', { friendlyName: 'Caimano' }),
        makeItem('PASSENGER_CAR', { friendlyName: null, seriesCode: 'UIC-X' })
      ])
    });
    const state = createDepotState();

    await state.load();

    expect(state.locomotives[0]).toMatchObject({
      group: 'Caimano',
      categoryLabel: 'Locomotive',
      dccAddress: 42
    });
    expect(state.passengerCars[0]).toMatchObject({
      type: 'UIC-X',
      category: 'passenger',
      categoryLabel: 'Passenger Car'
    });
  });

  it('filters every category without changing the underlying groups', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: true,
      data: makeDepot([
        makeItem('LOCOMOTIVE', { railwayCompanyName: 'FS', seriesCode: 'E656' }),
        makeItem('ELECTRIC_MULTIPLE_UNIT', { railwayCompanyName: 'SBB', seriesCode: 'Re 450' }),
        makeItem('PASSENGER_CAR', {
          friendlyName: 'Gran Comfort',
          seriesCode: 'UIC-X',
          railwayCompanyName: null
        }),
        makeItem('FREIGHT_CAR', { roadNumber: 'F-200', railwayCompanyName: null })
      ])
    });
    const state = createDepotState();
    await state.load();

    state.setQuery('fs');
    expect(state.filteredLocomotives).toHaveLength(1);
    expect(state.filteredRailcarsEmuDmu).toHaveLength(0);
    expect(state.totalFiltered).toBe(1);

    state.setQuery('comfort');
    expect(state.filteredPassengerCars).toHaveLength(1);

    state.clearQuery();
    expect(state.query).toBe('');
    expect(state.totalFiltered).toBe(4);
  });

  it('switches view mode independently of query state', async () => {
    const state = createDepotState();
    state.setQuery('E656');

    state.setViewMode('grid');
    expect(state.viewMode).toBe('grid');
    expect(state.query).toBe('E656');

    state.setViewMode('table');
    expect(state.viewMode).toBe('table');
  });

  it('records normalized backend failures and resets loading', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: false,
      error: { kind: 'database', message: 'Depot unavailable' }
    });
    const state = createDepotState();

    await state.load();

    expect(state.error).toBe('Depot unavailable');
    expect(state.isLoading).toBe(false);
    expect(state.totalFiltered).toBe(0);
  });

  it('converts thrown JavaScript failures to an error message', async () => {
    safeInvokeMock.mockRejectedValue(new Error('IPC crashed'));
    const state = createDepotState();

    await state.load();

    expect(state.error).toBe('IPC crashed');
    expect(state.isLoading).toBe(false);
  });
});
