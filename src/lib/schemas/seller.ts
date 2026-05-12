import { z } from 'zod';
import { sellerTypeSchema } from './common';

/**
 * Seller Form Schema
 * Validates seller creation and update forms
 */
export const sellerSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(1, 'Seller name is required'),
  sellerType: sellerTypeSchema,
  email: z.string().email('Invalid email address').nullable().or(z.literal('')).optional(),
  phone: z.string().nullable().or(z.literal('')).optional(),
  websiteUrl: z.string().url('Invalid URL').nullable().or(z.literal('')).optional(),
  streetAddress: z.string().nullable().or(z.literal('')).optional(),
  extendedAddress: z.string().nullable().or(z.literal('')).optional(),
  city: z.string().nullable().or(z.literal('')).optional(),
  stateRegion: z.string().nullable().or(z.literal('')).optional(),
  postalCode: z.string().nullable().or(z.literal('')).optional(),
  countryCode: z.string().nullable().or(z.literal('')).optional()
});

export type SellerFormData = z.infer<typeof sellerSchema>;
