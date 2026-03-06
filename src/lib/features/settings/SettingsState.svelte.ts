/**
 * Settings state management using Svelte 5 runes
 *
 * Provides reactive settings state and methods to load/update settings.
 */

import { invoke } from '@tauri-apps/api/core';
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
      const settings = await invoke<UserSettings>('get_settings');
      log.debug('SettingsState: Settings received');
      this.settings = settings;
    } catch (err) {
      this.error = String(err);
      log.error(`SettingsState: Failed to load settings: ${String(err)}`);
      throw err;
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
      const updated = await invoke<UserSettings>('update_settings', { input });
      log.debug('SettingsState: Update successful');
      this.settings = updated;
    } catch (err) {
      this.error = String(err);
      log.error(`SettingsState: Failed to update settings: ${String(err)}`);
      throw err;
    } finally {
      this.loading = false;
    }
  }

  /**
   * Initialize settings on first run
   */
  async initialize(): Promise<void> {
    try {
      const settings = await invoke<UserSettings>('initialize_settings');
      this.settings = settings;
    } catch (err) {
      log.error(`Failed to initialize settings: ${String(err)}`);
      throw err;
    }
  }
}

// Export singleton instance
export const settingsState = new SettingsState();
