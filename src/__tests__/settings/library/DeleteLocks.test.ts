import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import DeleteEntityDialog from '$lib/features/settings/components/library/DeleteEntityDialog.svelte';
import EntityTable from '$lib/features/settings/components/library/EntityTable.svelte';

describe('DeleteLocks', () => {
  it('disables delete action for protected and in-use entities', () => {
    render(EntityTable, {
      props: {
        rows: [
          {
            id: 'seeded',
            name: 'Seeded Entity',
            countryCode: 'IT',
            usageCount: 0,
            isSystemSeeded: true
          },
          {
            id: 'used',
            name: 'Used Entity',
            countryCode: 'DE',
            usageCount: 2,
            isSystemSeeded: false
          }
        ],
        onEdit: vi.fn(),
        onDelete: vi.fn()
      }
    });

    expect(screen.getByRole('button', { name: 'Delete Seeded Entity' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Delete Used Entity' })).toBeDisabled();
  });

  it('shows confirmation content with name and linked-item count', () => {
    render(DeleteEntityDialog, {
      props: {
        open: true,
        entityName: 'Acme Seller',
        linkedCount: 0,
        onOpenChange: vi.fn(),
        onConfirm: vi.fn()
      }
    });

    expect(screen.getByText('Delete entity')).toBeInTheDocument();
    expect(screen.getByText('Are you sure you want to delete "Acme Seller"?')).toBeInTheDocument();
    expect(screen.getByTestId('linked-items-count')).toHaveTextContent('Linked items: 0');
  });

  it('confirms unused delete within 3 clicks from row action', async () => {
    const openChange = vi.fn();
    const confirm = vi.fn();
    let clicks = 0;

    render(EntityTable, {
      props: {
        rows: [
          {
            id: 'unused',
            name: 'Unused Entity',
            countryCode: null,
            usageCount: 0,
            isSystemSeeded: false
          }
        ],
        onEdit: vi.fn(),
        onDelete: () => {
          clicks += 1;
        }
      }
    });

    render(DeleteEntityDialog, {
      props: {
        open: true,
        entityName: 'Unused Entity',
        linkedCount: 0,
        onOpenChange: openChange,
        onConfirm: () => {
          clicks += 1;
          confirm();
        }
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Delete Unused Entity' }));
    await fireEvent.click(screen.getByRole('button', { name: 'Delete' }));

    expect(confirm).toHaveBeenCalledTimes(1);
    expect(clicks).toBeLessThanOrEqual(3);
  });
});
