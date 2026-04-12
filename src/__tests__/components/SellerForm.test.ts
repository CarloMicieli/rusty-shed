import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import SellerForm from '$lib/components/SellerForm.svelte';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages.js', () => ({
  form_new_model_cancel: () => 'Cancel',
  wishlist_modal_saving: () => 'Saving...',
  seller_form_edit_title: () => 'Edit Seller',
  seller_form_new_title: () => 'New Seller',
  seller_form_field_name: () => 'Name',
  seller_form_placeholder_name: () => 'Seller name',
  seller_form_field_seller_type: () => 'Seller Type',
  seller_form_type_shop: () => 'Shop',
  seller_form_type_private: () => 'Private',
  seller_form_type_manufacturer: () => 'Manufacturer',
  seller_form_field_email: () => 'Email',
  seller_form_placeholder_email: () => 'email@example.com',
  seller_form_field_phone: () => 'Phone',
  seller_form_placeholder_phone: () => 'Phone number',
  seller_form_field_website: () => 'Website',
  seller_form_placeholder_website: () => 'https://example.com',
  seller_form_field_street_address: () => 'Street Address',
  seller_form_placeholder_street_address: () => '123 Main St',
  seller_form_field_extended_address: () => 'Extended Address',
  seller_form_placeholder_extended_address: () => 'Apt 4B',
  seller_form_field_city: () => 'City',
  seller_form_placeholder_city: () => 'City',
  seller_form_field_region: () => 'Region',
  seller_form_placeholder_region: () => 'Region',
  seller_form_field_postal_code: () => 'Postal Code',
  seller_form_placeholder_postal_code: () => '12345',
  seller_form_field_country_code: () => 'Country Code',
  seller_form_placeholder_country_code: () => 'US',
  seller_form_unsaved_changes: () => 'You have unsaved changes',
  seller_form_save_success: () => 'Seller saved successfully',
  seller_form_save_failed: () => 'Failed to save seller',
  seller_form_error_unexpected: () => 'An unexpected error occurred',
  seller_form_submit_create: () => 'Create Seller',
  seller_form_submit_update: () => 'Update Seller'
}));

// ── Seller service ───────────────────────────────────────────────────────────
const mockCreateSeller = vi.fn();
const mockUpdateSeller = vi.fn();
vi.mock('$lib/services/sellerService', () => ({
  createSeller: (...args: unknown[]) => mockCreateSeller(...args),
  updateSeller: (...args: unknown[]) => mockUpdateSeller(...args),
  getSellers: vi.fn(),
  getSellerById: vi.fn(),
  deleteSeller: vi.fn()
}));

// ── Toaster ──────────────────────────────────────────────────────────────────
vi.mock('$lib/toaster', () => ({
  toaster: {
    success: vi.fn(),
    error: vi.fn(),
    loading: vi.fn()
  }
}));

describe('SellerForm.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders "New Seller" heading with all form fields and buttons', () => {
    render(SellerForm, { props: {} });
    expect(screen.getByText('New Seller')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Seller name')).toBeInTheDocument();
    expect(screen.getByText('Shop')).toBeInTheDocument();
    expect(screen.getByText('Private')).toBeInTheDocument();
    expect(screen.getByText('Manufacturer')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('email@example.com')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Phone number')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('https://example.com')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('123 Main St')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('City')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('12345')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
    const submitBtn = screen.getByRole('button', {
      name: 'Create Seller'
    }) as HTMLButtonElement;
    expect(submitBtn).toBeInTheDocument();
    expect(submitBtn.disabled).toBe(false);
  });

  it('renders "Edit Seller" heading when initial value has an id', () => {
    render(SellerForm, {
      props: {
        initial: {
          id: 'trn:seller:s1',
          name: 'Test Shop',
          sellerType: 'SHOP',
          email: null,
          phone: null,
          websiteUrl: null,
          streetAddress: null,
          extendedAddress: null,
          city: null,
          stateRegion: null,
          postalCode: null,
          countryCode: null
        }
      }
    });
    expect(screen.getByText('Edit Seller')).toBeInTheDocument();
  });

  it('calls onClose when Cancel button is clicked', async () => {
    const onClose = vi.fn();
    render(SellerForm, { props: { onClose } });
    await fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('populates initial values into form fields and shows unsaved changes when tainted', async () => {
    render(SellerForm, {
      props: {
        initial: {
          id: undefined,
          name: 'Acme Models',
          sellerType: 'SHOP',
          email: 'info@acme.com',
          phone: null,
          websiteUrl: null,
          streetAddress: null,
          extendedAddress: null,
          city: null,
          stateRegion: null,
          postalCode: null,
          countryCode: null
        }
      }
    });
    const nameInput = screen.getByPlaceholderText('Seller name') as HTMLInputElement;
    expect(nameInput.value).toBe('Acme Models');

    await fireEvent.input(nameInput, { target: { value: 'New Seller Name' } });
    await waitFor(() => {
      expect(screen.getByText('You have unsaved changes')).toBeInTheDocument();
    });
  });
});
