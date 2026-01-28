/**
 * Locale Service - Manages application language/locale state.
 *
 * This service replaces the legacy locale store and provides:
 * - Reactive locale state using Svelte 5 $state
 * - Integration with Paraglide i18n
 * - Context API for dependency injection
 */

import { setContext, getContext } from 'svelte';
import { setLocale, getLocale, locales } from '$lib/paraglide/runtime.js';

// Type for available language tags (from Paraglide)
export type AvailableLanguageTag = (typeof locales)[number];

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('locale-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class LocaleService {
  // Private reactive state
  #currentLocale = $state<AvailableLanguageTag>(getLocale());

  // Public readonly getter (defensive encapsulation)
  get currentLocale(): AvailableLanguageTag {
    return this.#currentLocale;
  }

  // ─────────────────────────────────────────────────────────────
  // USE CASES (Public Methods)
  // ─────────────────────────────────────────────────────────────

  /**
   * Set the active locale for the application.
   *
   * @param locale - The locale code to set (e.g., 'en', 'it')
   */
  setLocale(locale: AvailableLanguageTag): void {
    this.#currentLocale = locale;
    setLocale(locale);
  }

  /**
   * Get the current locale.
   *
   * @returns The current locale code
   */
  getLocale(): AvailableLanguageTag {
    return this.#currentLocale;
  }

  /**
   * Check if a specific locale is currently active.
   *
   * @param locale - The locale to check
   * @returns True if the locale is active
   */
  isActive(locale: AvailableLanguageTag): boolean {
    return this.#currentLocale === locale;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS (Dependency Injection)
// ─────────────────────────────────────────────────────────────

/**
 * Initialize and set the LocaleService in the current context.
 *
 * @param service - Optional service instance (for testing)
 * @returns The service instance
 */
export function setLocaleService(service?: LocaleService): LocaleService {
  const instance = service ?? new LocaleService();
  setContext(SERVICE_KEY, instance);
  return instance;
}

/**
 * Get the LocaleService from the current context.
 *
 * @returns The service instance
 * @throws Error if service is not found in context
 */
export function getLocaleService(): LocaleService {
  const service = getContext<LocaleService>(SERVICE_KEY);
  if (!service) {
    throw new Error(
      'LocaleService not found in context. Did you call setLocaleService() in a parent component?'
    );
  }
  return service;
}

// ─────────────────────────────────────────────────────────────
// LEGACY COMPATIBILITY (to be removed after migration)
// ─────────────────────────────────────────────────────────────

/**
 * @deprecated Use LocaleService.setLocale() instead
 */
export function setActiveLocale(locale: AvailableLanguageTag): void {
  console.warn('setActiveLocale is deprecated. Use LocaleService.setLocale() instead.');
  setLocale(locale);
}
