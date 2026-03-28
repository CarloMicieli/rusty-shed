import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import SellerForm from '$lib/components/SellerForm.svelte';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages.js', () => ({
  form_new_model_cancel: () => 'Cancel',
  form_new_model_create: () => 'Create Railway Model',
  wishlist_modal_saving: () => 'Saving...'
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
      name: 'Create Railway Model'
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
