import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import { sellerTypeSchema } from './common';

/**
 * Seller Form Schema
 * Validates seller creation and update forms
 */
export const sellerSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(1, m.seller_form_validation_name_required()),
  sellerType: sellerTypeSchema,
  email: z
    .string()
    .email(m.seller_form_validation_email_invalid())
    .nullable()
    .or(z.literal(''))
    .optional(),
  phone: z.string().nullable().or(z.literal('')).optional(),
  websiteUrl: z
    .string()
    .url(m.seller_form_validation_website_invalid())
    .nullable()
    .or(z.literal(''))
    .optional(),
  streetAddress: z.string().nullable().or(z.literal('')).optional(),
  extendedAddress: z.string().nullable().or(z.literal('')).optional(),
  city: z.string().nullable().or(z.literal('')).optional(),
  stateRegion: z.string().nullable().or(z.literal('')).optional(),
  postalCode: z.string().nullable().or(z.literal('')).optional(),
  countryCode: z.string().nullable().or(z.literal('')).optional()
});

export type SellerFormData = z.infer<typeof sellerSchema>;
