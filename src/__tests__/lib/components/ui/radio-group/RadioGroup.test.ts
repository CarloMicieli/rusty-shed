import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import RadioGroup from '$lib/components/ui/radio-group/RadioGroup.svelte';

describe('RadioGroup', () => {
  const options = [
    { value: 'diesel', label: 'Diesel' },
    { value: 'electric', label: 'Electric' },
    { value: 'steam', label: 'Steam', disabled: true }
  ];

  it('checks only the selected option and updates on click', async () => {
    render(RadioGroup, {
      props: {
        value: 'diesel',
        name: 'traction',
        options
      }
    });

    const diesel = screen.getByRole('radio', { name: 'Diesel' });
    const electric = screen.getByRole('radio', { name: 'Electric' });

    expect(diesel).toBeChecked();
    expect(electric).not.toBeChecked();

    await fireEvent.click(electric);

    expect(diesel).not.toBeChecked();
    expect(electric).toBeChecked();
  });

  it('calls onchange with the newly selected value', async () => {
    const onchange = vi.fn();
    render(RadioGroup, {
      props: {
        value: 'diesel',
        name: 'traction',
        options,
        onchange
      }
    });

    await fireEvent.click(screen.getByRole('radio', { name: 'Electric' }));
    expect(onchange).toHaveBeenCalledWith('electric');
  });

  it('disables inputs when group is disabled or option is disabled', () => {
    const { rerender } = render(RadioGroup, {
      props: {
        value: 'diesel',
        name: 'traction',
        options
      }
    });

    expect(screen.getByRole('radio', { name: 'Steam' })).toBeDisabled();
    expect(screen.getByRole('radio', { name: 'Diesel' })).toBeEnabled();

    rerender({
      value: 'diesel',
      name: 'traction',
      options,
      disabled: true
    });

    expect(screen.getByRole('radio', { name: 'Diesel' })).toBeDisabled();
    expect(screen.getByRole('radio', { name: 'Electric' })).toBeDisabled();
  });
});
