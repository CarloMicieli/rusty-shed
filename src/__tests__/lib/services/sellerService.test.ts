import { beforeEach, describe, expect, it, vi } from 'vitest';

const { commandMocks, adapterMocks } = vi.hoisted(() => ({
  commandMocks: {
    getSellers: vi.fn(),
    getSellerById: vi.fn(),
    createSeller: vi.fn(),
    updateSeller: vi.fn(),
    deleteSeller: vi.fn()
  },
  adapterMocks: {
    mapSellerToForm: vi.fn(),
    mapFormToCreatePayload: vi.fn(),
    mapFormToUpdatePayload: vi.fn()
  }
}));

vi.mock('$lib/bindings', () => ({
  commands: commandMocks
}));

vi.mock('$lib/services/sellerAdapter', () => ({
  __esModule: true,
  default: adapterMocks
}));

import {
  getSellers,
  getSellerById,
  createSeller,
  updateSeller,
  deleteSeller
} from '$lib/services/sellerService';

describe('sellerService', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('getSellers', () => {
    it('maps wrapped ok data to form rows', async () => {
      commandMocks.getSellers.mockResolvedValueOnce({
        status: 'ok',
        data: [{ id: 's-1' }, { id: 's-2' }]
      });
      adapterMocks.mapSellerToForm.mockImplementation((row) => ({
        id: row.id,
        name: `seller-${row.id}`
      }));

      const result = await getSellers();

      expect(result).toEqual({
        status: 'ok',
        data: [
          { id: 's-1', name: 'seller-s-1' },
          { id: 's-2', name: 'seller-s-2' }
        ]
      });
    });

    it('returns wrapped command errors and normalizes thrown errors', async () => {
      commandMocks.getSellers.mockResolvedValueOnce({
        status: 'error',
        error: { DatabaseError: 'db down' }
      });

      const commandError = await getSellers();
      expect(commandError).toEqual({
        status: 'error',
        error: { DatabaseError: 'db down' }
      });

      commandMocks.getSellers.mockRejectedValueOnce(new Error('network'));
      const thrown = await getSellers();
      expect(thrown).toEqual({
        status: 'error',
        error: { Unknown: { message: 'network', error_id: 'frontend-unknown' } }
      });
    });
  });

  describe('getSellerById', () => {
    it('returns mapped seller and null when backend returns no row', async () => {
      commandMocks.getSellerById.mockResolvedValueOnce({ id: 's-1' });
      adapterMocks.mapSellerToForm.mockReturnValueOnce({ id: 's-1', name: 'seller-s-1' });

      const found = await getSellerById('s-1');
      expect(found).toEqual({ status: 'ok', data: { id: 's-1', name: 'seller-s-1' } });

      commandMocks.getSellerById.mockResolvedValueOnce(null);
      const missing = await getSellerById('missing');
      expect(missing).toEqual({ status: 'ok', data: null });
    });
  });

  describe('createSeller and updateSeller', () => {
    it('maps create/update payloads and rows', async () => {
      const form = { name: 'Shop', sellerType: 'SHOP' };
      adapterMocks.mapFormToCreatePayload.mockReturnValueOnce({ payload: 'create' });
      commandMocks.createSeller.mockResolvedValueOnce({ id: 'created' });
      adapterMocks.mapSellerToForm.mockReturnValueOnce({ id: 'created', name: 'Created' });

      const created = await createSeller(form as never);
      expect(adapterMocks.mapFormToCreatePayload).toHaveBeenCalledWith(form);
      expect(commandMocks.createSeller).toHaveBeenCalledWith({ payload: 'create' });
      expect(created).toEqual({ status: 'ok', data: { id: 'created', name: 'Created' } });

      adapterMocks.mapFormToUpdatePayload.mockReturnValueOnce({ payload: 'update' });
      commandMocks.updateSeller.mockResolvedValueOnce({ id: 'updated' });
      adapterMocks.mapSellerToForm.mockReturnValueOnce({ id: 'updated', name: 'Updated' });

      const updated = await updateSeller(form as never);
      expect(adapterMocks.mapFormToUpdatePayload).toHaveBeenCalledWith(form);
      expect(commandMocks.updateSeller).toHaveBeenCalledWith({ payload: 'update' });
      expect(updated).toEqual({ status: 'ok', data: { id: 'updated', name: 'Updated' } });
    });

    it('returns normalized errors for create/update failures', async () => {
      commandMocks.createSeller.mockRejectedValueOnce('');
      expect(await createSeller({} as never)).toEqual({
        status: 'error',
        error: { Unknown: { message: 'Unknown error', error_id: 'frontend-unknown' } }
      });

      commandMocks.updateSeller.mockRejectedValueOnce({ NotFound: 'missing' });
      expect(await updateSeller({} as never)).toEqual({
        status: 'error',
        error: { NotFound: 'missing' }
      });
    });
  });

  describe('deleteSeller', () => {
    it('returns ok on success and error on failure', async () => {
      commandMocks.deleteSeller.mockResolvedValueOnce(undefined);
      expect(await deleteSeller('s-1')).toEqual({ status: 'ok', data: null });

      commandMocks.deleteSeller.mockRejectedValueOnce({ PermissionDenied: 'denied' });
      expect(await deleteSeller('s-2')).toEqual({
        status: 'error',
        error: { PermissionDenied: 'denied' }
      });
    });
  });
});
