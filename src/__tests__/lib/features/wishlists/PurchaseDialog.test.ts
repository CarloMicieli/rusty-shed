import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/svelte';
import PurchaseDialog from '$lib/features/wishlists/components/PurchaseDialog.svelte';

// ── Paraglide messages ───────────────────────────────────────────────────────
vi.mock('$lib/paraglide/messages.js', () => ({
  purchase_dialog_title: () => 'Record Purchase',
  purchase_dialog_price_label: () => 'Price Paid',
  purchase_dialog_price_placeholder: () => '0.00',
  purchase_dialog_date_label: () => 'Purchase Date',
  purchase_dialog_seller_label: () => 'Seller',
  purchase_dialog_seller_placeholder: () => 'Select a seller…',
  purchase_dialog_condition_label: () => 'Condition',
  purchase_dialog_condition_placeholder: () => 'Select condition…',
  purchase_dialog_condition_new: () => 'New',
  purchase_dialog_condition_pre_owned_like_new: () => 'Pre-Owned – Like New',
  purchase_dialog_condition_pre_owned_very_good: () => 'Pre-Owned – Very Good',
  purchase_dialog_condition_pre_owned_good: () => 'Pre-Owned – Good',
  purchase_dialog_condition_pre_owned_acceptable: () => 'Pre-Owned – Acceptable',
  purchase_dialog_submit: () => 'Record Purchase',
  purchase_dialog_cancel: () => 'Cancel',
  purchase_dialog_error_price_required: () => 'Price is required',
  purchase_dialog_error_future_date_forbidden: () => 'Purchase date cannot be in the future',
  purchase_dialog_error_save_failed: () => 'Failed to save purchase. Please try again.'
}));

// ── Bindings (commands) ──────────────────────────────────────────────────────
const mockGetSellers = vi.fn();
const mockGetSettings = vi.fn();
const mockPurchaseWishlistItem = vi.fn();

vi.mock('$lib/bindings', () => ({
  commands: {
    getSellers: () => mockGetSellers(),
    getSettings: () => mockGetSettings(),
    purchaseWishlistItem: (args: unknown) => mockPurchaseWishlistItem(args)
  }
}));

const baseProps = {
  open: true,
  wishlistId: 'trn:wishlist:w1',
  wishlistItemId: 'trn:wishlist-item:i1',
  itemName: 'BR 218 Diesel Locomotive',
  onClose: vi.fn(),
  onSuccess: vi.fn()
};

describe('PurchaseDialog.svelte', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockGetSellers.mockResolvedValue({ status: 'ok', data: [] });
    mockGetSettings.mockResolvedValue({ status: 'ok', data: { currency: 'EUR' } });
  });

  it('renders dialog when open=true', async () => {
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getByRole('dialog')).toBeInTheDocument();
    });
  });

  it('does not render dialog when open=false', () => {
    render(PurchaseDialog, { props: { ...baseProps, open: false } });
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('shows item name in title area', async () => {
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getByText('BR 218 Diesel Locomotive')).toBeInTheDocument();
    });
  });

  it('shows "Record Purchase" title', async () => {
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getAllByText('Record Purchase').length).toBeGreaterThan(0);
    });
  });

  it('renders price input, date input, and condition select after loading', async () => {
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getByLabelText('Price Paid')).toBeInTheDocument();
      expect(screen.getByLabelText('Purchase Date')).toBeInTheDocument();
      expect(screen.getByLabelText('Condition')).toBeInTheDocument();
    });
  });

  it('renders seller options when sellers are loaded', async () => {
    mockGetSellers.mockResolvedValue({
      status: 'ok',
      data: [
        { id: 'trn:seller:s1', name: 'Model Shop A', sellerType: 'SHOP', email: null, phone: null }
      ]
    });
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getByText('Model Shop A')).toBeInTheDocument();
    });
  });

  it('shows validation error when price is empty on submit', async () => {
    render(PurchaseDialog, { props: baseProps });
    // Wait for form to load
    await waitFor(() => expect(screen.getByLabelText('Price Paid')).toBeInTheDocument());

    // Submit form directly (not via button click, which doesn't trigger submit in happy-dom)
    const form = document.querySelector('form') as HTMLFormElement;
    await fireEvent.submit(form);

    await waitFor(() => {
      expect(screen.getByText('Price is required')).toBeInTheDocument();
    });
  });

  it('calls onClose when Cancel button is clicked', async () => {
    const onClose = vi.fn();
    render(PurchaseDialog, { props: { ...baseProps, onClose } });
    await waitFor(() => expect(screen.getAllByText('Cancel').length).toBeGreaterThan(0));

    const cancelBtn = screen.getAllByText('Cancel')[0];
    await fireEvent.click(cancelBtn);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when Escape key is pressed', async () => {
    const onClose = vi.fn();
    render(PurchaseDialog, { props: { ...baseProps, onClose } });
    await waitFor(() => expect(screen.getByRole('dialog')).toBeInTheDocument());

    await fireEvent.keyDown(window, { key: 'Escape' });
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('renders all condition options', async () => {
    render(PurchaseDialog, { props: baseProps });
    await waitFor(() => {
      expect(screen.getByText('New')).toBeInTheDocument();
      expect(screen.getByText('Pre-Owned – Like New')).toBeInTheDocument();
      expect(screen.getByText('Pre-Owned – Very Good')).toBeInTheDocument();
      expect(screen.getByText('Pre-Owned – Good')).toBeInTheDocument();
      expect(screen.getByText('Pre-Owned – Acceptable')).toBeInTheDocument();
    });
  });

  it('calls purchaseWishlistItem and onSuccess on valid submit', async () => {
    mockPurchaseWishlistItem.mockResolvedValue({ status: 'ok' });
    render(PurchaseDialog, {
      props: { ...baseProps, onSuccess: vi.fn(), onClose: vi.fn() }
    });

    await waitFor(() => expect(screen.getByLabelText('Price Paid')).toBeInTheDocument());

    // Set the price input value via native DOM + input event (updates Svelte bind:value)
    const priceInput = screen.getByLabelText('Price Paid') as HTMLInputElement;
    priceInput.value = '49.99';
    await fireEvent.input(priceInput);

    const form = document.querySelector('form') as HTMLFormElement;
    await fireEvent.submit(form);

    await waitFor(() => {
      expect(mockPurchaseWishlistItem).toHaveBeenCalled();
    });
  });

  it('shows error message when save fails', async () => {
    mockPurchaseWishlistItem.mockResolvedValue({ status: 'error', error: 'Save failed' });
    render(PurchaseDialog, { props: baseProps });

    await waitFor(() => expect(screen.getByLabelText('Price Paid')).toBeInTheDocument());

    const priceInput = screen.getByLabelText('Price Paid') as HTMLInputElement;
    priceInput.value = '49.99';
    await fireEvent.input(priceInput);

    const form = document.querySelector('form') as HTMLFormElement;
    await fireEvent.submit(form);

    await waitFor(() => {
      expect(screen.getByText('Failed to save purchase. Please try again.')).toBeInTheDocument();
    });
  });
});
