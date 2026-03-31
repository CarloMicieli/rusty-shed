import { commands } from '$lib/bindings';
import adapter, { type FormSeller } from './sellerAdapter';
import type { CommandError } from '$lib/bindings';

type CmdResult<T, E> = { status: 'ok'; data: T } | { status: 'error'; error: E };

export async function getSellers(): Promise<CmdResult<FormSeller[], CommandError>> {
  const res = await commands.getSellers();
  if (res.status === 'ok') {
    return { status: 'ok', data: res.data.map(adapter.mapSellerToForm) };
  }
  return res as unknown as CmdResult<FormSeller[], CommandError>;
}

export async function getSellerById(
  id: string
): Promise<CmdResult<FormSeller | null, CommandError>> {
  const res = await commands.getSellerById(id);
  if (res.status === 'ok') {
    return { status: 'ok', data: res.data ? adapter.mapSellerToForm(res.data) : null };
  }
  return res as unknown as CmdResult<FormSeller | null, CommandError>;
}

export async function createSeller(form: FormSeller): Promise<CmdResult<FormSeller, CommandError>> {
  const payload = adapter.mapFormToCreatePayload(form);
  const res = await commands.createSeller(payload);
  if (res.status === 'ok') {
    return { status: 'ok', data: adapter.mapSellerToForm(res.data) };
  }
  return res as unknown as CmdResult<FormSeller, CommandError>;
}

export async function updateSeller(form: FormSeller): Promise<CmdResult<FormSeller, CommandError>> {
  const payload = adapter.mapFormToUpdatePayload(form);
  const res = await commands.updateSeller(payload);
  if (res.status === 'ok') {
    return { status: 'ok', data: adapter.mapSellerToForm(res.data) };
  }
  return res as unknown as CmdResult<FormSeller, CommandError>;
}

export async function deleteSeller(id: string): Promise<CmdResult<null, CommandError>> {
  return await commands.deleteSeller(id);
}

export default {
  getSellers,
  getSellerById,
  createSeller,
  updateSeller,
  deleteSeller
};
