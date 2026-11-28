import { beforeEach, describe, expect, it, vi } from 'vitest';

const { mockCommands } = vi.hoisted(() => ({
  mockCommands: {
    getManufacturers: vi.fn(),
    getSellers: vi.fn()
  }
}));

vi.mock('$lib/bindings', () => ({
  commands: mockCommands
}));

import { getBuyers, getManufacturers, getSellers } from '$lib/services/entityLibrary';

describe('entityLibrary service', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('maps manufacturer metadata into library rows', async () => {
    mockCommands.getManufacturers.mockResolvedValue({
      status: 'ok',
      data: [
        {
          id: 'trn:manufacturer:acme',
          name: 'ACME',
          registeredCompanyName: null,
          countryCode: 'IT',
          status: 'ACTIVE',
          websiteUrl: null,
          isSystemSeeded: true,
          usageCount: 3
        }
      ]
    });

    const result = await getManufacturers();

    expect(result).toEqual({
      status: 'ok',
      data: [
        {
          id: 'trn:manufacturer:acme',
          name: 'ACME',
          countryCode: 'IT',
          isSystemSeeded: true,
          usageCount: 3
        }
      ]
    });
  });

  it('maps seller and buyer views using address country and metadata', async () => {
    const payload = {
      status: 'ok',
      data: [
        {
          id: 'trn:seller:model-shop',
          name: 'Model Shop',
          sellerType: 'SHOP',
          email: null,
          phone: null,
          websiteUrl: null,
          address: {
            street_address: 'street',
            extended_address: null,
            city: 'Berlin',
            region: null,
            postal_code: '10115',
            country: 'DE'
          },
          isSystemSeeded: false,
          usageCount: 1
        }
      ]
    };

    mockCommands.getSellers.mockResolvedValue(payload);

    const sellers = await getSellers();
    const buyers = await getBuyers();

    expect(sellers.status).toBe('ok');
    expect(buyers.status).toBe('ok');
    expect(mockCommands.getSellers).toHaveBeenCalledTimes(2);
    expect(sellers.status === 'ok' && sellers.data[0]).toEqual({
      id: 'trn:seller:model-shop',
      name: 'Model Shop',
      countryCode: 'DE',
      isSystemSeeded: false,
      usageCount: 1
    });
  });

  it('passes through command errors unchanged', async () => {
    const errorResult = {
      status: 'error',
      error: { DatabaseError: 'db exploded' }
    };

    mockCommands.getSellers.mockResolvedValue(errorResult);

    const result = await getSellers();

    expect(result).toEqual(errorResult);
  });

  it('normalizes thrown javascript errors into Unknown command error shape', async () => {
    mockCommands.getManufacturers.mockRejectedValueOnce(new Error('network fail'));

    const result = await getManufacturers();

    expect(result).toEqual({
      status: 'error',
      error: { Unknown: { message: 'network fail', error_id: 'frontend-unknown' } }
    });
  });

  it('handles wrapped ok results with non-array data as empty list', async () => {
    mockCommands.getSellers.mockResolvedValueOnce({
      status: 'ok',
      data: { not: 'an-array' }
    });

    const result = await getSellers();

    expect(result).toEqual({ status: 'ok', data: [] });
  });

  it('maps direct-array command results (legacy shape)', async () => {
    mockCommands.getManufacturers.mockResolvedValueOnce([
      {
        id: 'trn:manufacturer:legacy',
        name: 'Legacy',
        registeredCompanyName: null,
        countryCode: null,
        status: 'ACTIVE',
        websiteUrl: null,
        isSystemSeeded: false,
        usageCount: 0
      }
    ]);

    const result = await getManufacturers();

    expect(result).toEqual({
      status: 'ok',
      data: [
        {
          id: 'trn:manufacturer:legacy',
          name: 'Legacy',
          countryCode: null,
          isSystemSeeded: false,
          usageCount: 0
        }
      ]
    });
  });

  it('preserves Unknown command errors with message and error_id', async () => {
    mockCommands.getSellers.mockResolvedValueOnce({
      status: 'error',
      error: { Unknown: { message: 'upstream unknown', error_id: 'u-1' } }
    });

    const result = await getSellers();

    expect(result).toEqual({
      status: 'error',
      error: { Unknown: { message: 'upstream unknown', error_id: 'u-1' } }
    });
  });

  it('falls back to frontend unknown for non-Error thrown values', async () => {
    mockCommands.getSellers.mockRejectedValueOnce('boom');

    const result = await getSellers();

    expect(result).toEqual({
      status: 'error',
      error: { Unknown: { message: 'Unknown error', error_id: 'frontend-unknown' } }
    });
  });
});
