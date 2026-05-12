import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import { scaleSchema, powerMethodSchema } from './railway-model';

// ---------------------------------------------------------------------------
// Helper: nullable field required on submit.
// manufacturerId and category start as null; productCode starts as ''.
// ---------------------------------------------------------------------------
const nullableRequired = (message: string) =>
  z
    .string()
    .nullable()
    .refine((v): v is string => v !== null && v.trim().length > 0, message);

// ---------------------------------------------------------------------------
// Per-item schema for a single acquisition line
// ---------------------------------------------------------------------------

export const acquisitionItemSchema = z.object({
  uid: z.string(),
  manufacturerId: nullableRequired(m.acquisition_validation_manufacturer()),
  productCode: z.string().min(1, m.acquisition_validation_product_code()),
  description: z.string().default(''),
  category: nullableRequired(m.acquisition_validation_category()),
  epoch: z.string().nullable().default(null),
  priceAmount: z.number().nullable().default(null)
});

// ---------------------------------------------------------------------------
// Batch defaults (scale + powerMethod) — both optional
// ---------------------------------------------------------------------------

const batchDefaultsSchema = z.object({
  scale: scaleSchema.nullable().default(null),
  powerMethod: powerMethodSchema.nullable().default(null)
});

// ---------------------------------------------------------------------------
// Top-level schema for the Acquisition drawer
// ---------------------------------------------------------------------------

export const acquisitionSchema = z.object({
  sellerId: z.string().nullable().default(null),
  purchaseDate: z.string().default(''),
  batchDefaults: batchDefaultsSchema,
  items: z.array(acquisitionItemSchema).min(1, m.acquisition_validation_empty_items())
});

export type AcquisitionFormData = z.infer<typeof acquisitionSchema>;
