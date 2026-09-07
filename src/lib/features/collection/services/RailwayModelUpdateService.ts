/**
 * Railway model update service.
 *
 * Wraps the individual `commands.updateRailwayModel*` Tauri commands so that
 * UI components do not depend directly on the IPC layer.
 */

import {
  commands,
  type Language,
  type Category,
  type Scale,
  type UpdateRailwayModelClassificationArgs,
  type UpdateRailwayModelDeliveryDateArgs,
  type UpdateRailwayModelTextArgs,
  type RailwayModelId
} from '$lib/bindings';

export type { RailwayModelId };

/** Update the scale, epoch, or category of a railway model. */
export async function updateRailwayModelClassification(
  args: UpdateRailwayModelClassificationArgs
): Promise<boolean> {
  const result = await commands.updateRailwayModelClassification(args);
  return result.status === 'ok';
}

/** Update the delivery date of a railway model. */
export async function updateRailwayModelDeliveryDate(
  args: UpdateRailwayModelDeliveryDateArgs
): Promise<boolean> {
  const result = await commands.updateRailwayModelDeliveryDate(args);
  return result.status === 'ok';
}

/** Update a text field (e.g. description) on a railway model. */
export async function updateRailwayModelText(args: UpdateRailwayModelTextArgs): Promise<boolean> {
  const result = await commands.updateRailwayModelText(args);
  return result.status === 'ok';
}

export type { Category, Scale, Language, UpdateRailwayModelClassificationArgs };
