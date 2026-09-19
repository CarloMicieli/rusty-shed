import { describe, expect, it } from 'vitest';
import { createDrawerRegistry } from '$lib/state/drawer-registry.svelte';

describe('drawer-registry.svelte', () => {
  it('keeps stack depth bounded to 2 layers', () => {
    const registry = createDrawerRegistry();

    expect(registry.openParent('parent')).toBe(true);
    expect(registry.openChild('child')).toBe(true);
    expect(registry.openChild('child-2')).toBe(false);
    expect(registry.depth).toBe(2);
    expect(registry.stack.map((layer) => layer.id)).toEqual(['parent', 'child']);
  });

  it('pops layers in child-first order', () => {
    const registry = createDrawerRegistry();

    registry.openParent('parent');
    registry.openChild('child');

    expect(registry.closeTop('back')?.id).toBe('child');
    expect(registry.closeTop('back')?.id).toBe('parent');
    expect(registry.closeTop('back')).toBeNull();
  });

  it('replaces existing child when replaceIfFull is enabled', () => {
    const registry = createDrawerRegistry();

    registry.openParent('parent');
    registry.openChild('child-a');

    expect(registry.openChild('child-b', { replaceIfFull: true })).toBe(true);
    expect(registry.stack.map((layer) => layer.id)).toEqual(['parent', 'child-b']);
  });
});
