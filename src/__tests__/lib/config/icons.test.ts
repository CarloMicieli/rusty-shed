import { describe, expect, it } from 'vitest';
import { resolveTagIcon, iconMap } from '$lib/config/icons';

describe('config/icons', () => {
  it('resolves known keys case-insensitively', () => {
    expect(resolveTagIcon('StEaM')).toBe(iconMap.steam);
    expect(resolveTagIcon('ELECTRIC')).toBe(iconMap.electric);
  });

  it('falls back to default icon for unknown keys', () => {
    expect(resolveTagIcon('not-configured')).toBe(iconMap.default);
  });
});
