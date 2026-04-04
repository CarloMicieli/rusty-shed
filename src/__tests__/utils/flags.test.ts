import { describe, it, expect } from 'vitest';
import { getFlag } from '../../lib/utils/flags';

describe('getFlag', () => {
  it('should convert "IT" to 🇮🇹', () => {
    expect(getFlag('IT')).toBe('🇮🇹');
  });

  it('should convert "GB" to 🇬🇧', () => {
    expect(getFlag('GB')).toBe('🇬🇧');
  });

  it('should convert lowercase "it" to 🇮🇹', () => {
    expect(getFlag('it')).toBe('🇮🇹');
  });

  it('should return 🌐 for null', () => {
    expect(getFlag(null)).toBe('🌐');
  });

  it('should return 🌐 for empty string', () => {
    expect(getFlag('')).toBe('🌐');
  });

  it('should return 🌐 for invalid length', () => {
    expect(getFlag('USA')).toBe('🌐');
    expect(getFlag('A')).toBe('🌐');
  });
});
