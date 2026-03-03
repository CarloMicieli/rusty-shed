import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';

// Mock Paraglide messages used by InPlaceEdit
vi.mock('$lib/paraglide/messages', () => ({
  edit_field_save: () => 'Save',
  edit_field_cancel: () => 'Cancel',
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save. Your changes are preserved.'
}));

describe('InPlaceEdit', () => {
  describe('Idle state', () => {
    it('renders value as plain text when not editing', () => {
      const { container } = render(InPlaceEdit, {
        props: { value: 'My description', onSave: vi.fn().mockResolvedValue(undefined) }
      });
      expect(container.textContent).toContain('My description');
      expect(container.querySelector('input')).toBeNull();
    });

    it('shows placeholder text when value is empty', () => {
      const { container } = render(InPlaceEdit, {
        props: { value: '', onSave: vi.fn().mockResolvedValue(undefined) }
      });
      expect(container.textContent).toContain('Click to add...');
    });

    it('shows custom placeholder when provided', () => {
      const { container } = render(InPlaceEdit, {
        props: {
          value: '',
          placeholder: 'Enter details…',
          onSave: vi.fn().mockResolvedValue(undefined)
        }
      });
      expect(container.textContent).toContain('Enter details…');
    });

    it('clicking the idle area switches to editing mode', async () => {
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', onSave: vi.fn().mockResolvedValue(undefined) }
      });
      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);
      expect(container.querySelector('input')).not.toBeNull();
    });

    it('renders a textarea in multiline mode when editing', async () => {
      const { container } = render(InPlaceEdit, {
        props: { value: 'Line one', multiline: true, onSave: vi.fn().mockResolvedValue(undefined) }
      });
      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);
      expect(container.querySelector('textarea')).not.toBeNull();
      expect(container.querySelector('input')).toBeNull();
    });
  });

  describe('blur triggers onSave', () => {
    it('blur on input calls onSave with the new value', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', onSave }
      });

      // Click to enter editing mode
      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      // Change the value
      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'New value' } });

      // Blur to trigger save
      await fireEvent.blur(input);

      expect(onSave).toHaveBeenCalledWith('New value');
    });

    it('blur with unchanged value still calls onSave', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceEdit, {
        props: { value: 'Same', onSave }
      });

      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.blur(input);

      expect(onSave).toHaveBeenCalledWith('Same');
    });
  });

  describe('Escape key cancels without saving', () => {
    it('Escape restores original value and exits editing without calling onSave', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', onSave }
      });

      // Enter editing mode
      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      // Type a new value
      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'Changed' } });

      // Press Escape
      await fireEvent.keyDown(input, { key: 'Escape', code: 'Escape' });

      expect(onSave).not.toHaveBeenCalled();
      // Should be back in idle state showing original value
      expect(container.querySelector('input')).toBeNull();
      expect(container.textContent).toContain('Original');
    });

    it('Escape on multiline textarea also cancels without saving', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', multiline: true, onSave }
      });

      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      const textarea = container.querySelector('textarea') as HTMLTextAreaElement;
      await fireEvent.keyDown(textarea, { key: 'Escape', code: 'Escape' });

      expect(onSave).not.toHaveBeenCalled();
      expect(container.querySelector('textarea')).toBeNull();
    });
  });

  describe('failed onSave shows error and preserves editing state', () => {
    it('rejected onSave shows error message with preserved unsaved value', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('Network error'));
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', onSave }
      });

      // Enter editing mode
      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      // Type a new value
      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'New value' } });

      // Blur to trigger save (which will reject)
      await fireEvent.blur(input);

      // Wait for the async save rejection to be processed
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Error message should be shown
      const alert = container.querySelector('[role="alert"]');
      expect(alert).not.toBeNull();
      expect(container.textContent).toContain('Failed to save. Your changes are preserved.');

      // Component should remain in editing state (input still visible)
      expect(container.querySelector('input')).not.toBeNull();
    });

    it('after failed save, Escape restores original value and exits editing', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('Network error'));
      const { container } = render(InPlaceEdit, {
        props: { value: 'Original', onSave }
      });

      const display = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(display);

      const input = container.querySelector('input') as HTMLInputElement;
      await fireEvent.input(input, { target: { value: 'New value' } });
      await fireEvent.blur(input);
      await new Promise((resolve) => setTimeout(resolve, 0));

      // Press Escape to cancel and exit editing
      const inputAfterError = container.querySelector('input') as HTMLInputElement;
      await fireEvent.keyDown(inputAfterError, { key: 'Escape', code: 'Escape' });

      // Should be back in idle state showing original value
      expect(container.querySelector('input')).toBeNull();
      expect(container.textContent).toContain('Original');
    });
  });
});
