import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('mobile i18n overflow protections', () => {
  it('enforces truncation and width constraints for localized mobile labels', () => {
    const bottomNav = readFileSync(resolve('src/lib/components/BottomNavigation.svelte'), 'utf8');
    const moreMenu = readFileSync(resolve('src/lib/components/navigation/MoreMenu.svelte'), 'utf8');

    expect(bottomNav).toContain('max-w-[5.5rem] truncate');
    expect(bottomNav).toContain('leading-tight');
    expect(moreMenu).toContain('!w-[min(100%,26rem)]');
    expect(moreMenu).toContain('min-h-11');
  });
});
