import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import FormationForm from '$lib/features/train-formations/components/FormationForm.svelte';
import { makeCategory, makeDetail } from '../fixtures';

vi.mock('$lib/paraglide/messages.js', () => ({
  formations_form_name_label: () => 'Name',
  formations_form_name_placeholder: () => 'Formation name',
  formations_form_category_label: () => 'Category',
  formations_form_start_year_label: () => 'Start year',
  formations_form_end_year_label: () => 'End year',
  formations_form_epoch_label: () => 'Epoch',
  formations_form_notes_label: () => 'Notes',
  formations_form_notes_placeholder: () => 'Write some notes',
  formations_cancel: () => 'Cancel',
  formations_save: () => 'Save'
}));

describe('FormationForm.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders the provided initial values', async () => {
    render(FormationForm, {
      props: {
        categories: [makeCategory()],
        initial: makeDetail({
          name: 'TEE Helvetia',
          epoch: 'IV',
          start_year: 1971,
          end_year: 1979,
          notes: 'International service'
        }),
        onsubmit: vi.fn().mockResolvedValue(undefined),
        oncancel: vi.fn()
      }
    });

    await waitFor(() => {
      expect(screen.getByDisplayValue('TEE Helvetia')).toBeInTheDocument();
      // EpochPicker renders pill buttons; the selected epoch has the primary highlight class
      expect(screen.getByRole('button', { name: 'IV' }).className).toContain('border-primary');
      expect(screen.getByDisplayValue('1971')).toBeInTheDocument();
      expect(screen.getByDisplayValue('1979')).toBeInTheDocument();
      expect(screen.getByDisplayValue('International service')).toBeInTheDocument();
    });
  });

  it('shows a validation error when the name is blank', async () => {
    const onsubmit = vi.fn().mockResolvedValue(undefined);
    const { container } = render(FormationForm, {
      props: {
        categories: [],
        onsubmit,
        oncancel: vi.fn()
      }
    });

    await fireEvent.submit(container.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(screen.getByText('Name is required')).toBeInTheDocument();
    });
    expect(onsubmit).not.toHaveBeenCalled();
  });

  it('shows a validation error when end year is before start year', async () => {
    const onsubmit = vi.fn().mockResolvedValue(undefined);
    const { container } = render(FormationForm, {
      props: {
        categories: [],
        onsubmit,
        oncancel: vi.fn()
      }
    });

    await fireEvent.input(screen.getByLabelText('Name'), { target: { value: 'EC 15' } });
    await fireEvent.input(screen.getByLabelText('Start year'), { target: { value: '1985' } });
    await fireEvent.input(screen.getByLabelText('End year'), { target: { value: '1980' } });
    await fireEvent.submit(container.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(screen.getByText('End year must be after start year')).toBeInTheDocument();
    });
    expect(onsubmit).not.toHaveBeenCalled();
  });

  it('submits trimmed values and preserves populated optional fields', async () => {
    const onsubmit = vi.fn().mockResolvedValue(undefined);
    const { container } = render(FormationForm, {
      props: {
        categories: [],
        onsubmit,
        oncancel: vi.fn()
      }
    });

    await fireEvent.input(screen.getByLabelText('Name'), { target: { value: '  EC 22  ' } });
    await fireEvent.input(screen.getByLabelText('Start year'), { target: { value: '1988' } });
    await fireEvent.input(screen.getByLabelText('End year'), { target: { value: '1991' } });
    // EpochPicker: click the "V" pill button to select it
    await fireEvent.click(screen.getByRole('button', { name: 'V' }));
    await fireEvent.input(screen.getByLabelText('Notes'), {
      target: { value: '  Cross-border service  ' }
    });
    await fireEvent.submit(container.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(onsubmit).toHaveBeenCalledWith({
        name: 'EC 22',
        category_id: null,
        epoch: 'V',
        start_year: 1988,
        end_year: 1991,
        notes: 'Cross-border service'
      });
    });
  });

  it('normalizes empty optional fields to null on submit', async () => {
    const onsubmit = vi.fn().mockResolvedValue(undefined);
    const { container } = render(FormationForm, {
      props: {
        categories: [],
        onsubmit,
        oncancel: vi.fn()
      }
    });

    await fireEvent.input(screen.getByLabelText('Name'), { target: { value: 'Night Express' } });
    await fireEvent.submit(container.querySelector('form') as HTMLFormElement);

    await waitFor(() => {
      expect(onsubmit).toHaveBeenCalledWith({
        name: 'Night Express',
        category_id: null,
        epoch: null,
        start_year: null,
        end_year: null,
        notes: null
      });
    });
  });

  it('calls oncancel when the cancel button is pressed', async () => {
    const oncancel = vi.fn();
    render(FormationForm, {
      props: {
        categories: [],
        onsubmit: vi.fn().mockResolvedValue(undefined),
        oncancel
      }
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
    expect(oncancel).toHaveBeenCalledTimes(1);
  });
});
