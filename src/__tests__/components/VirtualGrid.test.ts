import { cleanup, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import VirtualGridHarness from './VirtualGridHarness.svelte';

class ResizeObserverMock {
  observe = vi.fn();
  disconnect = vi.fn();
}

describe('VirtualGrid', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
    vi.stubGlobal('ResizeObserver', ResizeObserverMock);
    Object.defineProperty(HTMLElement.prototype, 'offsetWidth', {
      configurable: true,
      get: () => 480
    });
  });

  it('renders only items in the virtual window', () => {
    const { container } = render(VirtualGridHarness, {
      props: { items: ['A', 'B', 'C', 'D', 'E', 'F'] }
    });

    expect(container.querySelector('[style*="position: relative"]')).not.toBeNull();
    expect(screen.queryByText('A')).toBeInTheDocument();
  });

  it('handles an empty item list', () => {
    const { container } = render(VirtualGridHarness, {
      props: { items: [] }
    });

    expect(container.querySelector('[style*="position: relative"]')).not.toBeNull();
  });

  it('renders all items for a small list within the viewport', () => {
    render(VirtualGridHarness, {
      props: { items: ['A', 'B'] }
    });

    expect(screen.getByText('A')).toBeInTheDocument();
    expect(screen.getByText('B')).toBeInTheDocument();
  });
});
