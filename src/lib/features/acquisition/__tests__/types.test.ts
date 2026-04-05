import { describe, it, expect, vi } from 'vitest';
import { createDefaultItem, createDefaultFormState, toRecordAcquisitionArgs } from '../types.js';
import type { AcquisitionFormState, AcquisitionItemEntry } from '../types.js';

vi.mock('$lib/bindings', () => ({}));

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function makeItem(overrides: Partial<AcquisitionItemEntry> = {}): AcquisitionItemEntry {
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

function makeForm(overrides: Partial<AcquisitionFormState> = {}): AcquisitionFormState {
  return {
    sellerId: 'seller-1',
    purchaseDate: '2026-01-15',
    batchDefaults: { scale: 'H0', powerMethod: 'DC' },
    items: [makeItem()],
    ...overrides
  };
}

// ---------------------------------------------------------------------------
// createDefaultItem
// ---------------------------------------------------------------------------

describe('createDefaultItem', () => {
  it('starts with priceAmount null', () => {
    const item = createDefaultItem();
    expect(item.priceAmount).toBeNull();
  });

  it('starts with empty strings for productCode and description', () => {
    const item = createDefaultItem();
    expect(item.productCode).toBe('');
    expect(item.description).toBe('');
  });

  it('starts with null for manufacturerId, category, epoch', () => {
    const item = createDefaultItem();
    expect(item.manufacturerId).toBeNull();
    expect(item.category).toBeNull();
    expect(item.epoch).toBeNull();
  });

  it('generates a non-empty uid', () => {
    const item = createDefaultItem();
    expect(item.uid).toBeTruthy();
    expect(typeof item.uid).toBe('string');
  });

  it('generates unique uids for each call', () => {
    const a = createDefaultItem();
    const b = createDefaultItem();
    expect(a.uid).not.toBe(b.uid);
  });
});

// ---------------------------------------------------------------------------
// createDefaultFormState
// ---------------------------------------------------------------------------

describe('createDefaultFormState', () => {
  it('starts with sellerId null', () => {
    expect(createDefaultFormState().sellerId).toBeNull();
  });

  it('starts with one empty item', () => {
    const { items } = createDefaultFormState();
    expect(items).toHaveLength(1);
  });

  it('purchaseDate is a YYYY-MM-DD string', () => {
    const { purchaseDate } = createDefaultFormState();
    expect(purchaseDate).toMatch(/^\d{4}-\d{2}-\d{2}$/);
  });

  it('batchDefaults start as null/null', () => {
    const { batchDefaults } = createDefaultFormState();
    expect(batchDefaults.scale).toBeNull();
    expect(batchDefaults.powerMethod).toBeNull();
  });
});

// ---------------------------------------------------------------------------
// toRecordAcquisitionArgs — price conversion
// ---------------------------------------------------------------------------

describe('toRecordAcquisitionArgs', () => {
  it('passes sellerId and purchaseDate through', () => {
    const args = toRecordAcquisitionArgs(makeForm(), 'EUR');
    expect(args.sellerId).toBe('seller-1');
    expect(args.purchaseDate).toBe('2026-01-15');
  });

  it('converts cents integer to BigInt without multiplying again', () => {
    // CurrencyInput already gives us cents: 200.00 € → 20000
    const args = toRecordAcquisitionArgs(
      makeForm({ items: [makeItem({ priceAmount: 20000 })] }),
      'EUR'
    );
    expect(args.items[0].priceAmount).toBe(Number(20000));
  });

  it('handles whole-number price (e.g. 200 cents = €2.00) correctly', () => {
    const args = toRecordAcquisitionArgs(
      makeForm({ items: [makeItem({ priceAmount: 200 })] }),
      'EUR'
    );
    expect(args.items[0].priceAmount).toBe(Number(200));
  });

  it('maps null price to Number(0)', () => {
    const args = toRecordAcquisitionArgs(
      makeForm({ items: [makeItem({ priceAmount: null })] }),
      'EUR'
    );
    expect(args.items[0].priceAmount).toBe(Number(0));
  });

  it('passes the currency string to each item', () => {
    const args = toRecordAcquisitionArgs(makeForm(), 'GBP');
    expect(args.items[0].priceCurrency).toBe('GBP');
  });

  it('maps optional fields: null scale/epoch/powerMethod become empty strings', () => {
    const args = toRecordAcquisitionArgs(
      makeForm({
        batchDefaults: { scale: null, powerMethod: null },
        items: [makeItem({ epoch: null })]
      }),
      'EUR'
    );
    expect(args.items[0].scale).toBe('');
    expect(args.items[0].epoch).toBe('');
    expect(args.items[0].powerMethod).toBe('');
  });

  it('maps all items when multiple items are present', () => {
    const items = [
      makeItem({ uid: 'a', productCode: 'A1', priceAmount: 1000 }),
      makeItem({ uid: 'b', productCode: 'B2', priceAmount: 2550 })
    ];
    const args = toRecordAcquisitionArgs(makeForm({ items }), 'EUR');
    expect(args.items).toHaveLength(2);
    expect(args.items[0].priceAmount).toBe(Number(1000));
    expect(args.items[1].priceAmount).toBe(Number(2550));
  });
});
