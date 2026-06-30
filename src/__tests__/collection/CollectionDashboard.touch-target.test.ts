import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('CollectionDashboard touch-target contracts', () => {
  it('keeps removable filter controls at 44x44 minimum target size on mobile', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    const targetClassCount =
      source.split(
        'h-11 w-11 rounded-sm p-0.5 transition-all active:scale-[0.98] active:bg-white/20 md:h-9 md:w-9 md:hover:bg-white/20'
      ).length - 1;
    expect(targetClassCount).toBeGreaterThanOrEqual(5);
  });
});
