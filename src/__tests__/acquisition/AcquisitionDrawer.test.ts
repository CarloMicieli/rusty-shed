import { cleanup, render, screen, waitFor } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { get, writable, type Writable } from 'svelte/store';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import type { AcquisitionFormState } from '$lib/features/acquisition/types';

const commandMocks = vi.hoisted(() => ({
  getManufacturers: vi.fn().mockResolvedValue([{ id: 'mfg-1', name: 'Roco' }]),
  getSellers: vi.fn().mockResolvedValue([{ id: 'seller-1', name: 'Model Shop' }]),
  recordAcquisition: vi.fn().mockResolvedValue(undefined)
}));

const toasterMocks = vi.hoisted(() => ({
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

const superFormHarness = vi.hoisted(() => {
  let formStore: Writable<AcquisitionFormState> | null = null;
  let submitValid = true;

  return {
    reset() {
      formStore = null;
      submitValid = true;
    },
    setSubmitValid(next: boolean) {
      submitValid = next;
    },
    snapshot(): AcquisitionFormState {
      if (!formStore) {
        throw new Error('form store not initialized');
      }
      return get(formStore);
    },
    createFormStore(initial: AcquisitionFormState) {
      formStore = writable(structuredClone(initial));
      return formStore;
    },
    resetStore(next: AcquisitionFormState) {
      if (!formStore) return;
      formStore.set(structuredClone(next));
    },
    isValid() {
      return submitValid;
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

vi.mock('$lib/bindings', () => ({
  commands: commandMocks
}));

vi.mock('$lib/toaster', () => ({
  toaster: toasterMocks
}));

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    settings: {
      currency: 'EUR',
      favouriteScale: 'H0',
      powerMethod: 'DC'
    }
  }
}));

vi.mock('sveltekit-superforms/adapters', () => ({
  zod4: vi.fn((schema: unknown) => schema)
}));

vi.mock('sveltekit-superforms', () => ({
  superForm: vi.fn(
    (
      initial: AcquisitionFormState,
      options: {
        onUpdate?: (ctx: {
          form: { valid: boolean; data: AcquisitionFormState; errors: Record<string, unknown> };
        }) => Promise<void> | void;
      }
    ) => {
      const form = superFormHarness.createFormStore(initial);
      const errors = writable<Record<string, unknown>>({});
      const tainted = writable<Record<string, unknown>>({});

      const enhance = (node: HTMLFormElement) => {
        const onSubmit = async (event: Event) => {
          event.preventDefault();
          await options.onUpdate?.({
            form: {
              valid: superFormHarness.isValid(),
              data: get(form),
              errors: {}
            }
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
        reset: ({ data }: { data: AcquisitionFormState }) => superFormHarness.resetStore(data),
        isTainted: () => false
      };
    }
  )
}));

vi.mock('$lib/components', async () => {
  const buttonStub = await import('./stubs/ButtonStub.svelte');
  return {
    Button: buttonStub.default
  };
});

vi.mock('$lib/components/drawer', async () => {
  const [shellStub, headerStub, footerStub, quickAddShellStub] = await Promise.all([
    import('./stubs/DrawerShellStub.svelte'),
    import('./stubs/DrawerHeaderStub.svelte'),
    import('./stubs/DrawerFooterStub.svelte'),
    import('./stubs/QuickAddShellStub.svelte')
  ]);

  return {
    DrawerShell: shellStub.default,
    DrawerHeader: headerStub.default,
    DrawerFooter: footerStub.default,
    QuickAddShell: quickAddShellStub.default
  };
});

vi.mock('$lib/features/acquisition/components/AcquisitionBatchFields.svelte', async () => {
  const stub = await import('./stubs/AcquisitionBatchFieldsStub.svelte');
  return { default: stub.default };
});

vi.mock('$lib/features/acquisition/components/AcquisitionItemCard.svelte', async () => {
  const stub = await import('./stubs/AcquisitionItemCardStub.svelte');
  return { default: stub.default };
});

vi.mock('$lib/features/quick-add/QuickAddEntityForm.svelte', async () => {
  const stub = await import('../stubs/QuickAddEntityFormStub.svelte');
  return { default: stub.default };
});

import AcquisitionDrawer from '$lib/features/acquisition/AcquisitionDrawer.svelte';
import { commands } from '$lib/bindings';

describe('AcquisitionDrawer', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    superFormHarness.reset();
  });

  it('loads reference data when opening', async () => {
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    const { rerender } = render(AcquisitionDrawer, {
      props: { open: false, onClose, onSuccess }
    });

    expect(commands.getManufacturers).not.toHaveBeenCalled();
    expect(commands.getSellers).not.toHaveBeenCalled();

    await rerender({ open: true, onClose, onSuccess });

    await waitFor(() => {
      expect(commands.getManufacturers).toHaveBeenCalledOnce();
      expect(commands.getSellers).toHaveBeenCalledOnce();
    });
  });

  it('adds, duplicates, and removes items', async () => {
    const user = userEvent.setup();

    render(AcquisitionDrawer, {
      props: { open: true, onClose: vi.fn(), onSuccess: vi.fn() }
    });

    await waitFor(() => {
      expect(screen.getAllByTestId('acquisition-item-card')).toHaveLength(1);
    });

    await user.click(screen.getByRole('button', { name: 'acquisition_add_item_button' }));
    await waitFor(() => {
      expect(screen.getAllByTestId('acquisition-item-card')).toHaveLength(2);
    });

    const duplicateButtons = screen.getAllByRole('button', { name: /duplicate-/ });
    await user.click(duplicateButtons[0]);
    await waitFor(() => {
      expect(screen.getAllByTestId('acquisition-item-card')).toHaveLength(3);
    });

    const removeButtons = screen.getAllByRole('button', { name: /remove-/ });
    await user.click(removeButtons[0]);
    await waitFor(() => {
      expect(screen.getAllByTestId('acquisition-item-card')).toHaveLength(2);
    });
  });

  it('submits successfully and calls recordAcquisition + onSuccess + onClose', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    render(AcquisitionDrawer, {
      props: { open: true, onClose, onSuccess }
    });

    await user.click(screen.getByRole('button', { name: 'acquisition_finalize_button' }));

    await waitFor(() => {
      expect(commands.recordAcquisition).toHaveBeenCalledOnce();
      expect(onSuccess).toHaveBeenCalledOnce();
      expect(onClose).toHaveBeenCalledOnce();
      expect(toasterMocks.success).toHaveBeenCalledWith('acquisition_toast_success');
    });
  });

  it('shows error toast and does not call success callbacks when submit fails', async () => {
    const user = userEvent.setup();
    const onClose = vi.fn();
    const onSuccess = vi.fn();
    vi.mocked(commands.recordAcquisition).mockRejectedValueOnce(new Error('save failed'));

    render(AcquisitionDrawer, {
      props: { open: true, onClose, onSuccess }
    });

    await user.click(screen.getByRole('button', { name: 'acquisition_finalize_button' }));

    await waitFor(() => {
      expect(commands.recordAcquisition).toHaveBeenCalledOnce();
      expect(toasterMocks.error).toHaveBeenCalledWith('acquisition_error_finalize');
    });

    expect(onSuccess).not.toHaveBeenCalled();
    expect(onClose).not.toHaveBeenCalled();
  });

  it('opens manufacturer quick-add and applies created manufacturer to submit payload', async () => {
    const user = userEvent.setup();

    render(AcquisitionDrawer, {
      props: { open: true, onClose: vi.fn(), onSuccess: vi.fn() }
    });

    await waitFor(() => {
      expect(screen.getByTestId('acquisition-item-card')).toBeInTheDocument();
    });

    await user.click(screen.getByText(/^quick-add-/));
    expect(screen.getByLabelText('quick_add_drawer_title_manufacturer')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'quick-form-success' }));
    expect(toasterMocks.success).toHaveBeenCalledWith('quick_add_manufacturer_success');

    await user.click(screen.getByRole('button', { name: 'acquisition_finalize_button' }));

    await waitFor(() => {
      expect(commands.recordAcquisition).toHaveBeenCalledOnce();
    });

    expect(commands.recordAcquisition).toHaveBeenCalledWith(
      expect.objectContaining({
        items: expect.arrayContaining([
          expect.objectContaining({
            manufacturerId: 'trn:manufacturer:new-maker'
          })
        ])
      })
    );
  });

  it('opens seller quick-add and applies created seller to submit payload', async () => {
    const user = userEvent.setup();

    render(AcquisitionDrawer, {
      props: { open: true, onClose: vi.fn(), onSuccess: vi.fn() }
    });

    await user.click(screen.getByRole('button', { name: 'quick-add-seller' }));
    expect(screen.getByLabelText('quick_add_drawer_title_seller')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'quick-form-success' }));
    expect(toasterMocks.success).toHaveBeenCalledWith('quick_add_seller_success');

    await user.click(screen.getByRole('button', { name: 'acquisition_finalize_button' }));

    await waitFor(() => {
      expect(commands.recordAcquisition).toHaveBeenCalledOnce();
    });

    expect(commands.recordAcquisition).toHaveBeenCalledWith(
      expect.objectContaining({
        sellerId: 'trn:seller:new-party'
      })
    );
  });
});
