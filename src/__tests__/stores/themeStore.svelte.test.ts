import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get } from 'svelte/store';

// Mock Tauri bindings
vi.mock('$lib/bindings', () => ({
  commands: {
    getSettings: vi.fn(),
    updateSettings: vi.fn()
  }
}));

import {
  themeStore,
  resolvedTheme,
  currentTheme,
  isThemeLoading
} from '$lib/stores/themeStore.svelte';
import { commands } from '$lib/bindings';

const mockGetSettings = vi.mocked(commands.getSettings);
const mockUpdateSettings = vi.mocked(commands.updateSettings);

// ─── tests ────────────────────────────────────────────────────────────────

describe('themeStore', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store to initial state
    themeStore.set({ current: 'system', resolved: 'dark', isLoading: true });
  });

  describe('initial state', () => {
    it('starts with system theme preference', () => {
      themeStore.set({ current: 'system', resolved: 'dark', isLoading: true });
      const state = get(themeStore);
      expect(state.current).toBe('system');
    });

    it('starts with dark resolved theme', () => {
      const state = get(themeStore);
      expect(state.resolved).toBe('dark');
    });

    it('starts with isLoading=true', () => {
      const state = get(themeStore);
      expect(state.isLoading).toBe(true);
    });
  });

  describe('derived stores', () => {
    it('resolvedTheme mirrors resolved from store', () => {
      themeStore.set({ current: 'steampunk-light', resolved: 'light', isLoading: false });
      expect(get(resolvedTheme)).toBe('light');
    });

    it('currentTheme mirrors current from store', () => {
      themeStore.set({ current: 'steampunk-dark', resolved: 'dark', isLoading: false });
      expect(get(currentTheme)).toBe('steampunk-dark');
    });

    it('isThemeLoading mirrors isLoading from store', () => {
      themeStore.set({ current: 'system', resolved: 'dark', isLoading: false });
      expect(get(isThemeLoading)).toBe(false);

      themeStore.set({ current: 'system', resolved: 'dark', isLoading: true });
      expect(get(isThemeLoading)).toBe(true);
    });
  });

  describe('initializeFromSettings', () => {
    it('sets theme from Tauri settings on success', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-light' }
      } as ReturnType<typeof commands.getSettings> extends Promise<infer T> ? T : never);

      await themeStore.initializeFromSettings();

      const state = get(themeStore);
      expect(state.current).toBe('steampunk-light');
      expect(state.resolved).toBe('light');
      expect(state.isLoading).toBe(false);
    });

    it('applies steampunk-dark theme correctly', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-dark' }
      } as never);

      await themeStore.initializeFromSettings();

      const state = get(themeStore);
      expect(state.current).toBe('steampunk-dark');
      expect(state.resolved).toBe('dark');
      expect(state.isLoading).toBe(false);
    });

    it('falls back to dark theme on error', async () => {
      mockGetSettings.mockRejectedValueOnce(new Error('Tauri not available'));

      await themeStore.initializeFromSettings();

      const state = get(themeStore);
      expect(state.resolved).toBe('dark');
      expect(state.isLoading).toBe(false);
    });

    it('falls back to dark when status is not ok', async () => {
      mockGetSettings.mockResolvedValueOnce({ status: 'error', error: 'E_SETTINGS' } as never);

      await themeStore.initializeFromSettings();

      const state = get(themeStore);
      expect(state.isLoading).toBe(false);
    });

    it('uses system preference when theme field is absent', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: {} // no theme field
      } as never);

      await themeStore.initializeFromSettings();

      const state = get(themeStore);
      expect(state.current).toBe('system');
      expect(state.isLoading).toBe(false);
    });

    it('applies theme to DOM (sets body data-theme)', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-light' }
      } as never);

      await themeStore.initializeFromSettings();

      expect(document.body.dataset.theme).toBe('steampunk-light');
    });
  });

  describe('setTheme', () => {
    it('updates store and persists via Tauri on success', async () => {
      mockGetSettings.mockResolvedValueOnce({ status: 'ok', data: {} } as never);
      mockUpdateSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-dark' }
      } as never);

      await themeStore.setTheme('steampunk-dark');

      const state = get(themeStore);
      expect(state.current).toBe('steampunk-dark');
      expect(state.resolved).toBe('dark');
    });

    it('silently swallows errors on failure', async () => {
      mockGetSettings.mockRejectedValueOnce(new Error('Tauri unavailable'));

      // Should not throw
      await expect(themeStore.setTheme('steampunk-light')).resolves.toBeUndefined();
    });
  });

  describe('getState', () => {
    it('returns current state synchronously', () => {
      themeStore.set({ current: 'steampunk-dark', resolved: 'dark', isLoading: false });
      const state = themeStore.getState();
      expect(state?.current).toBe('steampunk-dark');
    });
  });
});
