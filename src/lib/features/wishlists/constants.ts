import type { Category, WishlistPriority } from '$lib/bindings';

export { CATEGORIES, SCALES, POWER_METHODS } from '$lib/utils/enum-options';

/**
 * Static dropdown options for wishlist priorities
 */
export const PRIORITIES: WishlistPriority[] = ['LOW', 'NORMAL', 'HIGH'];

/**
 * MOROP standard epoch values for railway models
 */
export const EPOCHS: string[] = [
  'I',
  'II',
  'IIa',
  'IIb',
  'IIc',
  'III',
  'IIIa',
  'IIIb',
  'IV',
  'IVa',
  'IVb',
  'V',
  'Va',
  'Vb',
  'VI'
];

/**
 * Rolling stock categories (subset of Category — excludes TRAIN_SETS, STARTER_SETS)
 */
export const ROLLING_STOCK_CATEGORIES: Category[] = [
  'LOCOMOTIVES',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];
