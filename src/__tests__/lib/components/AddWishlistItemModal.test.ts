import { beforeEach, describe, expect, it, vi } from 'vitest';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';

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
  wishlist_modal_select_placeholder: () => 'No wishlists found',
  wishlist_modal_new_list_placeholder: () => 'Or create new list',
  wishlist_picker_search_placeholder: () => 'Search wishlists or type to create new...',
  wishlist_picker_new_badge: () => '[new]',
  wishlist_picker_create_label: () => 'Create',
  wishlist_picker_no_results: () => 'No wishlists found',
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
  enum_category_locomotives: () => 'Locomotives',
  enum_category_train_sets: () => 'Train Sets',
  enum_category_starter_sets: () => 'Starter Sets',
  enum_category_freight_cars: () => 'Freight Cars',
  enum_category_passenger_cars: () => 'Passenger Cars',
  enum_category_electric_multiple_units: () => 'Electric Multiple Units',
  enum_category_railcars: () => 'Railcars',
  enum_power_method_ac: () => 'AC',
  enum_power_method_dc: () => 'DC',
  enum_power_method_trix_express: () => 'Trix Express',
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
  rolling_stock_select_category: () => '— Select category —',
  action_close: () => 'Close'
}));

vi.mock('$lib/bindings', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/bindings')>();
  return {
    ...actual,
    commands: {
      ...actual.commands,
      getManufacturers: vi.fn()
    }
  };
});

type WishlistServiceLike = {
  wishlists: WishlistPreviewLite[];
  defaultWishlist: WishlistPreviewLite | null;
  createWishlist: ReturnType<typeof vi.fn>;
  addRailwayModelToWishlist: ReturnType<typeof vi.fn>;
};

let activeService: WishlistServiceLike;

vi.mock('$lib/features/wishlists/WishlistState.svelte', async (importOriginal) => {
  const actual =
    await importOriginal<typeof import('$lib/features/wishlists/WishlistState.svelte')>();
  return {
    ...actual,
    getWishlistContext: () => activeService
  };
});

import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';
import { commands } from '$lib/bindings';
import type { WishlistPreviewLite } from '$lib/features/wishlists/WishlistState.svelte';
import type { Manufacturer } from '$lib/bindings';

const mockGetManufacturers = vi.mocked(commands.getManufacturers);

const wishlistFixtures: WishlistPreviewLite[] = [
  {
    id: 'wishlist-1',
    name: 'My Wishlist',
    notes: null,
    is_default: true,
    isDefault: true,
    count: 0,
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
    count: 0,
    updated_at: '2024-01-02T00:00:00Z',
    updatedAt: '2024-01-02T00:00:00Z',
    total_value: {},
    totalValue: {}
  } as unknown as WishlistPreviewLite
];

const manufacturerFixtures: Manufacturer[] = [
  {
    id: 'marklin',
    name: 'Märklin',
    registeredCompanyName: null,
    countryCode: null,
    status: 'ACTIVE',
    websiteUrl: null
  },
  {
    id: 'fleischmann',
    name: 'Fleischmann',
    registeredCompanyName: null,
    countryCode: null,
    status: 'ACTIVE',
    websiteUrl: null
  }
];

function createServiceMock(overrides?: Partial<WishlistServiceLike>): WishlistServiceLike {
  const base: WishlistServiceLike = {
    wishlists: [],
    defaultWishlist: null,
    createWishlist: vi.fn().mockResolvedValue(null),
    addRailwayModelToWishlist: vi.fn().mockResolvedValue(true)
  };

  return { ...base, ...overrides };
}

async function renderDrawer(
  props: {
    open: boolean;
    onClose: () => void;
    onSaved: () => void;
    preselectedWishlistId?: string;
  } = defaultProps
) {
  render(AddWishlistItemDrawer, { props });
  await screen.findByRole('button', { name: /^manufacturer/i });
}

async function selectManufacturerWithUser(user: ReturnType<typeof userEvent.setup>, name: string) {
  await user.click(screen.getByRole('button', { name: /^manufacturer/i }));
  await user.click(await screen.findByRole('option', { name: new RegExp(name, 'i') }));
}

const defaultProps = {
  open: true,
  onClose: vi.fn(),
  onSaved: vi.fn()
};

describe('AddWishlistItemDrawer', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();

    activeService = createServiceMock();

    mockGetManufacturers.mockResolvedValue({
      status: 'ok',
      data: manufacturerFixtures
    });
  });

  it('should render modal with title, form fields, and available wishlists', async () => {
    const user = userEvent.setup();

    activeService = createServiceMock({
      wishlists: wishlistFixtures,
      defaultWishlist: wishlistFixtures[0]
    });

    await renderDrawer();

    expect(screen.getByText('Add to Wishlist')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /^manufacturer/i })).toBeInTheDocument();
    expect(screen.getByLabelText(/product code/i)).toBeInTheDocument();
    expect(screen.getByLabelText(/description/i)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /save/i })).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: /select a wishlist/i }));
    expect(await screen.findByRole('option', { name: /my wishlist/i })).toBeInTheDocument();
    expect(screen.getByRole('option', { name: /future purchases/i })).toBeInTheDocument();
  });

  it('should show all validation errors in sequence (manufacturer → product code → description)', async () => {
    const user = userEvent.setup();
    await renderDrawer();

    const saveButton = screen.getByRole('button', { name: /save/i });

    await user.click(saveButton);
    expect(await screen.findByText('Please select a manufacturer')).toBeInTheDocument();

    await selectManufacturerWithUser(user, 'Märklin');
    await user.click(saveButton);
    expect(await screen.findByText('Please enter a product code')).toBeInTheDocument();

    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.click(saveButton);
    expect(await screen.findByText('Please enter a description')).toBeInTheDocument();
  });

  it('should add model to existing wishlist', async () => {
    const user = userEvent.setup();
    const onSaved = vi.fn();
    const onClose = vi.fn();

    activeService = createServiceMock({
      addRailwayModelToWishlist: vi.fn().mockResolvedValue(true)
    });

    await renderDrawer({ open: true, onSaved, onClose, preselectedWishlistId: 'wishlist-1' });

    await selectManufacturerWithUser(user, 'Märklin');
    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.type(screen.getByLabelText(/description/i), 'a');

    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(activeService.addRailwayModelToWishlist).toHaveBeenCalledWith(
        expect.objectContaining({
          wishlistId: 'wishlist-1'
        })
      );
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
      count: 0,
      updated_at: '2024-01-03T00:00:00Z',
      updatedAt: '2024-01-03T00:00:00Z',
      total_value: {},
      totalValue: {}
    } as unknown as WishlistPreviewLite;

    activeService = createServiceMock({
      wishlists: wishlistFixtures,
      defaultWishlist: wishlistFixtures[0],
      createWishlist: vi.fn().mockResolvedValue(mockCreatedWishlist),
      addRailwayModelToWishlist: vi.fn().mockResolvedValue(true)
    });

    await renderDrawer();

    await user.click(screen.getByRole('button', { name: /select a wishlist/i }));
    await user.type(
      await screen.findByPlaceholderText('Search wishlists or type to create new...'),
      'N'
    );
    await user.click(await screen.findByRole('button', { name: /create/i }));

    await selectManufacturerWithUser(user, 'Märklin');
    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.type(screen.getByLabelText(/description/i), 'a');

    await user.click(screen.getByRole('button', { name: /save/i }));

    await waitFor(() => {
      expect(activeService.createWishlist).toHaveBeenCalledWith('N', false, true);
      expect(activeService.addRailwayModelToWishlist).toHaveBeenCalledWith(
        expect.objectContaining({
          wishlistId: 'new-wishlist'
        })
      );
    });
  });

  it('should show error when creating wishlist fails', async () => {
    const user = userEvent.setup();

    activeService = createServiceMock({
      wishlists: wishlistFixtures,
      defaultWishlist: wishlistFixtures[0],
      createWishlist: vi.fn().mockResolvedValue(null)
    });

    await renderDrawer();

    await user.click(screen.getByRole('button', { name: /select a wishlist/i }));
    await user.type(
      await screen.findByPlaceholderText('Search wishlists or type to create new...'),
      'D'
    );
    await user.click(await screen.findByRole('button', { name: /create/i }));

    await selectManufacturerWithUser(user, 'Märklin');
    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.type(screen.getByLabelText(/description/i), 'a');
    await user.click(screen.getByRole('button', { name: /save/i }));

    expect(await screen.findByText('Failed to create wishlist')).toBeInTheDocument();
  });

  it('should show error when adding model fails', async () => {
    const user = userEvent.setup();

    activeService = createServiceMock({
      addRailwayModelToWishlist: vi.fn().mockResolvedValue(false)
    });

    await renderDrawer({
      open: true,
      onClose: vi.fn(),
      onSaved: vi.fn(),
      preselectedWishlistId: 'wishlist-1'
    });

    await selectManufacturerWithUser(user, 'Märklin');
    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.type(screen.getByLabelText(/description/i), 'a');
    await user.click(screen.getByRole('button', { name: /save/i }));

    expect(await screen.findByText('Failed to add item')).toBeInTheDocument();
  });

  it('should disable buttons while submitting', async () => {
    const user = userEvent.setup();

    let resolveSubmit!: (value: boolean) => void;
    const pendingSubmit = new Promise<boolean>((resolve) => {
      resolveSubmit = resolve;
    });

    activeService = createServiceMock({
      addRailwayModelToWishlist: vi.fn().mockReturnValue(pendingSubmit)
    });

    await renderDrawer({
      open: true,
      onClose: vi.fn(),
      onSaved: vi.fn(),
      preselectedWishlistId: 'wishlist-1'
    });

    await selectManufacturerWithUser(user, 'Märklin');
    await user.type(screen.getByLabelText(/product code/i), '1');
    await user.type(screen.getByLabelText(/description/i), 'a');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(saveButton).toBeDisabled();
    });

    resolveSubmit(true);
  });

  it('should close drawer directly when no changes have been made', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();

    await renderDrawer({
      ...defaultProps,
      onClose
    });

    await user.click(screen.getByRole('button', { name: /close/i }));

    expect(onClose).toHaveBeenCalled();
  });

  it('should show discard dialog when closing with unsaved changes', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();

    await renderDrawer({
      ...defaultProps,
      onClose
    });

    await user.type(screen.getByLabelText(/product code/i), 't');
    await user.click(screen.getByRole('button', { name: /close/i }));

    expect(await screen.findByText('Discard wishlist item?')).toBeInTheDocument();
    expect(onClose).not.toHaveBeenCalled();

    await user.click(screen.getByRole('button', { name: /discard/i }));
    expect(onClose).toHaveBeenCalled();
  });
});
