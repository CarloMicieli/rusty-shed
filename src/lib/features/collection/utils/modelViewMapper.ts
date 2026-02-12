import type {
  RailwayModelView,
  CollectionItemView,
  RailwayModelImageResponse,
  RollingStockView,
  OwnedRollingStockView
} from '$lib/bindings';

import type { RailwayModel, RollingStock } from '$lib/types/railway-model';

/**
 * Transform RailwayModelView + CollectionItemView + Image to RailwayModel.
 * 
 * This mapper combines data from multiple sources to create the simplified
 * RailwayModel interface expected by RailwayModelCard component.
 * 
 * @param modelView - The railway model view from the backend
 * @param collectionItem - Optional collection item with owned rolling stock details
 * @param imageResponse - Optional image response with image path
 * @returns Simplified RailwayModel for display purposes
 */
export function toRailwayModel(
  modelView: RailwayModelView,
  collectionItem: CollectionItemView | null = null,
  imageResponse: RailwayModelImageResponse | null = null
): RailwayModel {
  // Determine if this is in collection or wishlist
  const status = collectionItem ? 'InCollection' : 'Wishlist';

  // Get image path from response
  const imagePath = imageResponse?.imagePath ?? null;

  // Transform rolling stock
  const rollingStock = collectionItem
    ? transformOwnedRollingStock(collectionItem.rollingStocks, modelView.rollingStock)
    : transformRollingStock(modelView.rollingStock);

  return {
    id: modelView.id,
    manufacturer: modelView.manufacturer.display,
    product_code: modelView.productCode,
    scale: modelView.scale,
    era: modelView.epoch,
    power_method: modelView.powerMethod,
    category: modelView.category,
    description: modelView.details,
    image_path: imagePath,
    status,
    rolling_stock: rollingStock
  };
}

/**
 * Extract numeric ID from railway model TRN ID.
 * 
 * TRN format: trn:railway-model:manufacturer:productCode
 * We need a numeric ID for the component, so we hash the TRN string.
 * 
 * @param trnId - The TRN identifier string
 * @returns A positive 32-bit integer hash of the TRN
 */
function extractNumericId(trnId: string): number {
  // Simple hash function to convert string to number
  let hash = 0;
  for (let i = 0; i < trnId.length; i++) {
    const char = trnId.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // Convert to 32-bit integer
  }
  return Math.abs(hash);
}

/**
 * Transform RollingStockView array to RollingStock array.
 * 
 * Used when we don't have collection item data (e.g., wishlist items).
 * Extracts common fields from the discriminated union and maps them to
 * the simplified RollingStock interface.
 * 
 * @param rollingStockViews - Array of rolling stock views from backend
 * @returns Array of transformed rolling stock for display
 */
function transformRollingStock(rollingStockViews: RollingStockView[]): RollingStock[] {
  return rollingStockViews.map((view, _index) => {
    // Extract common fields from the discriminated union
    const common = extractRollingStockData(view);
    
    return {
      id: common.id,
      railway_model_id: 0, // Not available from view
      series_code: common.series_code,
      series_name: common.series ?? null,
      category: extractCategory(view),
      subcategory: extractSubcategory(view),
      road_number: common.road_number,
      depot: common.depot,
      livery: common.livery,
      control_type: common.control,
      dcc_interface: common.dcc_interface,
      coupling_type: null // Not available from view
    };
  });
}

/**
 * Transform OwnedRollingStockView array with RollingStockView array.
 * 
 * Combines owned data (from collection) with view data (from model)
 * to create complete rolling stock information for display.
 * Matches owned rolling stock with their corresponding views by ID.
 * 
 * @param ownedViews - Array of owned rolling stock from collection
 * @param rollingStockViews - Array of rolling stock views from model
 * @returns Array of transformed rolling stock combining both sources
 */
function transformOwnedRollingStock(
  ownedViews: OwnedRollingStockView[],
  rollingStockViews: RollingStockView[]
): RollingStock[] {
  return ownedViews.map((owned) => {
    // Find matching rolling stock view by ID
    const view = rollingStockViews.find((v) => {
      const viewData = extractRollingStockData(v);
      return viewData.id === owned.id;
    });

    const common = view ? extractRollingStockData(view) : null;
    
    return {
      id: owned.id,
      railway_model_id: 0, // Not used
      series_code: common?.series_code ?? '',
      series_name: common?.series ?? null,
      category: view ? extractCategory(view) : null,
      subcategory: view ? extractSubcategory(view) : null,
      road_number: owned.roadNumber,
      depot: common?.depot ?? null,
      livery: common?.livery ?? null,
      control_type: owned.control,
      dcc_interface: owned.digital?.interface ?? null,
      coupling_type: null // Not available
    };
  });
}

/**
 * Extract common data from RollingStockView discriminated union.
 * 
 * Handles all rolling stock types (locomotive, EMU, railcar, passenger car,
 * freight car) and extracts fields that are common across types.
 * 
 * @param view - The rolling stock view discriminated union
 * @returns Object with common fields (id, series_code, road_number, etc.)
 */
function extractRollingStockData(view: RollingStockView): {
  id: string;
  series_code: string;
  road_number: string | null;
  depot: string | null;
  livery: string | null;
  control: string | null;
  dcc_interface: string | null;
  series: string | null;
} {
  if ('locomotive' in view) {
    return {
      id: view.locomotive.id,
      series_code: view.locomotive.series_code,
      road_number: view.locomotive.road_number,
      depot: view.locomotive.depot,
      livery: view.locomotive.livery,
      control: view.locomotive.control,
      dcc_interface: view.locomotive.dcc_interface,
      series: view.locomotive.series
    };
  } else if ('electricMultipleUnit' in view) {
    return {
      id: view.electricMultipleUnit.id,
      series_code: view.electricMultipleUnit.series_code,
      road_number: view.electricMultipleUnit.road_number,
      depot: view.electricMultipleUnit.depot,
      livery: view.electricMultipleUnit.livery,
      control: view.electricMultipleUnit.control,
      dcc_interface: view.electricMultipleUnit.dcc_interface,
      series: view.electricMultipleUnit.series
    };
  } else if ('railcar' in view) {
    return {
      id: view.railcar.id,
      series_code: view.railcar.series_code,
      road_number: view.railcar.road_number,
      depot: view.railcar.depot,
      livery: view.railcar.livery,
      control: view.railcar.control,
      dcc_interface: view.railcar.dcc_interface,
      series: view.railcar.series
    };
  } else if ('passengerCar' in view) {
    return {
      id: view.passengerCar.id,
      series_code: view.passengerCar.series_code,
      road_number: view.passengerCar.road_number,
      depot: null, // Not available for passenger cars
      livery: view.passengerCar.livery,
      control: null,
      dcc_interface: null,
      series: view.passengerCar.series
    };
  } else if ('freightCar' in view) {
    return {
      id: view.freightCar.id,
      series_code: view.freightCar.series_code,
      road_number: view.freightCar.road_number,
      depot: null, // Not available for freight cars
      livery: view.freightCar.livery,
      control: null,
      dcc_interface: null,
      series: null // Not available for freight cars
    };
  }

  // Fallback for unknown types
  return {
    id: '',
    series_code: '',
    road_number: null,
    depot: null,
    livery: null,
    control: null,
    dcc_interface: null,
    series: null
  };
}

/**
 * Extract category from RollingStockView.
 * 
 * Maps the rolling stock type to a category string.
 * For locomotives, returns the specific locomotive type.
 * 
 * @param view - The rolling stock view discriminated union
 * @returns Category string or null if not available
 */
function extractCategory(view: RollingStockView): string | null {
  if ('locomotive' in view) {
    return view.locomotive.locomotive_type;
  } else if ('electricMultipleUnit' in view) {
    return 'ELECTRIC_MULTIPLE_UNIT';
  } else if ('railcar' in view) {
    return 'RAILCAR';
  } else if ('passengerCar' in view) {
    return view.passengerCar.passenger_car_type;
  } else if ('freightCar' in view) {
    return view.freightCar.freight_car_type;
  }
  return null;
}

/**
 * Extract subcategory from RollingStockView.
 * 
 * Returns the specific subtype for EMUs and railcars.
 * Other rolling stock types don't have subcategories.
 * 
 * @param view - The rolling stock view discriminated union
 * @returns Subcategory string or null if not available
 */
function extractSubcategory(view: RollingStockView): string | null {
  if ('electricMultipleUnit' in view) {
    return view.electricMultipleUnit.electric_multiple_unit_type;
  } else if ('railcar' in view) {
    return view.railcar.railcar_type;
  }
  return null;
}
