import { describe, expect, it } from 'vitest';
import { fireEvent, render, screen } from '@testing-library/svelte';
import DataTableExample from '$lib/components/examples/DataTableExample.svelte';

describe('DataTableExample', () => {
  it('sorts by name descending on second click', async () => {
    const { container } = render(DataTableExample);

    const nameSort = screen.getByRole('button', { name: /Name/i });
    await fireEvent.click(nameSort);
    await fireEvent.click(nameSort);

    const bodyRows = container.querySelectorAll('tbody tr');
    expect(bodyRows[0]?.textContent).toContain('Diana Prince');
    expect(bodyRows[bodyRows.length - 1]?.textContent).toContain('Alice Johnson');
  });

  it('switches sort field and resets direction to ascending', async () => {
    const { container } = render(DataTableExample);

    const nameSort = screen.getByRole('button', { name: /Name/i });
    await fireEvent.click(nameSort);
    await fireEvent.click(nameSort);

    const emailSort = screen.getByRole('button', { name: /Email/i });
    await fireEvent.click(emailSort);

    const bodyRows = container.querySelectorAll('tbody tr');
    expect(bodyRows[0]?.textContent).toContain('alice@example.com');
    expect(bodyRows[bodyRows.length - 1]?.textContent).toContain('diana@example.com');
  });
});
