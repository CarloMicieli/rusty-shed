import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('VirtualGrid mobile min-width behavior', () => {
  it('uses itemMinWidth in column count and template calculations', () => {
    const source = readFileSync(resolve('src/lib/components/VirtualGrid.svelte'), 'utf8');

    expect(source).toContain('itemMinWidth?: number;');
    expect(source).toContain('Math.floor((containerWidth + gap) / (itemMinWidth + gap))');
    expect(source).toContain('grid-template-columns: repeat({columnCount}, minmax(0, 1fr));');
  });
});
