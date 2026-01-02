import { describe, it, expect } from 'vitest';
import { mapFormToCreatePayload, mapSellerToForm } from '$lib/services/sellerAdapter';
import type { FormSeller } from '$lib/services/sellerAdapter';
import type { Seller } from '$lib/bindings';

describe('sellerAdapter', () => {
  it('maps form to create payload', () => {
    const form: FormSeller = {
      name: 'Shop A',
      sellerType: 'SHOP' as unknown as import('$lib/bindings').SellerType,
      email: 'a@example.com',
      phone: '+39 000',
      websiteUrl: 'https://shop.example',
      streetAddress: 'Via Test 1',
      extendedAddress: 'Pad 3',
      city: 'Foligno',
      stateRegion: 'PG',
      postalCode: '06034',
      countryCode: 'IT'
    };

    const payload = mapFormToCreatePayload(form);
    expect(payload.name).toBe('Shop A');
    expect(payload.streetAddress).toBe('Via Test 1');
    expect(payload.countryCode).toBe('IT');
  });

  it('maps seller to form with nested address', () => {
    const seller = {
      id: 'trn:seller:shop-a',
      name: 'Shop A',
      sellerType: 'SHOP',
      email: 'a@example.com',
      phone: null,
      websiteUrl: null,
      address: {
        street_address: 'Via Test 1',
        extended_address: null,
        city: 'Foligno',
        region: 'PG',
        postal_code: '06034',
        country: 'IT'
      },
      createdAt: '2025-01-01T00:00:00Z',
      updatedAt: '2025-01-02T00:00:00Z'
    } as unknown as Seller;

    const form = mapSellerToForm(seller);
    expect(form.streetAddress).toBe('Via Test 1');
    expect(form.countryCode).toBe('IT');
    expect(form.id).toBe('trn:seller:shop-a');
  });
});
