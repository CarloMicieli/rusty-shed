import { commands, type CommandError, type Manufacturer, type SellerView } from '$lib/bindings';

type CmdResult<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };

export interface LibraryEntityRow {
  id: string;
  name: string;
  countryCode: string | null;
  usageCount: number;
  isSystemSeeded: boolean;
}

function mapManufacturer(row: Manufacturer): LibraryEntityRow {
  return {
    id: row.id,
    name: row.name,
    countryCode: row.countryCode ?? null,
    usageCount: row.usageCount,
    isSystemSeeded: row.isSystemSeeded
  };
}

function mapSeller(row: SellerView): LibraryEntityRow {
  return {
    id: row.id,
    name: row.name,
    countryCode: row.address?.country ?? null,
    usageCount: row.usageCount,
    isSystemSeeded: row.isSystemSeeded
  };
}

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

export async function getManufacturers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
  try {
    const result: unknown = await commands.getManufacturers();
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
    const rows = Array.isArray(rawRows) ? (rawRows as Manufacturer[]) : [];
    return { status: 'ok', data: rows.map(mapManufacturer) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function getSellers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
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
    return { status: 'ok', data: rows.map(mapSeller) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}

export async function getBuyers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
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
    return { status: 'ok', data: rows.map(mapSeller) };
  } catch (error: unknown) {
    return { status: 'error', error: toCommandError(error) };
  }
}
