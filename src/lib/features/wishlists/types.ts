import type { Category, Scale, PowerMethod, WishlistPriority } from '$lib/bindings';
import type { RollingStockCategory } from './constants';

/**
 * Form state for rolling stock entry within the railway model form
 */
export type RollingStockFormEntry = {
  /** Unique key for Svelte each block */
  id: string;
  /** Railway company ID from dropdown */
  railwayCompanyId: string;
  /** Series code (user input) */
  seriesCode: string;
  /** Rolling stock category */
  category: RollingStockCategory | '';
  /** Road number (optional) */
  roadNumber: string;
};

/**
 * Complete form state for adding a railway model to a wishlist
 */
export type AddRailwayModelFormState = {
  // Wishlist selection
  wishlistId: string;

  // Railway model fields
  manufacturerId: string;
  productCode: string;
  description: string;
  category: Category | '';
  scale: Scale | '';
  powerMethod: PowerMethod | '';
  epoch: string | null;

  // Wishlist item fields
  desiredPriceAmount: string;
  desiredPriceCurrency: string;
  priority: WishlistPriority;
  notes: string;

  // Rolling stocks
  rollingStocks: RollingStockFormEntry[];
};
