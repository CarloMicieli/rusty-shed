import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  acquisition_seller_label: () => 'Seller',
  acquisition_date_label: () => 'Date',
  acquisition_batch_scale_label: () => 'Scale',
  acquisition_batch_power_label: () => 'Power',
  quick_add_drawer_title_seller: () => 'Add Seller',
  quick_add_drawer_title_buyer: () => 'Add Buyer'
}));

import AcquisitionBatchFields from '$lib/features/acquisition/components/AcquisitionBatchFields.svelte';

describe('Acquisition quick-add triggers', () => {
  it('emits seller and buyer quick-add callbacks', async () => {
    const user = userEvent.setup();
    const onQuickAddSeller = vi.fn();
    const onQuickAddBuyer = vi.fn();

    render(AcquisitionBatchFields, {
      props: {
        sellerId: null,
        onSellerChange: vi.fn(),
        purchaseDate: '2026-05-17',
        onDateChange: vi.fn(),
        batchDefaults: { scale: null, powerMethod: null },
        onBatchDefaultChange: vi.fn(),
        sellers: [],
        onQuickAddSeller,
        onQuickAddBuyer
      }
    });

    await user.click(screen.getByRole('button', { name: 'Add Seller' }));
    await user.click(screen.getByRole('button', { name: 'Add Buyer' }));

    expect(onQuickAddSeller).toHaveBeenCalledOnce();
    expect(onQuickAddBuyer).toHaveBeenCalledOnce();
  });
});
