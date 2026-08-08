/**
 * Rolling stock update service.
 *
 * Wraps the `commands.updateRollingStock*` and `commands.getRailwayModelById`
 * Tauri commands so that UI components do not depend directly on the IPC layer.
 */

import {
  commands,
  type Language,
  type RailwayModelId,
  type RailwayModelView,
  type UpdateRollingStockIdentificationArgs,
  type UpdateRollingStockCategoryArgs,
  type UpdateRollingStockSubcategoryArgs,
  type UpdateRollingStockDccArgs,
  type UpdateRollingStockSpecificationsArgs
} from '$lib/bindings';

export type {
  RailwayModelView,
  UpdateRollingStockIdentificationArgs,
  UpdateRollingStockCategoryArgs,
  UpdateRollingStockSubcategoryArgs,
  UpdateRollingStockDccArgs,
  UpdateRollingStockSpecificationsArgs
};

/**
 * Fetch a complete `RailwayModelView` by ID.
 *
 * Used by rolling-stock cards to lazily load technical specifications.
 */
export async function fetchRailwayModelById(
  railwayModelId: RailwayModelId,
  lang: Language
): Promise<RailwayModelView | null> {
  const result = await commands.getRailwayModelById(railwayModelId, lang);
  if (result.status !== 'ok' || !result.data) return null;
  return result.data;
}

/** Update identification fields (series, road number, livery, depot) on a rolling stock item. */
export async function updateRollingStockIdentification(
  args: UpdateRollingStockIdentificationArgs
): Promise<boolean> {
  const result = await commands.updateRollingStockIdentification(args);
  return result.status === 'ok';
}

/** Update the category of a rolling stock item. */
export async function updateRollingStockCategory(
  args: UpdateRollingStockCategoryArgs
): Promise<boolean> {
  const result = await commands.updateRollingStockCategory(args);
  return result.status === 'ok';
}

/** Update the sub-category of a rolling stock item. */
export async function updateRollingStockSubcategory(
  args: UpdateRollingStockSubcategoryArgs
): Promise<boolean> {
  const result = await commands.updateRollingStockSubcategory(args);
  return result.status === 'ok';
}

/** Update DCC / control / length fields on a rolling stock item. */
export async function updateRollingStockDcc(
  args: UpdateRollingStockDccArgs
): Promise<boolean> {
  const result = await commands.updateRollingStockDcc(args);
  return result.status === 'ok';
}

/** Persist all technical specification fields for a rolling stock item atomically. */
export async function updateRollingStockSpecifications(
  args: UpdateRollingStockSpecificationsArgs
): Promise<boolean> {
  const result = await commands.updateRollingStockSpecifications(args);
  return result.status === 'ok';
}
