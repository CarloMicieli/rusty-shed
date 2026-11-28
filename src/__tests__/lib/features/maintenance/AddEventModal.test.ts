import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

const maintenanceStateMock = vi.hoisted(() => ({
  card: { nextMaintenanceDate: '2026-12-01' },
  addEvent: vi.fn().mockResolvedValue(undefined)
}));

vi.mock('$lib/features/maintenance/MaintenanceDetailState.svelte', () => ({
  getMaintenanceDetailState: () => maintenanceStateMock
}));

vi.mock('$lib/features/maintenance/utils/maintenanceTypes', () => ({
  getMaintenanceTypes: () => [
    { value: 'cleaning', label: 'Cleaning' },
    { value: 'lubrication', label: 'Lubrication' }
  ]
}));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

import AddEventModal from '$lib/features/maintenance/components/AddEventModal.svelte';

describe('AddEventModal', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders modal fields and initializes next maintenance date from card state', () => {
    render(AddEventModal, {
      props: {
        open: true,
        onClose: vi.fn(),
        maintenanceCardId: 'card-1'
      }
    });

    expect(screen.getByText('maintenance_add_event_title')).toBeInTheDocument();
    expect(screen.getByLabelText('maintenance_add_event_type_label')).toBeInTheDocument();
    expect(screen.getByLabelText('maintenance_schedule_next_date_label')).toHaveValue('2026-12-01');
  });

  it('submits a valid event payload and closes the modal', async () => {
    const onClose = vi.fn();
    render(AddEventModal, {
      props: {
        open: true,
        onClose,
        maintenanceCardId: 'card-1'
      }
    });

    await fireEvent.change(screen.getByLabelText('maintenance_add_event_type_label'), {
      target: { value: 'cleaning' }
    });
    await fireEvent.input(screen.getByLabelText('maintenance_schedule_next_date_label'), {
      target: { value: '2026-12-15' }
    });
    await fireEvent.input(screen.getByLabelText('maintenance_add_event_notes_label'), {
      target: { value: '  Checked wheelsets  ' }
    });
    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(maintenanceStateMock.addEvent).toHaveBeenCalledWith(
        expect.objectContaining({
          maintenanceCardId: 'card-1',
          maintenanceType: 'cleaning',
          nextMaintenanceDate: '2026-12-15',
          notes: 'Checked wheelsets'
        })
      );
    });
    expect(onClose).toHaveBeenCalled();
  });

  it('shows backend errors and keeps modal open when submit fails', async () => {
    maintenanceStateMock.addEvent.mockRejectedValueOnce(new Error('Cannot save event'));
    const onClose = vi.fn();

    render(AddEventModal, {
      props: {
        open: true,
        onClose,
        maintenanceCardId: 'card-1'
      }
    });

    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    expect(await screen.findByText('Cannot save event')).toBeInTheDocument();
    expect(onClose).not.toHaveBeenCalled();
  });

  it('resets the form and closes when pressing the close button', async () => {
    const onClose = vi.fn();

    render(AddEventModal, {
      props: {
        open: true,
        onClose,
        maintenanceCardId: 'card-1'
      }
    });

    await fireEvent.input(screen.getByLabelText('maintenance_add_event_notes_label'), {
      target: { value: 'Temporary note' }
    });
    await fireEvent.click(screen.getByRole('button', { name: 'dialog_close_button' }));

    expect(onClose).toHaveBeenCalled();
  });
});
