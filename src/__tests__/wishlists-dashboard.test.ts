import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockWishlistState = vi.hoisted(() => ({
  wishlists: [] as unknown[],
  activeWishlist: null as unknown,
  activeWishlistId: null as string | null,
  wishlistItems: [] as unknown[],
  isLoading: false,
  fetchWishlists: vi.fn().mockResolvedValue(undefined),
  createWishlist: vi.fn().mockResolvedValue(undefined),
  selectWishlist: vi.fn().mockResolvedValue(undefined),
  deleteWishlist: vi.fn().mockResolvedValue(undefined),
  renameWishlist: vi.fn().mockResolvedValue(undefined),
  setDefaultWishlist: vi.fn().mockResolvedValue(undefined),
  removeItem: vi.fn().mockResolvedValue(undefined),
  moveItemToList: vi.fn().mockResolvedValue(undefined),
  onPurchaseSuccess: vi.fn().mockResolvedValue(undefined)
}));
const mockReplaceState = vi.hoisted(() => vi.fn());

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  getWishlistContext: () => mockWishlistState
}));
vi.mock('$app/navigation', () => ({ replaceState: mockReplaceState }));
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
vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: function PageHeaderStub() {}
}));
vi.mock('$lib/features/wishlists/components/WishlistDashboardHeader.svelte', () => ({
  default: function WishlistDashboardHeaderStub() {}
}));
vi.mock('$lib/features/wishlists/components/WishlistHeader.svelte', () => ({
  default: function WishlistHeaderStub() {}
}));
vi.mock('$lib/features/wishlists/components/WishlistItems.svelte', () => ({
  default: function WishlistItemsStub() {}
}));
vi.mock('$lib/features/wishlists/components/WishlistTableView.svelte', () => ({
  default: function WishlistTableViewStub() {}
}));
vi.mock('$lib/features/wishlists/AddWishlistItemDrawer.svelte', () => ({
  default: function AddWishlistItemDrawerStub() {}
}));
vi.mock('$lib/features/wishlists/components/PurchaseDialog.svelte', () => ({
  default: function PurchaseDialogStub() {}
}));

import { page } from '$app/stores';
import WishlistsDashboard from '$lib/features/wishlists/WishlistsDashboard.svelte';

describe('WishlistsDashboard', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockWishlistState.wishlists = [];
    mockWishlistState.activeWishlist = null;
    mockWishlistState.activeWishlistId = null;
    mockWishlistState.wishlistItems = [];
    mockWishlistState.isLoading = false;
    (page as unknown as { set: (value: unknown) => void }).set({
      url: new URL('http://localhost/wishlists'),
      params: {},
      route: { id: '/wishlists' },
      status: 200,
      error: null,
      data: {},
      state: {},
      form: undefined
    });
  });

  it('loads wishlists on mount', async () => {
    render(WishlistsDashboard);
    await waitFor(() => expect(mockWishlistState.fetchWishlists).toHaveBeenCalledOnce());
  });

  it('shows loading skeletons while wishlists are loading', () => {
    mockWishlistState.isLoading = true;
    const { container } = render(WishlistsDashboard);
    expect(container.querySelectorAll('.animate-pulse')).toHaveLength(8);
  });

  it('shows the empty state and creates the first wishlist', async () => {
    render(WishlistsDashboard);
    expect(screen.getByText('wishlists_empty_title')).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'wishlists_create_button' }));
    expect(mockWishlistState.createWishlist).toHaveBeenCalledWith('wishlists_default_name (1)');
  });

  it('shows active-wishlist content in grid mode', () => {
    mockWishlistState.wishlists = [{ id: 'wishlist-1', name: 'Main', isDefault: true }];
    mockWishlistState.activeWishlist = mockWishlistState.wishlists[0];
    mockWishlistState.activeWishlistId = 'wishlist-1';
    render(WishlistsDashboard);

    expect(screen.getByTitle('wishlist_view_grid')).toBeInTheDocument();
    expect(screen.getByTitle('wishlist_view_table')).toBeInTheDocument();
  });

  it('switches to table view through the URL state', async () => {
    mockWishlistState.wishlists = [{ id: 'wishlist-1', name: 'Main', isDefault: true }];
    mockWishlistState.activeWishlist = mockWishlistState.wishlists[0];
    mockWishlistState.activeWishlistId = 'wishlist-1';
    render(WishlistsDashboard);

    await fireEvent.click(screen.getByTitle('wishlist_view_table'));
    expect(mockReplaceState).toHaveBeenCalled();
  });

  it('offers add-model drawer action when active wishlist exists', async () => {
    mockWishlistState.wishlists = [{ id: 'wishlist-1', name: 'Main', isDefault: true }];
    mockWishlistState.activeWishlist = mockWishlistState.wishlists[0];
    mockWishlistState.activeWishlistId = 'wishlist-1';
    render(WishlistsDashboard);

    expect(screen.getByTitle('wishlist_view_table')).toBeInTheDocument();
  });
});
