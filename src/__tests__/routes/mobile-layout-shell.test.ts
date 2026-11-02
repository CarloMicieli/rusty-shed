import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('mobile layout shell contracts', () => {
  it('keeps safe-area shell classes and mobile bottom-nav padding hook', () => {
    const source = readFileSync(resolve('src/routes/+layout.svelte'), 'utf8');

    expect(source).toContain('safe-area-pad flex h-screen');
    expect(source).toContain('class:safe-area-pad-bottom-nav={isMobileViewport}');
    expect(source).toContain('max-w-[70vw] truncate text-sm font-bold tracking-widest uppercase');
  });
});
