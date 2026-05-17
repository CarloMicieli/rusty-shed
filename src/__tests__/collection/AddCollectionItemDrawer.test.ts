import { describe, it, expect, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_modal_manufacturer: () => 'Manufacturer',
  wishlist_modal_manufacturer_placeholder: () => 'Select',
  wishlist_modal_loading: () => 'Loading',
  quick_add_drawer_title_manufacturer: () => 'Add Manufacturer'
}));

import ManufacturerSelect from '$lib/components/drawer/ManufacturerSelect.svelte';

describe('Collection quick-add manufacturer trigger', () => {
  it('calls onQuickAdd when plus button is pressed', async () => {
    const user = userEvent.setup();
    const onQuickAdd = vi.fn();

    render(ManufacturerSelect, {
      props: {
        manufacturerId: null,
        manufacturers: [],
        onQuickAdd
      }
    });

    await user.click(screen.getByRole('button', { name: 'Add Manufacturer' }));
    expect(onQuickAdd).toHaveBeenCalledOnce();
  });
});
