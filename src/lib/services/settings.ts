import type { Language } from '$lib/bindings';
import type { ThemeValue } from '$lib/types/theme';
export type { ThemeValue };
import type { SafeResult } from './errors';
import { safeInvoke } from './tauri';

export type Currency = 'EUR' | 'USD' | 'GBP' | 'JPY';
export type MeasureUnit = 'Metric' | 'Imperial';
export type PowerMethod = 'AC' | 'DC' | 'TRIX_EXPRESS';
export type Scale = 'H0' | 'H0m' | 'H0e' | 'N' | 'TT' | 'Z' | 'G' | '1' | '0' | '00';

export interface SettingsDto {
  currency: Currency;
  language: Language;
  theme: ThemeValue;
  measureUnit: MeasureUnit;
  favouriteScale: Scale;
  powerMethod: PowerMethod;
  has_completed_onboarding: boolean;
}

export type UpdateSettingsPayload = Partial<SettingsDto> & {
  hasCompletedOnboarding?: boolean;
};

export async function fetchSettings(): Promise<SafeResult<SettingsDto>> {
  return safeInvoke<SettingsDto>('get_settings');
}

export async function saveSettings(
  payload: UpdateSettingsPayload
): Promise<SafeResult<SettingsDto>> {
  return safeInvoke<SettingsDto>('update_settings', { input: payload });
}

export interface OnboardingSettingsPayload {
  language: Language;
  theme: ThemeValue;
  measureUnit: MeasureUnit;
  favouriteScale: Scale;
  powerMethod: PowerMethod;
}

export async function saveOnboardingSettings(
  payload: OnboardingSettingsPayload
): Promise<SafeResult<SettingsDto>> {
  return saveSettings(payload);
}

export async function markOnboardingCompleted(): Promise<SafeResult<SettingsDto>> {
  return saveSettings({ hasCompletedOnboarding: true });
}
