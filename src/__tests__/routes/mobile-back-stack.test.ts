import { describe, expect, it } from 'vitest';
import { createDrawerRegistry } from '$lib/state/drawer-registry.svelte';

function handleBackNavigation(registry: ReturnType<typeof createDrawerRegistry>): boolean {
  if (registry.depth === 0) {
    return false;
  }

  registry.closeTop('back');
  return true;
}

describe('mobile back stack integration', () => {
  it('unwinds child then parent before route back can continue', () => {
    const registry = createDrawerRegistry();

    registry.openParent('acquisition');
    registry.openChild('image-upload');

    expect(handleBackNavigation(registry)).toBe(true);
    expect(registry.stack.map((layer) => layer.id)).toEqual(['acquisition']);

    expect(handleBackNavigation(registry)).toBe(true);
    expect(registry.depth).toBe(0);

    expect(handleBackNavigation(registry)).toBe(false);
  });
});
