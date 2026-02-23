import { describe, it, expect } from 'vitest';
import { isActive } from '$lib/components/navigation/utils';
import type { NavigationItem } from '$lib/components/navigation/types';

// Minimal stub to satisfy NavigationItem type without icon/label
function makeItem(overrides: Partial<NavigationItem>): NavigationItem {
  return {
    id: 'test',
    label: () => 'Test',
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon: null as any,
    href: '/test',
    isPrimary: false,
    ...overrides
  };
}

describe('isActive()', () => {
  describe('additionalPrefixes behaviour', () => {
    it("returns true for /collection/123 when additionalPrefixes: ['/collection']", () => {
      const item = makeItem({
        href: '/my-collection',
        additionalPrefixes: ['/collection']
      });
      expect(isActive(item, '/collection/123')).toBe(true);
    });

    it("returns true for /collection/any-id when additionalPrefixes: ['/collection']", () => {
      const item = makeItem({
        href: '/my-collection',
        additionalPrefixes: ['/collection']
      });
      expect(isActive(item, '/collection/trn:collection-item:abc-123')).toBe(true);
    });

    it('returns false for unrelated path even with additionalPrefixes', () => {
      const item = makeItem({
        href: '/my-collection',
        additionalPrefixes: ['/collection']
      });
      expect(isActive(item, '/my-dashboard')).toBe(false);
    });

    it('returns false for /my-budget when collection additionalPrefixes defined', () => {
      const item = makeItem({
        href: '/my-collection',
        additionalPrefixes: ['/collection']
      });
      expect(isActive(item, '/my-budget')).toBe(false);
    });
  });

  describe('exact href match still works', () => {
    it('returns true for exact href match when no additionalPrefixes', () => {
      const item = makeItem({ href: '/my-collection' });
      expect(isActive(item, '/my-collection')).toBe(true);
    });

    it('returns true for exact href match even with additionalPrefixes', () => {
      const item = makeItem({
        href: '/my-collection',
        additionalPrefixes: ['/collection']
      });
      expect(isActive(item, '/my-collection')).toBe(true);
    });

    it('returns false for non-matching path with no special options', () => {
      const item = makeItem({ href: '/my-collection' });
      expect(isActive(item, '/my-dashboard')).toBe(false);
    });
  });

  describe('usePrefixMatch behaviour (existing feature)', () => {
    it('returns true when usePrefixMatch and pathname starts with href', () => {
      const item = makeItem({ href: '/my-tracks', usePrefixMatch: true });
      expect(isActive(item, '/my-tracks/n-scale')).toBe(true);
    });

    it('returns true for exact match with usePrefixMatch', () => {
      const item = makeItem({ href: '/my-tracks', usePrefixMatch: true });
      expect(isActive(item, '/my-tracks')).toBe(true);
    });

    it('returns false for unrelated path with usePrefixMatch', () => {
      const item = makeItem({ href: '/my-tracks', usePrefixMatch: true });
      expect(isActive(item, '/my-collection')).toBe(false);
    });
  });
});
