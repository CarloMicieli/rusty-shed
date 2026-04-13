import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, cleanup, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    formatCurrencyWith: vi.fn((cents: number, currency: string) => {
      return `${currency} ${(cents / 100).toFixed(2)}`;
    })
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlists_switch_list_label: () => 'SWITCH LIST',
  wishlists_select_list_placeholder: () => 'Select a list…',
  wishlist_modal_save: () => 'Save',
  wishlist_modal_cancel: () => 'Cancel',
  wishlist_header_rename: () => 'Rename',
  wishlist_header_set_default: () => 'Set as default',
  wishlist_header_delete_list: () => 'Delete list',
  wishlist_procurement_summary: () => 'Summary',
  wishlist_value_bar_no_price_data: () => 'No price data',
  wishlist_stat_total_cost: () => 'Total Estimated Cost',
  wishlist_stat_item_count: () => 'Item Count',
  wishlist_priority_high: () => 'High',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_low: () => 'Low'
}));

import WishlistDashboardHeader from '../WishlistDashboardHeader.svelte';
import type { WishlistItem, WishlistPreview, WishlistId } from '$lib/bindings';

function makeWishlist(shortId: string, name: string, isDefault = false): WishlistPreview {
  return {
    id: `trn:wishlist:${shortId}` as WishlistId,
    name,
    notes: null,
    isDefault,
    count: Number(0),
    updatedAt: '2025-01-01T00:00:00Z',
    totalValue: {}
  } as unknown as WishlistPreview;
}

function makeItem(
  overrides: Partial<WishlistItem> = {},
  price?: { amount: number; currency: string }
): WishlistItem {
  return {
    id: 'trn:wishlist-item:test-1',
    railwayModelId: 'trn:railway-model:acme:test',
    priority: 'NORMAL',
    status: 'WANTED',
    addedDate: '2024-01-01',
    removedDate: null,
    notes: null,
    desiredPrice: price
      ? { amount: Number(price.amount), currency: price.currency as never }
      : null,
    purchasedPrice: null,
    ...overrides
  } as unknown as WishlistItem;
}

describe('WishlistDashboardHeader', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Header labels ────────────────────────────────────────────────────────────

  it('renders "SWITCH LIST" label', () => {
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items: [], onSelect: vi.fn() }
    });
    expect(screen.getByText('SWITCH LIST')).toBeInTheDocument();
  });

  // ── Zone C labels ────────────────────────────────────────────────────────────

  it('renders "Summary" label for the procurement zone', () => {
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items: [], onSelect: vi.fn() }
    });
    expect(screen.getByText('Summary')).toBeInTheDocument();
  });

  // ── Zone A: Select trigger ────────────────────────────────────────────────────

  it('shows active wishlist name in the trigger', () => {
    const wishlists = [makeWishlist('aaa', 'Main List'), makeWishlist('bbb', 'Track Plans')];
    render(WishlistDashboardHeader, {
      props: {
        wishlists,
        activeWishlistId: 'trn:wishlist:aaa',
        items: [],
        onSelect: vi.fn()
      }
    });
    expect(screen.getByRole('button', { name: 'Main List' })).toBeInTheDocument();
  });

  it('shows placeholder text when no wishlist is active', () => {
    const wishlists = [makeWishlist('aaa', 'Main List')];
    render(WishlistDashboardHeader, {
      props: { wishlists, activeWishlistId: null, items: [], onSelect: vi.fn() }
    });
    expect(screen.getByRole('button', { name: 'Select a list…' })).toBeInTheDocument();
  });

  // ── Zone A: Select options ────────────────────────────────────────────────────

  it('renders all wishlist names as selectable options', async () => {
    const user = userEvent.setup();
    const wishlists = [makeWishlist('aaa', 'Main List'), makeWishlist('bbb', 'Track Plans')];
    render(WishlistDashboardHeader, {
      props: { wishlists, activeWishlistId: null, items: [], onSelect: vi.fn() }
    });

    const trigger = screen.getByRole('button', { name: 'Select a list…' });
    await user.click(trigger);

    await waitFor(
      () => {
        expect(screen.getAllByText('Main List').length).toBeGreaterThan(0);
        expect(screen.getByText('Track Plans')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('calls onSelect with the chosen wishlist id when selection changes', async () => {
    const user = userEvent.setup();
    const onSelect = vi.fn();
    const wishlists = [makeWishlist('aaa', 'Main List'), makeWishlist('bbb', 'Track Plans')];
    render(WishlistDashboardHeader, {
      props: {
        wishlists,
        activeWishlistId: 'trn:wishlist:aaa',
        items: [],
        onSelect
      }
    });

    const trigger = screen.getByRole('button', { name: 'Main List' });
    await user.click(trigger);

    await waitFor(() => expect(screen.getByText('Track Plans')).toBeInTheDocument(), {
      timeout: 2000
    });

    await user.click(screen.getByText('Track Plans'));

    await waitFor(() => {
      expect(onSelect).toHaveBeenCalledWith('trn:wishlist:bbb');
    });
  });

  // ── Zone B: Metrics ───────────────────────────────────────────────────────────

  it('renders metric column labels when items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 8999, currency: 'EUR' })];
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items, onSelect: vi.fn() }
    });
    expect(screen.getByText('Total Estimated Cost')).toBeInTheDocument();
    expect(screen.getByText('Item Count')).toBeInTheDocument();
  });

  it('renders priority legend labels when items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 9000, currency: 'EUR' })];
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items, onSelect: vi.fn() }
    });
    expect(screen.getByText('High')).toBeInTheDocument();
    expect(screen.getByText('Normal')).toBeInTheDocument();
    expect(screen.getByText('Low')).toBeInTheDocument();
  });

  it('shows correct item count including items without prices', () => {
    const items = [
      makeItem({ id: 'trn:wishlist-item:1' }, { amount: 8999, currency: 'EUR' }),
      makeItem({ id: 'trn:wishlist-item:2' }) // no price
    ];
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items, onSelect: vi.fn() }
    });
    expect(screen.getByText('2')).toBeInTheDocument();
  });

  it('shows no-price placeholder when no items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' })];
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items, onSelect: vi.fn() }
    });
    expect(screen.getByText('No price data')).toBeInTheDocument();
  });

  it('shows no-price placeholder when items array is empty', () => {
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items: [], onSelect: vi.fn() }
    });
    expect(screen.getByText('No price data')).toBeInTheDocument();
  });

  // ── Zone C: Gauge bar ─────────────────────────────────────────────────────────

  it('renders gauge bar when items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 9999, currency: 'EUR' })];
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items, onSelect: vi.fn() }
    });
    expect(screen.getByTestId('gauge-bar')).toBeInTheDocument();
  });

  it('does not render gauge bar when no items have prices', () => {
    render(WishlistDashboardHeader, {
      props: { wishlists: [], activeWishlistId: null, items: [], onSelect: vi.fn() }
    });
    expect(screen.queryByTestId('gauge-bar')).toBeNull();
  });
});
