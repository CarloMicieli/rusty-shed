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
   * Subcategory for the rolling stock (e.g., STEAM_LOCOMOTIVE, BAGGAGE_CAR, TANK_CARS).
   * Available options depend on the selected category.
   * Maps to SimplifiedRollingStockArgs.subcategory in backend.
   */
  subcategory: string | null;
}

/**
 * Form state for optional purchase information.
 * All fields are optional except when the user wants to record a purchase.
 */
export interface PurchaseFormState {
  /** Seller from whom the model was purchased */
  sellerId: string | null;

  /** Purchase price amount (integer cents) */
  priceAmount: number | null;

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

  /** Date of purchase or preorder placement (YYYY-MM-DD) */
  purchaseDate: string;

  /** Purchase type: "STANDARD" (default) or "PREORDER" */
  purchaseType: 'STANDARD' | 'PREORDER';

  /** Deposit amount paid for preorders (integer cents) */
  depositAmount: number | null;

  /** Currency code for the deposit */
  depositCurrency: string | null;

  /** Total preorder price in integer cents */
  preorderTotalAmount: number | null;

  /** Currency code for the total preorder price */
  preorderTotalCurrency: string | null;

  /** Expected delivery date for preorders (YYYY-MM-DD) */
  expectedDate: string | null;
}
