/**
 * Simplified Railway Model type for the RailwayModelCard component.
 * This is a flattened view optimized for display purposes.
 *
 * TODO: Map from RailwayModelView (bindings.ts) to this simplified structure
 */
export interface RailwayModel {
  id: number;
  manufacturer: string;
  product_code: string;
  scale: string;
  era: string | null;
  power_method: string | null;
  category: string | null;
  description: string | null;
  image_path: string | null;
  status: 'InCollection' | 'Wishlist';
  rolling_stock: RollingStock[];
}

export interface RollingStock {
  id: number;
  railway_model_id: number;
  series_code: string;
  series_name: string | null;
  category: string | null;
  subcategory: string | null;
  road_number: string | null;
  depot: string | null;
  livery: string | null;
  control_type: string | null;
  dcc_interface: string | null;
  coupling_type: string | null;
}
