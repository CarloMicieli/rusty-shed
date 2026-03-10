/**
 * Simplified Railway Model type for the RailwayModelCard component.
 * This is a flattened view optimized for display purposes.
 *
 * TODO: Map from RailwayModelView (bindings.ts) to this simplified structure
 */
import type { Language } from '$lib/bindings';

export interface RailwayModel {
  id: string;
  manufacturer: string;
  product_code: string;
  scale: string;
  era: string | null;
  power_method: string | null;
  category: string | null;
  delivery_date: string | null;
  description: string | null;
  descriptionLang: Language;
  details: string | null;
  detailsLang: Language | null;
  image_path: string | null;
  status: 'InCollection' | 'Wishlist';
  rolling_stock: RollingStock[];
}

export interface RollingStock {
  id: string;
  railway_model_id: number;
  railway_company: string | null;
  series_code: string;
  series_name: string | null;
  rolling_stock_type: string | null;
  category: string | null;
  subcategory: string | null;
  road_number: string | null;
  depot: string | null;
  livery: string | null;
  length_mm: number | null;
  control_type: string | null;
  dcc_interface: string | null;
  coupling_type: string | null;
  close_couplers: boolean | null;
  digital_shunting: boolean | null;
}
