import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import Sonner from '$lib/components/ui/sonner/sonner.svelte';

vi.mock('mode-watcher', () => ({
  mode: {
    current: 'dark'
  }
}));

vi.mock('svelte-sonner', async () => {
  const module = await import('../../../../stubs/SonnerToasterStub.svelte');
  return {
    Toaster: module.default
  };
});

describe('sonner wrapper', () => {
  it('forwards theme and static styling to the underlying toaster', () => {
    render(Sonner, {
      props: {
        position: 'top-right',
        richColors: true
      }
    });

    const root = screen.getByTestId('sonner-root');
    expect(root).toHaveAttribute('data-theme', 'dark');
    expect(root).toHaveAttribute('data-class', 'toaster group');
    expect(root.getAttribute('data-style')).toContain('--normal-bg: var(--color-popover)');
    expect(root.getAttribute('data-rest')).toContain('"position":"top-right"');
    expect(root.getAttribute('data-rest')).toContain('"richColors":true');
  });

  it('provides all status icon snippets', () => {
    render(Sonner);

    expect(screen.getByTestId('sonner-loading').querySelector('svg')).not.toBeNull();
    expect(screen.getByTestId('sonner-success').querySelector('svg')).not.toBeNull();
    expect(screen.getByTestId('sonner-error').querySelector('svg')).not.toBeNull();
    expect(screen.getByTestId('sonner-info').querySelector('svg')).not.toBeNull();
    expect(screen.getByTestId('sonner-warning').querySelector('svg')).not.toBeNull();
  });
});
