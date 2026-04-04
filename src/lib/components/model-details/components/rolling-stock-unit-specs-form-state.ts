/** Form slice used by the rolling-stock tab spec grid (mirrors editor state). */
export interface RollingStockUnitSpecsFormState {
  seriesCode: string;
  roadNumber: string;
  livery: string;
  depot: string;
  control: string;
  dccInterface: string;
  couplingSocket: string;
  closeCouplers: boolean | null;
  digitalShunting: boolean | null;
  category: string | null;
  subcategory: string | null;
  serviceLevel: string | null;
  subcategoryFlashed: boolean;
}
