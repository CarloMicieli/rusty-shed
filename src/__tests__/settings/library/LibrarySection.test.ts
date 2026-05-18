import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const {
  mockGetManufacturers,
  mockGetSellers,
  mockGetBuyers,
  mockUpdateManufacturer,
  mockUpdateSeller,
  mockUpdateBuyer,
  mockToasterSuccess,
  mockToasterSignal
} = vi.hoisted(() => ({
  mockGetManufacturers: vi.fn(),
  mockGetSellers: vi.fn(),
  mockGetBuyers: vi.fn(),
  mockUpdateManufacturer: vi.fn(),
  mockUpdateSeller: vi.fn(),
  mockUpdateBuyer: vi.fn(),
  mockToasterSuccess: vi.fn(),
  mockToasterSignal: vi.fn()
}));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [
      k,
      typeof v === 'function'
        ? (params?: { name?: string }) =>
            params?.name ? `${k}:${params.name}` : k
        : v
    ])
  );
});

vi.mock('$lib/toaster', () => ({
  toaster: {
    success: mockToasterSuccess,
    signal: mockToasterSignal
  }
}));

vi.mock('$lib/services/entityLibrary', () => ({
  getManufacturers: (...args: unknown[]) => mockGetManufacturers(...args),
  getSellers: (...args: unknown[]) => mockGetSellers(...args),
  getBuyers: (...args: unknown[]) => mockGetBuyers(...args)
}));

vi.mock('$lib/bindings', () => ({
  commands: {
    updateManufacturer: (...args: unknown[]) => mockUpdateManufacturer(...args),
    updateSeller: (...args: unknown[]) => mockUpdateSeller(...args),
    updateBuyer: (...args: unknown[]) => mockUpdateBuyer(...args)
  }
}));

vi.mock('$lib/features/quick-add/QuickAddEntityForm.svelte', async () => {
  const module = await import('../../stubs/QuickAddEntityFormStub.svelte');
  return { default: module.default };
});

import LibrarySection from '$lib/features/settings/components/library/LibrarySection.svelte';
import { settingsState } from '$lib/features/settings/SettingsState.svelte';

const manufacturerRow = {
  id: 'trn:manufacturer:acme',
  name: 'ACME',
  countryCode: 'IT',
  usageCount: 0,
  isSystemSeeded: false
};

const sellerRow = {
  id: 'trn:seller:model-shop',
  name: 'Model Shop',
  countryCode: 'DE',
  usageCount: 0,
  isSystemSeeded: false
};

describe('LibrarySection', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    settingsState.setLibraryTab('manufacturers');
    settingsState.setLibrarySearchQuery('');
    settingsState.libraryError = null;
    settingsState.setLibraryRows({ manufacturers: [], sellers: [], buyers: [] });

    mockGetManufacturers.mockResolvedValue({ status: 'ok', data: [manufacturerRow] });
    mockGetSellers.mockResolvedValue({ status: 'ok', data: [sellerRow] });
    mockGetBuyers.mockResolvedValue({ status: 'ok', data: [sellerRow] });
    mockUpdateManufacturer.mockResolvedValue({
      status: 'ok',
      data: {
        id: manufacturerRow.id,
        name: 'acme updated',
        registeredCompanyName: null,
        countryCode: 'IT',
        status: 'ACTIVE',
        websiteUrl: null,
        isSystemSeeded: false,
        usageCount: 0
      }
    });
    mockUpdateSeller.mockResolvedValue({
      status: 'ok',
      data: {
        id: sellerRow.id,
        name: 'model shop updated',
        sellerType: 'SHOP',
        email: null,
        phone: null,
        websiteUrl: null,
        address: null
      }
    });
    mockUpdateBuyer.mockResolvedValue({
      status: 'ok',
      data: {
        id: sellerRow.id,
        name: 'model shop updated',
        sellerType: 'SHOP',
        email: null,
        phone: null,
        websiteUrl: null,
        address: null
      }
    });
  });

  it('loads library rows on mount and renders initial tab', async () => {
    render(LibrarySection);

    await waitFor(() => {
      expect(screen.getAllByText('ACME').length).toBeGreaterThan(0);
    });

    expect(mockGetManufacturers).toHaveBeenCalledOnce();
    expect(mockGetSellers).toHaveBeenCalledOnce();
    expect(mockGetBuyers).toHaveBeenCalledOnce();
  });

  it('adds a new manufacturer via quick-add success and prepends it to the list', async () => {
    render(LibrarySection);

    await waitFor(() => {
      expect(screen.getAllByText('ACME').length).toBeGreaterThan(0);
    });

    await fireEvent.click(screen.getByRole('button', { name: 'settings_library_add_new' }));
    await fireEvent.click(screen.getByRole('button', { name: 'quick-form-success' }));

    expect((await screen.findAllByText('New Maker')).length).toBeGreaterThan(0);
    expect(mockToasterSuccess).toHaveBeenCalled();
  });

  it('edits a manufacturer and calls updateManufacturer through onSubmit', async () => {
    render(LibrarySection);

    await waitFor(() => {
      expect(screen.getAllByText('ACME').length).toBeGreaterThan(0);
    });

    await fireEvent.click(
      screen.getAllByRole('button', {
        name: 'settings_library_edit_row:ACME'
      })[0]
    );

    expect(screen.getByText('settings_library_update_action')).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'quick-form-success' }));

    await waitFor(() => {
      expect(mockUpdateManufacturer).toHaveBeenCalledOnce();
    });

    expect((await screen.findAllByText('acme updated')).length).toBeGreaterThan(0);
  });

  it('propagates seller edit updates to buyers tab using canonical upsert', async () => {
    settingsState.setLibraryTab('sellers');
    render(LibrarySection);

    await waitFor(() => {
      expect(screen.getAllByText('Model Shop').length).toBeGreaterThan(0);
    });

    await fireEvent.click(
      screen.getAllByRole('button', {
        name: 'settings_library_edit_row:Model Shop'
      })[0]
    );

    await fireEvent.click(screen.getByRole('button', { name: 'quick-form-success' }));

    await waitFor(() => {
      expect(mockUpdateSeller).toHaveBeenCalledOnce();
      expect(screen.getAllByText('model shop updated').length).toBeGreaterThan(0);
    });

    await fireEvent.click(screen.getByRole('tab', { name: 'settings_library_tab_buyers' }));

    expect(screen.getAllByText('model shop updated').length).toBeGreaterThan(0);
  });
});
