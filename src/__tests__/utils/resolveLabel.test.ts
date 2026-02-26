import { describe, it, expect, vi } from 'vitest';

vi.mock('$lib/paraglide/messages.js', () => ({
  scale_h0: () => 'H0 Scale (1:87)',
  scale_n: () => 'N Scale (1:160)',
  era_vi: () => 'Era VI (2000-present)'
}));

import { resolveLabel } from '../../utils/resolveLabel';
import type { ConstantItem } from '$lib/types/constant_item';

// ─── tests ────────────────────────────────────────────────────────────────

describe('resolveLabel', () => {
  describe('when labelKey is provided and resolves', () => {
    it('returns the translated string from paraglide messages', () => {
      const item: ConstantItem = { id: 'H0', labelKey: 'scale_h0' };
      expect(resolveLabel(item)).toBe('H0 Scale (1:87)');
    });

    it('uses the correct message function for different keys', () => {
      const item: ConstantItem = { id: 'N', labelKey: 'scale_n' };
      expect(resolveLabel(item)).toBe('N Scale (1:160)');
    });

    it('ignores display field when labelKey resolves', () => {
      const item: ConstantItem = { id: 'H0', labelKey: 'scale_h0', display: 'Should be ignored' };
      expect(resolveLabel(item)).toBe('H0 Scale (1:87)');
    });
  });

  // Note: the "labelKey not in messages" branch is unreachable in practice —
  // TypeScript's `keyof typeof m` cast prevents referencing non-existent keys.
  // Vitest's strict mock interceptor also prevents accessing undefined mock exports.

  describe('when no labelKey is provided', () => {
    it('returns display when labelKey is absent', () => {
      const item: ConstantItem = { id: 'custom', display: 'Custom Display' };
      expect(resolveLabel(item)).toBe('Custom Display');
    });

    it('falls back to id when both labelKey and display are absent', () => {
      const item: ConstantItem = { id: 'fallback-id' };
      expect(resolveLabel(item)).toBe('fallback-id');
    });

    it('returns id when display is explicitly undefined', () => {
      const item: ConstantItem = { id: 'my-id', display: undefined };
      expect(resolveLabel(item)).toBe('my-id');
    });
  });

  describe('edge cases', () => {
    it('handles empty labelKey string (falsy) — falls back to display', () => {
      const item: ConstantItem = { id: 'my-id', labelKey: '', display: 'My Display' };
      expect(resolveLabel(item)).toBe('My Display');
    });

    it('handles empty display string — returns empty string', () => {
      const item: ConstantItem = { id: 'my-id', display: '' };
      // display ?? id: empty string is not null/undefined, so returns ''
      expect(resolveLabel(item)).toBe('');
    });
  });
});
