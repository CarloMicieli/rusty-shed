import { describe, it, expect } from 'vitest';
import { settingsSchema } from '$lib/schemas/settings';
import type { SettingsFormData } from '$lib/schemas/settings';

describe('settingsSchema', () => {
  const validSettings: SettingsFormData = {
    currency: 'EUR',
    measureUnit: 'Metric',
    theme: 'steampunk-dark',
    favouriteScale: 'H0',
    powerMethod: 'DC',
    language: 'en'
  };

  describe('complete valid input', () => {
    it('accepts a fully valid settings object', () => {
      const result = settingsSchema.parse(validSettings);
      expect(result.currency).toBe('EUR');
      expect(result.measureUnit).toBe('Metric');
      expect(result.theme).toBe('steampunk-dark');
      expect(result.favouriteScale).toBe('H0');
      expect(result.powerMethod).toBe('DC');
      expect(result.language).toBe('en');
    });
  });

  describe('currency field', () => {
    it('accepts EUR', () => {
      expect(settingsSchema.parse({ ...validSettings, currency: 'EUR' }).currency).toBe('EUR');
    });

    it('accepts USD', () => {
      expect(settingsSchema.parse({ ...validSettings, currency: 'USD' }).currency).toBe('USD');
    });

    it('accepts GBP', () => {
      expect(settingsSchema.parse({ ...validSettings, currency: 'GBP' }).currency).toBe('GBP');
    });

    it('accepts JPY', () => {
      expect(settingsSchema.parse({ ...validSettings, currency: 'JPY' }).currency).toBe('JPY');
    });

    it('rejects an unknown currency', () => {
      expect(() =>
        settingsSchema.parse({ ...validSettings, currency: 'CHF' as unknown })
      ).toThrow();
    });

    it('rejects a missing currency', () => {
      const { currency: _c, ...invalid } = validSettings;
      expect(() => settingsSchema.parse(invalid)).toThrow();
    });
  });

  describe('measureUnit field', () => {
    it('accepts Metric', () => {
      expect(settingsSchema.parse({ ...validSettings, measureUnit: 'Metric' }).measureUnit).toBe(
        'Metric'
      );
    });

    it('accepts Imperial', () => {
      expect(settingsSchema.parse({ ...validSettings, measureUnit: 'Imperial' }).measureUnit).toBe(
        'Imperial'
      );
    });

    it('rejects an invalid measureUnit', () => {
      expect(() =>
        settingsSchema.parse({ ...validSettings, measureUnit: 'metric' as unknown })
      ).toThrow();
    });
  });

  describe('theme field', () => {
    it('accepts steampunk-light', () => {
      expect(settingsSchema.parse({ ...validSettings, theme: 'steampunk-light' }).theme).toBe(
        'steampunk-light'
      );
    });

    it('accepts steampunk-dark', () => {
      expect(settingsSchema.parse({ ...validSettings, theme: 'steampunk-dark' }).theme).toBe(
        'steampunk-dark'
      );
    });

    it('accepts system', () => {
      expect(settingsSchema.parse({ ...validSettings, theme: 'system' }).theme).toBe('system');
    });

    it('rejects an unknown theme', () => {
      expect(() => settingsSchema.parse({ ...validSettings, theme: 'dark' as unknown })).toThrow();
    });
  });

  describe('favouriteScale field', () => {
    it('accepts H0', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'H0' }).favouriteScale).toBe(
        'H0'
      );
    });

    it('accepts N', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'N' }).favouriteScale).toBe(
        'N'
      );
    });

    it('accepts TT', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'TT' }).favouriteScale).toBe(
        'TT'
      );
    });

    it('accepts Z', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'Z' }).favouriteScale).toBe(
        'Z'
      );
    });

    it('accepts G', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'G' }).favouriteScale).toBe(
        'G'
      );
    });

    it('accepts 0', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: '0' }).favouriteScale).toBe(
        '0'
      );
    });

    it('accepts 00', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: '00' }).favouriteScale).toBe(
        '00'
      );
    });

    it('accepts 1', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: '1' }).favouriteScale).toBe(
        '1'
      );
    });

    it('accepts H0m', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'H0m' }).favouriteScale).toBe(
        'H0m'
      );
    });

    it('accepts H0e', () => {
      expect(settingsSchema.parse({ ...validSettings, favouriteScale: 'H0e' }).favouriteScale).toBe(
        'H0e'
      );
    });

    it('rejects HO (misspelling)', () => {
      expect(() =>
        settingsSchema.parse({ ...validSettings, favouriteScale: 'HO' as unknown })
      ).toThrow();
    });

    it('rejects an unknown scale', () => {
      expect(() =>
        settingsSchema.parse({ ...validSettings, favouriteScale: 'OO' as unknown })
      ).toThrow();
    });
  });

  describe('powerMethod field', () => {
    it('accepts AC', () => {
      expect(settingsSchema.parse({ ...validSettings, powerMethod: 'AC' }).powerMethod).toBe('AC');
    });

    it('accepts DC', () => {
      expect(settingsSchema.parse({ ...validSettings, powerMethod: 'DC' }).powerMethod).toBe('DC');
    });

    it('accepts TRIX_EXPRESS', () => {
      expect(
        settingsSchema.parse({ ...validSettings, powerMethod: 'TRIX_EXPRESS' }).powerMethod
      ).toBe('TRIX_EXPRESS');
    });

    it('rejects an invalid powerMethod', () => {
      expect(() =>
        settingsSchema.parse({ ...validSettings, powerMethod: 'DCC' as unknown })
      ).toThrow();
    });
  });

  describe('language field', () => {
    it('accepts en', () => {
      expect(settingsSchema.parse({ ...validSettings, language: 'en' }).language).toBe('en');
    });

    it('accepts it', () => {
      expect(settingsSchema.parse({ ...validSettings, language: 'it' }).language).toBe('it');
    });

    it('rejects an unsupported language code', () => {
      expect(() => settingsSchema.parse({ ...validSettings, language: 'de' as unknown })).toThrow();
    });

    it('rejects a missing language', () => {
      const { language: _l, ...invalid } = validSettings;
      expect(() => settingsSchema.parse(invalid)).toThrow();
    });
  });

  describe('all fields required', () => {
    it('rejects an empty object', () => {
      expect(() => settingsSchema.parse({})).toThrow();
    });

    it('rejects partial settings (missing theme)', () => {
      const { theme: _t, ...partial } = validSettings;
      expect(() => settingsSchema.parse(partial)).toThrow();
    });
  });
});
