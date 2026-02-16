/**
 * Settings state management using Svelte 5 runes
 *
 * Provides reactive settings state and methods to load/update settings.
 */

import { invoke } from '@tauri-apps/api/core';
import type { UserSettings, UpdateSettingsInput } from '$lib/bindings';

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
    console.log('[SettingsState] load() called');
    this.loading = true;
    this.error = null;

    try {
      console.log('[SettingsState] Calling get_settings command...');
      const settings = await invoke<UserSettings>('get_settings');
      console.log('[SettingsState] Settings received:', JSON.stringify(settings, null, 2));
      this.settings = settings;
    } catch (err) {
      this.error = String(err);
      console.error('[SettingsState] Failed to load settings:', err);
      throw err;
    } finally {
      this.loading = false;
    }
  }

  /**
   * Update settings (partial update supported)
   */
  async update(input: UpdateSettingsInput): Promise<void> {
    console.log('[SettingsState] Updating settings with input:', JSON.stringify(input, null, 2));
    this.loading = true;
    this.error = null;

    try {
      console.log('[SettingsState] Calling update_settings command...');
      const updated = await invoke<UserSettings>('update_settings', { input });
      console.log('[SettingsState] Update successful, received:', JSON.stringify(updated, null, 2));
      this.settings = updated;
    } catch (err) {
      this.error = String(err);
      console.error('[SettingsState] Failed to update settings:', err);
      console.error('[SettingsState] Error details:', JSON.stringify(err, null, 2));
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
      console.error('Failed to initialize settings:', err);
      throw err;
    }
  }
}

// Export singleton instance
export const settingsState = new SettingsState();
