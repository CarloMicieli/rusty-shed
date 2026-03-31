/**
 * Formations service — typed wrappers around the tauri-specta bindings.
 *
 * Uses a `safeCmd` helper to convert tauri-specta `Result<T, CommandError>`
 * to the application `SafeResult<T>` (`{ ok: true, data } | { ok: false, error }`).
 */

import { commands } from '$lib/bindings';
import type {
  CommandError,
  TrainFormationSummary,
  TrainFormationDetail,
  TrainFormationView,
  FormationElementView,
  PrototypeGroupView,
  PrototypeView,
  FormationCategoryView,
  CreateTrainFormationArgs,
  UpdateTrainFormationArgs,
  AddFormationElementArgs,
  ReorderFormationElementsArgs,
  AssignRollingStockToElementArgs,
  SetTractionOverrideArgs,
  CreateCustomPrototypeArgs,
  CreateFormationCategoryArgs
} from '$lib/bindings';
import type { NormalizedError, SafeResult } from '$lib/services';

type CommandResult<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };

function fromCommandError(err: CommandError): NormalizedError {
  if ('DatabaseError' in err && err.DatabaseError)
    return { kind: 'database', message: err.DatabaseError };
  if ('NotFound' in err && err.NotFound) return { kind: 'not_found', message: err.NotFound };
  if ('ValidationError' in err) return { kind: 'validation', message: 'Validation failed' };
  if ('PermissionDenied' in err && err.PermissionDenied)
    return { kind: 'permission_denied', message: err.PermissionDenied };
  if ('Conflict' in err && err.Conflict) return { kind: 'unknown', message: err.Conflict };
  if ('BusinessRule' in err && err.BusinessRule)
    return { kind: 'unknown', message: err.BusinessRule };
  if ('Unknown' in err && err.Unknown)
    return { kind: 'unknown', message: err.Unknown.message ?? 'Unknown error' };
  return { kind: 'unknown', message: 'Unknown error' };
}

async function safeCmd<T>(
  fn: () => Promise<CommandResult<T, CommandError>>
): Promise<SafeResult<T>> {
  try {
    const result = await fn();
    if (result.status === 'ok') return { ok: true, data: result.data };
    return { ok: false, error: fromCommandError(result.error) };
  } catch (e) {
    return {
      ok: false,
      error: { kind: 'unknown', message: e instanceof Error ? e.message : String(e) }
    };
  }
}

// ── Train Formation commands ──────────────────────────────────────────────────

export async function getTrainFormations(): Promise<SafeResult<TrainFormationSummary[]>> {
  return safeCmd(() => commands.getTrainFormations());
}

export async function getTrainFormation(id: string): Promise<SafeResult<TrainFormationDetail>> {
  return safeCmd(() => commands.getTrainFormation(id));
}

export async function createTrainFormation(
  args: CreateTrainFormationArgs
): Promise<SafeResult<TrainFormationView>> {
  return safeCmd(() => commands.createTrainFormation(args));
}

export async function updateTrainFormation(
  id: string,
  args: UpdateTrainFormationArgs
): Promise<SafeResult<TrainFormationView>> {
  return safeCmd(() => commands.updateTrainFormation(id, args));
}

export async function deleteTrainFormation(id: string): Promise<SafeResult<null>> {
  return safeCmd(() => commands.deleteTrainFormation(id));
}

// ── Formation element commands ────────────────────────────────────────────────

export async function addFormationElement(
  formationId: string,
  args: AddFormationElementArgs
): Promise<SafeResult<FormationElementView>> {
  return safeCmd(() => commands.addFormationElement(formationId, args));
}

export async function removeFormationElement(elementId: string): Promise<SafeResult<null>> {
  return safeCmd(() => commands.removeFormationElement(elementId));
}

export async function reorderFormationElements(
  formationId: string,
  args: ReorderFormationElementsArgs
): Promise<SafeResult<TrainFormationDetail>> {
  return safeCmd(() => commands.reorderFormationElements(formationId, args));
}

export async function assignRollingStockToElement(
  elementId: string,
  args: AssignRollingStockToElementArgs
): Promise<SafeResult<FormationElementView>> {
  return safeCmd(() => commands.assignRollingStockToElement(elementId, args));
}

export async function setTractionOverride(
  elementId: string,
  args: SetTractionOverrideArgs
): Promise<SafeResult<FormationElementView>> {
  return safeCmd(() => commands.setTractionOverride(elementId, args));
}

// ── Prototype commands ────────────────────────────────────────────────────────

export async function getPrototypes(
  query: string | null
): Promise<SafeResult<PrototypeGroupView[]>> {
  return safeCmd(() => commands.getPrototypes(query));
}

export async function createCustomPrototype(
  args: CreateCustomPrototypeArgs
): Promise<SafeResult<PrototypeView>> {
  return safeCmd(() => commands.createCustomPrototype(args));
}

// ── Formation category commands ───────────────────────────────────────────────

export async function getFormationCategories(): Promise<SafeResult<FormationCategoryView[]>> {
  return safeCmd(() => commands.getFormationCategories());
}

export async function createFormationCategory(
  args: CreateFormationCategoryArgs
): Promise<SafeResult<FormationCategoryView>> {
  return safeCmd(() => commands.createFormationCategory(args));
}
