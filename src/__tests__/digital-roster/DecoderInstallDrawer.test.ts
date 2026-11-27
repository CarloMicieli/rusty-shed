import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

type DecoderInstallFormData = {
  selectedRollingStockId: string | null;
  selectedDecoderId: string | null;
  dccAddress: number | null;
  installationDate: string;
};

type DuplicateResult = {
  isDuplicate: boolean;
  existingId?: string;
};

type MockStore<T> = {
  subscribe: (run: (value: T) => void) => () => void;
  set: (value: T) => void;
  update: (updater: (value: T) => T) => void;
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
    update(updater) {
      this.set(updater(value));
    },
    get() {
      return value;
    }
  };
}

const mockCommands = vi.hoisted(() => ({
  getInstallableRollingStocks: vi.fn(),
  getDecoders: vi.fn(),
  getManufacturers: vi.fn()
}));

const mockController = vi.hoisted(() => ({
  state: {
    rollingStocks: [] as Array<{
      id: string;
      owned_rolling_stock_id: string;
      dcc_address: number;
      decoder: { id: string };
    }>
  },
  checkDuplicateAddress: vi.fn<(...args: [number, string | null]) => Promise<DuplicateResult>>(),
  installDecoder: vi.fn<(...args: [string, string, number]) => Promise<boolean>>(),
  replaceDecoder: vi.fn<(...args: [string, string]) => Promise<boolean>>(),
  changeDccAddress: vi.fn<(...args: [string, number]) => Promise<boolean>>()
}));

const mockSuperForm = vi.hoisted(() => {
  const form = createMockStore<DecoderInstallFormData>({
    selectedRollingStockId: null,
    selectedDecoderId: null,
    dccAddress: null,
    installationDate: '2026-11-27'
  });
  const errors = createMockStore<Record<string, string[]>>({});
  const tainted = createMockStore<Record<string, boolean>>({});

  const reset = vi.fn((options?: { data?: DecoderInstallFormData }) => {
    if (options?.data) form.set({ ...options.data });
    errors.set({});
    tainted.set({});
  });

  return {
    form,
    errors,
    tainted,
    reset,
    isTainted: vi.fn(() => false)
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

vi.mock('$lib/paraglide/messages', async (importOriginal) => {
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
      getInstallableRollingStocks: (...args: unknown[]) =>
        mockCommands.getInstallableRollingStocks(...args),
      getDecoders: (...args: unknown[]) => mockCommands.getDecoders(...args),
      getManufacturers: (...args: unknown[]) => mockCommands.getManufacturers(...args)
    }
  };
});

vi.mock('$lib/features/digital-roster/DigitalRosterState.svelte', () => ({
  getDigitalRosterContext: () => mockController
}));

vi.mock('$lib/components', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  const buttonStub = await import('../stubs/ButtonStub.svelte');
  return {
    ...actual,
    Button: buttonStub.default
  };
});

vi.mock('$lib/components/drawer', async () => {
  const [shell, header, section] = await Promise.all([
    import('../stubs/DrawerShellStub.svelte'),
    import('../stubs/DrawerHeaderStub.svelte'),
    import('../stubs/DigitalSectionStub.svelte')
  ]);
  return {
    DrawerShell: shell.default,
    DrawerHeader: header.default,
    DigitalSection: section.default
  };
});

vi.mock('$lib/features/digital-roster/components/DecoderRollingStockPicker.svelte', async () => ({
  default: (await import('../stubs/DecoderRollingStockPickerStub.svelte')).default
}));

vi.mock('$lib/features/digital-roster/components/DecoderPicker.svelte', async () => ({
  default: (await import('../stubs/DecoderPickerStub.svelte')).default
}));

vi.mock('$lib/features/digital-roster/components/DecoderInstallConfirmDialog.svelte', async () => ({
  default: (await import('../stubs/DecoderInstallConfirmDialogStub.svelte')).default
}));

vi.mock('sveltekit-superforms', () => ({
  superForm: (
    initialData: DecoderInstallFormData,
    options: {
      onUpdate?: (payload: { form: { valid: boolean } }) => Promise<void> | void;
    }
  ) => {
    mockSuperForm.form.set({ ...initialData });

    return {
      form: mockSuperForm.form,
      errors: mockSuperForm.errors,
      tainted: mockSuperForm.tainted,
      reset: mockSuperForm.reset,
      isTainted: mockSuperForm.isTainted,
      enhance: (node: HTMLFormElement) => {
        const onSubmit = async (event: Event) => {
          event.preventDefault();
          await options.onUpdate?.({ form: { valid: true } });
        };
        node.addEventListener('submit', onSubmit);
        return {
          destroy: () => node.removeEventListener('submit', onSubmit)
        };
      }
    };
  }
}));

vi.mock('sveltekit-superforms/adapters', () => ({
  zod4: () => undefined
}));

import DecoderInstallDrawer from '$lib/features/digital-roster/components/DecoderInstallDrawer.svelte';

const installableStocks = [
  {
    owned_rolling_stock_id: 'stock-no-decoder',
    has_decoder: false,
    dcc_interface: 'NEM652'
  },
  {
    owned_rolling_stock_id: 'stock-with-decoder',
    has_decoder: true,
    dcc_interface: 'NEM652'
  }
];

const decoders = [
  {
    id: 'decoder-old',
    decoderType: 'SOUND',
    decoderInterface: 'NEM652',
    manufacturerId: 'man-1',
    productCode: 'OLD'
  },
  {
    id: 'decoder-new',
    decoderType: 'SOUND',
    decoderInterface: 'NEM652',
    manufacturerId: 'man-1',
    productCode: 'NEW'
  },
  {
    id: 'decoder-function',
    decoderType: 'FUNCTION',
    decoderInterface: 'NEM652',
    manufacturerId: 'man-1',
    productCode: 'FN'
  }
];

const manufacturers = [{ id: 'man-1', name: 'ESU' }];

describe('DecoderInstallDrawer.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();

    mockCommands.getInstallableRollingStocks.mockResolvedValue(installableStocks);
    mockCommands.getDecoders.mockResolvedValue(decoders);
    mockCommands.getManufacturers.mockResolvedValue(manufacturers);

    mockController.state.rollingStocks = [
      {
        id: 'digital-1',
        owned_rolling_stock_id: 'stock-with-decoder',
        dcc_address: 88,
        decoder: { id: 'decoder-old' }
      }
    ];
    mockController.checkDuplicateAddress.mockResolvedValue({ isDuplicate: false });
    mockController.installDecoder.mockResolvedValue(true);
    mockController.replaceDecoder.mockResolvedValue(true);
    mockController.changeDccAddress.mockResolvedValue(true);
  });

  it('loads reference data when the drawer is opened', async () => {
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    const { rerender } = render(DecoderInstallDrawer, {
      props: {
        open: false,
        onClose,
        onSuccess
      }
    });

    expect(mockCommands.getInstallableRollingStocks).not.toHaveBeenCalled();

    await rerender({
      open: true,
      onClose,
      onSuccess
    });

    await waitFor(() => {
      expect(mockCommands.getInstallableRollingStocks).toHaveBeenCalledOnce();
      expect(mockCommands.getDecoders).toHaveBeenCalledOnce();
      expect(mockCommands.getManufacturers).toHaveBeenCalledOnce();
    });
  });

  it('installs a decoder for stock without an existing decoder', async () => {
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    render(DecoderInstallDrawer, {
      props: {
        open: true,
        onClose,
        onSuccess
      }
    });

    await screen.findByTestId('stock-stock-no-decoder');

    await fireEvent.click(screen.getByTestId('stock-stock-no-decoder'));
    await fireEvent.click(screen.getByTestId('decoder-decoder-new'));
    await fireEvent.input(screen.getByTestId('dcc-address-input'), { target: { value: '77' } });
    await fireEvent.click(screen.getByRole('button', { name: 'digital_roster_save' }));

    await waitFor(() => {
      expect(mockController.installDecoder).toHaveBeenCalledWith(
        'stock-no-decoder',
        'decoder-new',
        77
      );
      expect(mockController.replaceDecoder).not.toHaveBeenCalled();
      expect(onSuccess).toHaveBeenCalledOnce();
      expect(onClose).toHaveBeenCalledOnce();
    });
  });

  it('opens confirm dialog and replaces decoder when stock already has one', async () => {
    const onClose = vi.fn();
    const onSuccess = vi.fn();

    render(DecoderInstallDrawer, {
      props: {
        open: true,
        onClose,
        onSuccess
      }
    });

    await screen.findByTestId('stock-stock-with-decoder');

    await fireEvent.click(screen.getByTestId('stock-stock-with-decoder'));
    await fireEvent.click(screen.getByTestId('decoder-decoder-new'));
    await fireEvent.click(screen.getByRole('button', { name: 'digital_roster_save' }));

    expect(screen.getByTestId('decoder-install-confirm-dialog')).toBeInTheDocument();
    expect(mockController.replaceDecoder).not.toHaveBeenCalled();

    await fireEvent.click(screen.getByTestId('confirm-replace'));

    await waitFor(() => {
      expect(mockController.replaceDecoder).toHaveBeenCalledWith('digital-1', 'decoder-new');
      expect(mockController.installDecoder).not.toHaveBeenCalled();
      expect(onSuccess).toHaveBeenCalledOnce();
      expect(onClose).toHaveBeenCalledOnce();
    });
  });

  it('shows duplicate warning when controller reports duplicate address', async () => {
    mockController.checkDuplicateAddress
      .mockResolvedValueOnce({ isDuplicate: true, existingId: 'digital-2' })
      .mockResolvedValueOnce({ isDuplicate: false });

    render(DecoderInstallDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess: vi.fn()
      }
    });

    await screen.findByTestId('dcc-address-input');

    await fireEvent.input(screen.getByTestId('dcc-address-input'), { target: { value: '55' } });

    await waitFor(() => {
      expect(mockController.checkDuplicateAddress).toHaveBeenCalledWith(55, null);
      expect(screen.getByTestId('duplicate-warning')).toHaveTextContent(
        'digital_roster_duplicate_warning'
      );
    });

    await fireEvent.input(screen.getByTestId('dcc-address-input'), { target: { value: '56' } });

    await waitFor(() => {
      expect(screen.queryByTestId('duplicate-warning')).not.toBeInTheDocument();
    });
  });

  it('initializes form with preselected stock values', async () => {
    render(DecoderInstallDrawer, {
      props: {
        open: true,
        onClose: vi.fn(),
        onSuccess: vi.fn(),
        preselectedStockId: 'stock-with-decoder'
      }
    });

    await screen.findByTestId('selected-stock-id');

    await waitFor(() => {
      expect(screen.getByTestId('selected-stock-id')).toHaveTextContent('stock-with-decoder');
      expect(screen.getByTestId('selected-decoder-id')).toHaveTextContent('decoder-old');
      expect(screen.getByTestId('dcc-address-input')).toHaveValue(88);
    });
  });
});
