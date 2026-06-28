import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('CollectionDashboard mobile layout contracts', () => {
  it('enforces mobile grid minimum width and hides table toggle on mobile', () => {
    const source = readFileSync(
      resolve('src/lib/features/collection/CollectionDashboard.svelte'),
      'utf8'
    );

    expect(source).toContain('itemMinWidth={isMobileViewport ? 320 : 240}');
    expect(source).toContain('class="mb-4 hidden items-center justify-end md:flex"');
    expect(source).toContain('safe-area-inset-bottom fixed right-4 bottom-4');
  });
});
