import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

vi.mock('$lib/paraglide/messages.js', () => ({
  quick_add_entity_manufacturer: () => 'manufacturer',
  quick_add_entity_seller: () => 'seller',
  quick_add_entity_buyer: () => 'buyer',
  quick_add_duplicate_warning: ({ entity }: { entity: string }) =>
    `A ${entity} with this name already exists.`,
  quick_add_save_failed: () => 'Could not save. Please try again.',
  quick_add_name_required: () => 'Name is required.',
  quick_add_field_name: () => 'Name',
  quick_add_field_website: () => 'Website',
  quick_add_field_country: () => 'Country',
  quick_add_cancel: () => 'Cancel',
  quick_add_save: () => 'Save',
  settings_saving_button: () => 'Saving...'
}));

const createManufacturer = vi.fn();
const createSeller = vi.fn();

vi.mock('$lib/bindings', () => ({
  commands: {
    createManufacturer: (...args: unknown[]) => createManufacturer(...args),
    createSeller: (...args: unknown[]) => createSeller(...args)
  }
}));

import QuickAddEntityForm from '$lib/features/quick-add/QuickAddEntityForm.svelte';

describe('QuickAddEntityForm', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('disables save for duplicate manufacturer names (case-insensitive)', async () => {
    const user = userEvent.setup();
    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        existingNames: ['ACME'],
        onSuccess: vi.fn(),
        onCancel: vi.fn()
      }
    });

    await user.type(screen.getByLabelText('Name'), 'acme');

    expect(screen.getByText('A manufacturer with this name already exists.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
  });

  it('calls createManufacturer and emits success when valid', async () => {
    const user = userEvent.setup();
    const onSuccess = vi.fn();
    createManufacturer.mockResolvedValue({
      status: 'ok',
      data: {
        id: 'trn:manufacturer:acme',
        name: 'ACME',
        registeredCompanyName: null,
        countryCode: 'IT',
        status: 'ACTIVE',
        websiteUrl: null
      }
    });

    render(QuickAddEntityForm, {
      props: {
        target: 'manufacturer',
        existingNames: [],
        onSuccess,
        onCancel: vi.fn()
      }
    });

    await user.type(screen.getByLabelText('Name'), 'ACME');
    await user.click(screen.getByRole('button', { name: 'Save' }));

    expect(createManufacturer).toHaveBeenCalledOnce();
    expect(onSuccess).toHaveBeenCalledOnce();
  });

  it('calls onCancel when cancel is pressed', async () => {
    const user = userEvent.setup();
    const onCancel = vi.fn();

    render(QuickAddEntityForm, {
      props: {
        target: 'seller',
        existingNames: [],
        onSuccess: vi.fn(),
        onCancel
      }
    });

    await user.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onCancel).toHaveBeenCalledOnce();
  });
});
