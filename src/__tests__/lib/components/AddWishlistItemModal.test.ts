import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, cleanup } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

// Mock @tauri-apps/api/core BEFORE importing
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

// Mock toaster
vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn()
  }
}));

// Mock paraglide messages
vi.mock('$lib/paraglide/messages.js', () => ({
  wishlist_modal_title: () => 'Add to Wishlist',
  wishlist_modal_cancel: () => 'Cancel',
  wishlist_modal_save: () => 'Save',
  wishlist_modal_saving: () => 'Saving...',
  wishlist_modal_choose_or_create: () => 'Choose or Create Wishlist',
  wishlist_modal_select_list: () => 'Select a wishlist',
  wishlist_modal_select_placeholder: () => 'Select a wishlist',
  wishlist_modal_new_list_placeholder: () => 'Or create new list',
  wishlist_modal_notes_label: () => 'Notes',
  wishlist_modal_notes_placeholder: () => 'Optional notes',
  wishlist_modal_create_failed: () => 'Failed to create wishlist',
  wishlist_modal_select_list_error: () => 'Please select a wishlist',
  wishlist_modal_add_failed: () => 'Failed to add item',
  wishlist_modal_model_details: () => 'Model Details',
  wishlist_modal_manufacturer: () => 'Manufacturer',
  wishlist_modal_manufacturer_placeholder: () => '— Select manufacturer —',
  wishlist_modal_product_code: () => 'Product Code',
  wishlist_modal_product_code_placeholder: () => 'e.g., 37858',
  wishlist_modal_description: () => 'Description',
  wishlist_modal_description_placeholder: () => 'e.g., Class 218 Diesel Locomotive',
  wishlist_modal_category: () => 'Category',
  wishlist_modal_scale: () => 'Scale',
  wishlist_modal_power_method: () => 'Power Method',
  wishlist_modal_epoch: () => 'Epoch',
  wishlist_modal_epoch_placeholder: () => 'e.g., IV or III/IV',
  wishlist_modal_wishlist_prefs: () => 'Wishlist Preferences',
  wishlist_modal_priority: () => 'Priority',
  wishlist_modal_desired_price: () => 'Desired Price',
  wishlist_modal_price_placeholder: () => 'e.g., 89.99',
  wishlist_modal_loading: () => 'Loading...',
  wishlist_modal_missing_manufacturer: () => 'Please select a manufacturer',
  wishlist_modal_missing_product_code: () => 'Please enter a product code',
  wishlist_modal_missing_description: () => 'Please enter a description',
  wishlist_modal_invalid_price: () => 'Price must be greater than 0',
  wishlist_add_item_drawer_subtitle: () => 'Add to your collection wish list',
  wishlist_add_item_drawer_discard_title: () => 'Discard wishlist item?',
  wishlist_add_item_drawer_discard_description: () => 'You have unsaved changes. Discard them?',
  wishlist_add_item_drawer_discard_confirm: () => 'Discard',
  wishlist_add_item_drawer_discard_cancel: () => 'Keep editing',
  wishlist_category_locomotives: () => 'Locomotives',
  wishlist_category_train_sets: () => 'Train Sets',
  wishlist_category_starter_sets: () => 'Starter Sets',
  wishlist_category_freight_cars: () => 'Freight Cars',
  wishlist_category_passenger_cars: () => 'Passenger Cars',
  wishlist_category_electric_multiple_units: () => 'Electric Multiple Units',
  wishlist_category_railcars: () => 'Railcars',
  wishlist_power_ac: () => 'AC',
  wishlist_power_dc: () => 'DC',
  wishlist_power_trix_express: () => 'Trix Express',
  wishlist_priority_low: () => 'Low',
  wishlist_priority_normal: () => 'Normal',
  wishlist_priority_high: () => 'High',
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry',
  drawer_discard_title: () => 'Discard changes?',
  drawer_discard_description: () => 'You have unsaved changes. Discard them?',
  drawer_discard_confirm: () => 'Discard',
  drawer_discard_cancel: () => 'Keep editing',
  drawer_section_model_info: () => 'Model Details',
  drawer_section_wishlist: () => 'Wishlist Preferences',
  drawer_section_rolling_stocks: () => 'Rolling Stocks',
  drawer_section_purchase: () => 'Purchase Details',
  drawer_section_digital: () => 'Digital Settings',
  rolling_stock_select_category: () => '— Select category —'
}));

let activeService: ReturnType<
  typeof import('$lib/features/wishlists/WishlistState.svelte').createWishlistState
>;

vi.mock('$lib/features/wishlists/WishlistState.svelte', async (importOriginal) => {
  const actual =
    await importOriginal<typeof import('$lib/features/wishlists/WishlistState.svelte')>();
  return {
    ...actual,
    getWishlistContext: () => activeService
  };
});

// Now import after mocks
import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';
import {
  createWishlistState,
  type WishlistPreviewLite
} from '$lib/features/wishlists/WishlistState.svelte';
import { invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);
type InvokeArgType = InvokeArgs | undefined;
type InvokeOptionType = InvokeOptions | undefined;
type Handler = (args?: InvokeArgType) => unknown;

activeService = createWishlistState();

const wishlistFixtures: WishlistPreviewLite[] = [
  {
    id: 'wishlist-1',
    name: 'My Wishlist',
    notes: null,
    is_default: true,
    isDefault: true,
    count: 0n,
    updated_at: '2024-01-01T00:00:00Z',
    updatedAt: '2024-01-01T00:00:00Z',
    total_value: {},
    totalValue: {}
  } as unknown as WishlistPreviewLite,
  {
    id: 'wishlist-2',
    name: 'Future Purchases',
    notes: null,
    is_default: false,
    isDefault: false,
    count: 0n,
    updated_at: '2024-01-02T00:00:00Z',
    updatedAt: '2024-01-02T00:00:00Z',
    total_value: {},
    totalValue: {}
  } as unknown as WishlistPreviewLite
];

const manufacturerFixtures = [
  { id: 'märklin', name: 'Märklin', groupName: null },
  { id: 'fleischmann', name: 'Fleischmann', groupName: null }
];

const wishlistViewFixture = {
  id: 'wishlist-1',
  name: 'My Wishlist',
  notes: null,
  is_default: true,
  items: []
};

// Helper for Tauri mock
const tauriMock = {
  handlers: new Map<string, Handler>(),
  delays: new Map<string, number>(),

  mockCommand<T>(command: string, response: T) {
    this.handlers.set(command, () => response);
  },

  mockCommandError(command: string, error: unknown) {
    this.handlers.set(command, () => {
      throw error;
    });
  },

  mockCommandWithDelay<T>(command: string, delay: number, response: T) {
    this.delays.set(command, delay);
    this.mockCommand(command, response);
  },

  reset() {
    this.handlers.clear();
    this.delays.clear();
    mockInvoke.mockReset();
    // Re-apply the implementation
    mockInvoke.mockImplementation(
      async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
        const handler = this.handlers.get(command);
        const delay = this.delays.get(command) || 0;

        if (!handler) {
          throw new Error(`Unmocked Tauri command: ${command}`);
        }

        if (delay > 0) {
          await new Promise((resolve) => setTimeout(resolve, delay));
        }

        return handler(args);
      }
    );
  }
};

// Initial setup
mockInvoke.mockImplementation(
  async (command: string, args?: InvokeArgType, _options?: InvokeOptionType) => {
    const handler = tauriMock.handlers.get(command);
    const delay = tauriMock.delays.get(command) || 0;

    if (!handler) {
      throw new Error(`Unmocked Tauri command: ${command}`);
    }

    if (delay > 0) {
      await new Promise((resolve) => setTimeout(resolve, delay));
    }

    return handler(args);
  }
);

/**
 * Wait for the manufacturer Select.Trigger to appear (loading done),
 * click it to open the dropdown, then click the matching option.
 */
async function selectManufacturer(name: string) {
  const user = userEvent.setup();
  // Wait for loading state to disappear
  await waitFor(() => {
    expect(screen.queryByText('Loading...')).toBeNull();
  });
  const trigger = screen.getByRole('button', { name: /^manufacturer/i });
  await user.click(trigger);
  const item = await screen.findByRole('option', { name: new RegExp(name, 'i') });
  await user.click(item);
}

describe('AddWishlistItemDrawer', () => {
  const defaultProps = {
    open: true,
    onClose: vi.fn(),
    onSaved: vi.fn()
  };

  beforeEach(async () => {
    cleanup();
    activeService = createWishlistState();
    vi.clearAllMocks();
    tauriMock.reset();

    tauriMock.mockCommand('get_wishlists', wishlistFixtures);
    tauriMock.mockCommand('get_manufacturers', manufacturerFixtures);
    tauriMock.mockCommand('get_wishlist_by_id', wishlistViewFixture);
    await activeService.fetchWishlists();
  });

  it('should render modal with title and form fields', async () => {
    render(AddWishlistItemDrawer, { props: defaultProps });

    expect(screen.getByText('Add to Wishlist')).toBeInTheDocument();
    await waitFor(() => {
      expect(screen.getByRole('button', { name: /^manufacturer/i })).toBeInTheDocument();
    });
    expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /save/i })).toBeInTheDocument();
  });

  it('should display available wishlists in dropdown', async () => {
    const user = userEvent.setup();
    render(AddWishlistItemDrawer, { props: defaultProps });

    const trigger = await screen.findByRole('button', { name: /select a wishlist/i });
    await user.click(trigger);

    await waitFor(() => {
      expect(screen.getByRole('option', { name: /my wishlist/i })).toBeInTheDocument();
      expect(screen.getByRole('option', { name: /future purchases/i })).toBeInTheDocument();
    });
  });

  it('should show validation error when manufacturer is missing', async () => {
    const user = userEvent.setup();
    render(AddWishlistItemDrawer, { props: defaultProps });

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Please select a manufacturer')).toBeInTheDocument();
    });
  });

  it('should show validation error when product code is missing', async () => {
    const user = userEvent.setup();
    render(AddWishlistItemDrawer, { props: defaultProps });

    await selectManufacturer('Märklin');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Please enter a product code')).toBeInTheDocument();
    });
  });

  it('should show validation error when description is missing', async () => {
    const user = userEvent.setup();
    render(AddWishlistItemDrawer, { props: defaultProps });

    await selectManufacturer('Märklin');

    await user.type(screen.getByLabelText(/product code/i), '37858');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Please enter a description')).toBeInTheDocument();
    });
  });

  it('should add model to existing wishlist', async () => {
    const user = userEvent.setup();

    tauriMock.mockCommand('add_railway_model_to_wish_list', null);

    const onSaved = vi.fn();
    const onClose = vi.fn();
    render(AddWishlistItemDrawer, {
      props: { open: true, onSaved, onClose }
    });

    // Select manufacturer via shadcn Select
    await selectManufacturer('Märklin');

    // Fill product code
    await user.type(screen.getByLabelText(/product code/i), '37858');

    // Fill description
    await user.type(screen.getByLabelText(/description/i), 'Class 218 Diesel');

    // Submit
    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(onSaved).toHaveBeenCalled();
      expect(onClose).toHaveBeenCalled();
    });
  });

  it('should create new wishlist and add model', async () => {
    const user = userEvent.setup();

    const mockCreatedWishlist: WishlistPreviewLite = {
      id: 'new-wishlist',
      name: 'New List',
      notes: null,
      is_default: false,
      isDefault: false,
      count: 0n,
      updated_at: '2024-01-03T00:00:00Z',
      updatedAt: '2024-01-03T00:00:00Z',
      total_value: {},
      totalValue: {}
    } as unknown as WishlistPreviewLite;

    tauriMock.mockCommand('create_wishlist', mockCreatedWishlist);
    tauriMock.mockCommand('add_railway_model_to_wish_list', null);

    render(AddWishlistItemDrawer, { props: defaultProps });

    // Enter new list name (fills wishlistId indirectly via newListName)
    await user.type(screen.getByPlaceholderText('Or create new list'), 'New List');

    // Fill required fields
    await selectManufacturer('Märklin');

    await user.type(screen.getByLabelText(/product code/i), '37858');
    await user.type(screen.getByLabelText(/description/i), 'Class 218 Diesel');

    // Submit
    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('create_wishlist', expect.any(Object));
      expect(mockInvoke).toHaveBeenCalledWith(
        'add_railway_model_to_wish_list',
        expect.objectContaining({
          args: expect.objectContaining({ wishlistId: 'new-wishlist' })
        })
      );
    });
  });

  it('should show error when creating wishlist fails', async () => {
    const user = userEvent.setup();
    const error = { ValidationError: { name: 'Name already exists' } };

    tauriMock.mockCommandError('create_wishlist', error);

    render(AddWishlistItemDrawer, { props: defaultProps });

    await user.type(screen.getByPlaceholderText('Or create new list'), 'Duplicate Name');

    await selectManufacturer('Märklin');

    await user.type(screen.getByLabelText(/product code/i), '37858');
    await user.type(screen.getByLabelText(/description/i), 'Test Model');

    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(screen.getByText('Failed to create wishlist')).toBeInTheDocument();
    });
  });

  it('should show error when adding model fails', async () => {
    const user = userEvent.setup();

    tauriMock.mockCommandError('add_railway_model_to_wish_list', { NotFound: 'Not found' });

    render(AddWishlistItemDrawer, { props: defaultProps });

    await selectManufacturer('Märklin');

    await user.type(screen.getByLabelText(/product code/i), '37858');
    await user.type(screen.getByLabelText(/description/i), 'Test Model');

    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(screen.getByText('Failed to add item')).toBeInTheDocument();
    });
  });

  it('should disable buttons while submitting', async () => {
    const user = userEvent.setup();

    tauriMock.mockCommandWithDelay('add_railway_model_to_wish_list', 200, null);

    render(AddWishlistItemDrawer, { props: defaultProps });

    await selectManufacturer('Märklin');

    await user.type(screen.getByLabelText(/product code/i), '37858');
    await user.type(screen.getByLabelText(/description/i), 'Class 218 Diesel');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    // Button should be disabled during submission
    await waitFor(() => {
      expect(saveButton).toBeDisabled();
    });
  });

  it('should close drawer directly when no changes have been made', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(AddWishlistItemDrawer, {
      props: { ...defaultProps, onClose }
    });

    await waitFor(() => {
      expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    });

    // No changes made — close button should call onClose directly
    const closeButton = screen.getByRole('button', { name: /close/i });
    await user.click(closeButton);

    expect(onClose).toHaveBeenCalled();
  });

  it('should show discard dialog when closing with unsaved changes', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(AddWishlistItemDrawer, {
      props: { ...defaultProps, onClose }
    });

    await waitFor(() => {
      expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    });

    await user.type(screen.getByLabelText(/product code/i), 'test-value');

    const closeButton = screen.getByRole('button', { name: /close/i });
    await user.click(closeButton);

    // Discard dialog should appear, not close immediately
    await waitFor(() => {
      expect(screen.getByText('Discard wishlist item?')).toBeInTheDocument();
    });
    expect(onClose).not.toHaveBeenCalled();

    // Confirm discard
    await user.click(screen.getByRole('button', { name: /discard/i }));
    expect(onClose).toHaveBeenCalled();
  });
});
