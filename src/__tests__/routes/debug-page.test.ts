import { beforeEach, describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';

const { mockFetchDbStats, mockFetchRecentLogs } = vi.hoisted(() => ({
  mockFetchDbStats: vi.fn(),
  mockFetchRecentLogs: vi.fn()
}));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

vi.mock('$lib/services', () => ({
  fetchDbStats: mockFetchDbStats,
  fetchRecentLogs: mockFetchRecentLogs
}));

vi.mock('$lib/services/errors', () => ({
  getToastMessage: vi.fn((error: unknown) => String(error))
}));

import DebugPage from '../../routes/debug/+page.svelte';

const MOCK_DB_STATS = [
  {
    tableName: 'collection_items',
    rowCount: 42,
    estimatedBytes: 2048
  }
];

describe('routes/debug/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('shows loading placeholders while debug data is being fetched', () => {
    mockFetchDbStats.mockImplementation(() => new Promise(() => {}));
    mockFetchRecentLogs.mockImplementation(() => new Promise(() => {}));

    const { container } = render(DebugPage);

    expect(container.querySelector('.animate-pulse')).not.toBeNull();
  });

  it('renders database stats and recent log lines after a successful load', async () => {
    mockFetchDbStats.mockResolvedValue({ ok: true, data: MOCK_DB_STATS });
    mockFetchRecentLogs.mockResolvedValue({ ok: true, data: ['log one', 'log two'] });

    render(DebugPage);

    await waitFor(() => {
      expect(screen.getByText('collection_items')).toBeInTheDocument();
    });

    expect(screen.getByText('2.0 KB')).toBeInTheDocument();
    expect(screen.getByText('log one')).toBeInTheDocument();
    expect(screen.getByText('log two')).toBeInTheDocument();
  });

  it('shows retry controls when either debug panel fails to load', async () => {
    mockFetchDbStats.mockResolvedValue({ ok: false, error: 'db unavailable' });
    mockFetchRecentLogs.mockResolvedValue({ ok: false, error: 'log unavailable' });

    render(DebugPage);

    await waitFor(() => {
      expect(screen.getAllByText('errors_retry_page')).toHaveLength(2);
    });
  });

  it('refreshes log lines when the refresh button is pressed', async () => {
    mockFetchDbStats.mockResolvedValue({ ok: true, data: MOCK_DB_STATS });
    mockFetchRecentLogs
      .mockResolvedValueOnce({ ok: true, data: ['initial log'] })
      .mockResolvedValueOnce({ ok: true, data: ['refreshed log'] });

    render(DebugPage);

    await waitFor(() => {
      expect(screen.getByText('initial log')).toBeInTheDocument();
    });

    await fireEvent.click(screen.getByText('debug_logs_refresh'));

    await waitFor(() => {
      expect(mockFetchRecentLogs).toHaveBeenCalledTimes(2);
      expect(screen.getByText('refreshed log')).toBeInTheDocument();
    });
  });
});
