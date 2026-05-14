import { describe, expect, it } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import CheckboxHarness from '../../../../stubs/CheckboxHarness.svelte';

describe('Checkbox', () => {
  it('syncs bound state and emits onchange on click', async () => {
    render(CheckboxHarness);

    const checkbox = screen.getByRole('checkbox', { name: 'Enable feature' });
    expect(screen.getByTestId('checkbox-state')).toHaveTextContent('unchecked');

    await fireEvent.click(checkbox);

    expect(screen.getByTestId('checkbox-state')).toHaveTextContent('checked');
    expect(screen.getByTestId('checkbox-changes')).toHaveTextContent('1');
  });

  it('handles indeterminate checkbox input by resolving to checked on user click', async () => {
    render(CheckboxHarness);

    const checkbox = screen.getByRole('checkbox', { name: 'Enable feature' }) as HTMLInputElement;
    checkbox.indeterminate = true;
    expect(checkbox.indeterminate).toBe(true);

    await fireEvent.click(checkbox);

    expect(screen.getByTestId('checkbox-state')).toHaveTextContent('checked');
  });
});
