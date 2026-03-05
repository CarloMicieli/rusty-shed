import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

// Mutable shared state object so individual tests can tweak behaviour
const mockStateInstance = vi.hoisted(() => ({
  cards: [] as {
    id: string;
    name: string;
    nextMaintenanceDate: string | null;
    events: unknown[];
  }[],
  isLoading: false,
  error: null as string | null,
  hasCards: false,
  loadDashboard: vi.fn().mockResolvedValue(undefined),
  retry: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/maintenance/MaintenanceState.svelte', () => ({
  default: function MockMaintenanceState() {
    return mockStateInstance;
  },
  setMaintenanceState: vi.fn(),
  getMaintenanceState: vi.fn(() => mockStateInstance)
}));

// Stub complex children
vi.mock('$lib/features/maintenance/components/MaintenanceCardList.svelte', () => ({
  default: function MaintenanceCardListStub() {}
}));
vi.mock('$lib/features/maintenance/components/EmptyMaintenanceState.svelte', () => ({
  default: function EmptyMaintenanceStateStub() {}
}));
vi.mock('$lib/features/maintenance/components/AddMaintenanceCardModal.svelte', () => ({
  default: function AddMaintenanceCardModalStub() {}
}));
vi.mock('$lib/features/maintenance/components/AddMaintenanceEventModal.svelte', () => ({
  default: function AddMaintenanceEventModalStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import MaintenancePage from '../../routes/maintenance/+page.svelte';

describe('routes/maintenance/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockStateInstance.cards = [];
    mockStateInstance.isLoading = false;
    mockStateInstance.error = null;
    mockStateInstance.hasCards = false;
    mockStateInstance.loadDashboard = vi.fn().mockResolvedValue(undefined);
  });

  it('renders without throwing', () => {
    expect(() => render(MaintenancePage)).not.toThrow();
  });

  it('shows a loading spinner while data is loading', () => {
    mockStateInstance.isLoading = true;
    const { container } = render(MaintenancePage);
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).not.toBeNull();
  });

  it('shows the loading message text while isLoading', () => {
    mockStateInstance.isLoading = true;
    render(MaintenancePage);
    expect(screen.getByText('maintenance_loading')).toBeInTheDocument();
  });

  it('shows an error state with retry when error is set', () => {
    mockStateInstance.error = 'Database error';
    render(MaintenancePage);
    expect(screen.getByText('maintenance_error_load')).toBeInTheDocument();
    expect(screen.getByText('maintenance_error_retry')).toBeInTheDocument();
  });

  it('shows the empty state when hasCards is false and not loading', () => {
    mockStateInstance.isLoading = false;
    mockStateInstance.hasCards = false;
    render(MaintenancePage);
    // EmptyMaintenanceState stub is rendered — just check no loading spinner
    const spinner = document.querySelector('.animate-spin');
    expect(spinner).toBeNull();
  });

  it('renders stats grid when cards exist', () => {
    mockStateInstance.hasCards = true;
    mockStateInstance.cards = [
      {
        id: 'card-1',
        name: 'BR 218 Overhaul',
        nextMaintenanceDate: '2027-01-01',
        events: [{}]
      }
    ];
    const { container } = render(MaintenancePage);
    // The stats grid is conditionally rendered when hasCards is true
    // It contains 3 stat boxes (Active, Upcoming, Completed)
    const statBoxes = container.querySelectorAll('[class*="font-mono text-2xl"]');
    expect(statBoxes.length).toBeGreaterThanOrEqual(1);
  });

  it('calls loadDashboard on mount', async () => {
    render(MaintenancePage);
    await waitFor(() => {
      expect(mockStateInstance.loadDashboard).toHaveBeenCalledOnce();
    });
  });

  it('shows the page title', () => {
    render(MaintenancePage);
    expect(screen.getByText('maintenance_title')).toBeInTheDocument();
  });

  it('shows action buttons in the header', () => {
    render(MaintenancePage);
    expect(screen.getByText('maintenance_add_card_button')).toBeInTheDocument();
  });
});
