import { writable } from 'svelte/store';
import { getLocale } from '$lib/paraglide/runtime.js';
import type { Language } from '$lib/bindings';

const initialLocale = (getLocale() as Language) ?? 'en';

export const localeStore = writable<Language>(initialLocale);

export function setActiveLocale(locale: Language) {
  localeStore.set(locale);
}
