/**
 * Theme Store - Runes-based singleton
 * Manages theme preference and resolution
 *
 * State:
 * - current: User's stored theme preference ('steampunk-light', 'steampunk-dark', 'system')
 * - resolved: Actual theme applied after resolving 'system' preference ('light', 'dark')
 * - isLoading: True during initial settings load from Tauri
 *
 * Feature: 011-steampunk-theme
 */

import type { ThemeValue } from '$lib/types/theme';
import type { UpdateSettingsInput, UserSettings } from '$lib/bindings';

export interface ThemeState {
  current: ThemeValue;
  resolved: 'light' | 'dark';
  isLoading: boolean;
}

class ThemeStateClass {
  current = $state<ThemeValue>('system');
  resolved = $state<'light' | 'dark'>('dark');
  isLoading = $state<boolean>(true);

  private mediaQuery: MediaQueryList | null = null;
  private mediaQueryListener: ((e: MediaQueryListEvent) => void) | null = null;

  /**
   * Initialize theme from Tauri settings and detect system preference
   */
  async initializeFromSettings(): Promise<void> {
    try {
      // Import commands from generated bindings
      const { commands } = await import('$lib/bindings');
      const result = await commands.getSettings();

      if (result.status !== 'ok') {
        throw new Error('Failed to get settings');
      }

      const settings: UserSettings = result.data;
      const theme = settings.theme ?? 'system';
      const resolved = resolveTheme(theme);
      this.current = theme;
      this.resolved = resolved;
      this.isLoading = false;

      // Apply to DOM
      applyTheme(resolved);

      // Listen for system theme changes if 'system' mode
      if (theme === 'system') {
        this.setupSystemThemeListener();
      }
    } catch (error) {
      console.error('Failed to initialize theme:', error);
      // Fall back to dark theme
      this.isLoading = false;
      this.resolved = 'dark';
      applyTheme('dark');
    }
  }

  /**
   * Set user's theme preference and persist to Tauri settings
   */
  async setTheme(theme: ThemeValue): Promise<void> {
    try {
      const { commands } = await import('$lib/bindings');
      const getResult = await commands.getSettings();

      if (getResult.status !== 'ok') {
        throw new Error('Failed to get settings');
      }

      const current: UserSettings = getResult.data;
      const updatePayload: UpdateSettingsInput = {
        currency: current.currency ?? null,
        language: current.language ?? null,
        measureUnit: current.measureUnit ?? null,
        favouriteScale: current.favouriteScale ?? null,
        powerMethod: current.powerMethod ?? null,
        theme
      };

      const updateResult = await commands.updateSettings(updatePayload);

      if (updateResult.status !== 'ok') {
        throw new Error('Failed to update settings');
      }

      const updated: UserSettings = updateResult.data;
      const updatedTheme = updated.theme ?? theme;
      const resolved = resolveTheme(updatedTheme);
      this.current = updatedTheme;
      this.resolved = resolved;

      applyTheme(resolved);

      // Re-setup system listener if switched to system mode
      if (theme === 'system') {
        this.setupSystemThemeListener();
      } else {
        this.cleanupSystemThemeListener();
      }
    } catch (error) {
      console.error('Failed to set theme:', error);
    }
  }

  /**
   * Get current state synchronously (for emergency use only)
   */
  getState(): ThemeState {
    return {
      current: this.current,
      resolved: this.resolved,
      isLoading: this.isLoading
    };
  }

  private setupSystemThemeListener(): void {
    if (typeof window === 'undefined') return;
    if (this.mediaQuery) return; // Already listening

    this.mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    this.mediaQueryListener = (e: MediaQueryListEvent) => {
      if (this.current === 'system') {
        const resolved = e.matches ? 'dark' : 'light';
        applyTheme(resolved);
        this.resolved = resolved;
      }
    };

    this.mediaQuery.addEventListener('change', this.mediaQueryListener);
  }

  private cleanupSystemThemeListener(): void {
    if (this.mediaQuery && this.mediaQueryListener) {
      this.mediaQuery.removeEventListener('change', this.mediaQueryListener);
      this.mediaQuery = null;
      this.mediaQueryListener = null;
    }
  }
}

/**
 * Resolve 'system' preference to actual 'light' or 'dark' theme
 */
function resolveTheme(theme: ThemeValue): 'light' | 'dark' {
  if (theme !== 'system') {
    return theme === 'steampunk-light' ? 'light' : 'dark';
  }

  // Detect system preference
  if (typeof window === 'undefined') {
    return 'dark'; // SSR fallback
  }

  const darkMode = window.matchMedia('(prefers-color-scheme: dark)').matches;
  return darkMode ? 'dark' : 'light';
}

/**
 * Apply resolved theme to DOM
 */
function applyTheme(resolved: 'light' | 'dark'): void {
  if (typeof document === 'undefined') return;

  const themeValue = resolved === 'light' ? 'steampunk-light' : 'steampunk-dark';
  document.body.dataset.theme = themeValue;
}

/**
 * Public theme store singleton
 */
export const themeState = new ThemeStateClass();
