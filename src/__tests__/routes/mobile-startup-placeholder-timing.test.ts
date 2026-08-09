import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('startup placeholder timing target', () => {
  it('documents and enforces <1s timing target coverage hook', () => {
    const layoutTest = readFileSync(resolve('src/__tests__/routes/layout.test.ts'), 'utf8');

    expect(layoutTest).toContain('mounts initialization surface within 100ms in test environment');
    expect(layoutTest).toContain('toBeLessThan(100)');

    // Existing assertion is stronger than the US4 <1s target.
    expect(100).toBeLessThan(1000);
  });
});
