import { describe, it, expect } from 'vitest';
import { isSoldItem, extractPurchaseDate } from '../cardDataMapper';
import type { PurchaseInfo } from '$lib/bindings';

// ── Helpers ─────────────────────────────────────────────────────────────────

const soldInfo: PurchaseInfo = {
  kind: 'sold',
  data: {
    id: 'trn:purchase-info:aaa',
    purchaseDate: '2024-01-01',
    purchasePrice: null,
    saleDate: '2025-03-10',
    salePrice: { amount: 5000, currency: 'EUR' },
    buyer: null,
    seller: null
  }
};

const purchasedInfo: PurchaseInfo = {
  kind: 'purchased',
  data: {
    id: 'trn:purchase-info:bbb',
    purchaseDate: '2024-06-15',
    price: null,
    seller: null
  }
};

const preOrderedInfo: PurchaseInfo = {
  kind: 'preOrdered',
  data: {
    id: 'trn:purchase-info:ccc',
    orderDate: '2024-02-01',
    deposit: { amount: 1000, currency: 'EUR' },
    totalPrice: { amount: 8000, currency: 'EUR' },
    seller: null,
    expectedDate: null
  }
};

// ── isSoldItem ───────────────────────────────────────────────────────────────

describe('isSoldItem', () => {
  it('returns true when purchaseInfo kind is sold', () => {
    expect(isSoldItem(soldInfo, null)).toBe(true);
  });

  it('returns true when removedDate is set (even without sold purchaseInfo)', () => {
    expect(isSoldItem(null, '2025-01-01')).toBe(true);
  });

  it('returns true when both sold purchaseInfo and removedDate are set', () => {
    expect(isSoldItem(soldInfo, '2025-01-01')).toBe(true);
  });

  it('returns false for purchased purchaseInfo with no removedDate', () => {
    expect(isSoldItem(purchasedInfo, null)).toBe(false);
  });

  it('returns false when purchaseInfo is null and removedDate is null', () => {
    expect(isSoldItem(null, null)).toBe(false);
  });
});

// ── extractPurchaseDate ──────────────────────────────────────────────────────

describe('extractPurchaseDate', () => {
  it('returns purchaseDate for purchased items', () => {
    expect(extractPurchaseDate(purchasedInfo)).toBe('2024-06-15');
  });

  it('returns orderDate for preOrdered items', () => {
    expect(extractPurchaseDate(preOrderedInfo)).toBe('2024-02-01');
  });

  it('returns null for sold items', () => {
    expect(extractPurchaseDate(soldInfo)).toBeNull();
  });

  it('returns null when purchaseInfo is null', () => {
    expect(extractPurchaseDate(null)).toBeNull();
  });
});
