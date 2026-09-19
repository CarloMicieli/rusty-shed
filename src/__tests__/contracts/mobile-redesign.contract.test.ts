import { describe, expect, it } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { createDrawerRegistry } from '$lib/state/drawer-registry.svelte';

describe('mobile redesign contract conformance', () => {
  it('conforms drawer depth and replace semantics to contract', () => {
    const registry = createDrawerRegistry();

    registry.openParent('parent-layer');
    registry.openChild('child-layer');

    expect(registry.depth).toBeLessThanOrEqual(2);
    expect(registry.openChild('blocked-layer')).toBe(false);
    expect(registry.openChild('replacement-layer', { replaceIfFull: true })).toBe(true);
    expect(registry.stack.map((layer) => layer.id)).toEqual(['parent-layer', 'replacement-layer']);
  });

  it('keeps media fallback contract hooks in upload/drop components', () => {
    const uploadSource = readFileSync(
      resolve('src/lib/components/model-details/ImageUpload.svelte'),
      'utf8'
    );
    const dropZoneSource = readFileSync(
      resolve('src/lib/components/model-details/ImageDropZone.svelte'),
      'utf8'
    );

    expect(uploadSource).toContain('navigator.mediaDevices.enumerateDevices');
    expect(uploadSource).toContain("activeInputMode = 'gallery_picker'");
    expect(dropZoneSource).toContain("capture={useCaptureAttribute ? 'environment' : undefined}");
  });
});
