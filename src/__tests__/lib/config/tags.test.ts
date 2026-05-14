import { describe, expect, it, vi } from 'vitest';

const mockResolveTagIcon = vi.hoisted(() => vi.fn((key: string) => `icon:${key}`));

vi.mock('$lib/config/icons', () => ({
  resolveTagIcon: mockResolveTagIcon
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  collection_tag_steam: () => 'Steam',
  collection_tag_diesel: () => 'Diesel',
  collection_tag_electric: () => 'Electric',
  collection_tag_passenger: () => 'Passenger',
  collection_tag_freight: () => 'Freight'
}));

import { resolveTagMeta, sortAvailableTags, tagIcon } from '$lib/config/tags';

describe('tags config', () => {
  it('resolves fixed tag metadata case-insensitively', () => {
    const meta = resolveTagMeta('StEaM');

    expect(meta.key).toBe('steam');
    expect(meta.iconKey).toBe('steam');
    expect(meta.label()).toBe('Steam');
  });

  it('falls back to default metadata for unknown tags', () => {
    const meta = resolveTagMeta('custom-tag');

    expect(meta.key).toBe('custom-tag');
    expect(meta.iconKey).toBe('default');
    expect(meta.label()).toBe('custom-tag');
  });

  it('sorts fixed tags first, then dynamic tags alphabetically', () => {
    const sorted = sortAvailableTags(['zeta', 'Diesel', 'alpha', 'steam']);

    expect(sorted).toEqual(['steam', 'diesel', 'alpha', 'zeta']);
  });

  it('delegates icon resolution through tag metadata', () => {
    const icon = tagIcon('passenger');

    expect(icon).toBe('icon:passenger');
    expect(mockResolveTagIcon).toHaveBeenCalledWith('passenger');
  });
});
