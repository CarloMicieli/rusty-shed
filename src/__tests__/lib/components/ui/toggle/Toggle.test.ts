import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import Toggle from '$lib/components/ui/toggle/Toggle.svelte';

describe('Toggle', () => {
  it('toggles aria-checked state on click', async () => {
    render(Toggle, {
      props: {
        'aria-label': 'Enable sync'
      }
    });

    const toggle = screen.getByRole('switch', { name: 'Enable sync' });
    expect(toggle).toHaveAttribute('aria-checked', 'false');

    await fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-checked', 'true');

    await fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-checked', 'false');
  });

  it('calls onclick callback with the next pressed state', async () => {
    const onclick = vi.fn();
    render(Toggle, {
      props: {
        'aria-label': 'Enable logs',
        onclick
      }
    });

    const toggle = screen.getByRole('switch', { name: 'Enable logs' });
    await fireEvent.click(toggle);
    await fireEvent.click(toggle);

    expect(onclick).toHaveBeenNthCalledWith(1, true);
    expect(onclick).toHaveBeenNthCalledWith(2, false);
  });

  it('does not toggle when disabled', async () => {
    const onclick = vi.fn();
    render(Toggle, {
      props: {
        'aria-label': 'Disable persistence',
        disabled: true,
        pressed: true,
        onclick
      }
    });

    const toggle = screen.getByRole('switch', { name: 'Disable persistence' });
    expect(toggle).toBeDisabled();
    expect(toggle).toHaveAttribute('aria-checked', 'true');

    await fireEvent.click(toggle);

    expect(toggle).toHaveAttribute('aria-checked', 'true');
    expect(onclick).not.toHaveBeenCalled();
  });
});
