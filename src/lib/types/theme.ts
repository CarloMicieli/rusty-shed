/**
 * Theme Type Definitions
 * Feature: 011-steampunk-theme
 */

/**
 * User's theme preference choice
 * Stored in SQLite settings table
 */
export type ThemeValue = 'steampunk-light' | 'steampunk-dark' | 'system';

/**
 * The actual theme applied after resolving system preference
 * Used for CSS class application
 */
export type ResolvedTheme = 'light' | 'dark';

/**
 * Frontend theme store state
 * Managed by themeStore.svelte.ts
 */
export interface ThemeState {
  /** User's stored preference from settings */
  current: ThemeValue;
  /** Actual theme after system preference resolution */
  resolved: ResolvedTheme;
  /** True during initial settings load */
  isLoading: boolean;
}
