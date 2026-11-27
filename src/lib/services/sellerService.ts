import { commands } from '$lib/bindings';
import adapter, { type FormSeller } from './sellerAdapter';
import type { CommandError, Seller, SellerView } from '$lib/bindings';

type CmdResult<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };

function toCommandError(error: unknown): CommandError {
  if (error !== null && typeof error === 'object') {
    if ('DatabaseError' in error && typeof error.DatabaseError === 'string')
      return error as CommandError;
    if ('NotFound' in error && typeof error.NotFound === 'string') return error as CommandError;
    if ('ValidationError' in error && typeof error.ValidationError === 'object')
      return error as CommandError;
    if ('PermissionDenied' in error && typeof error.PermissionDenied === 'string')
      return error as CommandError;
    if ('Unknown' in error && error.Unknown !== null && typeof error.Unknown === 'object') {
      const unknown = error.Unknown as { message?: unknown; error_id?: unknown };
      if (typeof unknown.message === 'string' && typeof unknown.error_id === 'string') {
        return error as CommandError;
      }
    }
    if ('BusinessRule' in error && typeof error.BusinessRule === 'string')
      return error as CommandError;
    if ('Conflict' in error && typeof error.Conflict === 'string') return error as CommandError;
  }

  const message =
    error instanceof Error
      ? error.message.length > 0
        ? error.message
        : 'Unknown error'
      : 'Unknown error';
  return { Unknown: { message, error_id: 'frontend-unknown' } };
}

export async function getSellers(): Promise<CmdResult<FormSeller[], CommandError>> {
  try {
    const result: unknown = await commands.getSellers();
    if (
      result !== null &&
      typeof result === 'object' &&
      'status' in result &&
      result.status === 'error' &&
      'error' in result
    ) {
      return { status: 'error', error: toCommandError(result.error) };
    }
    const rawRows =
      result !== null &&
      typeof result === 'object' &&
      'status' in result &&
      result.status === 'ok' &&
      'data' in result
        ? result.data
        : result;
    const rows = Array.isArray(rawRows) ? (rawRows as SellerView[]) : [];
    return { status: 'ok', data: rows.map((row) => adapter.mapSellerToForm(row)) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function getSellerById(
  id: string
): Promise<CmdResult<FormSeller | null, CommandError>> {
  try {
    const row: unknown = await commands.getSellerById(id);
    return {
      status: 'ok',
      data: row ? adapter.mapSellerToForm(row as SellerView | Seller) : null
    };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function createSeller(form: FormSeller): Promise<CmdResult<FormSeller, CommandError>> {
  const payload = adapter.mapFormToCreatePayload(form);
  try {
    const row: unknown = await commands.createSeller(payload);
    return { status: 'ok', data: adapter.mapSellerToForm(row as Seller) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function updateSeller(form: FormSeller): Promise<CmdResult<FormSeller, CommandError>> {
  const payload = adapter.mapFormToUpdatePayload(form);
  try {
    const row: unknown = await commands.updateSeller(payload);
    return { status: 'ok', data: adapter.mapSellerToForm(row as Seller) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function deleteSeller(id: string): Promise<CmdResult<null, CommandError>> {
  try {
    await commands.deleteSeller(id);
    return { status: 'ok', data: null };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export default {
  getSellers,
  getSellerById,
  createSeller,
  updateSeller,
  deleteSeller
};
