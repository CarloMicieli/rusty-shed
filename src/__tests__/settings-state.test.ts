import { beforeEach, describe, expect, it, vi } from 'vitest';

const safeInvokeMock = vi.hoisted(() => vi.fn());
const logMocks = vi.hoisted(() => ({
  debug: vi.fn(),
  error: vi.fn()
}));

vi.mock('$lib/services', () => ({ safeInvoke: safeInvokeMock }));
vi.mock('$lib/tauri-logger', () => ({ log: logMocks }));

import { SettingsState } from '$lib/features/settings/SettingsState.svelte';
import type { UpdateSettingsInput, UserSettings_Serialize } from '$lib/bindings';

const baseSettings: UserSettings_Serialize = {
  currency: 'EUR',
  language: 'en',
  theme: 'steampunk-dark',
  measureUnit: 'Metric',
  favouriteScale: 'H0',
  powerMethod: 'DC',
  has_completed_onboarding: false
};

describe('SettingsState', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('loads settings and resets loading/error state', async () => {
    safeInvokeMock.mockResolvedValue({ ok: true, data: baseSettings });
    const state = new SettingsState();

    await state.load();

    expect(state.settings).toEqual(baseSettings);
    expect(state.loading).toBe(false);
    expect(state.error).toBeNull();
  });

  it('records backend load errors and throws', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: false,
      error: { kind: 'database', message: 'Cannot read settings' }
    });
    const state = new SettingsState();

    await expect(state.load()).rejects.toThrow('Cannot read settings');
    expect(state.error).toBe('Cannot read settings');
    expect(state.loading).toBe(false);
  });

  it('updates partial settings and replaces the loaded settings', async () => {
    const updated = { ...baseSettings, currency: 'USD' as const };
    safeInvokeMock.mockResolvedValue({ ok: true, data: updated });
    const state = new SettingsState();
    const input: UpdateSettingsInput = { currency: 'USD' };

    await state.update(input);

    expect(safeInvokeMock).toHaveBeenCalledWith('update_settings', { input });
    expect(state.settings.currency).toBe('USD');
    expect(state.loading).toBe(false);
  });

  it('records update failures and throws', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: false,
      error: { kind: 'validation', message: 'Invalid settings' }
    });
    const state = new SettingsState();

    await expect(state.update({ currency: 'JPY' })).rejects.toThrow('Invalid settings');
    expect(state.error).toBe('Invalid settings');
    expect(state.loading).toBe(false);
  });

  it('initializes settings and markOnboardingCompleted updates the settings flag', async () => {
    safeInvokeMock.mockResolvedValueOnce({ ok: true, data: baseSettings }).mockResolvedValueOnce({
      ok: true,
      data: { ...baseSettings, has_completed_onboarding: true }
    });
    const state = new SettingsState();

    await state.initialize();
    await state.markOnboardingCompleted();

    expect(state.settings.has_completed_onboarding).toBe(true);
    expect(safeInvokeMock).toHaveBeenCalledWith('update_settings', {
      input: { hasCompletedOnboarding: true }
    });
  });

  it('throws initialization errors without changing settings', async () => {
    safeInvokeMock.mockResolvedValue({
      ok: false,
      error: { kind: 'database', message: 'Initialization failed' }
    });
    const state = new SettingsState();
    const original = { ...state.settings };

    await expect(state.initialize()).rejects.toThrow('Initialization failed');
    expect(state.settings).toEqual(original);
  });

  it('updates library tab/query and replaces library rows', () => {
    const state = new SettingsState();
    state.setLibraryTab('sellers');
    state.setLibrarySearchQuery('Roco');
    state.setLibraryRows({
      manufacturers: [
        { id: 'mfr-1', name: 'Roco', countryCode: null, usageCount: 1, isSystemSeeded: false }
      ],
      sellers: [
        {
          id: 'seller-1',
          name: 'Model Shop',
          countryCode: null,
          usageCount: 1,
          isSystemSeeded: false
        }
      ],
      buyers: []
    });

    expect(state.libraryActiveTab).toBe('sellers');
    expect(state.librarySearchQuery).toBe('Roco');
    expect(state.libraryManufacturers).toHaveLength(1);
    expect(state.librarySellers[0].id).toBe('seller-1');
  });

  it('upserts and removes library entities without mutating unrelated rows', () => {
    const state = new SettingsState();
    state.setLibraryRows({
      manufacturers: [
        { id: 'mfr-1', name: 'Roco', countryCode: null, usageCount: 1, isSystemSeeded: false }
      ],
      sellers: [
        { id: 'party-1', name: 'Old Name', countryCode: null, usageCount: 1, isSystemSeeded: false }
      ],
      buyers: [
        { id: 'party-1', name: 'Old Name', countryCode: null, usageCount: 1, isSystemSeeded: false }
      ]
    });

    state.upsertLibraryManufacturer({
      id: 'mfr-1',
      name: 'Roco Updated',
      countryCode: null,
      usageCount: 1,
      isSystemSeeded: false
    });
    state.upsertCanonicalParty({
      id: 'party-1',
      name: 'New Name',
      countryCode: null,
      usageCount: 1,
      isSystemSeeded: false
    });
    state.removeLibraryManufacturer('missing');

    expect(state.libraryManufacturers[0].name).toBe('Roco Updated');
    expect(state.librarySellers[0].name).toBe('New Name');
    expect(state.libraryBuyers[0].name).toBe('New Name');

    state.mergeLibraryManufacturer('mfr-1');
    state.mergeCanonicalParty('party-1');
    expect(state.libraryManufacturers).toHaveLength(0);
    expect(state.librarySellers).toHaveLength(0);
    expect(state.libraryBuyers).toHaveLength(0);
  });
});
