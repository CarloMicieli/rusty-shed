import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const trackContextMock = vi.hoisted(() => ({
  addPurchase: vi.fn().mockResolvedValue(undefined)
}));

const commandMocks = vi.hoisted(() => ({
  getTrackProducts: vi.fn().mockResolvedValue([]),
  getSellers: vi.fn().mockResolvedValue([]),
  getManufacturers: vi.fn().mockResolvedValue([])
}));

vi.mock('$lib/features/track-inventory', () => ({
  getTrackInventoryContext: () => trackContextMock
}));

vi.mock('$lib/bindings', () => ({
  commands: commandMocks
}));

vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

vi.mock('$lib/features/track-inventory/components/PurchaseFormFields.svelte', async () => {
  const mod =
    await import('/home/carlo/Projects/rusty-shed/src/__tests__/fixtures/PurchaseFormFieldsStub.svelte');
  return { default: mod.default };
});

vi.mock('$lib/features/track-inventory/components/CreateProductDialog.svelte', async () => {
  const mod =
    await import('/home/carlo/Projects/rusty-shed/src/__tests__/fixtures/CreateProductDialogStub.svelte');
  return { default: mod.default };
});

vi.mock('$lib/features/track-inventory/components/PurchaseDialogHeader.svelte', () => ({
  default: function PurchaseDialogHeaderStub() {}
}));

vi.mock('$lib/features/track-inventory/components/PurchaseLoadingState.svelte', () => ({
  default: function PurchaseLoadingStateStub() {}
}));

import AddPurchaseDialog from '$lib/features/track-inventory/components/AddPurchaseDialog.svelte';

describe('AddPurchaseDialog', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('loads products, sellers, and manufacturers when opened', async () => {
    render(AddPurchaseDialog, {
      props: {
        open: true,
        inventoryId: 'inventory-1'
      }
    });

    await waitFor(() => {
      expect(commandMocks.getTrackProducts).toHaveBeenCalled();
      expect(commandMocks.getSellers).toHaveBeenCalled();
      expect(commandMocks.getManufacturers).toHaveBeenCalled();
    });
  });

  it('shows validation error when submitting without selecting a product', async () => {
    render(AddPurchaseDialog, {
      props: {
        open: true,
        inventoryId: 'inventory-1'
      }
    });

    await waitFor(() => {
      expect(document.querySelector('form')).not.toBeNull();
    });
    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    expect(await screen.findByText('track_purchase_validation_product')).toBeInTheDocument();
    expect(trackContextMock.addPurchase).not.toHaveBeenCalled();
  });

  it('creates product, submits purchase, and notifies completion', async () => {
    const onPurchaseAdded = vi.fn().mockResolvedValue(undefined);
    const onClose = vi.fn();

    render(AddPurchaseDialog, {
      props: {
        open: true,
        inventoryId: 'inventory-1',
        onPurchaseAdded,
        onClose
      }
    });

    await waitFor(() => {
      expect(document.querySelector('form')).not.toBeNull();
    });
    await fireEvent.click(screen.getByTestId('open-create-product'));
    await fireEvent.click(await screen.findByTestId('create-product-submit'));
    await fireEvent.click(screen.getByTestId('fill-valid-data'));
    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(trackContextMock.addPurchase).toHaveBeenCalledWith({
        id: 'inventory-1',
        trackId: 'track-1',
        quantity: 2,
        price: { amount: 3000, currency: 'EUR' },
        sellerId: 'seller-1',
        purchaseDate: '2026-11-15'
      });
    });
    await waitFor(() => {
      expect(onPurchaseAdded).toHaveBeenCalled();
    });
    expect(onClose).not.toHaveBeenCalled();
  });

  it('shows service errors when addPurchase fails', async () => {
    trackContextMock.addPurchase.mockRejectedValueOnce(new Error('Purchase failed'));

    render(AddPurchaseDialog, {
      props: {
        open: true,
        inventoryId: 'inventory-1'
      }
    });

    await waitFor(() => {
      expect(document.querySelector('form')).not.toBeNull();
    });
    await fireEvent.click(screen.getByTestId('fill-valid-data'));
    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    expect(await screen.findByText('Purchase failed')).toBeInTheDocument();
  });

  it('closes on Escape key when create-product drawer is not open', async () => {
    const onClose = vi.fn();
    render(AddPurchaseDialog, {
      props: {
        open: true,
        inventoryId: 'inventory-1',
        onClose
      }
    });

    await waitFor(() => {
      expect(document.querySelector('form')).not.toBeNull();
    });
    await fireEvent.keyDown(window, { key: 'Escape' });

    expect(onClose).toHaveBeenCalled();
  });
});
