import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import {
  currencySchema,
  purchaseConditionSchema,
  modelConditionSchema,
  boxConditionSchema
} from './common';

// ---------------------------------------------------------------------------
// Helper: nullable field that is required on submit.
// Accepts `string | null` from the form state; fails when null or empty.
// ---------------------------------------------------------------------------
const nullableRequired = (message: string) =>
  z
    .string()
    .nullable()
    .refine((v): v is string => v !== null && v.trim().length > 0, message);

// ---------------------------------------------------------------------------
// Rolling stock entry inside the Add Collection form
// ---------------------------------------------------------------------------

export const rollingStockEntrySchema = z.object({
  uid: z.string(),
  railwayCompanyId: nullableRequired(m.add_model_validation_rs_company()),
  seriesCode: z.string().min(1, m.add_model_validation_rs_series()),
  category: nullableRequired(m.add_model_validation_rs_category()),
  roadNumber: z.string().default(''),
  subcategory: z.string().nullable().default(null)
});

// ---------------------------------------------------------------------------
// Optional purchase section (all fields optional — not validated)
// ---------------------------------------------------------------------------

const purchaseSchema = z.object({
  sellerId: z.string().nullable().default(null),
  priceAmount: z.number().nullable().default(null),
  priceCurrency: currencySchema.default('EUR'),
  purchaseCondition: purchaseConditionSchema.nullable().default(null),
  modelCondition: modelConditionSchema.nullable().default(null),
  boxCondition: boxConditionSchema.nullable().default(null),
  notes: z.string().default(''),
  purchaseDate: z.string().default(''),
  purchaseType: z.enum(['STANDARD', 'PREORDER']).default('STANDARD'),
  depositAmount: z.number().nullable().default(null),
  depositCurrency: currencySchema.nullable().default(null),
  preorderTotalAmount: z.number().nullable().default(null),
  preorderTotalCurrency: currencySchema.nullable().default(null),
  expectedDate: z.string().nullable().default(null)
});

// ---------------------------------------------------------------------------
// Top-level schema for the Add Collection Item drawer
// ---------------------------------------------------------------------------

export const addCollectionSchema = z.object({
  manufacturerId: nullableRequired(m.add_model_validation_manufacturer()),
  productCode: z.string().min(1, m.add_model_validation_product_code()),
  description: z.string().min(1, m.add_model_validation_description()),
  category: nullableRequired(m.add_model_validation_category()),
  scale: nullableRequired(m.add_model_validation_scale()),
  powerMethod: nullableRequired(m.add_model_validation_power()),
  epoch: nullableRequired(m.add_model_validation_epoch()),
  rollingStocks: z.array(rollingStockEntrySchema),
  purchase: purchaseSchema
});

export type AddCollectionFormData = z.infer<typeof addCollectionSchema>;
