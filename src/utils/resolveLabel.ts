import * as m from '$lib/paraglide/messages.js';
import type { ConstantItem } from '$lib/types/constant_item';

/**
 * Takes a ConstantItem and returns either the translated string
 * or the static display string.
 */
export function resolveLabel(item: ConstantItem): string {
  if (item.labelKey) {
    // This assumes your labelKey in JSON matches the function name in m
    // e.g., "constants_categories_locomotives"
    return (m as any)[item.labelKey]?.() ?? item.id;
  }
  return item.display ?? item.id;
}
