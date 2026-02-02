/**
 * Form state types for adding a railway model to the collection.
 * These types represent the UI form state before transformation to backend command args.
 */

/**
 * Form state for adding a railway model to collection.
 * Managed via Svelte 5 $state rune in the drawer component.
 */
export interface AddModelFormState {
  // Railway Model fields
  manufacturerId: string | null;
  productCode: string;
  description: string;
  category: string | null;
  scale: string | null;
  powerMethod: string | null;
  epoch: string | null;

  // Rolling stocks (dynamic list)
  rollingStocks: RollingStockFormEntry[];

  // Purchase info (optional section)
  purchase: PurchaseFormState;
}

/**
 * Form state for a single rolling stock entry.
 * Supports add/remove operations in the UI.
 */
export interface RollingStockFormEntry {
  /** Client-side unique ID for list keying */
  uid: string;

  /** Railway company operating this rolling stock */
  railwayCompanyId: string | null;

  /** Series/class code (e.g., "Re 4/4", "TEE") */
  seriesCode: string;

  /** Rolling stock category (locomotive, passenger_car, etc.) */
  category: string | null;

  /** Road/running number (optional) */
  roadNumber: string;

  /**
   * Locomotive type if category is 'locomotive' (e.g., STEAM, DIESEL, ELECTRIC).
   * Required when category is locomotive; auto-hidden for other categories.
   * Maps to SimplifiedRollingStockArgs.locomotiveType in backend.
   */
  locomotiveType: string | null;
}

/**
 * Form state for optional purchase information.
 * All fields are optional except when the user wants to record a purchase.
 */
export interface PurchaseFormState {
  /** Seller from whom the model was purchased */
  sellerId: string | null;

  /** Purchase price amount (user input as decimal string) */
  priceAmount: string;

  /** Currency code (default: user preference or "EUR") */
  priceCurrency: string;

  /** Purchase condition (NEW, PRE_OWNED) */
  purchaseCondition: string | null;

  /** Physical/mechanical condition of the model */
  modelCondition: string | null;

  /** Condition of the original box */
  boxCondition: string | null;

  /** Free-form notes */
  notes: string;

  /** Date of purchase (YYYY-MM-DD) */
  purchaseDate: string;
}
