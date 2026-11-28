import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { BudgetDashboardSummary } from '$lib/bindings';

function makeSummary(seed: number): BudgetDashboardSummary {
  return {
    totalSpent: seed
  } as unknown as BudgetDashboardSummary;
}

function createDeferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (reason?: unknown) => void;

  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });

  return { promise, resolve, reject };
}

async function loadFinanceState(options: { browser: boolean }) {
  vi.resetModules();

  const invokeMock = vi.fn();
  const listenMock = vi.fn();

  vi.doMock('$app/environment', () => ({
    browser: options.browser,
    building: false,
    dev: false,
    version: 'test'
  }));

  vi.doMock('@tauri-apps/api/core', () => ({
    invoke: invokeMock
  }));

  vi.doMock('@tauri-apps/api/event', () => ({
    listen: listenMock
  }));

  const module = await import('$lib/state/finance.svelte');

  return {
    financeState: module.financeState,
    invokeMock,
    listenMock
  };
}

describe('financeState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('ensureLoaded uses cached data after first load', async () => {
    const firstSummary = makeSummary(1);
    const { financeState, invokeMock } = await loadFinanceState({ browser: true });
    invokeMock.mockResolvedValueOnce(firstSummary);

    const firstResult = await financeState.ensureLoaded();
    const secondResult = await financeState.ensureLoaded();

    expect(firstResult).toEqual(firstSummary);
    expect(secondResult).toEqual(firstSummary);
    expect(invokeMock).toHaveBeenCalledTimes(1);
  });

  it('refresh forces a second backend load even when data is cached', async () => {
    const firstSummary = makeSummary(1);
    const refreshedSummary = makeSummary(2);
    const { financeState, invokeMock } = await loadFinanceState({ browser: true });

    invokeMock.mockResolvedValueOnce(firstSummary).mockResolvedValueOnce(refreshedSummary);

    await financeState.ensureLoaded();
    const refreshedResult = await financeState.refresh();

    expect(refreshedResult).toEqual(refreshedSummary);
    expect(financeState.data).toEqual(refreshedSummary);
    expect(invokeMock).toHaveBeenCalledTimes(2);
  });

  it('deduplicates in-flight ensureLoaded requests', async () => {
    const summary = makeSummary(3);
    const deferred = createDeferred<BudgetDashboardSummary>();
    const { financeState, invokeMock } = await loadFinanceState({ browser: true });

    invokeMock.mockReturnValueOnce(deferred.promise);

    const firstRequest = financeState.ensureLoaded();
    const secondRequest = financeState.ensureLoaded();

    expect(financeState.loading).toBe(true);
    expect(invokeMock).toHaveBeenCalledTimes(1);

    deferred.resolve(summary);

    await expect(firstRequest).resolves.toEqual(summary);
    await expect(secondRequest).resolves.toEqual(summary);
    expect(financeState.loading).toBe(false);
  });

  it('captures invoke failures in error and returns null', async () => {
    const { financeState, invokeMock } = await loadFinanceState({ browser: true });

    invokeMock.mockRejectedValueOnce(new Error('budget load failed'));

    await expect(financeState.ensureLoaded()).resolves.toBeNull();

    expect(financeState.error).toBe('budget load failed');
    expect(financeState.loading).toBe(false);
    expect(financeState.hasFetched).toBe(false);
  });

  it('startListening is a no-op outside browser', async () => {
    const { financeState, listenMock } = await loadFinanceState({ browser: false });

    await financeState.startListening();
    financeState.stopListening();

    expect(listenMock).not.toHaveBeenCalled();
  });

  it('prevents duplicate listeners and supports stop/start lifecycle', async () => {
    const initialSummary = makeSummary(4);
    const refreshedSummary = makeSummary(5);
    const unlistenMock = vi.fn();
    const { financeState, invokeMock, listenMock } = await loadFinanceState({ browser: true });

    invokeMock.mockResolvedValueOnce(initialSummary).mockResolvedValueOnce(refreshedSummary);
    listenMock.mockImplementationOnce(async (_event: string, callback: () => void) => {
      setTimeout(() => callback(), 0);
      return unlistenMock;
    });
    listenMock.mockResolvedValueOnce(unlistenMock);

    await financeState.ensureLoaded();

    await financeState.startListening();
    await financeState.startListening();

    expect(listenMock).toHaveBeenCalledTimes(1);

    await vi.waitFor(() => {
      expect(invokeMock).toHaveBeenCalledTimes(2);
    });

    financeState.stopListening();
    expect(unlistenMock).toHaveBeenCalledTimes(1);

    await financeState.startListening();
    expect(listenMock).toHaveBeenCalledTimes(2);
  });
});
