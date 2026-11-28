import { beforeEach, describe, expect, it, vi } from 'vitest';

const safeInvokeMock = vi.hoisted(() => vi.fn());

vi.mock('$lib/services/tauri', () => ({
  safeInvoke: (...args: unknown[]) => safeInvokeMock(...args)
}));

import { fetchDbStats, fetchRecentLogs } from '$lib/services/debug';

describe('debug service', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('fetchDbStats invokes get_db_stats', async () => {
    safeInvokeMock.mockResolvedValueOnce({ ok: true, data: [] });

    await fetchDbStats();

    expect(safeInvokeMock).toHaveBeenCalledWith('get_db_stats');
  });

  it('fetchRecentLogs uses default and explicit limits', async () => {
    safeInvokeMock.mockResolvedValue({ ok: true, data: [] });

    await fetchRecentLogs();
    await fetchRecentLogs(25);

    expect(safeInvokeMock).toHaveBeenNthCalledWith(1, 'get_recent_logs', { limit: 100 });
    expect(safeInvokeMock).toHaveBeenNthCalledWith(2, 'get_recent_logs', { limit: 25 });
  });
});
