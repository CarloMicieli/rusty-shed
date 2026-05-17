import * as m from '$lib/paraglide/messages.js';
import { z } from 'zod';

export const quickAddFormSchema = z.object({
  name: z.string().trim().min(1, { message: m.quick_add_name_required() }),
  websiteUrl: z.string().url().optional().or(z.literal('')),
  countryCode: z.string().length(2).optional().or(z.literal(''))
});

export type QuickAddFormValues = z.infer<typeof quickAddFormSchema>;
