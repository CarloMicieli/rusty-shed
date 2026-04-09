import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen, waitFor, cleanup } from '@testing-library/svelte';
import SettingsForm from '$lib/features/settings/components/SettingsForm.svelte';
import { settingsState } from '$lib/features/settings/SettingsState.svelte';
import * as m from '$lib/paraglide/messages';

// Mock settingsState
vi.mock('$lib/features/settings/SettingsState.svelte', () => ({
  settingsState: {
    settings: {
      currency: 'EUR',
      language: 'en',
      theme: 'steampunk-dark',
      measureUnit: 'Metric',
      favouriteScale: 'HO',
      powerMethod: 'DC'
    },
    update: vi.fn().mockResolvedValue(undefined)
  }
}));

vi.mock('$lib/paraglide/runtime.js', async (importOriginal) => {
  const actual = await importOriginal<typeof import('$lib/paraglide/runtime.js')>();
  return {
    ...actual,
    setLocale: vi.fn(),
    isServer: false,
    experimentalMiddlewareLocaleSplitting: false
  };
});

// Mock themeStore dynamic import
vi.mock('$lib/stores/themeStore.svelte', () => ({
  themeStore: {
    setTheme: vi.fn().mockResolvedValue(undefined)
  }
}));

// Mock the nested selectors if needed, or simply let them render if they are simple inputs.
// If they require Tauri invoke, we should mock Tauri. Since they likely just have logic or select tags, let them render.
// But some might call invoke() on mount? Let's assume they don't, or we mock Tauri if they do.
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue([])
}));

describe('SettingsForm.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders with initial values from settingsState', () => {
    render(SettingsForm);
    expect(screen.getByText(m.settings_heading())).toBeInTheDocument();
    expect(screen.getByText(m.settings_description())).toBeInTheDocument();
  });

  it('submits form with current values', async () => {
    render(SettingsForm);

    // Wait for potential effects to settle
    await new Promise((r) => setTimeout(r, 0));

    const submitButton = screen.getByRole('button', { name: new RegExp(m.save_button(), 'i') });
    expect(submitButton).toBeInTheDocument();

    // Submit form
    await fireEvent.click(submitButton);

    // Verify update was called
    expect(settingsState.update).toHaveBeenCalledWith({
      currency: 'EUR',
      language: 'en',
      theme: 'steampunk-dark',
      measureUnit: 'Metric',
      favouriteScale: 'HO',
      powerMethod: 'DC'
    });

    // Verify success message appears
    await waitFor(() => {
      expect(screen.getByText(m.settings_saved_toast())).toBeInTheDocument();
    });
  });

  it('displays error if save fails', async () => {
    const errorMessage = 'Network Error';
    vi.mocked(settingsState.update).mockRejectedValueOnce(new Error(errorMessage));

    render(SettingsForm);

    const submitButton = screen.getByRole('button', { name: new RegExp(m.save_button(), 'i') });
    await fireEvent.click(submitButton);

    await waitFor(() => {
      expect(screen.getByText(new RegExp(m.settings_update_failed()))).toBeInTheDocument();
    });
  });

  it('displays saving label when submitting', async () => {
    // Delay the update to check the 'saving' state
    let resolveUpdate!: (value?: void | PromiseLike<void>) => void;
    const updatePromise = new Promise<void>((r) => (resolveUpdate = r));
    vi.mocked(settingsState.update).mockImplementation(() => updatePromise as Promise<void>);

    render(SettingsForm);

    const submitButton = screen.getByRole('button', { name: new RegExp(m.save_button(), 'i') });
    await fireEvent.click(submitButton);

    // While saving, the button text changes
    await waitFor(() => {
      expect(screen.getByText(m.settings_saving_button())).toBeInTheDocument();
    });

    // Resolve the promise to finish saving
    resolveUpdate();
  });
});
