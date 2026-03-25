import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup } from '@testing-library/svelte';
import type { RailwayModelView } from '$lib/bindings';

vi.mock('$lib/bindings', () => ({
  commands: {
    getRailwayModelById: vi.fn().mockResolvedValue({
      status: 'ok',
      data: {
        id: 'trn:railway-model:acme:37858',
        description: 'Class 218 Diesel Locomotive',
        descriptionLang: 'en',
        manufacturer: { manufacturerId: 'acme', display: 'A.C.M.E.' },
        productCode: '37858',
        category: 'LOCOMOTIVES',
        scale: 'H0',
        epoch: 'IV',
        powerMethod: 'DC',
        deliveryDate: null,
        availabilityStatus: null,
        details: null
      } as RailwayModelView
    }),
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
  wishlist_item_status_wanted: () => 'Wanted',
  wishlist_item_status_on_order: () => 'On Order',
  wishlist_item_status_purchased: () => 'Purchased',
  wishlist_item_status_ignored: () => 'Ignored',
  wishlist_priority_high: () => 'High',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_low: () => 'Low'
}));

import WishlistTableRow from '../WishlistTableRow.svelte';
import type { WishlistItem } from '$lib/bindings';
import { commands } from '$lib/bindings';

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

describe('WishlistTableRow', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    // Reset to default resolved values before each test
    vi.mocked(commands.getRailwayModelById).mockResolvedValue({
      status: 'ok',
      data: {
        id: 'trn:railway-model:acme:37858',
        description: 'Class 218 Diesel Locomotive',
        descriptionLang: 'en',
        manufacturer: { manufacturerId: 'acme', display: 'A.C.M.E.' },
        productCode: '37858',
        category: 'LOCOMOTIVES',
        scale: 'H0',
        epoch: 'IV',
        powerMethod: 'DC',
        deliveryDate: null,
        availabilityStatus: null,
        details: null
      } as RailwayModelView
    });
    vi.mocked(commands.getRailwayModelImage).mockResolvedValue({
      status: 'ok',
      data: { hasImage: false, imagePath: null, placeholderHtml: null }
    });
  });

  // ── Rendering ───────────────────────────────────────────────────────────────

  it('renders a table row element without throwing', () => {
    const item = makeItem();
    expect(() =>
      render(WishlistTableRow, {
        props: { item, wishlistId: 'wl-1' }
      })
    ).not.toThrow();
  });

  it('fetches model details on mount', async () => {
    const item = makeItem();
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });

    await waitFor(() => {
      expect(vi.mocked(commands.getRailwayModelById)).toHaveBeenCalledWith(
        item.railwayModelId,
        'en'
      );
    });
  });

  it('displays manufacturer and description after model loads', async () => {
    const item = makeItem();
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });

    await waitFor(
      () => {
        expect(screen.getByText('A.C.M.E.')).toBeInTheDocument();
        expect(screen.getByText('Class 218 Diesel Locomotive')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('displays product code after model loads', async () => {
    const item = makeItem();
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });

    await waitFor(
      () => {
        expect(screen.getByText('37858')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  // ── Status pills ────────────────────────────────────────────────────────────

  it('renders WANTED status pill', () => {
    const item = makeItem({ status: 'WANTED' });
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });
    expect(screen.getByText('Wanted')).toBeInTheDocument();
  });

  it('renders ON_ORDER status pill', () => {
    const item = makeItem({ status: 'ON_ORDER' });
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });
    expect(screen.getByText('On Order')).toBeInTheDocument();
  });

  it('renders PURCHASED status pill', () => {
    const item = makeItem({ status: 'PURCHASED' });
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });
    expect(screen.getByText('Purchased')).toBeInTheDocument();
  });

  it('renders IGNORED status pill', () => {
    const item = makeItem({ status: 'IGNORED' });
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });
    expect(screen.getByText('Ignored')).toBeInTheDocument();
  });

  // ── Price target ────────────────────────────────────────────────────────────

  it('displays price target when desiredPrice is set', () => {
    const item = makeItem({
      desiredPrice: { amount: BigInt(8999), currency: 'EUR' as never }
    } as unknown as Partial<WishlistItem>);
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });

    // formatCurrencyWith is mocked to return "EUR 89.99"
    expect(screen.getByText('EUR 89.99')).toBeInTheDocument();
  });

  it('shows dash when no price is set', () => {
    const item = makeItem({ desiredPrice: null });
    const { container } = render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });
    // The dash character '—' should be present somewhere in the row
    expect(container.textContent).toContain('—');
  });

  // ── Graceful degradation ────────────────────────────────────────────────────

  it('falls back to railwayModelId when model fetch returns null', async () => {
    vi.mocked(commands.getRailwayModelById).mockResolvedValueOnce({ status: 'ok', data: null });

    const item = makeItem({ railwayModelId: 'trn:railway-model:acme:xyz' });
    render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } });

    await waitFor(
      () => {
        expect(screen.getByText('trn:railway-model:acme:xyz')).toBeInTheDocument();
      },
      { timeout: 2000 }
    );
  });

  it('renders without crashing when model fetch fails', async () => {
    vi.mocked(commands.getRailwayModelById).mockRejectedValueOnce(new Error('Network error'));

    const item = makeItem();
    expect(() => render(WishlistTableRow, { props: { item, wishlistId: 'wl-1' } })).not.toThrow();
  });

  // ── Action buttons ──────────────────────────────────────────────────────────

  it('renders purchase button for non-purchased items when onPurchase is provided', () => {
    const item = makeItem({ status: 'WANTED' });
    render(WishlistTableRow, {
      props: { item, wishlistId: 'wl-1', onPurchase: vi.fn() }
    });
    // ShoppingCart icon button should be present
    const { container } = render(WishlistTableRow, {
      props: { item, wishlistId: 'wl-1', onPurchase: vi.fn() }
    });
    const buttons = container.querySelectorAll('td button');
    expect(buttons.length).toBeGreaterThan(0);
  });

  it('hides purchase button for PURCHASED items', () => {
    const item = makeItem({ status: 'PURCHASED' });
    const { container } = render(WishlistTableRow, {
      props: { item, wishlistId: 'wl-1', onPurchase: vi.fn() }
    });
    // 'Purchased' title button (ShoppingCart) should not appear
    expect(container.querySelector('button[title="Purchased"]')).toBeNull();
  });
});
