import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/components/drawer', async () => {
  const drawerField = await import('../stubs/DrawerFieldStub.svelte');
  const epochStub = await import('../stubs/EpochPickerStub.svelte');
  return {
    DrawerInput: drawerField.default,
    EpochPicker: epochStub.default
  };
});

vi.mock('$lib/paraglide/messages.js', () => ({
  acquisition_item_manufacturer_label: () => 'Manufacturer',
  acquisition_item_manufacturer_placeholder: () => 'Select manufacturer',
  acquisition_item_product_code_label: () => 'Product code',
  acquisition_item_description_label: () => 'Description',
  acquisition_item_category_label: () => 'Category',
  acquisition_item_category_placeholder: () => 'Select category',
  acquisition_item_epoch_label: () => 'Epoch',
  acquisition_item_price_label: () => 'Price',
  action_duplicate_item: () => 'Duplicate',
  action_remove_item: () => 'Remove',
  acquisition_seller_label: () => 'Seller',
  acquisition_date_label: () => 'Date',
  acquisition_batch_scale_label: () => 'Scale',
  acquisition_batch_power_label: () => 'Power',
  enum_power_method_ac: () => 'AC',
  enum_power_method_dc: () => 'DC',
  enum_power_method_trix_express: () => 'Trix Express',
  enum_scale_ho: () => 'H0',
  enum_scale_tt: () => 'TT',
  enum_scale_n: () => 'N',
  enum_scale_z: () => 'Z',
  enum_scale_1: () => '1',
  enum_scale_0: () => '0',
  enum_scale_s: () => 'S',
  enum_scale_hon3: () => 'HOn3',
  enum_scale_hon30: () => 'HOn30',
  enum_scale_hon2: () => 'HOn2',
  enum_scale_hon2_5: () => 'HOn2 1/2',
  enum_scale_g: () => 'G',
  enum_scale_oo: () => 'OO',
  enum_scale_ooo: () => 'OOO',
  enum_scale_ii: () => 'II',
  enum_scale_iii: () => 'III',
  enum_scale_iv: () => 'IV',
  enum_scale_v: () => 'V',
  enum_scale_vi: () => 'VI',
  enum_scale_vii: () => 'VII',
  quick_add_drawer_title_seller: () => 'Add Seller',
  quick_add_drawer_title_manufacturer: () => 'Add Manufacturer'
}));

import AcquisitionItemCard from '$lib/features/acquisition/components/AcquisitionItemCard.svelte';
import AcquisitionBatchFields from '$lib/features/acquisition/components/AcquisitionBatchFields.svelte';

describe('Acquisition quick-add triggers', () => {
  it('emits manufacturer quick-add trigger with item uid', async () => {
    const user = userEvent.setup();
    const onQuickAddManufacturer = vi.fn();

    render(AcquisitionItemCard, {
      props: {
        item: {
          uid: 'item-1',
          manufacturerId: null,
          productCode: '',
          description: '',
          category: null,
          epoch: null,
          priceAmount: null
        },
        index: 0,
        manufacturers: [],
        currency: 'EUR',
        errors: {},
        canRemove: true,
        onUpdate: vi.fn(),
        onDuplicate: vi.fn(),
        onRemove: vi.fn(),
        onQuickAddManufacturer
      }
    });

    await user.click(screen.getByRole('button', { name: 'Add Manufacturer' }));
    expect(onQuickAddManufacturer).toHaveBeenCalledWith('item-1');
  });

  it('keeps existing batch field values when opening quick-add actions (cancel/persistence)', async () => {
    const user = userEvent.setup();
    const onSellerChange = vi.fn();
    const onDateChange = vi.fn();

    render(AcquisitionBatchFields, {
      props: {
        sellerId: 'trn:seller:existing-shop',
        onSellerChange,
        purchaseDate: '2026-05-17',
        onDateChange,
        batchDefaults: { scale: 'H0', powerMethod: 'DC' },
        onBatchDefaultChange: vi.fn(),
        sellers: [
          {
            id: 'trn:seller:existing-shop',
            name: 'Existing Shop',
            sellerType: 'SHOP',
            email: null,
            phone: null,
            websiteUrl: null,
            address: null
          }
        ],
        onQuickAddSeller: vi.fn()
      }
    });

    await user.click(screen.getByRole('button', { name: 'Add Seller' }));

    expect(onSellerChange).not.toHaveBeenCalled();
    expect(onDateChange).not.toHaveBeenCalled();
  });

  it('emits seller quick-add callback', async () => {
    const user = userEvent.setup();
    const onQuickAddSeller = vi.fn();

    render(AcquisitionBatchFields, {
      props: {
        sellerId: null,
        onSellerChange: vi.fn(),
        purchaseDate: '2026-05-17',
        onDateChange: vi.fn(),
        batchDefaults: { scale: null, powerMethod: null },
        onBatchDefaultChange: vi.fn(),
        sellers: [],
        onQuickAddSeller
      }
    });

    await user.click(screen.getByRole('button', { name: 'Add Seller' }));
    expect(onQuickAddSeller).toHaveBeenCalledOnce();
  });
});
