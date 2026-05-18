/**
 * Settings state management using Svelte 5 runes
 *
 * Provides reactive settings state and methods to load/update settings.
 */

import { safeInvoke } from '$lib/services';
import type { UpdateSettingsInput, UserSettings_Serialize } from '$lib/bindings';
import type { LibraryTab } from './library-types';
import { log } from '$lib/tauri-logger';
import type { LibraryEntityRow } from '$lib/services/entityLibrary';

export class SettingsState {
  settings = $state<UserSettings_Serialize>({
    currency: 'EUR',
    language: 'en',
    theme: 'steampunk-dark',
    measureUnit: 'Metric',
    favouriteScale: '',
    powerMethod: 'DC',
    firstRun: true
  });

  loading = $state(false);
  error = $state<string | null>(null);

  libraryActiveTab = $state<LibraryTab>('manufacturers');
  librarySearchQuery = $state('');
  libraryLoading = $state(false);
  libraryError = $state<string | null>(null);
  libraryManufacturers = $state<LibraryEntityRow[]>([]);
  librarySellers = $state<LibraryEntityRow[]>([]);
  libraryBuyers = $state<LibraryEntityRow[]>([]);

  /**
   * Load settings from backend
   */
  async load(): Promise<void> {
    log.debug('SettingsState: load() called');
    this.loading = true;
    this.error = null;

    try {
      log.debug('SettingsState: Calling get_settings command...');
      const result = await safeInvoke<UserSettings_Serialize>('get_settings');
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
      const result = await safeInvoke<UserSettings_Serialize>('update_settings', {
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
    const result = await safeInvoke<UserSettings_Serialize>('initialize_settings');
    if (result.ok) {
      this.settings = result.data;
    } else {
      log.error(`Failed to initialize settings: ${result.error.message}`);
      throw new Error(result.error.message);
    }
  }

  setLibraryTab(tab: LibraryTab): void {
    this.libraryActiveTab = tab;
  }

  setLibrarySearchQuery(query: string): void {
    this.librarySearchQuery = query;
  }

  setLibraryRows(payload: {
    manufacturers: LibraryEntityRow[];
    sellers: LibraryEntityRow[];
    buyers: LibraryEntityRow[];
  }): void {
    this.libraryManufacturers = payload.manufacturers;
    this.librarySellers = payload.sellers;
    this.libraryBuyers = payload.buyers;
  }

  upsertLibraryManufacturer(row: LibraryEntityRow): void {
    this.libraryManufacturers = [
      row,
      ...this.libraryManufacturers.filter((entry) => entry.id !== row.id)
    ];
  }

  upsertCanonicalParty(row: LibraryEntityRow): void {
    this.librarySellers = [row, ...this.librarySellers.filter((entry) => entry.id !== row.id)];
    this.libraryBuyers = [row, ...this.libraryBuyers.filter((entry) => entry.id !== row.id)];
  }

  removeLibraryManufacturer(id: string): void {
    this.libraryManufacturers = this.libraryManufacturers.filter((entry) => entry.id !== id);
  }

  removeCanonicalParty(id: string): void {
    this.librarySellers = this.librarySellers.filter((entry) => entry.id !== id);
    this.libraryBuyers = this.libraryBuyers.filter((entry) => entry.id !== id);
  }

  mergeLibraryManufacturer(sourceId: string): void {
    this.libraryManufacturers = this.libraryManufacturers.filter((entry) => entry.id !== sourceId);
  }

  mergeCanonicalParty(sourceId: string): void {
    this.librarySellers = this.librarySellers.filter((entry) => entry.id !== sourceId);
    this.libraryBuyers = this.libraryBuyers.filter((entry) => entry.id !== sourceId);
  }
}

// Export singleton instance
export const settingsState = new SettingsState();
