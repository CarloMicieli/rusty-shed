import { describe, it, expect } from 'vitest';
import { sellerSchema } from '$lib/schemas/seller';
import type { SellerFormData } from '$lib/schemas/seller';

describe('sellerSchema', () => {
  const validSeller: SellerFormData = {
    name: 'Train Shop Milano',
    sellerType: 'SHOP'
  };

  describe('required fields', () => {
    it('accepts a minimal valid seller (name + sellerType only)', () => {
      const result = sellerSchema.parse(validSeller);
      expect(result.name).toBe('Train Shop Milano');
      expect(result.sellerType).toBe('SHOP');
    });

    it('rejects an empty name', () => {
      const invalid = { ...validSeller, name: '' };
      expect(() => sellerSchema.parse(invalid)).toThrow();
    });

    it('rejects a missing name', () => {
      const { name: _name, ...invalid } = validSeller;
      expect(() => sellerSchema.parse(invalid)).toThrow();
    });

    it('rejects a missing sellerType', () => {
      const { sellerType: _type, ...invalid } = validSeller;
      expect(() => sellerSchema.parse(invalid)).toThrow();
    });
  });

  describe('sellerType enum', () => {
    it('accepts SHOP', () => {
      expect(sellerSchema.parse({ ...validSeller, sellerType: 'SHOP' }).sellerType).toBe('SHOP');
    });

    it('accepts PRIVATE', () => {
      expect(sellerSchema.parse({ ...validSeller, sellerType: 'PRIVATE' }).sellerType).toBe(
        'PRIVATE'
      );
    });

    it('accepts MANUFACTURER', () => {
      expect(sellerSchema.parse({ ...validSeller, sellerType: 'MANUFACTURER' }).sellerType).toBe(
        'MANUFACTURER'
      );
    });

    it('rejects an invalid sellerType', () => {
      expect(() =>
        sellerSchema.parse({ ...validSeller, sellerType: 'AUCTION' as unknown })
      ).toThrow();
    });

    it('rejects lowercase sellerType', () => {
      expect(() => sellerSchema.parse({ ...validSeller, sellerType: 'shop' as unknown })).toThrow();
    });
  });

  describe('optional id field', () => {
    it('accepts a seller with an id', () => {
      const result = sellerSchema.parse({ ...validSeller, id: 'seller-123' });
      expect(result.id).toBe('seller-123');
    });

    it('accepts a seller without an id', () => {
      const result = sellerSchema.parse(validSeller);
      expect(result.id).toBeUndefined();
    });
  });

  describe('email field', () => {
    it('accepts a valid email', () => {
      const result = sellerSchema.parse({ ...validSeller, email: 'shop@example.com' });
      expect(result.email).toBe('shop@example.com');
    });

    it('accepts an empty string for email', () => {
      const result = sellerSchema.parse({ ...validSeller, email: '' });
      expect(result.email).toBe('');
    });

    it('accepts null for email', () => {
      const result = sellerSchema.parse({ ...validSeller, email: null });
      expect(result.email).toBeNull();
    });

    it('rejects an invalid email format', () => {
      expect(() => sellerSchema.parse({ ...validSeller, email: 'not-an-email' })).toThrow();
    });
  });

  describe('websiteUrl field', () => {
    it('accepts a valid URL', () => {
      const result = sellerSchema.parse({ ...validSeller, websiteUrl: 'https://example.com' });
      expect(result.websiteUrl).toBe('https://example.com');
    });

    it('accepts an empty string for websiteUrl', () => {
      const result = sellerSchema.parse({ ...validSeller, websiteUrl: '' });
      expect(result.websiteUrl).toBe('');
    });

    it('accepts null for websiteUrl', () => {
      const result = sellerSchema.parse({ ...validSeller, websiteUrl: null });
      expect(result.websiteUrl).toBeNull();
    });

    it('rejects an invalid URL', () => {
      expect(() => sellerSchema.parse({ ...validSeller, websiteUrl: 'not-a-url' })).toThrow();
    });
  });

  describe('address fields (optional, nullable, or empty string)', () => {
    it('accepts null for all address fields', () => {
      const result = sellerSchema.parse({
        ...validSeller,
        streetAddress: null,
        extendedAddress: null,
        city: null,
        stateRegion: null,
        postalCode: null,
        countryCode: null
      });
      expect(result.streetAddress).toBeNull();
      expect(result.city).toBeNull();
      expect(result.postalCode).toBeNull();
    });

    it('accepts string values for all address fields', () => {
      const result = sellerSchema.parse({
        ...validSeller,
        streetAddress: 'Via Roma 1',
        extendedAddress: 'Piano 2',
        city: 'Milano',
        stateRegion: 'Lombardia',
        postalCode: '20100',
        countryCode: 'IT'
      });
      expect(result.city).toBe('Milano');
      expect(result.postalCode).toBe('20100');
    });

    it('accepts empty strings for all address fields', () => {
      const result = sellerSchema.parse({
        ...validSeller,
        streetAddress: '',
        city: '',
        postalCode: ''
      });
      expect(result.streetAddress).toBe('');
      expect(result.city).toBe('');
    });
  });

  describe('phone field', () => {
    it('accepts a phone number string', () => {
      const result = sellerSchema.parse({ ...validSeller, phone: '+39-02-12345678' });
      expect(result.phone).toBe('+39-02-12345678');
    });

    it('accepts null for phone', () => {
      const result = sellerSchema.parse({ ...validSeller, phone: null });
      expect(result.phone).toBeNull();
    });

    it('accepts empty string for phone', () => {
      const result = sellerSchema.parse({ ...validSeller, phone: '' });
      expect(result.phone).toBe('');
    });
  });

  describe('complete seller with all fields', () => {
    it('validates a fully populated seller', () => {
      const full: SellerFormData = {
        id: 'seller-42',
        name: 'Trains & More',
        sellerType: 'SHOP',
        email: 'info@trainsandmore.it',
        phone: '+39-02-99887766',
        websiteUrl: 'https://trainsandmore.it',
        streetAddress: 'Via Ferrovia 10',
        extendedAddress: 'Interno 3',
        city: 'Roma',
        stateRegion: 'Lazio',
        postalCode: '00100',
        countryCode: 'IT'
      };
      const result = sellerSchema.parse(full);
      expect(result.name).toBe('Trains & More');
      expect(result.sellerType).toBe('SHOP');
      expect(result.email).toBe('info@trainsandmore.it');
      expect(result.city).toBe('Roma');
    });
  });
});
