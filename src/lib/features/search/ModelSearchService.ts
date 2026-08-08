/**
 * Model search helpers.
 *
 * Encapsulates the two-step IPC sequence used by the header SearchBar
 * (search by query → resolve full model views) so the component does not
 * depend on `commands.*` directly.
 */

import { commands, type Language, type RailwayModelView } from '$lib/bindings';

/**
 * Search for railway model IDs matching `query`.
 *
 * @returns An array of matching `RailwayModelId` strings, or an empty array
 *          on error.
 */
export async function searchRailwayModelIds(query: string): Promise<string[]> {
  const result = await commands.searchRailwayModels({ query });
  if (result.status !== 'ok') return [];
  return result.data;
}

/**
 * Resolve a list of railway model IDs to full `RailwayModelView` objects.
 *
 * Filters out IDs for which the backend returned no data.
 *
 * @param ids  - Array of `RailwayModelId` strings to resolve.
 * @param lang - Language to use for localised text fields.
 */
export async function resolveRailwayModels(
  ids: string[],
  lang: Language
): Promise<RailwayModelView[]> {
  const results = await Promise.all(ids.map((id) => commands.getRailwayModelById(id, lang)));
  return results
    .filter((r) => r.status === 'ok' && r.data != null)
    .map((r) => (r as { status: 'ok'; data: RailwayModelView }).data);
}

/**
 * Fetch the image path for a railway model.
 *
 * @param modelId - The `RailwayModelId` to look up.
 * @returns The resolved image path string, or `null` if the model has no image.
 */
export async function fetchRailwayModelImagePath(modelId: string): Promise<string | null> {
  const result = await commands.getRailwayModelImage(modelId);
  if (result.status !== 'ok') return null;
  const { hasImage, imagePath } = result.data;
  return hasImage && imagePath ? imagePath : null;
}
