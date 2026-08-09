import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import { createDrawerRegistry } from '$lib/state/drawer-registry.svelte';
import DrawerShellWrapper from './DrawerShellWrapper.svelte';

function setViewport(width: number) {
  Object.defineProperty(window, 'matchMedia', {
    configurable: true,
    writable: true,
    value: (query: string): MediaQueryList => {
      const isMobileQuery = query.includes('max-width: 767px');
      const matches = isMobileQuery ? width < 768 : width >= 768;
      return {
        matches,
        media: query,
        onchange: null,
        addEventListener: () => {},
        removeEventListener: () => {},
        addListener: () => {},
        removeListener: () => {},
        dispatchEvent: () => false
      } as MediaQueryList;
    }
  });
}

describe('DrawerShell mobile depth behavior', () => {
  it('renders as bottom sheet on mobile and side panel on desktop', async () => {
    setViewport(375);
    const mobile = render(DrawerShellWrapper, { props: { open: true, onClose: () => {} } });
    const mobileDialog = await screen.findByRole('dialog');
    expect(mobileDialog.className).toContain('bottom-0');
    expect(mobileDialog.className).toContain('rounded-t-2xl');
    mobile.unmount();

    setViewport(1280);
    const desktop = render(DrawerShellWrapper, { props: { open: true, onClose: () => {} } });
    const desktopDialog = await screen.findByRole('dialog');
    expect(desktopDialog.className).toContain('inset-y-0');
    expect(desktopDialog.className).toContain('border-l-2');
    desktop.unmount();
  });

  it('keeps registry depth bounded and back dismiss order child then parent', () => {
    const registry = createDrawerRegistry();

    expect(registry.openParent('parent')).toBe(true);
    expect(registry.openChild('child')).toBe(true);
    expect(registry.openChild('overflow')).toBe(false);
    expect(registry.depth).toBe(2);

    expect(registry.closeTop('back')?.id).toBe('child');
    expect(registry.closeTop('back')?.id).toBe('parent');
    expect(registry.closeTop('back')).toBeNull();
  });
});
