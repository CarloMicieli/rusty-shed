import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

describe('desktop parity guards for mobile redesign', () => {
  it('keeps desktop layout classes while mobile variants remain scoped', () => {
    const layout = readFileSync(resolve('src/routes/+layout.svelte'), 'utf8');
    const bottomNav = readFileSync(resolve('src/lib/components/BottomNavigation.svelte'), 'utf8');
    const drawerShell = readFileSync(
      resolve('src/lib/components/drawer/DrawerShell.svelte'),
      'utf8'
    );

    expect(layout).toContain('lg:flex-row');
    expect(layout).toContain('lg:block');
    expect(layout).toContain('lg:p-8 lg:pb-8');

    expect(bottomNav).toContain('lg:hidden');

    expect(drawerShell).toContain('inset-y-0 right-0');
    expect(drawerShell).toContain('border-l-2');
    expect(drawerShell).toContain('max-w-2xl');
  });
});
