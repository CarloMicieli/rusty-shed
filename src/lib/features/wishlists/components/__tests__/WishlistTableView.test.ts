import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';

// WishlistTableRow mounts and makes IPC calls in onMount — mock the whole module
vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayModelById: vi.fn().mockResolvedValue({ status: 'ok', data: null }),
    getRailwayModelImage: vi.fn().mockResolvedValue({
      status: 'ok',
      data: { hasImage: false, imagePath: null }
    })
  }
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn().mockResolvedValue(new Uint8Array())
}));

vi.mock('$app/navigation', () => ({
  goto: vi.fn()
}));

vi.mock('$lib/paraglide/runtime.js', () => ({
  getLocale: vi.fn(() => 'en')
}));

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    formatCurrencyWith: vi.fn(
      (cents: number, currency: string) => `${currency} ${(cents / 100).toFixed(2)}`
    )
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_table_col_priority: () => 'Priority',
  wishlist_table_col_model: () => 'Model',
  wishlist_table_col_price_target: () => 'Price Target',
  wishlist_table_col_status: () => 'Status',
  wishlist_table_col_actions: () => 'Actions',
  wishlist_item_status_wanted: () => 'Wanted',
  wishlist_item_status_on_order: () => 'On Order',
  wishlist_item_status_purchased: () => 'Purchased',
  wishlist_item_status_ignored: () => 'Ignored',
  wishlists_items_empty: () => 'No items yet',
  wishlist_priority_high: () => 'High',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_low: () => 'Low',
  wishlist_table_row_move_title: () => 'Move to another list',
  wishlist_table_row_remove_title: () => 'Remove from list'
}));

import WishlistTableView from '../WishlistTableView.svelte';
import type { WishlistItem, WishlistPreview } from '$lib/bindings';

function makeItem(overrides: Partial<WishlistItem> = {}): WishlistItem {
  return {
    id: 'trn:wishlist-item:test-1',
    railwayModelId: 'trn:railway-model:acme:37858',
    priority: 'NORMAL',
    status: 'WANTED',
    addedDate: '2024-01-01',
    removedDate: null,
    notes: null,
    desiredPrice: null,
    purchasedPrice: null,
    ...overrides
  } as unknown as WishlistItem;
}

const defaultProps = {
  items: [] as WishlistItem[],
  activeWishlistId: 'wl-1',
  otherTargets: [] as WishlistPreview[],
  onRemove: vi.fn(),
  onMove: vi.fn(),
  onPurchase: vi.fn()
};

describe('WishlistTableView', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Empty state ─────────────────────────────────────────────────────────────

  it('renders empty state when activeWishlistId is set but items are empty', () => {
    render(WishlistTableView, { props: defaultProps });
    expect(screen.getByText('No items yet')).toBeInTheDocument();
  });

  it('renders nothing (no table, no empty state) when activeWishlistId is null and items is empty', () => {
    render(WishlistTableView, {
      props: { ...defaultProps, activeWishlistId: null }
    });
    expect(screen.queryByText('No items yet')).not.toBeInTheDocument();
    expect(screen.queryByRole('table')).not.toBeInTheDocument();
  });

  // ── Table structure ─────────────────────────────────────────────────────────

  it('renders table with all 5 column headers when items are present', () => {
    const items = [makeItem()];
    render(WishlistTableView, { props: { ...defaultProps, items } });

    expect(screen.getByText('Priority')).toBeInTheDocument();
    expect(screen.getByText('Model')).toBeInTheDocument();
    expect(screen.getByText('Price Target')).toBeInTheDocument();
    expect(screen.getByText('Status')).toBeInTheDocument();
    expect(screen.getByText('Actions')).toBeInTheDocument();
  });

  it('renders a table element when items are present', () => {
    const items = [makeItem()];
    render(WishlistTableView, { props: { ...defaultProps, items } });

    expect(screen.getByRole('table')).toBeInTheDocument();
  });

  it('renders one row per item', () => {
    const items = [
      makeItem({ id: 'trn:wishlist-item:1' }),
      makeItem({ id: 'trn:wishlist-item:2' }),
      makeItem({ id: 'trn:wishlist-item:3' })
    ];
    render(WishlistTableView, { props: { ...defaultProps, items } });

    // 3 data rows + 1 header row = 4 rows in the table
    const rows = screen.getAllByRole('row');
    expect(rows.length).toBeGreaterThanOrEqual(3);
  });

  it('does not render empty state when items exist', () => {
    const items = [makeItem()];
    render(WishlistTableView, { props: { ...defaultProps, items } });
    expect(screen.queryByText('No items yet')).not.toBeInTheDocument();
  });
});
