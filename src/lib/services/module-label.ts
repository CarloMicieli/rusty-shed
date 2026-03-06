/**
 * Module label derivation utility.
 *
 * Maps a SvelteKit pathname to a human-readable, localised module label
 * using Paraglide message functions. Used by the Signal Failure view to
 * identify which part of the application encountered a fault.
 */

import * as m from '$lib/paraglide/messages.js';

/**
 * Get the localised module label for the given pathname.
 *
 * @param pathname - The current URL pathname (e.g. `/collection`)
 * @returns A localised module label string
 */
export function getModuleLabel(pathname: string): string {
  if (pathname.startsWith('/dashboard')) {
    return m.module_label_yard_overview();
  }
  if (pathname.startsWith('/collection')) {
    return m.module_label_collection_depot();
  }
  if (pathname.startsWith('/wishlist')) {
    return m.module_label_wishlist();
  }
  if (pathname.startsWith('/maintenance')) {
    return m.module_label_maintenance_log();
  }
  if (pathname.startsWith('/finance')) {
    return m.module_label_finance_ledger();
  }
  if (pathname.startsWith('/search')) {
    return m.module_label_global_search();
  }
  if (pathname.startsWith('/settings')) {
    return m.module_label_settings();
  }
  return m.module_label_signal_box();
}
