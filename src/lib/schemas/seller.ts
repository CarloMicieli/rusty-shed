import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import { sellerTypeSchema } from './common';

function getMessage(key: string, fallback: string): string {
  try {
    const candidate = (m as Record<string, unknown>)[key];
    return typeof candidate === 'function' ? (candidate as () => string)() : fallback;
  } catch {
    return fallback;
  }
}

const nameRequiredMessage = getMessage(
  'seller_form_validation_name_required',
  'Seller name is required'
);
const invalidEmailMessage = getMessage(
  'seller_form_validation_email_invalid',
  'Invalid email address'
);
const invalidUrlMessage = getMessage('seller_form_validation_website_invalid', 'Invalid URL');

/**
 * Seller Form Schema
 * Validates seller creation and update forms
 */
export const sellerSchema = z.object({
  id: z.string().optional(),
  name: z.string().min(1, nameRequiredMessage),
  sellerType: sellerTypeSchema,
  email: z.string().email(invalidEmailMessage).nullable().or(z.literal('')).optional(),
  phone: z.string().nullable().or(z.literal('')).optional(),
  websiteUrl: z.string().url(invalidUrlMessage).nullable().or(z.literal('')).optional(),
  streetAddress: z.string().nullable().or(z.literal('')).optional(),
  extendedAddress: z.string().nullable().or(z.literal('')).optional(),
  city: z.string().nullable().or(z.literal('')).optional(),
  stateRegion: z.string().nullable().or(z.literal('')).optional(),
  postalCode: z.string().nullable().or(z.literal('')).optional(),
  countryCode: z.string().nullable().or(z.literal('')).optional()
});

export type SellerFormData = z.infer<typeof sellerSchema>;
