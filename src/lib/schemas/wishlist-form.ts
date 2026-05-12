import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import { wishlistPrioritySchema } from './common';
import { scaleSchema, powerMethodSchema, categorySchema } from './railway-model';

// ---------------------------------------------------------------------------
// Helper: nullable field that is required on submit.
// ---------------------------------------------------------------------------
const nullableRequired = (message: string) =>
  z
    .string()
    .nullable()
    .refine((v): v is string => v !== null && v.trim().length > 0, message);

// ---------------------------------------------------------------------------
// Wishlist item form schema
// ---------------------------------------------------------------------------

export const wishlistFormSchema = z.object({
  wishlistId: z.string().default(''),
  newListName: z.string().default(''),
  manufacturerId: nullableRequired(m.wishlist_modal_missing_manufacturer()),
  productCode: z.string().min(1, m.wishlist_modal_missing_product_code()),
  description: z.string().min(1, m.wishlist_modal_missing_description()),
  category: categorySchema.nullable().default(null),
  scale: z.union([scaleSchema, z.literal('')]).default(''),
  powerMethod: z.union([powerMethodSchema, z.literal('')]).default(''),
  epoch: z.string().nullable().default(null),
  priority: wishlistPrioritySchema.default('NORMAL'),
  desiredPrice: z.number().positive(m.wishlist_modal_invalid_price()).nullable().default(null)
});

export type WishlistFormData = z.infer<typeof wishlistFormSchema>;
