import { z } from 'zod';
import * as m from '$lib/paraglide/messages.js';

// ---------------------------------------------------------------------------
// Helper: nullable field that is required on submit.
// ---------------------------------------------------------------------------
const nullableRequired = (message: string) =>
  z
    .string()
    .nullable()
    .refine((v): v is string => v !== null && v.trim().length > 0, message);

// ---------------------------------------------------------------------------
// Decoder installation form schema
// ---------------------------------------------------------------------------

export const decoderInstallSchema = z.object({
  selectedRollingStockId: nullableRequired(m.digital_roster_validation_rolling_stock()),
  selectedDecoderId: nullableRequired(m.digital_roster_validation_decoder()),
  dccAddress: z
    .number()
    .nullable()
    .refine((v) => v !== null && v >= 1 && v <= 9999, m.digital_roster_address_range()),
  installationDate: z.string().default('')
});

export type DecoderInstallFormData = z.infer<typeof decoderInstallSchema>;
