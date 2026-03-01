/**
 * Settings types
 */

import type { Language } from '$lib/bindings';

export type MeasureUnit = 'Metric' | 'Imperial';
export type PowerSystem = 'DC' | 'AC' | 'DCC';

export interface UserSettings {
  currency: string;
  language: Language;
  measureUnit: MeasureUnit;
  favouriteScale: string;
  powerSystem: PowerSystem;
  firstRun: boolean;
}

export interface UpdateSettingsInput {
  currency?: string | null;
  language?: Language | null;
  measureUnit?: MeasureUnit | null;
  favouriteScale?: string | null;
  powerSystem?: PowerSystem | null;
}
