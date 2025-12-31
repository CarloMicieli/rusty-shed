import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';
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
  wishlist_modal_close: () => 'Close',
  wishlist_modal_cancel: () => 'Cancel',
  wishlist_modal_save: () => 'Save',
  wishlist_modal_saving: () => 'Saving...',
  wishlist_modal_choose_or_create: () => 'Choose or Create Wishlist',
  wishlist_modal_select_list: () => 'Select Wishlist',
  wishlist_modal_select_placeholder: () => 'Select a wishlist',
  wishlist_modal_new_list_placeholder: () => 'Or create new list',
  wishlist_modal_item_id_label: () => 'Model ID',
  wishlist_modal_item_id_placeholder: () => 'e.g., 79894',
  wishlist_modal_notes_label: () => 'Notes',
  wishlist_modal_notes_placeholder: () => 'Optional notes',
  wishlist_modal_missing_model: () => 'Model ID is required',
  wishlist_modal_create_failed: () => 'Failed to create wishlist',
  wishlist_modal_add_failed: () => 'Failed to add item',
  wishlist_modal_select_list_error: () => 'Please select a wishlist',
  collection_toast_loading: () => 'Loading...',
  collection_toast_success: () => 'Success',
  collection_toast_error: () => 'Error',
  collection_toast_retry: () => 'Retry'
}));

let activeService: ReturnType<
  typeof import('$lib/stores/WishlistService.svelte').createWishlistService
>;

vi.mock('$lib/stores/WishlistService.svelte', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/stores/WishlistService.svelte')>();
  return {
    ...actual,
    get wishlistService() {
      return activeService;
    }
  };
});

// Now import after mocks
import AddWishlistItemModal from '$lib/components/AddWishlistItemModal.svelte';
import {
  createWishlistService,
  type WishlistPreviewLite
} from '$lib/stores/WishlistService.svelte';
import { invoke, type InvokeArgs, type InvokeOptions } from '@tauri-apps/api/core';

const mockInvoke = vi.mocked(invoke);
type InvokeArgType = InvokeArgs | undefined;
type InvokeOptionType = InvokeOptions | undefined;
type Handler = (args?: InvokeArgType) => unknown;

activeService = createWishlistService();

const wishlistFixtures: WishlistPreviewLite[] = [
  {
    id: 'wishlist-1',
    name: 'My Wishlist',
    notes: null,
    is_default: true,
    count: 0,
    updated_at: '2024-01-01T00:00:00Z',
    total_value: {}
  },
  {
    id: 'wishlist-2',
    name: 'Future Purchases',
    notes: null,
    is_default: false,
    count: 0,
    updated_at: '2024-01-02T00:00:00Z',
    total_value: {}
  }
];

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

describe('AddWishlistItemModal', () => {
  beforeEach(async () => {
    activeService = createWishlistService();
    vi.clearAllMocks();
    tauriMock.reset();

    tauriMock.mockCommand('get_wishlists', wishlistFixtures);
    await activeService.fetchWishlists();
  });

  it('should render modal with title and form fields', () => {
    render(AddWishlistItemModal);

    expect(screen.getByText('Add to Wishlist')).toBeInTheDocument();
    expect(screen.getByLabelText('Model ID')).toBeInTheDocument();
    expect(screen.getByLabelText('Notes')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /save/i })).toBeInTheDocument();
  });

  it('should display available wishlists in dropdown', async () => {
    render(AddWishlistItemModal);

    const select = (await screen.findByLabelText('Select Wishlist')) as HTMLSelectElement;

    await waitFor(() => {
      const options = Array.from(select.options).map((opt) => opt.textContent?.trim());
      expect(options).toContain('My Wishlist (default)');
      expect(options).toContain('Future Purchases');
    });
  });

  it('should show validation error when model ID is missing', async () => {
    const user = userEvent.setup();
    render(AddWishlistItemModal);

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Model ID is required')).toBeInTheDocument();
    });
  });

  it('should add item to existing wishlist', async () => {
    const user = userEvent.setup();

    const mockAddedItem = {
      id: 'item-1',
      railway_model_id: '79894',
      priority: 'NORMAL',
      status: 'WANTED',
      added_date: '2024-01-01',
      removed_date: null,
      notes: null,
      desired_price: null,
      purchased_price: null
    };

    tauriMock.mockCommand('add_to_wishlist', mockAddedItem);

    const onSaved = vi.fn();
    const onClose = vi.fn();
    render(AddWishlistItemModal, {
      events: {
        saved: onSaved,
        close: onClose
      }
    });

    // Select wishlist
    const select = (await screen.findByLabelText('Select Wishlist')) as HTMLSelectElement;
    await user.selectOptions(select, 'wishlist-1');

    // Enter model ID
    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, '79894');

    // Submit
    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    // Wait for async operations
    await waitFor(() => {
      expect(onSaved).toHaveBeenCalled();
      expect(onClose).toHaveBeenCalled();
    });
  });

  it('should create new wishlist and add item', async () => {
    const user = userEvent.setup();

    const mockCreatedWishlist: WishlistPreviewLite = {
      id: 'new-wishlist',
      name: 'New List',
      notes: null,
      is_default: false,
      count: 0,
      updated_at: '2024-01-03T00:00:00Z',
      total_value: {}
    };

    const mockAddedItem = {
      id: 'item-1',
      wishlist_id: 'new-wishlist',
      railway_model_id: '79894',
      notes: null,
      purchase_status: 'not_purchased',
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z'
    };

    tauriMock.mockCommand('create_wishlist', mockCreatedWishlist);
    tauriMock.mockCommand('add_to_wishlist', mockAddedItem);

    render(AddWishlistItemModal);

    // Enter new list name
    const newListInput = screen.getByPlaceholderText('Or create new list');
    await user.type(newListInput, 'New List');

    // Enter model ID
    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, '79894');

    // Submit
    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(mockInvoke).toHaveBeenCalledWith('create_wishlist', expect.any(Object));
      expect(mockInvoke).toHaveBeenCalledWith(
        'add_to_wishlist',
        expect.objectContaining({ input: expect.objectContaining({ railway_model_id: '79894' }) })
      );
    });
  });

  it('should show error when creating wishlist fails', async () => {
    const user = userEvent.setup();
    const error = { ValidationError: { name: 'Name already exists' } };

    tauriMock.mockCommandError('create_wishlist', error);

    render(AddWishlistItemModal);

    const newListInput = screen.getByPlaceholderText('Or create new list');
    await user.type(newListInput, 'Duplicate Name');

    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, '79894');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Failed to create wishlist')).toBeInTheDocument();
    });
  });

  it('should show error when adding item fails', async () => {
    const user = userEvent.setup();
    const error = { NotFound: 'Railway model not found' };

    tauriMock.mockCommandError('add_to_wishlist', error);

    render(AddWishlistItemModal);

    const select = (await screen.findByLabelText('Select Wishlist')) as HTMLSelectElement;
    await user.selectOptions(select, 'wishlist-1');

    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, 'invalid-id');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    await waitFor(() => {
      expect(screen.getByText('Failed to add item')).toBeInTheDocument();
    });
  });

  it('should disable buttons while submitting', async () => {
    const user = userEvent.setup();

    // Mock with delay to observe loading state
    tauriMock.mockCommandWithDelay('add_to_wishlist', 100, {
      id: 'item-1',
      railway_model_id: '79894',
      priority: 'NORMAL',
      status: 'WANTED',
      added_date: '2024-01-01',
      removed_date: null,
      notes: null,
      desired_price: null,
      purchased_price: null
    });

    render(AddWishlistItemModal);

    const select = (await screen.findByLabelText('Select Wishlist')) as HTMLSelectElement;
    await user.selectOptions(select, 'wishlist-1');

    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, '79894');

    const saveButton = screen.getByRole('button', { name: /save/i });
    await user.click(saveButton);

    // Buttons should be disabled during submission
    await waitFor(() => {
      expect(saveButton).toBeDisabled();
    });
  });

  it('should close modal and reset form on close button click', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    render(AddWishlistItemModal, {
      events: { close: onClose }
    });

    const modelIdInput = screen.getByLabelText('Model ID');
    await user.type(modelIdInput, 'test-value');

    const closeButton = screen.getByRole('button', { name: /close/i });
    await user.click(closeButton);

    expect(onClose).toHaveBeenCalled();
  });
});
