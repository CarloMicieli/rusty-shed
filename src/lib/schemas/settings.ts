import { z } from 'zod';

/**
 * Settings Form Schema
 * Validates user settings form
 */
export const settingsSchema = z.object({
  currency: z.enum(['EUR', 'USD', 'GBP', 'JPY']),
  measureUnit: z.enum(['Metric', 'Imperial']),
  theme: z.enum(['steampunk-light', 'steampunk-dark', 'system']),
  favouriteScale: z.enum(['H0', 'N', 'TT', 'Z', 'G', '0', '00', '1', 'H0m', 'H0e']),
  powerSystem: z.enum(['AC', 'DC', 'DCC']),
  language: z.enum(['en', 'it'])
});

export type SettingsFormData = z.infer<typeof settingsSchema>;
