import { describe, it, expect } from 'vitest';
import { generateErrorId, isErrorId } from '$lib/services/error-id';

describe('generateErrorId', () => {
  it('returns a string matching ERR-NNNN-X format', () => {
    const id = generateErrorId();
    expect(/^ERR-\d{4}-[A-Z]$/.test(id)).toBe(true);
  });

  it('numeric segment is in range 1000–9999', () => {
    for (let i = 0; i < 100; i++) {
      const id = generateErrorId();
      const numeric = parseInt(id.slice(4, 8), 10);
      expect(numeric).toBeGreaterThanOrEqual(1000);
      expect(numeric).toBeLessThanOrEqual(9999);
    }
  });

  it('two consecutive calls return different IDs', () => {
    let differenceFound = false;
    for (let i = 0; i < 10; i++) {
      const a = generateErrorId();
      const b = generateErrorId();
      if (a !== b) {
        differenceFound = true;
        break;
      }
    }
    expect(differenceFound).toBe(true);
  });
});

describe('isErrorId', () => {
  it('returns true for valid IDs', () => {
    expect(isErrorId('ERR-1234-A')).toBe(true);
    expect(isErrorId('ERR-9999-Z')).toBe(true);
    expect(isErrorId('ERR-1000-B')).toBe(true);
  });

  it('returns false for invalid strings', () => {
    expect(isErrorId('err-1234-A')).toBe(false); // lowercase
    expect(isErrorId('ERR-123-A')).toBe(false); // 3 digits
    expect(isErrorId('ERR-12345-A')).toBe(false); // 5 digits
    expect(isErrorId('ERR-1234-a')).toBe(false); // lowercase letter
    expect(isErrorId('ERR-1234-AB')).toBe(false); // two letters
    expect(isErrorId('')).toBe(false);
    expect(isErrorId(null)).toBe(false);
    expect(isErrorId(undefined)).toBe(false);
    expect(isErrorId(1234)).toBe(false);
  });
});
