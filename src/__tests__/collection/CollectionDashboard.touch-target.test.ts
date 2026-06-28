import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('CollectionDashboard touch-target contracts', () => {
  it('keeps removable filter controls at 36x36 minimum target size', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    const targetClassCount = source.split('h-9 w-9 rounded-sm p-0.5 transition-colors').length - 1;
    expect(targetClassCount).toBeGreaterThanOrEqual(5);
  });
});
