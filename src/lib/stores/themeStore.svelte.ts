/**
 * Theme Store - Manages theme preference and resolution
 *
 * State:
 * - current: User's stored theme preference ('steampunk-light', 'steampunk-dark', 'system')
 * - resolved: Actual theme applied after resolving 'system' preference ('light', 'dark')
 * - isLoading: True during initial settings load from Tauri
 *
 * Feature: 011-steampunk-theme
 *
 * Note: Uses type 'any' for theme field until Tauri bindings regenerate at runtime
 */

import type { ThemeValue } from '$lib/types/theme';
import { writable, derived } from 'svelte/store';

export interface ThemeState {
  current: ThemeValue;
  resolved: 'light' | 'dark';
  isLoading: boolean;
}

// Private store - holds full theme state
const createThemeStore = () => {
  const {
    subscribe,
    set: privateSet,
    update: privateUpdate
  } = writable<ThemeState>({
    current: 'system',
    resolved: 'dark',
    isLoading: true
  });

  return {
    subscribe,
    set: privateSet,
    update: privateUpdate,

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

        const settings = result.data;
        const resolved = resolveTheme(settings.theme);
        privateUpdate((state) => ({
          ...state,
          current: settings.theme,
          resolved,
          isLoading: false
        }));

        // Apply to DOM
        applyTheme(resolved);

        // Listen for system theme changes if 'system' mode
        if (settings.theme === 'system') {
          setupSystemThemeListener();
        }
      } catch (error) {
        console.error('Failed to initialize theme:', error);
        // Fall back to dark theme
        privateUpdate((state) => ({ ...state, isLoading: false, resolved: 'dark' }));
        applyTheme('dark');
      }
    },

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

        const current = getResult.data;

        // Update via Tauri command
        const updateResult = await commands.updateSettings({
          ...current,
          theme
        });

        if (updateResult.status !== 'ok') {
          throw new Error('Failed to update settings');
        }

        const updated = updateResult.data;
        const resolved = resolveTheme(updated.theme);
        privateUpdate((state) => ({
          ...state,
          current: updated.theme,
          resolved
        }));

        applyTheme(resolved);

        // Re-setup system listener if switched to system mode
        if (theme === 'system') {
          setupSystemThemeListener();
        } else {
          cleanupSystemThemeListener();
        }
      } catch (error) {
        console.error('Failed to set theme:', error);
      }
    },

    /**
     * Get current state synchronously (for emergency use only)
     */
    getState(): ThemeState | null {
      let state: ThemeState | null = null;
      subscribe((s) => {
        state = s;
      })();
      return state;
    }
  };
};

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
 * Listen for OS theme changes and update if in 'system' mode
 */
let mediaQueryListener: ((e: MediaQueryListEvent) => void) | null = null;
let mediaQuery: MediaQueryList | null = null;

function setupSystemThemeListener(): void {
  if (typeof window === 'undefined') return;
  if (mediaQuery) return; // Already listening

  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

  mediaQueryListener = (e: MediaQueryListEvent) => {
    const state = themeStore.getState();
    if (state?.current === 'system') {
      const resolved = e.matches ? 'dark' : 'light';
      applyTheme(resolved);

      themeStore.update((s) => ({
        ...s,
        resolved
      }));
    }
  };

  mediaQuery.addEventListener('change', mediaQueryListener);
}

function cleanupSystemThemeListener(): void {
  if (mediaQuery && mediaQueryListener) {
    mediaQuery.removeEventListener('change', mediaQueryListener);
    mediaQuery = null;
    mediaQueryListener = null;
  }
}

/**
 * Public theme store
 */
export const themeStore = createThemeStore();

/**
 * Derived store for convenience: subscribe to just the resolved theme
 */
export const resolvedTheme = derived(themeStore, ($state) => $state.resolved);

/**
 * Derived store for convenience: subscribe to just the current preference
 */
export const currentTheme = derived(themeStore, ($state) => $state.current);

/**
 * Derived store for convenience: is the store loading?
 */
export const isThemeLoading = derived(themeStore, ($state) => $state.isLoading);
