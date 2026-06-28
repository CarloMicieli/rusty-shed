import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('mobile startup placeholder behavior contracts', () => {
  it('keeps a mobile-safe non-blocking loading surface during async startup', () => {
    const layout = readFileSync(resolve('src/routes/+layout.svelte'), 'utf8');

    expect(layout).toContain('data-testid="startup-loading-surface"');
    expect(layout).toContain('role="status"');
    expect(layout).toContain('aria-live="polite"');
    expect(layout).toContain('safe-area-pad');
    expect(layout).toContain("const loader = document.getElementById('app-loading');");
    expect(layout).toContain('loader.remove()');
  });
});
