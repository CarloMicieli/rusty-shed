import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$lib/shared/services/TauriAdapter', () => ({
  safeInvoke: vi.fn()
}));

vi.mock('$lib/shared/domain/errors', () => ({
  getErrorMessage: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    error: vi.fn()
  }
}));

import { getErrorMessage } from '$lib/shared/domain/errors';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { toaster } from '$lib/toaster';
import { DashboardService } from '$lib/features/dashboard/services/DashboardService.svelte';
import type { DashboardSummary, QueryCriteria } from '$lib/bindings';
import type { NormalizedError, SafeResult } from '$lib/shared/domain/errors';

const mockSafeInvoke = vi.mocked(safeInvoke);
const mockGetErrorMessage = vi.mocked(getErrorMessage);
const mockToaster = vi.mocked(toaster);

function makeSummary(overrides?: Partial<DashboardSummary>): DashboardSummary {
  return {
    totals: {
      collectionItems: 3,
      wishlists: 2,
      maintenanceDue: 1,
      totalValue: null
    },
    recentItems: [],
    purchaseGroups: [],
    ...overrides
  };
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((res) => {
    resolve = res;
  });

  return { promise, resolve };
}

describe('DashboardService', () => {
  let service: DashboardService;

  beforeEach(() => {
    service = new DashboardService();
    vi.resetAllMocks();
  });

  it('loads summary successfully with criteria and updates derived state', async () => {
    const criteria = {
      limit: 10
    } as unknown as QueryCriteria;
    const summary = makeSummary({
      totals: {
        collectionItems: 10,
        wishlists: 3,
        maintenanceDue: 2,
        totalValue: null
      },
      recentItems: [
        {
          id: 'item-1',
          title: 'V200',
          subtitle: null,
          source: 'Collection',
          createdAt: '2026-05-01T12:00:00Z'
        }
      ]
    });

    mockSafeInvoke.mockResolvedValue({ ok: true, data: summary });

    await service.load(criteria);

    expect(mockSafeInvoke).toHaveBeenCalledWith('get_dashboard_summary', { criteria });
    expect(service.data).toEqual(summary);
    expect(service.error).toBeNull();
    expect(service.isLoading).toBe(false);
    expect(service.hasMaintenance).toBe(true);
    expect(service.recentItemsCount).toBe(1);
  });

  it('sets error and toasts when loading fails', async () => {
    const normalizedError: NormalizedError = {
      kind: 'database',
      message: 'db unavailable',
      retryable: true
    };

    mockSafeInvoke.mockResolvedValue({
      ok: false,
      error: normalizedError
    } as SafeResult<DashboardSummary>);
    mockGetErrorMessage.mockReturnValue('Cannot load dashboard');

    await service.load();

    expect(service.error).toBe('dashboard_load_failed');
    expect(service.isLoading).toBe(false);
    expect(mockGetErrorMessage).toHaveBeenCalledWith(normalizedError);
    expect(mockToaster.error).toHaveBeenCalledWith({
      id: 'dashboard-error',
      title: 'Cannot load dashboard',
      duration: 4000
    });
  });

  it('ignores a second load while first request is in-flight', async () => {
    const pending = deferred<SafeResult<DashboardSummary>>();
    mockSafeInvoke.mockReturnValue(pending.promise);

    const firstLoad = service.load();
    const secondLoad = service.load();

    expect(service.isLoading).toBe(true);
    expect(mockSafeInvoke).toHaveBeenCalledTimes(1);

    pending.resolve({ ok: true, data: makeSummary() });
    await Promise.all([firstLoad, secondLoad]);

    expect(service.isLoading).toBe(false);
  });

  it('retry clears stale data before refreshing', async () => {
    mockSafeInvoke.mockResolvedValueOnce({
      ok: true,
      data: makeSummary({
        totals: {
          collectionItems: 2,
          wishlists: 1,
          maintenanceDue: 1,
          totalValue: null
        }
      })
    });

    await service.load();
    expect(service.data?.totals.collectionItems).toBe(2);

    const pending = deferred<SafeResult<DashboardSummary>>();
    mockSafeInvoke.mockReturnValueOnce(pending.promise);

    const retryPromise = service.retry();

    expect(service.data).toBeNull();

    pending.resolve({
      ok: true,
      data: makeSummary({
        totals: {
          collectionItems: 9,
          wishlists: 4,
          maintenanceDue: 0,
          totalValue: null
        }
      })
    });

    await retryPromise;

    expect(service.data?.totals.collectionItems).toBe(9);
  });

  it('supports snake_case maintenance_due fallback in derived state', async () => {
    const snakeCaseSummary = {
      totals: {
        collectionItems: 1,
        wishlists: 0,
        maintenance_due: 4,
        totalValue: null
      },
      recentItems: [],
      purchaseGroups: []
    } as unknown as DashboardSummary;

    mockSafeInvoke.mockResolvedValueOnce({ ok: true, data: snakeCaseSummary });
    await service.load();

    expect(service.hasMaintenance).toBe(true);

    mockSafeInvoke.mockResolvedValueOnce({
      ok: true,
      data: makeSummary({
        totals: {
          collectionItems: 1,
          wishlists: 0,
          maintenanceDue: 0,
          totalValue: null
        }
      })
    });

    await service.load();

    expect(service.hasMaintenance).toBe(false);
  });
});
