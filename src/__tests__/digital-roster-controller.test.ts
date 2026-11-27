import { beforeEach, describe, expect, it, vi } from 'vitest';

const commandMocks = vi.hoisted(() => ({
  getDigitalSummary: vi.fn(),
  getDigitalRollingStocks: vi.fn(),
  changeDccAddress: vi.fn(),
  checkDccAddressDuplicate: vi.fn(),
  newDigitalRollingStock: vi.fn(),
  changeDecoder: vi.fn()
}));
const toasterMocks = vi.hoisted(() => ({
  success: vi.fn(),
  error: vi.fn()
}));

vi.mock('$lib/bindings', () => ({ commands: commandMocks }));
vi.mock('$lib/toaster', () => ({ toaster: toasterMocks }));

import { DigitalRosterController } from '$lib/features/digital-roster/DigitalRosterController.svelte';
import { DigitalRosterState } from '$lib/features/digital-roster/DigitalRosterState.svelte';

describe('DigitalRosterController', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('loads summary and clears loading/error state', async () => {
    const summary = { totalCount: 5, assignedCount: 3, availableAddresses: 97 };
    commandMocks.getDigitalSummary.mockResolvedValue(summary);
    const state = new DigitalRosterState();
    state.setError('old error');
    const controller = new DigitalRosterController(state);

    await controller.loadSummary();

    expect(state.summary).toEqual(summary);
    expect(state.error).toBeNull();
    expect(state.isLoading).toBe(false);
  });

  it('loads rolling stocks and handles command failures', async () => {
    commandMocks.getDigitalRollingStocks
      .mockResolvedValueOnce([{ id: 'stock-1', dcc_address: 10 }])
      .mockRejectedValueOnce(new Error('IPC failed'));
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await controller.loadRollingStocks();
    expect(state.rollingStocks).toEqual([{ id: 'stock-1', dcc_address: 10 }]);

    await controller.loadRollingStocks();
    expect(state.isLoading).toBe(false);
    expect(toasterMocks.error).toHaveBeenCalled();
  });

  it('loads summary and stocks together', async () => {
    const summary = { totalCount: 2, assignedCount: 1, availableAddresses: 98 };
    const stocks = [{ id: 'stock-1', dcc_address: 10 }];
    commandMocks.getDigitalSummary.mockResolvedValue(summary);
    commandMocks.getDigitalRollingStocks.mockResolvedValue(stocks);
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await controller.loadAll();

    expect(state.summary).toEqual(summary);
    expect(state.rollingStocks).toEqual(stocks);
    expect(state.isLoading).toBe(false);
  });

  it('handles partial loadAll failures', async () => {
    commandMocks.getDigitalSummary.mockResolvedValue({
      totalCount: 2,
      assignedCount: 1,
      availableAddresses: 98
    });
    commandMocks.getDigitalRollingStocks.mockRejectedValue(new Error('Stock IPC failed'));
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await controller.loadAll();

    expect(state.isLoading).toBe(false);
    expect(toasterMocks.error).toHaveBeenCalledWith(
      expect.objectContaining({
        title: 'An error occurred while loading data',
        description: 'Stock IPC failed'
      })
    );
  });

  it('updates and clears the filter', () => {
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    controller.updateFilter('42');
    expect(state.filterText).toBe('42');

    controller.clearFilter();
    expect(state.filterText).toBe('');
  });

  it('changes DCC address, reloads stocks, and reports success', async () => {
    commandMocks.changeDccAddress.mockResolvedValue(undefined);
    commandMocks.getDigitalRollingStocks.mockResolvedValue([{ id: 'stock-1', dcc_address: 42 }]);
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await expect(controller.changeDccAddress('stock-1', 42)).resolves.toBe(true);

    expect(commandMocks.changeDccAddress).toHaveBeenCalledWith({
      id: 'stock-1',
      newDccAddress: 42
    });
    expect(state.rollingStocks).toEqual([{ id: 'stock-1', dcc_address: 42 }]);
    expect(toasterMocks.success).toHaveBeenCalled();
  });

  it('returns false and shows an error when DCC address changes fail', async () => {
    commandMocks.changeDccAddress.mockRejectedValue(new Error('Address conflict'));
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await expect(controller.changeDccAddress('stock-1', 42)).resolves.toBe(false);
    expect(toasterMocks.error).toHaveBeenCalled();
  });

  it('maps duplicate-address results and handles lookup exceptions', async () => {
    commandMocks.checkDccAddressDuplicate
      .mockResolvedValueOnce({
        is_duplicate: true,
        existing_rolling_stock_id: 'stock-9'
      })
      .mockRejectedValueOnce(new Error('Lookup failed'));
    const controller = new DigitalRosterController(new DigitalRosterState());

    await expect(controller.checkDuplicateAddress(42, 'stock-1')).resolves.toEqual({
      isDuplicate: true,
      existingId: 'stock-9'
    });
    expect(commandMocks.checkDccAddressDuplicate).toHaveBeenCalledWith({
      dccAddress: 42,
      excludeId: 'stock-1'
    });

    await expect(controller.checkDuplicateAddress(43)).resolves.toEqual({
      isDuplicate: false
    });
  });

  it('installs a decoder and reloads summary and stocks', async () => {
    commandMocks.newDigitalRollingStock.mockResolvedValue(undefined);
    commandMocks.getDigitalSummary.mockResolvedValue({
      totalCount: 3,
      assignedCount: 2,
      availableAddresses: 97
    });
    commandMocks.getDigitalRollingStocks.mockResolvedValue([{ id: 'stock-1', dcc_address: 42 }]);
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await expect(controller.installDecoder('owned-1', 'decoder-1', 42)).resolves.toBe(true);
    expect(commandMocks.newDigitalRollingStock).toHaveBeenCalledWith({
      ownedRollingStockId: 'owned-1',
      decoderId: 'decoder-1',
      dccAddress: 42
    });
    expect(state.summary).not.toBeNull();
  });

  it('replaces a decoder and reports failure without stale success', async () => {
    commandMocks.changeDecoder
      .mockResolvedValueOnce(undefined)
      .mockRejectedValueOnce(new Error('Replace failed'));
    commandMocks.getDigitalSummary.mockResolvedValue({
      totalCount: 3,
      assignedCount: 2,
      availableAddresses: 97
    });
    commandMocks.getDigitalRollingStocks.mockResolvedValue([{ id: 'stock-1', dcc_address: 42 }]);
    const state = new DigitalRosterState();
    const controller = new DigitalRosterController(state);

    await expect(controller.replaceDecoder('stock-1', 'decoder-2')).resolves.toBe(true);
    await expect(controller.replaceDecoder('stock-1', 'decoder-3')).resolves.toBe(false);

    expect(toasterMocks.error).toHaveBeenCalled();
  });
});
