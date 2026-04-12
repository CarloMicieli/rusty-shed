import { z } from 'zod';

const featureFlagSchema = z.enum(['YES', 'NO', 'NOT_APPLICABLE']);

export const rollingStockSpecsSchema = z.object({
  category: z.string().default(''),
  railwayCompanyId: z.string().default(''),
  seriesCode: z.string().min(1, 'Required'),
  series: z.string().default(''),
  roadNumber: z.string().default(''),
  friendlyName: z.string().default(''),
  livery: z.string().default(''),
  depot: z.string().default(''),
  flywheelFitted: featureFlagSchema.default('NOT_APPLICABLE'),
  sprungBuffers: featureFlagSchema.default('NOT_APPLICABLE'),
  bodyShell: z.string().default(''),
  chassis: z.string().default(''),
  interiorLights: featureFlagSchema.default('NOT_APPLICABLE'),
  lights: featureFlagSchema.default('NOT_APPLICABLE'),
  dccInterface: z.string().default(''),
  control: z.string().default(''),
  couplingSocket: z.string().default(''),
  closeCouplers: featureFlagSchema.default('NOT_APPLICABLE'),
  digitalShunting: featureFlagSchema.default('NOT_APPLICABLE'),
  selectedCouplerTypeId: z.string().nullable().default(null),
  lengthMm: z.number().nullable().default(null),
  isDummy: featureFlagSchema.default('NOT_APPLICABLE')
});

export type RollingStockSpecsFormData = z.infer<typeof rollingStockSpecsSchema>;
