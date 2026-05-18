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
        manufacturers: [
          { id: 'm1', name: 'Acme', countryCode: 'IT', usageCount: 0, isSystemSeeded: false }
        ],
        sellers: [{ id: 's1', name: 'Shop', countryCode: 'US', usageCount: 0, isSystemSeeded: false }],
        buyers: [
          { id: 'b1', name: 'Collector', countryCode: 'DE', usageCount: 0, isSystemSeeded: false }
        ]
      }
    });

    expect(screen.getAllByText('Acme').length).toBeGreaterThan(0);

    await fireEvent.click(screen.getByRole('tab', { name: 'Sellers' }));
    expect(onTabChange).toHaveBeenCalledWith('sellers');
  });
});
