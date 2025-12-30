import * as m from '$lib/paraglide/messages.js';
import type { ConstantItem } from '$lib/types/constant_item';

/**
 * Takes a ConstantItem and returns either the translated string
 * or the static display string.
 */
export function resolveLabel(item: ConstantItem): string {
  if (item.labelKey) {
    const fn = m[item.labelKey as keyof typeof m];
    if (typeof fn === 'function') {
      return (fn as () => string)();
    }
    return item.id;
  }
  return item.display ?? item.id;
}
