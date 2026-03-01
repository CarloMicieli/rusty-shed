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

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: vi.fn(() => 'en'),
  setLocale: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

const mockCommands = vi.hoisted(() => ({
  getWishlistById: vi.fn(),
  getRailwayModelById: vi.fn(),
  getRailwayModelImage: vi.fn()
}));

vi.mock('$lib/bindings', () => ({
  commands: mockCommands
}));

vi.mock('$lib/features/collection/utils/modelViewMapper', () => ({
  toRailwayModel: vi.fn((model: unknown) => model)
}));

// Stub heavy child components
vi.mock('$lib/components/RailwayModelCard.svelte', () => ({
  default: function RailwayModelCardStub() {}
}));
vi.mock('$lib/features/wishlists/components/WishlistItemSidebar.svelte', () => ({
  default: function WishlistItemSidebarStub() {}
}));
vi.mock('$lib/features/wishlists/components/PurchaseDialog.svelte', () => ({
  default: function PurchaseDialogStub() {}
}));

// ── Explicit $app/stores mock ─────────────────────────────────

const WISHLIST_ID = 'wl-001';
const ITEM_ID = 'item-001';
const MODEL_ID = 'trn:railway-model:acme:60100';

// Use the alias mock directly (vitest.config resolves $app →
// src/__tests__/mocks/sveltekit which exports writable stores)
import { page as mockPageStoreRaw } from '$app/stores';

// Cast to any to bypass strict SvelteKit types in tests

const mockPageStore = mockPageStoreRaw as any;

vi.mock('$app/navigation', () => ({
  goto: vi.fn().mockResolvedValue(undefined)
}));

// ── Test target ───────────────────────────────────────────────
// The path contains literal square brackets as required by SvelteKit routing.
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore — square-bracket directory names are valid SvelteKit routes
import WishlistItemPage from '../../routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte';

const mockWishlistView = {
  id: WISHLIST_ID,
  name: 'My Wish List',
  items: [
    {
      id: ITEM_ID,
      railway_model_id: MODEL_ID,
      priority: 'NORMAL',
      status: 'WANTED',
      added_date: '2026-01-15',
      removed_date: null,
      notes: null,
      desired_price: null,
      purchased_price: null
    }
  ]
};

describe('routes/wishlists/[wishlistId]/items/[itemId]/+page.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockPageStore.set({
      url: new URL(`http://localhost/wishlists/${WISHLIST_ID}/items/${ITEM_ID}`),
      params: { wishlistId: WISHLIST_ID, itemId: ITEM_ID },
      route: { id: '/wishlists/[wishlistId]/items/[itemId]' },
      status: 200,
      error: null,
      data: {},
      state: {},
      form: undefined
    });
  });

  it('renders the loading state initially', () => {
    // Delay the command so loading stays true during the initial synchronous render
    mockCommands.getWishlistById.mockImplementation(() => new Promise(() => {}));
    render(WishlistItemPage);
    expect(screen.getByText('wishlist_item_loading')).toBeInTheDocument();
  });

  it('shows not-found state when wishlist is missing', async () => {
    mockCommands.getWishlistById.mockResolvedValue({ status: 'error', error: 'not found' });
    render(WishlistItemPage);
    await waitFor(() => expect(screen.getByText('wishlist_item_not_found')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('shows not-found state when itemId is missing from wishlist items', async () => {
    mockCommands.getWishlistById.mockResolvedValue({
      status: 'ok',
      data: { ...mockWishlistView, items: [] }
    });
    render(WishlistItemPage);
    await waitFor(() => expect(screen.getByText('wishlist_item_not_found')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('shows error state when an unexpected error is thrown', async () => {
    mockCommands.getWishlistById.mockRejectedValue(new Error('Network failure'));
    render(WishlistItemPage);
    await waitFor(() => expect(screen.getByText('Network failure')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('renders the back button after a successful data load', async () => {
    mockCommands.getWishlistById.mockResolvedValue({ status: 'ok', data: mockWishlistView });
    mockCommands.getRailwayModelById.mockResolvedValue({ status: 'ok', data: null });
    mockCommands.getRailwayModelImage.mockResolvedValue({ status: 'ok', data: null });
    render(WishlistItemPage);
    await waitFor(() => expect(screen.getByText('wishlist_item_back')).toBeInTheDocument(), {
      timeout: 2000
    });
  });

  it('uses the wishlistId param from the $page store when loading', async () => {
    mockCommands.getWishlistById.mockResolvedValue({ status: 'ok', data: mockWishlistView });
    mockCommands.getRailwayModelById.mockResolvedValue({ status: 'ok', data: null });
    mockCommands.getRailwayModelImage.mockResolvedValue({ status: 'ok', data: null });
    render(WishlistItemPage);
    await waitFor(() => expect(mockCommands.getWishlistById).toHaveBeenCalledWith(WISHLIST_ID), {
      timeout: 2000
    });
  });
});
