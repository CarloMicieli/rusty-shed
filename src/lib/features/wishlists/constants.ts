import type { Category, Scale, PowerMethod, WishlistPriority } from '$lib/bindings';

/**
 * Static dropdown options for railway model categories
 */
export const CATEGORIES: Category[] = [
  'LOCOMOTIVES',
  'TRAIN_SETS',
  'STARTER_SETS',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];

/**
 * Static dropdown options for railway model scales
 */
export const SCALES: Scale[] = [
  'H0',
  'H0m',
  'H0e',
  'N',
  'TT',
  'Z',
  'G',
  'Scale1',
  'Scale0',
  'Scale00'
];

/**
 * Static dropdown options for power methods
 */
export const POWER_METHODS: PowerMethod[] = ['AC', 'DC', 'TRIX_EXPRESS'];

/**
 * Static dropdown options for wishlist priorities
 */
export const PRIORITIES: WishlistPriority[] = ['LOW', 'NORMAL', 'HIGH'];

/**
 * Rolling stock categories (subset of Category enum)
 */
export type RollingStockCategory =
  | 'LOCOMOTIVES'
  | 'FREIGHT_CARS'
  | 'PASSENGER_CARS'
  | 'ELECTRIC_MULTIPLE_UNITS'
  | 'RAILCARS';

/**
 * Static dropdown options for rolling stock categories
 */
export const ROLLING_STOCK_CATEGORIES: RollingStockCategory[] = [
  'LOCOMOTIVES',
  'FREIGHT_CARS',
  'PASSENGER_CARS',
  'ELECTRIC_MULTIPLE_UNITS',
  'RAILCARS'
];
