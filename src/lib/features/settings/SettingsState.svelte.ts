/**
 * Settings state management using Svelte 5 runes
 *
 * Provides reactive settings state and methods to load/update settings.
 */

import { safeInvoke } from '$lib/services';
import type { UserSettings, UpdateSettingsInput } from '$lib/bindings';
import { log } from '$lib/tauri-logger';

export class SettingsState {
  settings = $state<UserSettings>({
    currency: 'EUR',
    language: 'en',
    theme: 'steampunk-dark',
    measureUnit: 'Metric',
    favouriteScale: '',
    powerSystem: 'DC',
    firstRun: true
  });

  loading = $state(false);
  error = $state<string | null>(null);

  /**
   * Load settings from backend
   */
  async load(): Promise<void> {
    log.debug('SettingsState: load() called');
    this.loading = true;
    this.error = null;

    try {
      log.debug('SettingsState: Calling get_settings command...');
      const result = await safeInvoke<UserSettings>('get_settings');
      if (result.ok) {
        log.debug('SettingsState: Settings received');
        this.settings = result.data;
      } else {
        this.error = result.error.message;
        log.error(`SettingsState: Failed to load settings: ${result.error.message}`);
        throw new Error(result.error.message);
      }
    } finally {
      this.loading = false;
    }
  }

  /**
   * Update settings (partial update supported)
   */
  async update(input: UpdateSettingsInput): Promise<void> {
    log.debug('SettingsState: Updating settings');
    this.loading = true;
    this.error = null;

    try {
      log.debug('SettingsState: Calling update_settings command...');
      const result = await safeInvoke<UserSettings>('update_settings', {
        input
      } as Record<string, unknown>);
      if (result.ok) {
        log.debug('SettingsState: Update successful');
        this.settings = result.data;
      } else {
        this.error = result.error.message;
        log.error(`SettingsState: Failed to update settings: ${result.error.message}`);
        throw new Error(result.error.message);
      }
    } finally {
      this.loading = false;
    }
  }

  /**
   * Initialize settings on first run
   */
  async initialize(): Promise<void> {
    const result = await safeInvoke<UserSettings>('initialize_settings');
    if (result.ok) {
      this.settings = result.data;
    } else {
      log.error(`Failed to initialize settings: ${result.error.message}`);
      throw new Error(result.error.message);
    }
  }
}

// Export singleton instance
export const settingsState = new SettingsState();
