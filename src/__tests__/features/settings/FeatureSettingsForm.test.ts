import { beforeEach, describe, expect, it, vi } from 'vitest';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import FeatureSettingsForm from '$lib/features/settings/components/SettingsForm.svelte';
import * as m from '$lib/paraglide/messages';

const { mockUpdateSettings, mockSetLocale, mockSetTheme, mockLog } = vi.hoisted(() => ({
  mockUpdateSettings: vi.fn(),
  mockSetLocale: vi.fn(),
  mockSetTheme: vi.fn().mockResolvedValue(undefined),
  mockLog: {
    debug: vi.fn(),
    error: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

vi.mock('$lib/paraglide/runtime.js', () => ({
  setLocale: mockSetLocale
}));

vi.mock('$lib/tauri-logger', () => ({
  log: mockLog
}));

vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    settings: {
      currency: 'EUR',
      language: 'en',
      theme: 'steampunk-dark',
      measureUnit: 'Metric',
      favouriteScale: 'HO',
      powerMethod: 'DC',
      has_completed_onboarding: false
    },
    update: mockUpdateSettings
  }
}));

vi.mock('$lib/features/settings/SettingsState.svelte.ts', () => ({
  settingsState: {
    settings: {
      currency: 'EUR',
      language: 'en',
      theme: 'steampunk-dark',
      measureUnit: 'Metric',
      favouriteScale: 'HO',
      powerMethod: 'DC',
      has_completed_onboarding: false
    },
    update: mockUpdateSettings
  }
}));

vi.mock('$lib/stores/themeStore.svelte', () => ({
  themeState: {
    setTheme: mockSetTheme
  }
}));

vi.mock('$lib/stores/themeStore.svelte.ts', () => ({
  themeState: {
    setTheme: mockSetTheme
  }
}));

vi.mock('$lib/features/settings/components/CurrencySelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

vi.mock('$lib/features/settings/components/LanguageSelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

vi.mock('$lib/features/settings/components/MeasureUnitSelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

vi.mock('$lib/features/settings/components/ThemeSelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

vi.mock('$lib/features/settings/components/ScaleSelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

vi.mock('$lib/features/settings/components/PowerSystemSelector.svelte', async () => {
  const module = await import('../../stubs/feature-settings-selector-stub.svelte');
  return { default: module.default };
});

describe('Feature SettingsForm.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    mockUpdateSettings.mockResolvedValue(undefined);
    mockSetTheme.mockResolvedValue(undefined);
  });

  it('renders initial values in selectors', async () => {
    render(FeatureSettingsForm);

    await waitFor(() => {
      expect(screen.getByDisplayValue('EUR')).toBeInTheDocument();
      expect(screen.getByDisplayValue('en')).toBeInTheDocument();
      expect(screen.getByDisplayValue('Metric')).toBeInTheDocument();
      expect(screen.getByDisplayValue('steampunk-dark')).toBeInTheDocument();
      expect(screen.getByDisplayValue('HO')).toBeInTheDocument();
      expect(screen.getByDisplayValue('DC')).toBeInTheDocument();
      expect(mockSetLocale).toHaveBeenCalledWith('en');
    });
  });

  it('submits successfully and shows success feedback', async () => {
    render(FeatureSettingsForm);

    await fireEvent.input(screen.getByDisplayValue('EUR'), { target: { value: 'USD' } });
    await fireEvent.input(screen.getByDisplayValue('en'), { target: { value: 'it' } });
    await fireEvent.input(screen.getByDisplayValue('Metric'), { target: { value: 'Imperial' } });
    await fireEvent.input(screen.getByDisplayValue('steampunk-dark'), {
      target: { value: 'steampunk-light' }
    });
    await fireEvent.input(screen.getByDisplayValue('HO'), { target: { value: 'N' } });
    await fireEvent.input(screen.getByDisplayValue('DC'), { target: { value: 'AC' } });

    await fireEvent.submit(screen.getByRole('button', { name: m.save_button() }));

    await waitFor(() => {
      expect(mockUpdateSettings).toHaveBeenCalledWith({
        currency: 'USD',
        language: 'it',
        theme: 'steampunk-light',
        measureUnit: 'Imperial',
        favouriteScale: 'N',
        powerMethod: 'AC'
      });
      expect(mockSetTheme).toHaveBeenCalledWith('steampunk-light');
      expect(screen.getByRole('alert')).toHaveTextContent(m.settings_saved_toast());
    });
  });

  it('shows an error alert when submit fails', async () => {
    mockUpdateSettings.mockRejectedValueOnce(new Error('save failed'));

    render(FeatureSettingsForm);

    await fireEvent.submit(screen.getByRole('button', { name: m.save_button() }));

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent(m.settings_update_failed());
      expect(screen.getByRole('alert')).toHaveTextContent('Error: save failed');
      expect(mockSetTheme).not.toHaveBeenCalled();
    });
  });

  it('shows saving state while submit is in progress', async () => {
    mockUpdateSettings.mockReturnValueOnce(new Promise<void>(() => {}));

    render(FeatureSettingsForm);

    await fireEvent.submit(screen.getByRole('button', { name: m.save_button() }));

    await waitFor(() => {
      expect(screen.getByRole('button', { name: m.settings_saving_button() })).toBeDisabled();
    });

    expect(screen.queryByRole('button', { name: m.save_button() })).not.toBeInTheDocument();
  });
});
