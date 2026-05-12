import { z } from 'zod';
import { powerMethodSchema, scaleSchema } from './railway-model';
import { currencySchema, languageSchema, measureUnitSchema, appThemeSchema } from './common';

/**
 * Settings Form Schema
 * Validates user settings form
 */
export const settingsSchema = z.object({
  currency: currencySchema,
  measureUnit: measureUnitSchema,
  theme: appThemeSchema,
  favouriteScale: scaleSchema,
  powerMethod: powerMethodSchema,
  language: languageSchema
});

export type SettingsFormData = z.infer<typeof settingsSchema>;
