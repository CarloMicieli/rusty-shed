import { describe, it, expect, vi, beforeEach } from 'vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';

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

const { mockDashboardState, mockWishlistState } = vi.hoisted(() => ({
  mockDashboardState: {
    isLoading: false,
    data: null as null | { totals: null; purchaseGroups: [] },
    budgetData: null as null | object,
    load: vi.fn().mockResolvedValue(undefined),
    loadBudget: vi.fn().mockResolvedValue(undefined)
  },
  mockWishlistState: {
    wishlists: [] as unknown[],
    activeWishlist: null as unknown,
    activeWishlistId: null as string | null,
    wishlistItems: [] as unknown[],
    isLoading: false,
    fetchWishlists: vi.fn().mockResolvedValue(undefined),
    createWishlist: vi.fn(),
    addRailwayModel: vi.fn()
  }
}));

vi.mock('$lib/features/dashboard/DashboardState.svelte', () => ({
  getDashboardContext: vi.fn(() => mockDashboardState),
  setDashboardContext: vi.fn(),
  createDashboardState: vi.fn(() => mockDashboardState)
}));

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  getWishlistContext: vi.fn(() => mockWishlistState),
  setWishlistContext: vi.fn(),
  createWishlistState: vi.fn(() => mockWishlistState)
}));

// Stub heavy UI children
vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: function PageHeaderStub() {}
}));
vi.mock('$lib/components/StatsCard.svelte', () => ({
  default: function StatsCardStub() {}
}));
vi.mock('$lib/features/dashboard', () => ({
  DashboardCharts: function DashboardChartsStub() {},
  PurchaseGroupCard: function PurchaseGroupCardStub() {}
}));
vi.mock('$lib/features/dashboard/components/DashboardSectionHeader.svelte', () => ({
  default: function DashboardSectionHeaderStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import DashboardPageHarness from './DashboardPageHarness.svelte';

describe('routes/dashboard/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockDashboardState.isLoading = false;
    mockDashboardState.data = null;
    mockDashboardState.budgetData = null;
  });

  it('renders without throwing', () => {
    expect(() => render(DashboardPageHarness)).not.toThrow();
  });

  it('shows a loading skeleton while isLoading is true', () => {
    mockDashboardState.isLoading = true;
    const { container } = render(DashboardPageHarness);
    // Loading state renders skeleton divs
    const skeletons = container.querySelectorAll('.skeleton, .animate-pulse, [class*="animate"]');
    expect(skeletons.length).toBeGreaterThan(0);
  });

  it('shows empty-acquisitions message when data has no purchase groups', async () => {
    mockDashboardState.isLoading = false;
    mockDashboardState.data = { totals: null, purchaseGroups: [] };
    render(DashboardPageHarness);
    await waitFor(() => {
      expect(screen.getByText('dashboard_empty_acquisitions')).toBeInTheDocument();
    });
  });

  it('opens acquisition drawer from command center action', async () => {
    const openAcquisitionDrawer = vi.fn();

    render(DashboardPageHarness, { openAcquisitionDrawer });

    const buttons = screen.getAllByRole('button', { name: 'dashboard_action_new_acquisition' });
    await fireEvent.click(buttons[0]);

    expect(openAcquisitionDrawer).toHaveBeenCalledTimes(1);
  });

  it('opens wishlist drawer and preloads wishlists when empty', async () => {
    const openWishlistDrawer = vi.fn();
    mockWishlistState.wishlists = [];

    render(DashboardPageHarness, { openWishlistDrawer });

    const buttons = screen.getAllByRole('button', { name: 'actions_add_wishlist_item' });
    await fireEvent.click(buttons[0]);

    expect(mockWishlistState.fetchWishlists).toHaveBeenCalledTimes(1);
    expect(openWishlistDrawer).toHaveBeenCalledTimes(1);
  });
});
