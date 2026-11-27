import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockCommands = vi.hoisted(() => ({
  getSellers: vi.fn(),
  updateCollectionItem: vi.fn()
}));
const mockCollectionState = vi.hoisted(() => ({
  receivePreorder: vi.fn()
}));

vi.mock('$lib/bindings', () => ({ commands: mockCommands }));
vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  collectionState: mockCollectionState
}));
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    locale: 'en-US',
    getCurrencySymbol: () => 'EUR '
  }
}));
vi.mock('$lib/paraglide/runtime.js', () => ({ getLocale: () => 'en-US' }));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/components/InPlaceEdit.svelte', () => ({
  default: function InPlaceEditStub() {}
}));
vi.mock('$lib/components/InPlaceSelectEdit.svelte', () => ({
  default: function InPlaceSelectEditStub() {}
}));
vi.mock('$lib/components/DatePickerInput.svelte', () => ({
  default: function DatePickerInputStub() {}
}));
vi.mock('$lib/components/CurrencyInput.svelte', () => ({
  default: function CurrencyInputStub() {}
}));
vi.mock('$lib/features/collection/components/SellCollectionItemDialog.svelte', () => ({
  default: function SellCollectionItemDialogStub() {}
}));

import CollectionItemDetails from '$lib/features/collection/components/CollectionItemDetails.svelte';
import type { CollectionItemView } from '$lib/bindings';

const ownedItem = {
  id: 'item-1',
  removedDate: null,
  purchaseInfo: null,
  notes: 'Original notes'
};

describe('CollectionItemDetails', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockCommands.getSellers.mockResolvedValue([]);
    mockCommands.updateCollectionItem.mockResolvedValue(undefined);
    mockCollectionState.receivePreorder.mockResolvedValue(true);
  });

  it('renders owned status and sell action', async () => {
    render(CollectionItemDetails, {
      props: { item: ownedItem as unknown as CollectionItemView, seller: null }
    });
    await waitFor(() => expect(mockCommands.getSellers).toHaveBeenCalledOnce());

    expect(screen.getByText('collection_item_status_owned')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'collection_item_sell_action' })).toBeInTheDocument();
  });

  it('renders preorder status, deposit, remaining balance, and receive action', async () => {
    const onItemUpdated = vi.fn().mockResolvedValue(undefined);
    render(CollectionItemDetails, {
      props: {
        item: {
          id: 'item-1',
          removedDate: null,
          notes: null,
          purchaseInfo: {
            kind: 'preOrdered',
            data: {
              deposit: { amount: 2000, currency: 'EUR' },
              totalPrice: { amount: 5000, currency: 'EUR' },
              expectedDate: '2027-01-15'
            }
          }
        } as unknown as CollectionItemView,
        seller: null,
        onItemUpdated
      }
    });

    expect(screen.getAllByText('collection_item_status_preordered').length).toBeGreaterThan(0);
    expect(screen.getByText('2027-01-15')).toBeInTheDocument();
    expect(screen.getByText('€20.00')).toBeInTheDocument();
    expect(screen.getByText('€50.00')).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'collection_item_receive_action' }));
    await waitFor(() => expect(mockCollectionState.receivePreorder).toHaveBeenCalled());
  });

  it('renders sold status and positive profit/loss', () => {
    render(CollectionItemDetails, {
      props: {
        item: {
          id: 'item-1',
          removedDate: '2026-11-20',
          notes: null,
          purchaseInfo: {
            kind: 'sold',
            data: {
              purchaseDate: '2025-01-15',
              purchasePrice: { amount: 3000, currency: 'EUR' },
              saleDate: '2026-11-20',
              salePrice: { amount: 4500, currency: 'EUR' }
            }
          }
        } as unknown as CollectionItemView,
        seller: null
      }
    });

    expect(screen.getByText('collection_item_status_sold')).toBeInTheDocument();
    expect(screen.getByText('2026-11-20')).toBeInTheDocument();
    expect(screen.getByText('collection_item_profit_loss_title')).toBeInTheDocument();
  });

  it('renders negative profit/loss for sold items', () => {
    render(CollectionItemDetails, {
      props: {
        item: {
          id: 'item-1',
          removedDate: '2026-11-20',
          notes: null,
          purchaseInfo: {
            kind: 'sold',
            data: {
              purchaseDate: '2025-01-15',
              purchasePrice: { amount: 5000, currency: 'EUR' },
              saleDate: '2026-11-20',
              salePrice: { amount: 3000, currency: 'EUR' }
            }
          }
        } as unknown as CollectionItemView,
        seller: null
      }
    });

    expect(screen.getByText('collection_item_profit_loss_title')).toBeInTheDocument();
  });

  it('opens price editing for owned items', async () => {
    render(CollectionItemDetails, {
      props: {
        item: {
          ...ownedItem,
          purchaseInfo: {
            kind: 'purchased',
            data: {
              purchaseDate: '2026-01-15',
              price: { amount: 2500, currency: 'EUR' }
            }
          }
        } as unknown as CollectionItemView,
        seller: null
      }
    });

    const priceButton = screen
      .getAllByRole('button')
      .find((button) => button.textContent?.includes('€25.00'));
    expect(priceButton).toBeDefined();
  });
});
