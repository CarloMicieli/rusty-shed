import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';

// ---------------------------------------------------------------------------
// Track purchase form schema
// ---------------------------------------------------------------------------

export const trackPurchaseSchema = z.object({
  selectedProductId: z.string().min(1, m.track_purchase_validation_product()),
  quantity: z.number().min(1, m.track_purchase_validation_quantity()),
  priceAmount: z
    .number()
    .nullable()
    .refine((v) => v !== null && v >= 0, m.track_purchase_validation_price()),
  priceCurrency: z.string().default('EUR'),
  selectedSellerId: z.string().default(''),
  purchaseDate: z.string().default('')
});

export type TrackPurchaseFormData = z.infer<typeof trackPurchaseSchema>;
