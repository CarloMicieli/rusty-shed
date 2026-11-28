import { cleanup, render, screen } from '@testing-library/svelte';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import MaintenanceCardItem from '$lib/features/maintenance/components/MaintenanceCardItem.svelte';
import type { MaintenanceCardView } from '$lib/bindings';

vi.mock('$app/paths', () => ({
  resolve: (value: string) => value
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  maintenance_health_status_pending: () => 'Pending',
  maintenance_card_last_maintenance: () => 'Last maintenance',
  maintenance_card_due_date: () => 'Due date',
  maintenance_schedule_missing_due: () => 'Missing due date'
}));

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { locale: 'en-US' }
}));

function makeCard(overrides: Partial<MaintenanceCardView> = {}): MaintenanceCardView {
  return {
    id: 'card-1',
    displayInfo: {
      manufacturerName: 'Roco',
      productCode: 'BR185',
      seriesCode: '218',
      roadNumber: '218 217-8'
    },
    lastMaintenanceDate: '2026-01-01',
    nextMaintenanceDate: '2026-01-20',
    ...overrides
  } as MaintenanceCardView;
}

describe('MaintenanceCardItem', () => {
  beforeEach(() => {
    cleanup();
    vi.useFakeTimers();
    vi.setSystemTime(new Date('2026-01-10T10:00:00Z'));
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders overdue state and overdue days when due date is in the past', () => {
    render(MaintenanceCardItem, {
      props: {
        card: makeCard({ nextMaintenanceDate: '2026-01-05' })
      }
    });

    expect(screen.getByText('Days Overdue')).toBeInTheDocument();
    expect(screen.getByText('5')).toBeInTheDocument();
  });

  it('renders pending status when next maintenance date is missing', () => {
    render(MaintenanceCardItem, {
      props: {
        card: makeCard({ nextMaintenanceDate: null })
      }
    });

    expect(screen.getByText('Pending')).toBeInTheDocument();
    expect(screen.getByText('Missing due date')).toBeInTheDocument();
  });

  it('renders encoded maintenance details link', () => {
    render(MaintenanceCardItem, {
      props: {
        card: makeCard({ id: 'card with spaces' })
      }
    });

    const link = screen.getByRole('link');
    expect(link).toHaveAttribute('href', '/maintenance/card%20with%20spaces');
  });
});
