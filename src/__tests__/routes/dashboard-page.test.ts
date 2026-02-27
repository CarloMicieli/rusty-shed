import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', () => new Proxy({}, { get: (_t, k) => () => String(k) }));

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
  default: { name: 'PageHeaderStub', _: { props: [] } }
}));
vi.mock('$lib/components/StatsCard.svelte', () => ({
  default: { name: 'StatsCardStub', _: { props: [] } }
}));
vi.mock('$lib/components/QuickActionButtons.svelte', () => ({
  default: { name: 'QuickActionButtonsStub', _: { props: [] } }
}));
vi.mock('$lib/components/AddWishlistItemModal.svelte', () => ({
  default: { name: 'AddWishlistItemModalStub', _: { props: [] } }
}));
vi.mock('$lib/features/dashboard', () => ({
  DashboardCharts: { name: 'DashboardChartsStub', _: { props: [] } },
  PurchaseGroupCard: { name: 'PurchaseGroupCardStub', _: { props: [] } }
}));
vi.mock('$lib/features/dashboard/components/DashboardAction.svelte', () => ({
  default: { name: 'DashboardActionStub', _: { props: [] } }
}));
vi.mock('$lib/features/dashboard/components/DashboardSectionHeader.svelte', () => ({
  default: { name: 'DashboardSectionHeaderStub', _: { props: [] } }
}));

// ── Test target ───────────────────────────────────────────────

import DashboardPage from '../../routes/dashboard/+page.svelte';

describe('routes/dashboard/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockDashboardState.isLoading = false;
    mockDashboardState.data = null;
    mockDashboardState.budgetData = null;
  });

  it('renders without throwing', () => {
    expect(() => render(DashboardPage)).not.toThrow();
  });

  it('shows a loading skeleton while isLoading is true', () => {
    mockDashboardState.isLoading = true;
    const { container } = render(DashboardPage);
    // Loading state renders skeleton divs
    const skeletons = container.querySelectorAll('.skeleton, .animate-pulse, [class*="animate"]');
    expect(skeletons.length).toBeGreaterThan(0);
  });

  it('shows empty-acquisitions message when data has no purchase groups', async () => {
    mockDashboardState.isLoading = false;
    mockDashboardState.data = { totals: null, purchaseGroups: [] };
    render(DashboardPage);
    await waitFor(() => {
      expect(screen.getByText('dashboard_empty_acquisitions')).toBeInTheDocument();
    });
  });
});
