/**
 * Pure traction evaluation functions.
 *
 * These mirror the Rust domain logic so the UI can compute traction state
 * without waiting for a round-trip to the backend.
 */

import type { FormationElementView } from '$lib/bindings';

/**
 * Returns `true` if this element counts as an effective traction source.
 *
 * Logic mirrors `FormationElementView.is_traction_slot` from the backend:
 * - prototype must be motorized (`is_motorized = true`)
 * - `traction_override !== -1` (not force-excluded)
 * - If `default_is_dummy`, it only counts when `traction_override === 1`
 * - A non-motorized unit with `traction_override === 1` counts as traction
 */
export function isTractionSlot(element: FormationElementView): boolean {
  const { prototype, traction_override } = element;

  // Force-include: any unit with override = 1 counts as traction
  if (traction_override === 1) return true;

  // Force-exclude
  if (traction_override === -1) return false;

  // Default: motorized AND not a dummy
  return prototype.is_motorized && !prototype.default_is_dummy;
}

/**
 * Returns `true` if the formation has at least one effective traction slot.
 */
export function hasTraction(elements: FormationElementView[]): boolean {
  return elements.some(isTractionSlot);
}
