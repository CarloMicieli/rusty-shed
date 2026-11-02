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

  it('applies tactile press feedback to mobile filter sheet controls', () => {
    const controlPanelSource = readFileSync(
      resolve('src/lib/features/collection/components/ControlPanel.svelte'),
      'utf8'
    );

    expect(controlPanelSource).toContain('min-h-11');
    expect(controlPanelSource).toContain('active:scale-[0.98]');
    expect(controlPanelSource).toContain('active:bg-muted/50');
  });
});
