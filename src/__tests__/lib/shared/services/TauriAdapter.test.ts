import { describe, it, expect, vi, beforeEach } from 'vitest';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

import { invoke } from '@tauri-apps/api/core';
import { safeInvoke, invokeOrThrow, safeInvokeWithRetry } from '$lib/shared/services/TauriAdapter';

const mockInvoke = vi.mocked(invoke);

// ─── tests ────────────────────────────────────────────────────────────────

describe('TauriAdapter', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('safeInvoke', () => {
    it('returns ok:true with data on success', async () => {
      const data = { id: 'w1', name: 'Default' };
      mockInvoke.mockResolvedValueOnce(data);

      const result = await safeInvoke<typeof data>('get_wishlists');

      expect(result.ok).toBe(true);
      if (result.ok) {
        expect(result.data).toEqual(data);
      }
    });

    it('passes command and args to invoke', async () => {
      mockInvoke.mockResolvedValueOnce(null);

      await safeInvoke('get_wishlist_by_id', { id: '123' });

      expect(mockInvoke).toHaveBeenCalledWith('get_wishlist_by_id', { id: '123' });
    });

    it('passes empty args when args not provided', async () => {
      mockInvoke.mockResolvedValueOnce(null);

      await safeInvoke('get_collection');

      expect(mockInvoke).toHaveBeenCalledWith('get_collection', {});
    });

    it('normalizes DatabaseError', async () => {
      mockInvoke.mockRejectedValueOnce({ DatabaseError: 'Constraint violated' });

      const result = await safeInvoke('some_command');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('database');
        expect(result.error.message).toBe('Constraint violated');
        expect(result.error.retryable).toBe(true);
      }
    });

    it('normalizes NotFound error', async () => {
      mockInvoke.mockRejectedValueOnce({ NotFound: 'Item not found' });

      const result = await safeInvoke('get_item');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('not_found');
        expect(result.error.message).toBe('Item not found');
        expect(result.error.retryable).toBe(false);
      }
    });

    it('normalizes ValidationError with structured fields', async () => {
      mockInvoke.mockRejectedValueOnce({
        ValidationError: {
          name: [
            {
              code: 'required',
              message: 'Name is required',
              params: {}
            }
          ],
          amount: [
            {
              code: 'invalid',
              message: null,
              params: {}
            }
          ],
          _general: [{ code: 'error_budget_state_invalid', message: null, params: {} }]
        }
      });

      const result = await safeInvoke('create_item');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('validation');
        expect(result.error.fields).toEqual({
          name: 'Name is required',
          amount: 'invalid',
          _general: 'error_budget_state_invalid'
        });
        expect(result.error.retryable).toBe(false);
      }
    });

    it('normalizes PermissionDenied error', async () => {
      mockInvoke.mockRejectedValueOnce({ PermissionDenied: 'Access denied' });

      const result = await safeInvoke('restricted_cmd');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('permission_denied');
        expect(result.error.retryable).toBe(false);
      }
    });

    it('normalizes Unknown error string', async () => {
      mockInvoke.mockRejectedValueOnce({ Unknown: 'Something went wrong' });

      const result = await safeInvoke('cmd');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('unknown');
        expect(result.error.message).toBe('Something went wrong');
        expect(result.error.retryable).toBe(false);
      }
    });

    it('normalizes Unknown error object', async () => {
      mockInvoke.mockRejectedValueOnce({
        Unknown: {
          message: 'Internal failure',
          error_id: 'ERR-1234-A'
        }
      });

      const result = await safeInvoke('cmd');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('unknown');
        expect(result.error.message).toBe('Internal failure');
        expect(result.error.errorId).toBe('ERR-1234-A');
        expect(result.error.retryable).toBe(false);
      }
    });

    it('normalizes JavaScript Error objects', async () => {
      mockInvoke.mockRejectedValueOnce(new Error('JS error'));

      const result = await safeInvoke('cmd');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('unknown');
        expect(result.error.message).toBe('JS error');
      }
    });

    it('normalizes unknown primitives', async () => {
      mockInvoke.mockRejectedValueOnce('plain string error');

      const result = await safeInvoke('cmd');

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('unknown');
      }
    });
  });

  describe('invokeOrThrow', () => {
    it('returns data on success', async () => {
      const data = [{ id: 'item1' }];
      mockInvoke.mockResolvedValueOnce(data);

      const result = await invokeOrThrow('list_items');
      expect(result).toEqual(data);
    });

    it('throws NormalizedError on failure', async () => {
      mockInvoke.mockRejectedValueOnce({ NotFound: 'Entity not found' });

      await expect(invokeOrThrow('get_item')).rejects.toMatchObject({
        kind: 'not_found',
        message: 'Entity not found'
      });
    });
  });

  describe('safeInvokeWithRetry', () => {
    it('returns immediately on success without retrying', async () => {
      const data = { id: '1' };
      mockInvoke.mockResolvedValueOnce(data);

      const result = await safeInvokeWithRetry('cmd', {}, 3);

      expect(result.ok).toBe(true);
      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });

    it('does not retry non-retryable errors', async () => {
      mockInvoke.mockRejectedValue({ NotFound: 'Not found' }); // NotFound is retryable=false

      const result = await safeInvokeWithRetry('cmd', {}, 3, 0);

      expect(result.ok).toBe(false);
      expect(mockInvoke).toHaveBeenCalledTimes(1); // No retry
    });

    it('retries on retryable DatabaseError', async () => {
      // First 2 fail, 3rd succeeds
      mockInvoke
        .mockRejectedValueOnce({ DatabaseError: 'busy' })
        .mockRejectedValueOnce({ DatabaseError: 'busy' })
        .mockResolvedValueOnce({ id: '1' });

      const result = await safeInvokeWithRetry('cmd', {}, 3, 0);

      expect(result.ok).toBe(true);
      expect(mockInvoke).toHaveBeenCalledTimes(3);
    });

    it('returns last error after exhausting retries', async () => {
      mockInvoke.mockRejectedValue({ DatabaseError: 'persistent error' });

      const result = await safeInvokeWithRetry('cmd', {}, 2, 0);

      expect(result.ok).toBe(false);
      if (!result.ok) {
        expect(result.error.kind).toBe('database');
      }
      expect(mockInvoke).toHaveBeenCalledTimes(3); // 1 initial + 2 retries
    });
  });
});
