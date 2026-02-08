import { z } from 'zod';

/**
 * Settings Form Schema
 * Validates user settings form
 */
export const settingsSchema = z.object({
  currency: z.enum(['EUR', 'USD', 'GBP', 'JPY']),
  lengthUnit: z.enum(['MILLIMETERS', 'INCHES']),
  favoriteScale: z.enum(['H0', 'N', 'TT', 'Z', 'G', '0', '00', '1', 'H0m', 'H0e']),
  favoritePowerMethod: z.enum(['AC', 'DC', 'TRIX_EXPRESS']),
  languageCode: z.enum(['en', 'it'])
});

export type SettingsFormData = z.infer<typeof settingsSchema>;
