import type {
  RailwayModelView,
  CollectionItemView,
  RailwayModelImageResponse,
  RollingStockView,
  OwnedRollingStockView,
  LengthOverBuffers,
  DeliveryDate
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
    delivery_date: formatDeliveryDate(modelView.deliveryDate),
    description: modelView.description,
    descriptionLang: modelView.descriptionLang,
    details: modelView.details,
    detailsLang: modelView.detailsLang,
    image_path: imagePath,
    status,
    rolling_stock: rollingStock
  };
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
      railway_company: common.railway_company,
      series_code: common.series_code,
      series_name: common.series ?? null,
      rolling_stock_type: common.rolling_stock_type,
      category: extractCategory(view),
      subcategory: extractSubcategory(view),
      road_number: common.road_number,
      depot: common.depot,
      livery: common.livery,
      length_mm: common.length_mm,
      control_type: common.control,
      dcc_interface: common.dcc_interface,
      coupling_type: common.coupling_socket,
      close_couplers: common.close_couplers,
      digital_shunting: common.digital_shunting
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
    // Match against the catalog rolling stock ID (rollingStockId), not the
    // collecting-domain owned rolling stock ID (id) — these are different types.
    const view = rollingStockViews.find((v) => {
      const viewData = extractRollingStockData(v);
      return viewData.id === owned.rollingStockId;
    });

    const common = view ? extractRollingStockData(view) : null;

    return {
      // Use the catalog rolling stock ID so the backend update command can
      // locate the unit within the railway model aggregate.
      id: owned.rollingStockId,
      railway_model_id: 0, // Not used
      railway_company: common?.railway_company ?? null,
      series_code: common?.series_code ?? '',
      series_name: common?.series ?? null,
      rolling_stock_type: common?.rolling_stock_type ?? null,
      category: view ? extractCategory(view) : null,
      subcategory: view ? extractSubcategory(view) : null,
      road_number: owned.roadNumber,
      depot: common?.depot ?? null,
      livery: common?.livery ?? null,
      length_mm: common?.length_mm ?? null,
      control_type: owned.control,
      dcc_interface: owned.digital?.interface ?? null,
      coupling_type: common?.coupling_socket ?? null,
      close_couplers: common?.close_couplers ?? null,
      digital_shunting: common?.digital_shunting ?? null
    };
  });
}

/**
 * Extract millimeter length from a LengthOverBuffers value.
 */
function extractMm(len: LengthOverBuffers | null): number | null {
  const mm = len?.millimeters;
  if (mm && 'Millimeters' in mm) return Number(mm.Millimeters);
  return null;
}

/**
 * Format a DeliveryDate value to a human-readable string.
 *
 * At runtime, Rust serialises DeliveryDate as a plain string via Display
 * (e.g. "2026", "2026/01", "2026/Q3"), so the fast-path handles that.
 * The tagged-union branches are kept for type completeness.
 */
function formatDeliveryDate(d: DeliveryDate | null): string | null {
  if (!d) return null;
  // Runtime fast-path: Rust serialises DeliveryDate as a Display string.
  if (typeof d === 'string') return d;
  if ('Year' in d) return String(d.Year);
  if ('YearMonth' in d) return `${d.YearMonth.year}/${String(d.YearMonth.month).padStart(2, '0')}`;
  if ('YearQuarter' in d) return `${d.YearQuarter.year}/${d.YearQuarter.quarter}`;
  return null;
}

/**
 * Extract the high-level rolling stock type label from a RollingStockView.
 */
function extractRollingStockType(view: RollingStockView): string | null {
  if ('locomotive' in view) return 'Locomotive';
  if ('electricMultipleUnit' in view) return 'Electric Multiple Unit';
  if ('freightCar' in view) return 'Freight Car';
  if ('passengerCar' in view) return 'Passenger Car';
  if ('railcar' in view) return 'Railcar';
  return null;
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
  rolling_stock_type: string | null;
  railway_company: string;
  series_code: string;
  road_number: string | null;
  depot: string | null;
  livery: string | null;
  length_mm: number | null;
  control: string | null;
  dcc_interface: string | null;
  series: string | null;
  coupling_socket: string | null;
  close_couplers: boolean | null;
  digital_shunting: boolean | null;
} {
  const rolling_stock_type = extractRollingStockType(view);

  const flagToBool = (flag: string | null | undefined): boolean | null => {
    if (flag === 'YES') return true;
    if (flag === 'NO') return false;
    return null;
  };

  if ('locomotive' in view) {
    const ts = view.locomotive.technical_specifications;
    return {
      id: view.locomotive.id,
      rolling_stock_type,
      railway_company: view.locomotive.railway.display,
      series_code: view.locomotive.series_code,
      road_number: view.locomotive.road_number,
      depot: view.locomotive.depot,
      livery: view.locomotive.livery,
      length_mm: extractMm(view.locomotive.length_over_buffer),
      control: view.locomotive.control,
      dcc_interface: view.locomotive.dcc_interface,
      series: view.locomotive.series,
      coupling_socket: ts?.coupling?.socket ?? null,
      close_couplers: flagToBool(ts?.coupling?.close_couplers),
      digital_shunting: flagToBool(ts?.coupling?.digital_shunting)
    };
  } else if ('electricMultipleUnit' in view) {
    const ts = view.electricMultipleUnit.technical_specifications;
    return {
      id: view.electricMultipleUnit.id,
      rolling_stock_type,
      railway_company: view.electricMultipleUnit.railway.display,
      series_code: view.electricMultipleUnit.series_code,
      road_number: view.electricMultipleUnit.road_number,
      depot: view.electricMultipleUnit.depot,
      livery: view.electricMultipleUnit.livery,
      length_mm: extractMm(view.electricMultipleUnit.length_over_buffer),
      control: view.electricMultipleUnit.control,
      dcc_interface: view.electricMultipleUnit.dcc_interface,
      series: view.electricMultipleUnit.series,
      coupling_socket: ts?.coupling?.socket ?? null,
      close_couplers: flagToBool(ts?.coupling?.close_couplers),
      digital_shunting: flagToBool(ts?.coupling?.digital_shunting)
    };
  } else if ('railcar' in view) {
    const ts = view.railcar.technical_specifications;
    return {
      id: view.railcar.id,
      rolling_stock_type,
      railway_company: view.railcar.railway.display,
      series_code: view.railcar.series_code,
      road_number: view.railcar.road_number,
      depot: view.railcar.depot,
      livery: view.railcar.livery,
      length_mm: extractMm(view.railcar.length_over_buffer),
      control: view.railcar.control,
      dcc_interface: view.railcar.dcc_interface,
      series: view.railcar.series,
      coupling_socket: ts?.coupling?.socket ?? null,
      close_couplers: flagToBool(ts?.coupling?.close_couplers),
      digital_shunting: flagToBool(ts?.coupling?.digital_shunting)
    };
  } else if ('passengerCar' in view) {
    const ts = view.passengerCar.technical_specifications;
    return {
      id: view.passengerCar.id,
      rolling_stock_type,
      railway_company: view.passengerCar.railway.display,
      series_code: view.passengerCar.series_code,
      road_number: view.passengerCar.road_number,
      depot: null,
      livery: view.passengerCar.livery,
      length_mm: extractMm(view.passengerCar.length_over_buffer),
      control: null,
      dcc_interface: null,
      series: view.passengerCar.series,
      coupling_socket: ts?.coupling?.socket ?? null,
      close_couplers: flagToBool(ts?.coupling?.close_couplers),
      digital_shunting: flagToBool(ts?.coupling?.digital_shunting)
    };
  } else if ('freightCar' in view) {
    const ts = view.freightCar.technical_specifications;
    return {
      id: view.freightCar.id,
      rolling_stock_type,
      railway_company: view.freightCar.railway.display,
      series_code: view.freightCar.series_code,
      road_number: view.freightCar.road_number,
      depot: null,
      livery: view.freightCar.livery,
      length_mm: extractMm(view.freightCar.length_over_buffer),
      control: null,
      dcc_interface: null,
      series: null,
      coupling_socket: ts?.coupling?.socket ?? null,
      close_couplers: flagToBool(ts?.coupling?.close_couplers),
      digital_shunting: flagToBool(ts?.coupling?.digital_shunting)
    };
  }

  // Fallback for unknown types
  return {
    id: '',
    rolling_stock_type: null,
    railway_company: '',
    series_code: '',
    road_number: null,
    depot: null,
    livery: null,
    length_mm: null,
    control: null,
    dcc_interface: null,
    series: null,
    coupling_socket: null,
    close_couplers: null,
    digital_shunting: null
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
