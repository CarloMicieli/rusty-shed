/**
 * RegionalManager — singleton for locale-aware formatting.
 *
 * Tracks the OS locale (detected at startup via Tauri) and derives currency /
 * unit-system preferences from SettingsState. Provides reactive Intl-based
 * formatters for dates, currency amounts, and lengths.
 */

import { settingsState } from './SettingsState.svelte';
import { commands } from '$lib/bindings';
import { log } from '$lib/tauri-logger';

class RegionalManager {
  locale = $state('en-US');

  // Derived directly from SettingsState — stay in sync automatically
  currency = $derived(settingsState.settings.currency);
  unitSystem = $derived(settingsState.settings.measureUnit);

  // Reactive formatter instances — rebuilt whenever locale or currency changes
  #dateFormatter = $derived(new Intl.DateTimeFormat(this.locale, { dateStyle: 'medium' }));
  #currencyFormatter = $derived(
    new Intl.NumberFormat(this.locale, {
      style: 'currency',
      currency: this.currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    })
  );

  /** Call once at app startup (after settingsState.initialize()). */
  async init(): Promise<void> {
    try {
      const locale = await commands.getLocale();
      if (locale) this.locale = locale;
    } catch (e) {
      log.warn(`RegionalManager: could not detect OS locale, using default. ${e}`);
    }
  }

  /**
   * Format a Date object or ISO string (YYYY-MM-DD) using the current locale.
   * Uses local-timezone parsing to avoid UTC-offset display bugs.
   */
  formatDate(dateOrIso: Date | string): string {
    const date =
      typeof dateOrIso === 'string'
        ? (() => {
            const [y, mo, d] = dateOrIso.split('-').map(Number);
            return new Date(y, mo - 1, d);
          })()
        : dateOrIso;
    return this.#dateFormatter.format(date);
  }

  /**
   * Format an amount stored as integer cents using the user's default currency
   * and current locale.
   */
  formatCurrency(cents: number | bigint): string {
    return this.#currencyFormatter.format(Number(cents) / 100);
  }

  /**
   * Format an amount in any explicit currency using the current locale.
   * Use this when the price currency may differ from the user's default currency.
   */
  formatCurrencyWith(cents: number | bigint, currency: string): string {
    try {
      return new Intl.NumberFormat(this.locale, {
        style: 'currency',
        currency,
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(Number(cents) / 100);
    } catch {
      return `${currency} ${(Number(cents) / 100).toFixed(2)}`;
    }
  }

  /**
   * Return the symbol character for any currency code using the current locale.
   * Replaces getCurrencySymbol() from $lib/utils/currency.
   */
  getCurrencySymbol(currency: string): string {
    try {
      return (
        new Intl.NumberFormat(this.locale, { style: 'currency', currency })
          .formatToParts(0)
          .find((p) => p.type === 'currency')?.value ?? currency
      );
    } catch {
      return currency;
    }
  }

  /**
   * Format a length in millimetres, converting to inches when Imperial is selected.
   */
  formatLength(mm: number): string {
    return this.unitSystem === 'Imperial' ? `${(mm / 25.4).toFixed(2)} in` : `${mm} mm`;
  }
}

export const regionalManager = new RegionalManager();
