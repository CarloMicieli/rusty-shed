import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';

// ---------------------------------------------------------------------------
// Schema for the Log Maintenance drawer.
// selectedLocoId starts as null (no loco selected yet) so uses nullable+refine.
// datePerformed has a default (today) so uses plain string.min(1).
// ---------------------------------------------------------------------------

export const maintenanceSchema = z.object({
  selectedLocoId: z
    .string()
    .nullable()
    .refine((v): v is string => v !== null && v.trim().length > 0, m.error_required()),
  datePerformed: z.string().min(1, m.error_required()),
  maintenanceType: z.string().nullable().default(null),
  notes: z.string().default(''),
  initialCondition: z.string().default(''),
  lastRunDate: z.string().default(''),
  serviceInterval: z.string().default('')
});

export type MaintenanceFormData = z.infer<typeof maintenanceSchema>;
