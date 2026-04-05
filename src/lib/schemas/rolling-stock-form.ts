import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';

// ---------------------------------------------------------------------------
// Rolling stock create form schema
// ---------------------------------------------------------------------------

export const rollingStockCreateSchema = z.object({
  prototypeId: z.string().default(''),
  railwayCompanyId: z.string().min(1, m.error_required()),
  category: z.string().min(1, m.error_required()),
  seriesCode: z.string().min(1, m.error_required()),
  series: z.string().default(''),
  friendlyName: z.string().default(''),
  roadNumber: z.string().default(''),
  livery: z.string().default(''),
  depot: z.string().default(''),
  control: z.string().default(''),
  dccInterface: z.string().default(''),
  couplingSocket: z.string().default(''),
  closeCouplers: z.string().default('NOT_APPLICABLE'),
  subType: z.string().default(''),
  flywheelFitted: z.string().default('NOT_APPLICABLE'),
  sprungBuffers: z.string().default('NOT_APPLICABLE'),
  bodyShell: z.string().default(''),
  chassis: z.string().default(''),
  interiorLights: z.string().default('NOT_APPLICABLE'),
  lights: z.string().default('NOT_APPLICABLE'),
  digitalShunting: z.string().default('NOT_APPLICABLE'),
  lengthMm: z.number().nullable().default(null),
  selectedCouplerTypeId: z.string().nullable().default(null),
  isDummy: z.string().default('NOT_APPLICABLE')
});

export type RollingStockCreateFormData = z.infer<typeof rollingStockCreateSchema>;
