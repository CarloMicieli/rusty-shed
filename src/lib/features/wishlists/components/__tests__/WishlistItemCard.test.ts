import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup, fireEvent } from '@testing-library/svelte';
import type { RailwayModelView_Serialize, WishlistItem } from '$lib/bindings';

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
      } as unknown as RailwayModelView_Serialize
    }),
    getRailwayModelImage: vi.fn().mockResolvedValue({
      status: 'ok',
      data: { hasImage: false, imagePath: null, placeholderHtml: null }
    })
  }
}));

vi.mock('@tauri-apps/plugin-fs', () => ({
  readFile: vi.fn().mockResolvedValue(new Uint8Array())
}));

const mockGoto = vi.fn();

vi.mock('$app/navigation', () => ({
  goto: (...args: unknown[]) => mockGoto(...args)
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
  wishlist_item_card_searching: () => 'Searching',
  wishlist_item_card_high_priority: () => 'High priority',
  wishlist_item_card_move: () => 'Move',
  wishlist_item_card_purchase: () => 'Purchase',
  wishlist_table_col_price_target: () => 'Price target',
  wishlist_table_col_product_code: () => 'Product code',
  wishlist_table_row_remove_title: () => 'Remove from list'
}));

import WishlistItemCard from '../WishlistItemCard.svelte';
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

describe('WishlistItemCard', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
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
      } as unknown as RailwayModelView_Serialize
    });
    vi.mocked(commands.getRailwayModelImage).mockResolvedValue({
      status: 'ok',
      data: { hasImage: false, imagePath: null, placeholderHtml: null }
    });
  });

  it('renders the purchase action in the bottom action row for wanted items', async () => {
    render(WishlistItemCard, {
      props: { item: makeItem(), wishlistId: 'wl-1', onPurchase: vi.fn() }
    });

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Purchase' })).toBeInTheDocument();
    });
  });

  it('calls purchase without triggering card navigation', async () => {
    const onPurchase = vi.fn();

    render(WishlistItemCard, {
      props: { item: makeItem(), wishlistId: 'wl-1', onPurchase }
    });

    const purchaseButton = await screen.findByRole('button', { name: 'Purchase' });
    await fireEvent.click(purchaseButton);

    expect(onPurchase).toHaveBeenCalledWith('trn:wishlist-item:test-1');
    expect(mockGoto).not.toHaveBeenCalled();
  });

  it('renders move and remove actions in the bottom action row', async () => {
    render(WishlistItemCard, {
      props: {
        item: makeItem(),
        wishlistId: 'wl-1',
        onMove: vi.fn(),
        onRemove: vi.fn()
      }
    });

    await waitFor(() => {
      expect(screen.getByRole('button', { name: 'Move' })).toBeInTheDocument();
      expect(screen.getByRole('button', { name: 'Remove from list' })).toBeInTheDocument();
    });
  });

  it('hides the purchase action for purchased items', () => {
    render(WishlistItemCard, {
      props: {
        item: makeItem({ status: 'PURCHASED' }),
        wishlistId: 'wl-1',
        onPurchase: vi.fn()
      }
    });

    expect(screen.queryByRole('button', { name: 'Purchase' })).toBeNull();
  });
});
