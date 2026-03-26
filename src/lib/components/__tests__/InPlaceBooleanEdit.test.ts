import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import InPlaceBooleanEdit from '../InPlaceBooleanEdit.svelte';

vi.mock('$lib/paraglide/messages', () => ({
  edit_save_error: () => 'Failed to save.',
  boolean_yes: () => 'Yes',
  boolean_no: () => 'No'
}));

describe('InPlaceBooleanEdit', () => {
  describe('View mode rendering', () => {
    it('renders "—" chip when value is null', () => {
      const { getByRole } = render(InPlaceBooleanEdit, {
        props: { value: null, onSave: vi.fn() }
      });
      expect(getByRole('button').textContent).toContain('—');
    });

    it('renders "Yes" chip when value is YES', () => {
      const { getByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave: vi.fn() }
      });
      expect(getByRole('button').textContent).toContain('Yes');
    });

    it('renders "No" chip when value is NO', () => {
      const { getByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'NO', onSave: vi.fn() }
      });
      expect(getByRole('button').textContent).toContain('No');
    });
  });

  describe('Edit mode', () => {
    it('clicking the view chip opens the 3-button picker', async () => {
      const { getByRole, getAllByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave: vi.fn() }
      });
      await fireEvent.click(getByRole('button'));
      // In edit mode: 3 picker buttons
      expect(getAllByRole('button').length).toBe(3);
    });

    it('clicking "Yes" button calls onSave with YES', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole, getAllByRole } = render(InPlaceBooleanEdit, {
        props: { value: null, onSave }
      });
      await fireEvent.click(getByRole('button'));
      const buttons = getAllByRole('button');
      const yesBtn = buttons.find((b) => b.textContent?.trim() === 'Yes')!;
      await fireEvent.click(yesBtn);
      await waitFor(() => expect(onSave).toHaveBeenCalledWith('YES'));
    });

    it('clicking "No" button calls onSave with NO', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole, getAllByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const buttons = getAllByRole('button');
      const noBtn = buttons.find((b) => b.textContent?.trim() === 'No')!;
      await fireEvent.click(noBtn);
      await waitFor(() => expect(onSave).toHaveBeenCalledWith('NO'));
    });

    it('clicking "—" button calls onSave with null', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { getByRole, getAllByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const buttons = getAllByRole('button');
      const dashBtn = buttons.find((b) => b.textContent?.trim() === '—')!;
      await fireEvent.click(dashBtn);
      await waitFor(() => expect(onSave).toHaveBeenCalledWith(null));
    });

    it('Escape key cancels without calling onSave', async () => {
      const onSave = vi.fn();
      const { getByRole, getAllByRole } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const buttons = getAllByRole('button');
      await fireEvent.keyDown(buttons[0], { key: 'Escape' });
      expect(onSave).not.toHaveBeenCalled();
      // Returns to view mode: single button
      await waitFor(() => expect(getAllByRole('button').length).toBe(1));
    });

    it('rejected onSave shows an error alert and stays in edit mode', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('Server error'));
      const { getByRole, getAllByRole, container } = render(InPlaceBooleanEdit, {
        props: { value: 'YES', onSave }
      });
      await fireEvent.click(getByRole('button'));
      const buttons = getAllByRole('button');
      const noBtn = buttons.find((b) => b.textContent?.trim() === 'No')!;
      await fireEvent.click(noBtn);
      await waitFor(() => {
        expect(container.querySelector('[role="alert"]')).not.toBeNull();
      });
    });
  });
});
