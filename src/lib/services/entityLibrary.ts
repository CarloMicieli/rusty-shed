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
    countryCode: row.countryCode,
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

export async function getManufacturers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
  const result = await commands.getManufacturers();
  if (result.status === 'ok') {
    const rows = Array.isArray(result.data) ? result.data : [];
    return { status: 'ok', data: rows.map(mapManufacturer) };
  }
  return result as CmdResult<LibraryEntityRow[], CommandError>;
}

export async function getSellers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
  const result = await commands.getSellers();
  if (result.status === 'ok') {
    const rows = Array.isArray(result.data) ? result.data : [];
    return { status: 'ok', data: rows.map(mapSeller) };
  }
  return result as CmdResult<LibraryEntityRow[], CommandError>;
}

export async function getBuyers(): Promise<CmdResult<LibraryEntityRow[], CommandError>> {
  const result = await commands.getSellers();
  if (result.status === 'ok') {
    const rows = Array.isArray(result.data) ? result.data : [];
    return { status: 'ok', data: rows.map(mapSeller) };
  }
  return result as CmdResult<LibraryEntityRow[], CommandError>;
}
