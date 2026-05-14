import { beforeEach, describe, expect, it, vi } from 'vitest';

const mockSetLocale = vi.hoisted(() => vi.fn());
const mockGetLocale = vi.hoisted(() => vi.fn(() => 'it'));

vi.mock('$lib/paraglide/runtime.js', () => ({
  setLocale: mockSetLocale,
  getLocale: mockGetLocale,
  locales: ['en', 'it'] as const
}));

import {
  LocaleService,
  setActiveLocale,
  type AvailableLanguageTag
} from '$lib/shared/services/LocaleService.svelte';

describe('LocaleService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockGetLocale.mockReturnValue('it');
  });

  it('initializes currentLocale from paraglide runtime', () => {
    const service = new LocaleService();

    expect(service.currentLocale).toBe('it');
    expect(service.getLocale()).toBe('it');
  });

  it('updates local state and runtime when setLocale is called', () => {
    const service = new LocaleService();

    service.setLocale('en');

    expect(service.currentLocale).toBe('en');
    expect(service.getLocale()).toBe('en');
    expect(mockSetLocale).toHaveBeenCalledWith('en');
  });

  it('reports active locale correctly', () => {
    const service = new LocaleService();

    service.setLocale('en');

    expect(service.isActive('en')).toBe(true);
    expect(service.isActive('it')).toBe(false);
  });
});

describe('setActiveLocale (deprecated)', () => {
  it('warns and forwards locale to runtime setLocale', () => {
    const warnSpy = vi.spyOn(console, 'warn').mockImplementation(() => undefined);

    setActiveLocale('it' as AvailableLanguageTag);

    expect(warnSpy).toHaveBeenCalledOnce();
    expect(mockSetLocale).toHaveBeenCalledWith('it');

    warnSpy.mockRestore();
  });
});
