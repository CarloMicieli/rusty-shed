import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, waitFor, cleanup } from '@testing-library/svelte';
import InPlaceEdit from '../InPlaceEdit.svelte';

vi.mock('$lib/paraglide/messages', () => ({
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save.'
}));

describe('InPlaceEdit', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  // ── View mode ─────────────────────────────────────────────────────────────

  describe('View mode', () => {
    it('renders the value in view mode', () => {
      const { getByText } = render(InPlaceEdit, {
        props: { value: 'BR 218', onSave: vi.fn() }
      });
      expect(getByText('BR 218')).toBeInTheDocument();
    });

    it('renders placeholder when value is empty', () => {
      const { getByText } = render(InPlaceEdit, {
        props: { value: '', placeholder: 'Enter series…', onSave: vi.fn() }
      });
      expect(getByText('Enter series…')).toBeInTheDocument();
    });

    it('renders default placeholder when value is empty and no placeholder prop', () => {
      const { getByText } = render(InPlaceEdit, {
        props: { value: '', onSave: vi.fn() }
      });
      expect(getByText('Click to add...')).toBeInTheDocument();
    });

    it('renders displayValue override instead of raw value', () => {
      const { getByText, queryByText } = render(InPlaceEdit, {
        props: { value: '2024-01-15', displayValue: '15 Jan 2024', onSave: vi.fn() }
      });
      expect(getByText('15 Jan 2024')).toBeInTheDocument();
      expect(queryByText('2024-01-15')).toBeNull();
    });

    it('view trigger is a button element', () => {
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'test', onSave: vi.fn() }
      });
      expect(getByRole('button')).toBeInTheDocument();
    });
  });

  // ── Edit mode activation ──────────────────────────────────────────────────

  describe('Edit mode activation', () => {
    it('clicking the view button switches to edit mode (shows input)', async () => {
      const { getByRole, queryByRole } = render(InPlaceEdit, {
        props: { value: 'BR 218', onSave: vi.fn() }
      });
      await fireEvent.click(getByRole('button'));
      await waitFor(() => {
        expect(queryByRole('textbox')).not.toBeNull();
      });
    });

    it('edit input is pre-filled with current value', async () => {
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'BR 218', onSave: vi.fn() }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox') as HTMLInputElement);
      expect(input.value).toBe('BR 218');
    });

    it('calls onActivate when edit starts', async () => {
      const onActivate = vi.fn();
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'BR 218', onSave: vi.fn(), onActivate }
      });
      await fireEvent.click(getByRole('button'));
      await waitFor(() => expect(onActivate).toHaveBeenCalledOnce());
    });
  });

  // ── Save ─────────────────────────────────────────────────────────────────

  describe('Save behaviour', () => {
    it('blur commits the new value via onSave', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'old', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.input(input, { target: { value: 'new value' } });
      await fireEvent.blur(input);
      await waitFor(() => expect(onSave).toHaveBeenCalledWith('new value'));
    });

    it('Enter key commits the value', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'old', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.input(input, { target: { value: 'entered' } });
      await fireEvent.keyDown(input, { key: 'Enter' });
      await waitFor(() => expect(onSave).toHaveBeenCalledWith('entered'));
    });

    it('calls onDeactivate after successful save', async () => {
      const onDeactivate = vi.fn();
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'old', onSave, onDeactivate }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.blur(input);
      await waitFor(() => expect(onDeactivate).toHaveBeenCalledOnce());
    });

    it('returns to view mode after successful save', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole, queryByRole } = render(InPlaceEdit, {
        props: { value: 'old', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.blur(input);
      await waitFor(() => {
        expect(queryByRole('textbox')).toBeNull();
        expect(getByRole('button')).toBeInTheDocument();
      });
    });
  });

  // ── Cancel ───────────────────────────────────────────────────────────────

  describe('Cancel behaviour', () => {
    it('Escape key cancels without calling onSave', async () => {
      const onSave = vi.fn();
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'original', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.keyDown(input, { key: 'Escape' });
      expect(onSave).not.toHaveBeenCalled();
    });

    it('Escape returns to view mode', async () => {
      const { getByRole, queryByRole } = render(InPlaceEdit, {
        props: { value: 'original', onSave: vi.fn() }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.keyDown(input, { key: 'Escape' });
      await waitFor(() => {
        expect(queryByRole('textbox')).toBeNull();
        expect(getByRole('button')).toBeInTheDocument();
      });
    });

    it('calls onDeactivate on Escape', async () => {
      const onDeactivate = vi.fn();
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'original', onSave: vi.fn(), onDeactivate }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.keyDown(input, { key: 'Escape' });
      await waitFor(() => expect(onDeactivate).toHaveBeenCalledOnce());
    });
  });

  // ── Error handling ────────────────────────────────────────────────────────

  describe('Error handling', () => {
    it('shows error alert when onSave rejects', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('DB error'));
      const { getByRole, container } = render(InPlaceEdit, {
        props: { value: 'old', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.blur(input);
      await waitFor(() => {
        expect(container.querySelector('[role="alert"]')).not.toBeNull();
      });
    });

    it('stays in edit mode when onSave rejects', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('fail'));
      const { getByRole } = render(InPlaceEdit, {
        props: { value: 'old', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const input = await waitFor(() => getByRole('textbox'));
      await fireEvent.blur(input);
      await waitFor(() => {
        expect(getByRole('textbox')).toBeInTheDocument();
      });
    });
  });

  // ── Multiline mode ────────────────────────────────────────────────────────

  describe('Multiline mode', () => {
    it('renders a textarea when multiline=true', async () => {
      const { getByRole, container } = render(InPlaceEdit, {
        props: { value: 'some notes', multiline: true, onSave: vi.fn() }
      });
      await fireEvent.click(getByRole('button'));
      await waitFor(() => {
        expect(container.querySelector('textarea')).not.toBeNull();
      });
    });

    it('Enter does not submit in multiline mode', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole, container } = render(InPlaceEdit, {
        props: { value: 'line1', multiline: true, onSave }
      });
      await fireEvent.click(getByRole('button'));
      const textarea = await waitFor(
        () => container.querySelector('textarea') as HTMLTextAreaElement
      );
      await fireEvent.keyDown(textarea, { key: 'Enter' });
      // onSave should NOT be triggered by Enter in multiline mode
      expect(onSave).not.toHaveBeenCalled();
    });
  });
});
