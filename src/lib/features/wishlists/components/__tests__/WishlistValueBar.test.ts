import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, cleanup } from '@testing-library/svelte';

// Mock regionalManager before importing component
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    formatCurrencyWith: vi.fn((cents: number, currency: string) => {
      return `${currency} ${(cents / 100).toFixed(2)}`;
    })
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_procurement_summary: () => 'PROCUREMENT SUMMARY',
  wishlist_value_bar_no_price_data: () => 'No price data',
  wishlist_stat_total_cost: () => 'Total Cost',
  wishlist_stat_item_count: () => 'Items',
  wishlist_stat_avg_price: () => 'Avg. Price',
  wishlist_priority_high: () => 'High',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_low: () => 'Low'
}));

import WishlistValueBar from '../WishlistValueBar.svelte';
import type { WishlistItem } from '$lib/bindings';
import { regionalManager } from '$lib/features/settings/RegionalManager.svelte';

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
      ? { amount: BigInt(price.amount), currency: price.currency as never }
      : null,
    purchasedPrice: null,
    ...overrides
  } as unknown as WishlistItem;
}

describe('WishlistValueBar', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── Empty state ─────────────────────────────────────────────────────────────

  it('renders procurement summary header', () => {
    render(WishlistValueBar, { props: { items: [] } });
    expect(screen.getByText('PROCUREMENT SUMMARY')).toBeInTheDocument();
  });

  it('shows empty state when no items have prices', () => {
    const items = [
      makeItem({ id: 'trn:wishlist-item:1' }),
      makeItem({ id: 'trn:wishlist-item:2' })
    ];
    render(WishlistValueBar, { props: { items } });
    expect(screen.getByText('No price data')).toBeInTheDocument();
  });

  it('shows empty state when items array is empty', () => {
    render(WishlistValueBar, { props: { items: [] } });
    expect(screen.getByText('No price data')).toBeInTheDocument();
  });

  // ── Stats display ───────────────────────────────────────────────────────────

  it('renders stat column labels when items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 8999, currency: 'EUR' })];
    render(WishlistValueBar, { props: { items } });

    expect(screen.getByText('Total Cost')).toBeInTheDocument();
    expect(screen.getByText('Items')).toBeInTheDocument();
    expect(screen.getByText('Avg. Price')).toBeInTheDocument();
  });

  it('displays total item count (including items without prices)', () => {
    const items = [
      makeItem({ id: 'trn:wishlist-item:1' }, { amount: 8999, currency: 'EUR' }),
      makeItem({ id: 'trn:wishlist-item:2' }) // no price
    ];
    render(WishlistValueBar, { props: { items } });

    // total count = 2, not 1 (items.length, not pricedItems.length)
    expect(screen.getByText('2')).toBeInTheDocument();
  });

  it('calls formatCurrencyWith to display total cost', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 8999, currency: 'EUR' })];
    render(WishlistValueBar, { props: { items } });

    expect(vi.mocked(regionalManager.formatCurrencyWith)).toHaveBeenCalled();
  });

  // ── Priority breakdown ──────────────────────────────────────────────────────

  it('does not show empty state when at least one item has a price', () => {
    const items = [
      makeItem({ id: 'trn:wishlist-item:1', priority: 'HIGH' }, { amount: 12000, currency: 'EUR' }),
      makeItem({ id: 'trn:wishlist-item:2', priority: 'LOW' }, { amount: 5000, currency: 'EUR' })
    ];
    render(WishlistValueBar, { props: { items } });

    expect(screen.queryByText('No price data')).not.toBeInTheDocument();
  });

  it('renders gauge bar track when items have prices', () => {
    const items = [makeItem({ id: 'trn:wishlist-item:1' }, { amount: 9999, currency: 'EUR' })];
    const { container } = render(WishlistValueBar, { props: { items } });

    // The gauge bar track element is present
    const track = container.querySelector('.bg-\\[\\#1F1F1F\\]');
    expect(track).toBeTruthy();
  });
});
