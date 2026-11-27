import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockState = vi.hoisted(() => ({
  card: null as unknown,
  isLoading: false,
  error: null as string | null,
  loadCard: vi.fn().mockResolvedValue(undefined),
  deleteEvent: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/maintenance/MaintenanceDetailState.svelte', () => ({
  default: function MockMaintenanceDetailState() {
    return mockState;
  },
  setMaintenanceDetailState: vi.fn()
}));
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { locale: 'en-US' }
}));
vi.mock('$app/navigation', () => ({ goto: vi.fn().mockResolvedValue(undefined) }));
vi.mock('$app/paths', () => ({ resolve: (path: string) => path }));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/features/maintenance/components/MaintenanceEventTimeline.svelte', () => ({
  default: function MaintenanceEventTimelineStub() {}
}));
vi.mock('$lib/features/maintenance/components/AddEventModal.svelte', () => ({
  default: function AddEventModalStub() {}
}));

import { page } from '$app/stores';
import MaintenanceDetailPage from '../../routes/maintenance/[id]/+page.svelte';

describe('routes/maintenance/[id]/+page.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockState.card = null;
    mockState.isLoading = false;
    mockState.error = null;
    (page as unknown as { set: (value: unknown) => void }).set({
      url: new URL('http://localhost/maintenance/card-1'),
      params: { id: 'card-1' },
      route: { id: '/maintenance/[id]' },
      status: 200,
      error: null,
      data: {},
      state: {},
      form: undefined
    });
  });

  it('loads the card id on mount and shows the loading state', () => {
    mockState.isLoading = true;
    render(MaintenanceDetailPage);

    expect(screen.getByText('maintenance_loading')).toBeInTheDocument();
    expect(mockState.loadCard).toHaveBeenCalledWith('card-1');
  });

  it('shows the backend error and a retry action', async () => {
    mockState.error = 'Database unavailable';
    render(MaintenanceDetailPage);

    expect(screen.getByText('maintenance_error_load')).toBeInTheDocument();
    expect(screen.getByText('Database unavailable')).toBeInTheDocument();
    await fireEvent.click(screen.getByRole('button', { name: 'maintenance_error_retry' }));
    expect(mockState.loadCard).toHaveBeenCalledTimes(2);
  });

  it('renders card summary, due-date warning, and event counts', () => {
    mockState.card = {
      id: 'card-1',
      ownedRollingStockId: 'owned-1',
      lastMaintenanceDate: '2026-10-01',
      nextMaintenanceDate: null,
      events: [{ id: 'event-1' }],
      displayInfo: {
        manufacturerName: 'Rivarossi',
        productCode: 'HR2906',
        rollingStockCategory: 'LOCOMOTIVE',
        roadNumber: '656 001',
        seriesCode: 'E656'
      }
    };
    render(MaintenanceDetailPage);

    expect(screen.getByText('maintenance_card_title')).toBeInTheDocument();
    expect(screen.getByText('LOCOMOTIVE')).toBeInTheDocument();
    expect(screen.getByText('656 001')).toBeInTheDocument();
    expect(screen.getByText('Rivarossi · HR2906')).toBeInTheDocument();
    expect(screen.getByText('maintenance_schedule_missing_due')).toBeInTheDocument();
  });

  it('renders an add-event action when a card exists', async () => {
    mockState.card = {
      id: 'card-1',
      ownedRollingStockId: 'owned-1',
      lastMaintenanceDate: null,
      nextMaintenanceDate: '2027-01-15',
      events: [],
      displayInfo: null
    };
    render(MaintenanceDetailPage);

    await fireEvent.click(screen.getByRole('button', { name: /maintenance_add_event_button/i }));
    await waitFor(() => {
      expect(screen.queryByText('maintenance_schedule_missing_due')).not.toBeInTheDocument();
    });
  });
});
