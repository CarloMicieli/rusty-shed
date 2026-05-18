import { fireEvent, render, screen } from '@testing-library/svelte';
import { describe, expect, it, vi } from 'vitest';
import EntityTabs from '$lib/features/settings/components/library/EntityTabs.svelte';

describe('LibraryView', () => {
  it('renders tabs and switches between entity datasets', async () => {
    const onTabChange = vi.fn();

    render(EntityTabs, {
      props: {
        activeTab: 'manufacturers',
        onTabChange,
        onEdit: vi.fn(),
        onDelete: vi.fn(),
        onMerge: vi.fn(),
        rows: [{ id: 'm1', name: 'Acme', countryCode: 'IT', usageCount: 0, isSystemSeeded: false }],
        totalItems: 1,
        totalPages: 1,
        pageStart: 1,
        pageEnd: 1,
        currentPage: 1,
        onPageChange: vi.fn()
      }
    });

    expect(screen.getAllByText('Acme').length).toBeGreaterThan(0);

    await fireEvent.click(screen.getByRole('tab', { name: 'Sellers' }));
    expect(onTabChange).toHaveBeenCalledWith('sellers');
  });
});
