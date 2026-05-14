import { describe, expect, it } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import AlertHarness from '../../../../stubs/AlertHarness.svelte';

describe('Alert', () => {
  it('renders warning variant with title and description composition', () => {
    render(AlertHarness, { props: { variant: 'warning' } });

    const alert = screen.getByRole('alert');
    expect(alert.className).toContain('bg-yellow-50');
    expect(screen.getByText('Heads up')).toHaveClass('alert-title');
    expect(screen.getByText('Reactive alert body')).toHaveClass('alert-description');
  });

  it('supports transient mount and dismiss cycles', async () => {
    render(AlertHarness);

    expect(screen.getByRole('alert')).toBeInTheDocument();

    await fireEvent.click(screen.getByTestId('toggle-alert'));
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();

    await fireEvent.click(screen.getByTestId('toggle-alert'));
    expect(screen.getByRole('alert')).toBeInTheDocument();
  });
});
