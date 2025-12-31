import type { SafeResult } from './errors';
import { safeInvoke } from './tauri';

export type Currency = 'EUR' | 'USD' | 'GBP' | 'JPY';
export type MeasureUnit = 'MILLIMETERS' | 'INCHES' | 'METERS' | 'MILES' | 'KILOMETERS';
export type PowerMethod = 'AC' | 'DC' | 'TRIX_EXPRESS';
export type Scale = 'H0' | 'H0m' | 'H0e' | 'N' | 'TT' | 'Z' | 'G' | '1' | '0' | '00';
export type LanguageCode = 'en' | 'it' | string;

export interface SettingsDto {
  id: number;
  currency: Currency;
  lengthUnit: MeasureUnit;
  favoriteScale: Scale;
  favoritePowerMethod: PowerMethod;
  languageCode: LanguageCode;
}

export type UpdateSettingsPayload = Omit<SettingsDto, 'id'>;

export async function fetchSettings(): Promise<SafeResult<SettingsDto>> {
  return safeInvoke<SettingsDto>('get_settings');
}

export async function saveSettings(
  payload: UpdateSettingsPayload
): Promise<SafeResult<SettingsDto>> {
  return safeInvoke<SettingsDto>('update_settings', { payload });
}
