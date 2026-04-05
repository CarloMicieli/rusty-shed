import { describe, it, expect, vi } from 'vitest';
import { acquisitionSchema, acquisitionItemSchema } from '../acquisition-form.js';

vi.mock('$lib/paraglide/messages.js', () => ({
  acquisition_validation_manufacturer: () => 'Manufacturer is required',
  acquisition_validation_product_code: () => 'Product code is required',
  acquisition_validation_category: () => 'Category is required',
  acquisition_validation_empty_items: () => 'Add at least one item'
}));

function makeItem(overrides: Record<string, unknown> = {}) {
  return {
    uid: 'test-uid',
    manufacturerId: 'mfg-1',
    productCode: 'CODE-123',
    description: 'A test item',
    category: 'LOCOMOTIVES',
    epoch: 'IV',
    priceAmount: null,
    ...overrides
  };
}

function makeForm(overrides: Record<string, unknown> = {}) {
  return {
    sellerId: null,
    purchaseDate: '2026-01-15',
    batchDefaults: { scale: 'H0', powerMethod: 'DC' },
    items: [makeItem()],
    ...overrides
  };
}

// ---------------------------------------------------------------------------
// acquisitionItemSchema
// ---------------------------------------------------------------------------

describe('acquisitionItemSchema', () => {
  it('passes with a valid item', () => {
    const result = acquisitionItemSchema.safeParse(makeItem());
    expect(result.success).toBe(true);
  });

  it('fails when manufacturerId is null', () => {
    const result = acquisitionItemSchema.safeParse(makeItem({ manufacturerId: null }));
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.flatten().fieldErrors.manufacturerId).toBeTruthy();
    }
  });

  it('fails when productCode is empty', () => {
    const result = acquisitionItemSchema.safeParse(makeItem({ productCode: '' }));
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.flatten().fieldErrors.productCode).toBeTruthy();
    }
  });

  it('fails when category is null', () => {
    const result = acquisitionItemSchema.safeParse(makeItem({ category: null }));
    expect(result.success).toBe(false);
    if (!result.success) {
      expect(result.error.flatten().fieldErrors.category).toBeTruthy();
    }
  });

  it('allows null epoch and priceAmount', () => {
    const result = acquisitionItemSchema.safeParse(makeItem({ epoch: null, priceAmount: null }));
    expect(result.success).toBe(true);
  });
});

// ---------------------------------------------------------------------------
// acquisitionSchema
// ---------------------------------------------------------------------------

describe('acquisitionSchema', () => {
  it('passes with a valid form', () => {
    const result = acquisitionSchema.safeParse(makeForm());
    expect(result.success).toBe(true);
  });

  it('fails when items array is empty', () => {
    const result = acquisitionSchema.safeParse(makeForm({ items: [] }));
    expect(result.success).toBe(false);
    if (!result.success) {
      const flat = result.error.flatten();
      expect(flat.fieldErrors.items).toBeTruthy();
    }
  });

  it('propagates item-level errors for null manufacturerId', () => {
    const result = acquisitionSchema.safeParse(
      makeForm({ items: [makeItem({ manufacturerId: null })] })
    );
    expect(result.success).toBe(false);
  });

  it('propagates item-level errors for null category', () => {
    const result = acquisitionSchema.safeParse(makeForm({ items: [makeItem({ category: null })] }));
    expect(result.success).toBe(false);
  });

  it('allows nullable sellerId and empty purchaseDate', () => {
    const result = acquisitionSchema.safeParse(makeForm({ sellerId: null, purchaseDate: '' }));
    expect(result.success).toBe(true);
  });
});
