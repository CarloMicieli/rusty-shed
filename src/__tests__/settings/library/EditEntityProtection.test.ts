import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import EntityTable from '$lib/features/settings/components/library/EntityTable.svelte';

describe('EditEntityProtection', () => {
  it('disables edit action for protected entities', () => {
    const onEdit = vi.fn();

    render(EntityTable, {
      props: {
        rows: [
          {
            id: 'm-seeded',
            name: 'Seeded Manufacturer',
            countryCode: 'IT',
            usageCount: 2,
            isSystemSeeded: true
          }
        ],
        onEdit
      }
    });

    const editButton = screen.getByRole('button', { name: 'Edit Seeded Manufacturer' });
    expect(screen.getByText('Protected')).toBeInTheDocument();
    expect(editButton).toBeDisabled();
  });

  it('opens edit action for user-created entities', async () => {
    const onEdit = vi.fn();

    const row = {
      id: 's-user',
      name: 'Model Shop',
      countryCode: 'DE',
      usageCount: 0,
      isSystemSeeded: false
    };

    render(EntityTable, {
      props: {
        rows: [row],
        onEdit
      }
    });

    const editButton = screen.getByRole('button', { name: 'Edit Model Shop' });
    await fireEvent.click(editButton);

    expect(screen.getByText('Unused')).toBeInTheDocument();
    expect(onEdit).toHaveBeenCalledWith(row);
  });
});
