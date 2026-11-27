import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});
vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    getCurrencySymbol: () => 'EUR '
  }
}));
vi.mock('$lib/components/SearchableSelect.svelte', () => ({
  default: function SearchableSelectStub() {}
}));
vi.mock('$lib/components/DatePickerInput.svelte', () => ({
  default: function DatePickerInputStub() {}
}));

import RenameInventoryDialog from '$lib/features/track-inventory/components/RenameInventoryDialog.svelte';
import PurchaseFormFields from '$lib/features/track-inventory/components/PurchaseFormFields.svelte';

describe('RenameInventoryDialog', () => {
  beforeEach(() => cleanup());

  it('populates the current name and validates empty names', async () => {
    render(RenameInventoryDialog, {
      props: { open: true, currentName: 'Main Layout', onRename: vi.fn() }
    });

    expect(screen.getByDisplayValue('Main Layout')).toBeInTheDocument();
    await fireEvent.input(screen.getByDisplayValue('Main Layout'), { target: { value: '   ' } });
    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);
    expect(screen.getByText('track_inventory_rename_validation_name')).toBeInTheDocument();
  });

  it('closes unchanged names without calling rename', async () => {
    const onRename = vi.fn();
    const onClose = vi.fn();
    render(RenameInventoryDialog, {
      props: { open: true, currentName: 'Main Layout', onRename, onClose }
    });

    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);
    expect(onRename).not.toHaveBeenCalled();
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('trims and submits a changed name', async () => {
    const onRename = vi.fn().mockResolvedValue(undefined);
    render(RenameInventoryDialog, {
      props: { open: true, currentName: 'Main Layout', onRename }
    });

    await fireEvent.input(screen.getByDisplayValue('Main Layout'), {
      target: { value: '  Staging Yard  ' }
    });
    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);

    expect(onRename).toHaveBeenCalledWith('Staging Yard');
  });

  it('shows rename callback failures', async () => {
    const onRename = vi.fn().mockRejectedValue(new Error('Rename failed'));
    render(RenameInventoryDialog, {
      props: { open: true, currentName: 'Main Layout', onRename }
    });

    await fireEvent.input(screen.getByDisplayValue('Main Layout'), {
      target: { value: 'Staging Yard' }
    });
    await fireEvent.submit(screen.getByRole('dialog').querySelector('form') as HTMLFormElement);

    expect(await screen.findByText('Rename failed')).toBeInTheDocument();
  });
});

describe('PurchaseFormFields', () => {
  beforeEach(() => cleanup());

  it('renders quantity, market price, seller, and date fields', () => {
    render(PurchaseFormFields, {
      props: {
        products: [],
        sellers: [],
        quantity: 1,
        priceAmount: null,
        priceCurrency: 'EUR',
        purchaseDate: '2026-11-27'
      }
    });

    expect(screen.getByText('track_purchase_field_quantity')).toBeInTheDocument();
    expect(screen.getByText(/track_purchase_field_market_price/)).toBeInTheDocument();
    expect(screen.getByText('track_purchase_field_seller')).toBeInTheDocument();
  });

  it('shows total for multiple units and a unit price', () => {
    render(PurchaseFormFields, {
      props: {
        products: [],
        sellers: [],
        quantity: 4,
        priceAmount: 1250,
        priceCurrency: 'EUR',
        purchaseDate: '2026-11-27'
      }
    });

    expect(screen.getByText(/50.00 EUR/)).toBeInTheDocument();
  });

  it('does not show quantity total for a single unit', () => {
    render(PurchaseFormFields, {
      props: {
        products: [],
        sellers: [],
        quantity: 1,
        priceAmount: 1250,
        priceCurrency: 'EUR'
      }
    });

    expect(screen.queryByText(/50.00 EUR/)).not.toBeInTheDocument();
  });

  it('shows validation error text when provided', () => {
    render(PurchaseFormFields, {
      props: {
        products: [],
        sellers: [],
        quantity: 1,
        error: 'Price is required'
      }
    });

    expect(screen.getByText(/Price is required/)).toBeInTheDocument();
  });
});
