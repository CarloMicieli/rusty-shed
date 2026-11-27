import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

type Manufacturer = { id: string; name: string };
type WishlistPriority = 'LOW' | 'NORMAL' | 'HIGH';
type FormState = {
  wishlistId: string;
  newListName: string;
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: string | null;
  scale: string;
  powerMethod: string;
  epoch: string | null;
  priority: WishlistPriority;
  desiredPrice: number | null;
};

type MockStore<T> = {
  subscribe: (run: (value: T) => void) => () => void;
  set: (value: T) => void;
  get: () => T;
};

function createMockStore<T>(initialValue: T): MockStore<T> {
  let value = initialValue;
  const subscribers = new Set<(value: T) => void>();

  return {
    subscribe(run) {
      run(value);
      subscribers.add(run);
      return () => subscribers.delete(run);
    },
    set(nextValue) {
      value = nextValue;
      subscribers.forEach((run) => run(value));
    },
    get() {
      return value;
    }
  };
}

const DEFAULT_FORM_STATE: FormState = {
  wishlistId: '',
  newListName: '',
  manufacturerId: null,
  productCode: '',
  description: '',
  category: null,
  scale: 'HO',
  powerMethod: 'DC',
  epoch: null,
  priority: 'NORMAL',
  desiredPrice: null
};

const mockWishlistService = vi.hoisted(() => ({
  wishlists: [{ id: 'default-wishlist', name: 'Default', isDefault: true }],
  defaultWishlist: { id: 'default-wishlist', name: 'Default', isDefault: true },
  createWishlist: vi.fn().mockResolvedValue({ id: 'created-wishlist' }),
  addRailwayModelToWishlist: vi.fn().mockResolvedValue(true)
}));

const mockGetManufacturers = vi.hoisted(() => vi.fn());
const mockToasterSuccess = vi.hoisted(() => vi.fn());

const mockSuperForm = vi.hoisted(() => {
  const form = createMockStore<FormState>({
    wishlistId: '',
    newListName: '',
    manufacturerId: null,
    productCode: '',
    description: '',
    category: null,
    scale: 'HO',
    powerMethod: 'DC',
    epoch: null,
    priority: 'NORMAL',
    desiredPrice: null
  });
  const errors = createMockStore<Record<string, string[]>>({});
  const tainted = createMockStore<Record<string, boolean>>({});

  const reset = vi.fn((options?: { data?: FormState }) => {
    if (options?.data) {
      form.set({ ...options.data });
    }
    errors.set({});
    tainted.set({});
  });

  return {
    form,
    errors,
    tainted,
    reset,
    isTainted: vi.fn(() => false),
    validateForm: vi.fn().mockResolvedValue({ valid: true })
  };
});

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

vi.mock('$lib/bindings', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return {
    ...actual,
    commands: {
      getManufacturers: (...args: unknown[]) => mockGetManufacturers(...args)
    }
  };
});

vi.mock('sveltekit-superforms', () => ({
  superForm: () => ({
    form: mockSuperForm.form,
    errors: mockSuperForm.errors,
    tainted: mockSuperForm.tainted,
    reset: mockSuperForm.reset,
    isTainted: mockSuperForm.isTainted,
    validateForm: mockSuperForm.validateForm
  })
}));

vi.mock('sveltekit-superforms/adapters', () => ({
  zod4: () => undefined
}));

vi.mock('$lib/features/wishlists/WishlistState.svelte', () => ({
  getWishlistContext: () => mockWishlistService
}));

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    settings: {
      currency: 'EUR',
      favouriteScale: 'HO',
      powerMethod: 'DC'
    }
  }
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    success: (...args: unknown[]) => mockToasterSuccess(...args)
  }
}));

vi.mock('$lib/components/drawer/EpochPicker.svelte', () => ({
  default: function EpochPickerStub() {}
}));

import AddWishlistItemDrawer from '$lib/features/wishlists/AddWishlistItemDrawer.svelte';

function makeFormState(overrides: Partial<FormState> = {}): FormState {
  return {
    ...DEFAULT_FORM_STATE,
    wishlistId: 'default-wishlist',
    manufacturerId: 'man-1',
    productCode: 'PC-123',
    description: 'Freight Wagon',
    ...overrides
  };
}

describe('AddWishlistItemDrawer', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockGetManufacturers.mockResolvedValue({
      status: 'ok',
      data: [{ id: 'man-1', name: 'ACME Rail' }] satisfies Manufacturer[]
    });
    mockWishlistService.createWishlist.mockResolvedValue({ id: 'created-wishlist' });
    mockWishlistService.addRailwayModelToWishlist.mockResolvedValue(true);
    mockSuperForm.validateForm.mockResolvedValue({ valid: true });
    mockSuperForm.form.set({ ...DEFAULT_FORM_STATE });
    mockSuperForm.errors.set({});
    mockSuperForm.tainted.set({});
  });

  it('renders only when open and loads manufacturers on mount', async () => {
    mockGetManufacturers.mockResolvedValueOnce({
      status: 'ok',
      data: [{ id: 'man-1', name: 'ACME Rail' }]
    });

    const onClose = vi.fn();
    const onSaved = vi.fn();
    const { rerender } = render(AddWishlistItemDrawer, {
      props: { open: false, onClose, onSaved }
    });

    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

    await rerender({ open: true, onClose, onSaved });

    await waitFor(() => {
      expect(mockGetManufacturers).toHaveBeenCalledOnce();
    });
  });

  it('does not submit when validation fails', async () => {
    mockSuperForm.validateForm.mockResolvedValueOnce({ valid: false });
    const onClose = vi.fn();
    const onSaved = vi.fn();
    const user = userEvent.setup();

    render(AddWishlistItemDrawer, {
      props: { open: true, onClose, onSaved }
    });

    await user.click(screen.getByRole('button', { name: 'wishlist_modal_save' }));

    expect(mockSuperForm.validateForm).toHaveBeenCalledOnce();
    expect(mockWishlistService.addRailwayModelToWishlist).not.toHaveBeenCalled();
    expect(onSaved).not.toHaveBeenCalled();
    expect(onClose).not.toHaveBeenCalled();
  });

  it('submits successfully and calls onSaved + onClose', async () => {
    const onClose = vi.fn();
    const onSaved = vi.fn();
    const user = userEvent.setup();

    render(AddWishlistItemDrawer, {
      props: {
        open: true,
        onClose,
        onSaved
      }
    });

    mockSuperForm.form.set(
      makeFormState({
        wishlistId: 'default-wishlist',
        desiredPrice: 199.99
      })
    );

    await user.click(screen.getByRole('button', { name: 'wishlist_modal_save' }));

    await waitFor(() => {
      expect(mockWishlistService.addRailwayModelToWishlist).toHaveBeenCalledOnce();
    });

    expect(mockWishlistService.addRailwayModelToWishlist).toHaveBeenCalledWith(
      expect.objectContaining({
        wishlistId: 'default-wishlist',
        railwayModel: expect.objectContaining({
          manufacturerId: 'man-1',
          productCode: 'PC-123',
          description: 'Freight Wagon'
        }),
        desiredPriceAmount: 199.99,
        desiredPriceCurrency: 'EUR'
      })
    );
    expect(onSaved).toHaveBeenCalledOnce();
    expect(onClose).toHaveBeenCalledOnce();
  });

  it('shows an async error when addRailwayModelToWishlist fails', async () => {
    mockWishlistService.addRailwayModelToWishlist.mockResolvedValueOnce(false);
    const onClose = vi.fn();
    const onSaved = vi.fn();
    const user = userEvent.setup();

    render(AddWishlistItemDrawer, {
      props: { open: true, onClose, onSaved }
    });

    mockSuperForm.form.set(makeFormState());

    await user.click(screen.getByRole('button', { name: 'wishlist_modal_save' }));

    await waitFor(() => {
      expect(screen.getByText('wishlist_modal_add_failed')).toBeInTheDocument();
    });

    expect(onSaved).not.toHaveBeenCalled();
    expect(onClose).not.toHaveBeenCalled();
  });
});
