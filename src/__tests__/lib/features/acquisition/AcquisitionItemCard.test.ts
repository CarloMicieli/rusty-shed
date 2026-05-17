import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import AcquisitionItemCard from '$lib/features/acquisition/components/AcquisitionItemCard.svelte';
import type { AcquisitionItemEntry, AcquisitionItemErrors } from '$lib/features/acquisition/types';
import type { Manufacturer } from '$lib/bindings';

vi.mock('$lib/paraglide/messages.js', () => ({
  action_duplicate_item: () => 'duplicate-item',
  action_remove_item: () => 'remove-item',
  acquisition_item_manufacturer_label: () => 'Manufacturer',
  acquisition_item_manufacturer_placeholder: () => 'Select manufacturer',
  acquisition_item_product_code_label: () => 'Product code',
  acquisition_item_product_code_placeholder: () => 'Product code placeholder',
  acquisition_item_description_label: () => 'Description',
  acquisition_item_description_placeholder: () => 'Description placeholder',
  acquisition_item_category_label: () => 'Category',
  acquisition_item_category_placeholder: () => 'Select category',
  acquisition_item_epoch_label: () => 'Epoch',
  acquisition_item_price_label: () => 'Price',
  quick_add_drawer_title_manufacturer: () => 'Add Manufacturer'
}));

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: {
    getCurrencySymbol: vi.fn(() => 'EUR')
  }
}));

vi.mock('$lib/components/ui/select', () => ({
  Root: function SelectRoot() {},
  Trigger: function SelectTrigger() {},
  Content: function SelectContent() {},
  Item: function SelectItem() {}
}));

vi.mock('$lib/components', () => ({
  CurrencyInput: function CurrencyInput() {}
}));

vi.mock('$lib/components/drawer', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/components/drawer')>();
  return {
    ...actual,
    EpochPicker: function EpochPicker() {}
  };
});

vi.mock('$lib/utils/enum-options', () => ({
  categoryOptions: () => [{ value: 'LOCOMOTIVES', label: 'Locomotives' }],
  categoryLabel: (value: string) => value
}));

function makeItem(overrides: Partial<AcquisitionItemEntry> = {}): AcquisitionItemEntry {
  return {
    uid: 'item-1',
    manufacturerId: 'm1',
    productCode: 'AB-12',
    description: 'Description',
    category: 'LOCOMOTIVES',
    epoch: 'IV',
    priceAmount: 12000,
    ...overrides
  };
}

const manufacturers: Manufacturer[] = [
  {
    id: 'm1',
    name: 'Roco',
    registeredCompanyName: null,
    countryCode: null,
    status: 'ACTIVE',
    websiteUrl: null
  }
];

const errors: AcquisitionItemErrors = {};

describe('AcquisitionItemCard', () => {
  it('calls onDuplicate with item uid', async () => {
    const onDuplicate = vi.fn();

    render(AcquisitionItemCard, {
      props: {
        item: makeItem(),
        index: 0,
        manufacturers,
        currency: 'EUR',
        errors,
        canRemove: true,
        onUpdate: vi.fn(),
        onDuplicate,
        onRemove: vi.fn(),
        onQuickAddManufacturer: vi.fn()
      }
    });

    await fireEvent.click(screen.getByLabelText('duplicate-item'));

    expect(onDuplicate).toHaveBeenCalledWith('item-1');
  });

  it('hides remove button when canRemove is false', () => {
    render(AcquisitionItemCard, {
      props: {
        item: makeItem(),
        index: 0,
        manufacturers,
        currency: 'EUR',
        errors,
        canRemove: false,
        onUpdate: vi.fn(),
        onDuplicate: vi.fn(),
        onRemove: vi.fn(),
        onQuickAddManufacturer: vi.fn()
      }
    });

    expect(screen.queryByLabelText('remove-item')).toBeNull();
  });

  it('calls onUpdate when product code changes', async () => {
    const onUpdate = vi.fn();

    render(AcquisitionItemCard, {
      props: {
        item: makeItem(),
        index: 0,
        manufacturers,
        currency: 'EUR',
        errors,
        canRemove: true,
        onUpdate,
        onDuplicate: vi.fn(),
        onRemove: vi.fn(),
        onQuickAddManufacturer: vi.fn()
      }
    });

    const input = screen.getByLabelText('Product code') as HTMLInputElement;
    await fireEvent.input(input, { target: { value: 'NEW-42' } });

    expect(onUpdate).toHaveBeenCalledWith('item-1', { productCode: 'NEW-42' });
  });
});
