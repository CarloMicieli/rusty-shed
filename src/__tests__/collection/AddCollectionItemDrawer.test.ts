import { describe, it, expect, vi, beforeEach } from 'vitest';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { get, writable, type Writable } from 'svelte/store';

type FormState = {
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: string | null;
  scale: string | null;
  powerMethod: string | null;
  epoch: string | null;
  rollingStocks: Array<{
    uid: string;
    railwayCompanyId: string | null;
    seriesCode: string;
    category: string | null;
    roadNumber: string;
    subcategory: string | null;
  }>;
  purchase: {
    sellerId: string | null;
    priceAmount: number | null;
    priceCurrency: string;
    purchaseCondition: string | null;
    modelCondition: string | null;
    boxCondition: string | null;
    notes: string;
    purchaseDate: string;
    purchaseType: 'STANDARD' | 'PREORDER';
    depositAmount: number | null;
    depositCurrency: string | null;
    preorderTotalAmount: number | null;
    preorderTotalCurrency: string | null;
    expectedDate: string | null;
  };
};

const mockCommands = vi.hoisted(() => ({
  getManufacturers: vi.fn().mockResolvedValue([{ id: 'mfg-1', name: 'Roco' }]),
  getRailwayCompanies: vi.fn().mockResolvedValue([{ id: 'rc-1', name: 'FS' }]),
  getSellers: vi.fn().mockResolvedValue([{ id: 'seller-1', name: 'Model Shop' }])
}));

const mockCollectionState = vi.hoisted(() => ({
  addRailwayModel: vi.fn().mockResolvedValue(true)
}));

const mockToaster = vi.hoisted(() => ({
  success: vi.fn(),
  error: vi.fn(),
  info: vi.fn(),
  warning: vi.fn(),
  loading: vi.fn(),
  promise: vi.fn(),
  custom: vi.fn(),
  message: vi.fn(),
  dismiss: vi.fn()
}));

const mockSuperFormState = vi.hoisted(() => {
  let latestFormStore: Writable<FormState> | null = null;

  return {
    createStore(initial: FormState) {
      latestFormStore = writable(structuredClone(initial));
      return latestFormStore;
    },
    resetStore(next: FormState) {
      if (!latestFormStore) return;
      latestFormStore.set(structuredClone(next));
    },
    snapshot() {
      if (!latestFormStore) {
        throw new Error('superForm store not initialized');
      }
      return get(latestFormStore);
    },
    reset() {
      latestFormStore = null;
    }
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

vi.mock('sveltekit-superforms/adapters', () => ({
  zod4: vi.fn((schema: unknown) => schema)
}));

vi.mock('sveltekit-superforms', () => ({
  superForm: vi.fn(
    (
      initial: FormState,
      options: {
        onUpdate?: (ctx: {
          form: { valid: boolean; data: FormState; errors: object };
        }) => Promise<void> | void;
      }
    ) => {
      const form = mockSuperFormState.createStore(initial);
      const errors = writable<Record<string, unknown>>({});
      const tainted = writable(false);

      const enhance = (node: HTMLFormElement) => {
        const onSubmit = async (event: Event) => {
          event.preventDefault();
          await options.onUpdate?.({
            form: { valid: true, data: get(form), errors: {} }
          });
        };

        node.addEventListener('submit', onSubmit);
        return {
          destroy() {
            node.removeEventListener('submit', onSubmit);
          }
        };
      };

      return {
        form,
        errors,
        tainted,
        enhance,
        reset: ({ data }: { data: FormState }) => {
          mockSuperFormState.resetStore(data);
        },
        isTainted: () => false
      };
    }
  )
}));

vi.mock('$lib/bindings', () => ({
  commands: mockCommands
}));

vi.mock('$lib/features/collection/CollectionState.svelte', () => ({
  collectionState: mockCollectionState
}));

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: { settings: { currency: 'EUR' } }
}));

vi.mock('$lib/toaster', () => ({
  toaster: mockToaster
}));

vi.mock('$lib/features/collection/components/ModelSearchSection.svelte', async () => {
  const mod = await import('./ModelSearchSection.mock.svelte');
  return { default: mod.default };
});

vi.mock('$lib/features/quick-add/QuickAddEntityForm.svelte', async () => {
  const mod = await import('../stubs/QuickAddEntityFormStub.svelte');
  return { default: mod.default };
});

import AddCollectionItemDrawer from '$lib/features/collection/components/AddCollectionItemDrawer.svelte';
import { commands } from '$lib/bindings';
import { collectionState } from '$lib/features/collection/CollectionState.svelte';

describe('AddCollectionItemDrawer', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockSuperFormState.reset();
  });

  it('loads reference data when drawer opens', async () => {
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    const { rerender } = render(AddCollectionItemDrawer, {
      props: { open: false, onClose, onSuccess }
    });

    expect(commands.getManufacturers).not.toHaveBeenCalled();
    expect(commands.getRailwayCompanies).not.toHaveBeenCalled();
    expect(commands.getSellers).not.toHaveBeenCalled();

    await rerender({ open: true, onClose, onSuccess });

    await waitFor(() => {
      expect(commands.getManufacturers).toHaveBeenCalledOnce();
      expect(commands.getRailwayCompanies).toHaveBeenCalledOnce();
      expect(commands.getSellers).toHaveBeenCalledOnce();
      expect(screen.getByTestId('loading-state')).toHaveTextContent('ready');
    });
  });

  it('adds and removes rolling stock entries through callbacks', async () => {
    const user = userEvent.setup();
    render(AddCollectionItemDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess: vi.fn()
      }
    });

    await waitFor(() => {
      expect(screen.getByTestId('rolling-stock-count')).toHaveTextContent('0');
    });

    await user.click(screen.getByRole('button', { name: 'add-rolling-stock' }));
    await waitFor(() => {
      expect(screen.getByTestId('rolling-stock-count')).toHaveTextContent('1');
    });

    await user.click(screen.getByRole('button', { name: 'remove-rolling-stock' }));
    await waitFor(() => {
      expect(screen.getByTestId('rolling-stock-count')).toHaveTextContent('0');
    });
  });

  it('maps form data and submits successfully', async () => {
    const user = userEvent.setup();
    const onSuccess = vi.fn();

    render(AddCollectionItemDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess
      }
    });

    await user.click(screen.getByRole('button', { name: 'prepare-valid-form' }));
    await user.click(screen.getByRole('button', { name: 'add_model_submit' }));

    await waitFor(() => {
      expect(collectionState.addRailwayModel).toHaveBeenCalledOnce();
      expect(onSuccess).toHaveBeenCalledOnce();
    });

    expect(collectionState.addRailwayModel).toHaveBeenCalledWith(
      expect.objectContaining({
        railwayModel: expect.objectContaining({
          manufacturerId: 'mfg-1',
          productCode: 'HR2906',
          description: 'Electric Locomotive',
          category: 'LOCOMOTIVE',
          scale: 'H0',
          epoch: 'IV',
          powerMethod: 'DC',
          rollingStocks: [
            {
              railwayCompanyId: 'rc-1',
              seriesCode: 'E.656',
              roadNumber: '656 001',
              subcategory: 'ELECTRIC_LOCOMOTIVE',
              category: 'LOCOMOTIVE'
            }
          ]
        }),
        priceAmount: 25999,
        priceCurrency: 'EUR',
        sellerId: 'seller-1',
        purchaseType: 'PREORDER',
        depositAmount: 5000,
        depositCurrency: 'EUR',
        preorderTotalAmount: 25999,
        preorderTotalCurrency: 'EUR',
        expectedDate: '2026-12-01'
      })
    );
  });

  it('keeps drawer open and does not call onSuccess when submit fails', async () => {
    const user = userEvent.setup();
    const onSuccess = vi.fn();
    vi.mocked(collectionState.addRailwayModel).mockRejectedValueOnce(new Error('save failed'));

    render(AddCollectionItemDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess
      }
    });

    await user.click(screen.getByRole('button', { name: 'prepare-valid-form' }));
    await user.click(screen.getByRole('button', { name: 'add_model_submit' }));

    await waitFor(() => {
      expect(collectionState.addRailwayModel).toHaveBeenCalledOnce();
      expect(onSuccess).not.toHaveBeenCalled();
    });
  });

  it('quick-add manufacturer updates submit payload manufacturerId', async () => {
    const user = userEvent.setup();
    render(AddCollectionItemDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess: vi.fn()
      }
    });

    await user.click(screen.getByRole('button', { name: 'open-quick-add-manufacturer' }));
    await user.click(screen.getByRole('button', { name: 'quick-form-success' }));

    await user.click(screen.getByRole('button', { name: 'prepare-valid-form' }));
    await user.click(screen.getByRole('button', { name: 'add_model_submit' }));

    await waitFor(() => {
      expect(collectionState.addRailwayModel).toHaveBeenCalled();
    });

    const lastCallArgs = vi.mocked(collectionState.addRailwayModel).mock.calls.at(-1)?.[0];
    expect(lastCallArgs).toEqual(
      expect.objectContaining({
        railwayModel: expect.objectContaining({
          manufacturerId: 'trn:manufacturer:new-maker'
        })
      })
    );
    expect(mockToaster.success).toHaveBeenCalledWith('quick_add_manufacturer_success');
  });

  it('quick-add seller updates submit payload sellerId', async () => {
    const user = userEvent.setup();
    render(AddCollectionItemDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess: vi.fn()
      }
    });

    await user.click(screen.getByRole('button', { name: 'open-quick-add-seller' }));
    await user.click(screen.getByRole('button', { name: 'quick-form-success' }));

    await user.click(screen.getByRole('button', { name: 'prepare-valid-form' }));
    await user.click(screen.getByRole('button', { name: 'add_model_submit' }));

    await waitFor(() => {
      expect(collectionState.addRailwayModel).toHaveBeenCalled();
    });

    const lastCallArgs = vi.mocked(collectionState.addRailwayModel).mock.calls.at(-1)?.[0];
    expect(lastCallArgs).toEqual(
      expect.objectContaining({
        sellerId: 'trn:seller:new-party'
      })
    );
    expect(mockToaster.success).toHaveBeenCalledWith('quick_add_seller_success');
  });
});
