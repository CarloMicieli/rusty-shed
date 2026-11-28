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
import ProductFormFields from '$lib/features/track-inventory/components/ProductFormFields.svelte';

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

describe('ProductFormFields', () => {
  beforeEach(() => cleanup());

  it('renders core product fields and manufacturer options', () => {
    render(ProductFormFields, {
      props: {
        manufacturers: [
          { id: 'm1', name: 'Roco' },
          { id: 'm2', name: 'Peco' }
        ],
        manufacturerId: 'm1',
        productCode: '6110',
        description: 'Curved section'
      }
    });

    expect(screen.getByLabelText('track_product_field_product_code *')).toHaveValue('6110');
    expect(screen.getByLabelText('track_product_field_description')).toHaveValue('Curved section');
    expect(screen.getByText(/track_product_field_track_type/)).toBeInTheDocument();
    expect(screen.getByText(/track_product_field_track_code/)).toBeInTheDocument();
  });

  it('updates text, numeric, and checkbox fields through bindings', async () => {
    const { container } = render(ProductFormFields, {
      props: {
        productCode: '',
        description: '',
        length: '',
        radius: '',
        withRoadbed: false
      }
    });

    const productCodeInput = screen.getByLabelText('track_product_field_product_code *');
    const descriptionInput = screen.getByLabelText('track_product_field_description');
    const numberInputs = container.querySelectorAll('input[type="number"]');
    const roadbedCheckbox = container.querySelector('input[type="checkbox"]') as HTMLInputElement;

    await fireEvent.input(productCodeInput, { target: { value: 'R5-100' } });
    await fireEvent.input(descriptionInput, { target: { value: 'Straight 100mm' } });
    await fireEvent.input(numberInputs[0], { target: { value: '100' } });
    await fireEvent.input(numberInputs[1], { target: { value: '500' } });
    await fireEvent.click(roadbedCheckbox);

    expect(productCodeInput).toHaveValue('R5-100');
    expect(descriptionInput).toHaveValue('Straight 100mm');
    expect(numberInputs[0]).toHaveValue(100);
    expect(numberInputs[1]).toHaveValue(500);
    expect(roadbedCheckbox.checked).toBe(true);
  });

  it('disables form controls while submitting and shows errors', () => {
    const { container } = render(ProductFormFields, {
      props: {
        submitting: true,
        error: 'Duplicate product code'
      }
    });

    expect(screen.getByLabelText('track_product_field_product_code *')).toBeDisabled();
    expect(screen.getByLabelText('track_product_field_description')).toBeDisabled();
    expect(container.querySelector('input[type="checkbox"]')).toBeDisabled();
    expect(
      screen.getByText(/track_product_error_prefix: Duplicate product code/)
    ).toBeInTheDocument();
  });
});
