import { writable } from 'svelte/store';
import { getLocale } from '$lib/paraglide/runtime.js';
import type { LanguageCode } from '$lib/services';

const initialLocale = (getLocale() as LanguageCode) ?? 'en';

export const localeStore = writable<LanguageCode>(initialLocale);

export function setActiveLocale(locale: LanguageCode) {
  localeStore.set(locale);
}
