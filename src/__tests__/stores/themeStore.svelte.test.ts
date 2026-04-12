import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock Tauri bindings
vi.mock('$lib/bindings', () => ({
  commands: {
    getSettings: vi.fn(),
    updateSettings: vi.fn()
  }
}));

import { themeState } from '$lib/stores/themeStore.svelte';
import { commands } from '$lib/bindings';

const mockGetSettings = vi.mocked(commands.getSettings);
const mockUpdateSettings = vi.mocked(commands.updateSettings);

// ─── tests ────────────────────────────────────────────────────────────────

describe('themeState (runes-based)', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset store to initial state
    themeState.current = 'system';
    themeState.resolved = 'dark';
    themeState.isLoading = true;
  });

  describe('initial state', () => {
    it('starts with system theme preference', () => {
      expect(themeState.current).toBe('system');
    });

    it('starts with dark resolved theme', () => {
      expect(themeState.resolved).toBe('dark');
    });

    it('starts with isLoading=true', () => {
      expect(themeState.isLoading).toBe(true);
    });
  });

  describe('getState synchronous access', () => {
    it('returns current state object', () => {
      themeState.current = 'steampunk-light';
      themeState.resolved = 'light';
      themeState.isLoading = false;

      const state = themeState.getState();
      expect(state.current).toBe('steampunk-light');
      expect(state.resolved).toBe('light');
      expect(state.isLoading).toBe(false);
    });
  });

  describe('initializeFromSettings', () => {
    it('sets theme from Tauri settings on success', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-light' }
      } as never);

      await themeState.initializeFromSettings();

      expect(themeState.current).toBe('steampunk-light');
      expect(themeState.resolved).toBe('light');
      expect(themeState.isLoading).toBe(false);
    });

    it('applies steampunk-dark theme correctly', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-dark' }
      } as never);

      await themeState.initializeFromSettings();

      expect(themeState.current).toBe('steampunk-dark');
      expect(themeState.resolved).toBe('dark');
      expect(themeState.isLoading).toBe(false);
    });

    it('falls back to dark theme on error', async () => {
      mockGetSettings.mockRejectedValueOnce(new Error('Tauri not available'));

      await themeState.initializeFromSettings();

      expect(themeState.resolved).toBe('dark');
      expect(themeState.isLoading).toBe(false);
    });

    it('falls back to dark when status is not ok', async () => {
      mockGetSettings.mockResolvedValueOnce({ status: 'error', error: 'E_SETTINGS' } as never);

      await themeState.initializeFromSettings();

      expect(themeState.isLoading).toBe(false);
    });

    it('uses system preference when theme field is absent', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: {} // no theme field
      } as never);

      await themeState.initializeFromSettings();

      expect(themeState.current).toBe('system');
      expect(themeState.isLoading).toBe(false);
    });

    it('applies theme to DOM (sets body data-theme)', async () => {
      mockGetSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-light' }
      } as never);

      await themeState.initializeFromSettings();

      expect(document.body.dataset.theme).toBe('steampunk-light');
    });
  });

  describe('setTheme', () => {
    it('updates state and persists via Tauri on success', async () => {
      mockGetSettings.mockResolvedValueOnce({ status: 'ok', data: {} } as never);
      mockUpdateSettings.mockResolvedValueOnce({
        status: 'ok',
        data: { theme: 'steampunk-dark' }
      } as never);

      await themeState.setTheme('steampunk-dark');

      expect(themeState.current).toBe('steampunk-dark');
      expect(themeState.resolved).toBe('dark');
    });

    it('silently swallows errors on failure', async () => {
      mockGetSettings.mockRejectedValueOnce(new Error('Tauri unavailable'));

      // Should not throw
      await expect(themeState.setTheme('steampunk-light')).resolves.toBeUndefined();
    });
  });

  describe('getState', () => {
    it('returns current state synchronously', () => {
      themeState.current = 'steampunk-dark';
      themeState.resolved = 'dark';
      themeState.isLoading = false;

      const state = themeState.getState();
      expect(state.current).toBe('steampunk-dark');
    });
  });
});
