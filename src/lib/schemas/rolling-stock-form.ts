import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';
import {
  featureFlagSchema,
  controlSchema,
  dccInterfaceSchema,
  couplingSocketSchema,
  bodyShellChassisTypeSchema
} from './railway-model';

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
  control: z.union([controlSchema, z.literal('')]).default(''),
  dccInterface: z.union([dccInterfaceSchema, z.literal('')]).default(''),
  couplingSocket: z.union([couplingSocketSchema, z.literal('')]).default(''),
  closeCouplers: featureFlagSchema.default('NOT_APPLICABLE'),
  subType: z.string().default(''),
  flywheelFitted: featureFlagSchema.default('NOT_APPLICABLE'),
  sprungBuffers: featureFlagSchema.default('NOT_APPLICABLE'),
  bodyShell: z.union([bodyShellChassisTypeSchema, z.literal('')]).default(''),
  chassis: z.union([bodyShellChassisTypeSchema, z.literal('')]).default(''),
  interiorLights: featureFlagSchema.default('NOT_APPLICABLE'),
  lights: featureFlagSchema.default('NOT_APPLICABLE'),
  digitalShunting: featureFlagSchema.default('NOT_APPLICABLE'),
  lengthMm: z.number().nullable().default(null),
  selectedCouplerTypeId: z.string().nullable().default(null),
  isDummy: featureFlagSchema.default('NOT_APPLICABLE')
});

export type RollingStockCreateFormData = z.infer<typeof rollingStockCreateSchema>;
