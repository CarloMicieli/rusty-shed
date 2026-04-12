import { getLocale } from '$lib/paraglide/runtime.js';
import type { Language } from '$lib/bindings';

/**
 * Locale Store - Runes-based singleton
 * Manages the active language preference
 */
class LocaleStateClass {
  activeLocale = $state<Language>((getLocale() as Language) ?? 'en');

  setActiveLocale(locale: Language): void {
    this.activeLocale = locale;
  }
}

// Singleton instance
export const localeState = new LocaleStateClass();

/**
 * Backwards-compat: Legacy function for migration path
 */
export function setActiveLocale(locale: Language): void {
  localeState.setActiveLocale(locale);
}
