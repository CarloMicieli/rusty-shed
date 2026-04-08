import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import InPlaceSelectEdit from '$lib/components/InPlaceSelectEdit.svelte';

vi.mock('$lib/paraglide/messages', () => ({
  edit_save_error: () => 'Failed to save',
  edit_field_placeholder_empty: () => '—'
}));

const OPTIONS = [
  { value: 'PLASTIC', label: 'Plastic' },
  { value: 'METAL', label: 'Metal' },
  { value: 'COMPOSITE', label: 'Composite' }
];

const BASE_PROPS = {
  value: 'PLASTIC',
  displayLabel: 'Plastic',
  options: OPTIONS,
  onSave: vi.fn().mockResolvedValue(undefined)
};

describe('InPlaceSelectEdit', () => {
  describe('Closed state', () => {
    it('renders the display label as trigger text', () => {
      const { container } = render(InPlaceSelectEdit, { props: BASE_PROPS });
      expect(container.textContent).toContain('Plastic');
      expect(container.querySelector('[role="listbox"]')).toBeNull();
    });

    it('shows placeholder when value is empty', () => {
      const { container } = render(InPlaceSelectEdit, {
        props: { ...BASE_PROPS, value: '', displayLabel: '', placeholder: 'Select…' }
      });
      expect(container.textContent).toContain('Select…');
    });

    it('does not render a listbox when closed', () => {
      const { container } = render(InPlaceSelectEdit, { props: BASE_PROPS });
      expect(container.querySelector('[role="listbox"]')).toBeNull();
    });
  });

  describe('Open state', () => {
    it('opens the floating panel when trigger is clicked', async () => {
      const { container } = render(InPlaceSelectEdit, { props: BASE_PROPS });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      expect(container.querySelector('[role="listbox"]')).not.toBeNull();
    });

    it('renders all options in the panel', async () => {
      const { container } = render(InPlaceSelectEdit, { props: BASE_PROPS });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      const items = container.querySelectorAll('[role="option"]');
      expect(items).toHaveLength(OPTIONS.length);
    });

    it('calls onSave and closes panel when an option is clicked', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceSelectEdit, { props: { ...BASE_PROPS, onSave } });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);

      const metalOption = Array.from(container.querySelectorAll('[role="option"]')).find((el) =>
        el.textContent?.includes('Metal')
      ) as HTMLElement;
      await fireEvent.click(metalOption);

      expect(onSave).toHaveBeenCalledWith('METAL');
    });

    it('closes the panel without saving when Escape is pressed', async () => {
      const onSave = vi.fn().mockResolvedValue(undefined);
      const { container } = render(InPlaceSelectEdit, { props: { ...BASE_PROPS, onSave } });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      expect(container.querySelector('[role="listbox"]')).not.toBeNull();

      await fireEvent.keyDown(document, { key: 'Escape' });
      expect(container.querySelector('[role="listbox"]')).toBeNull();
      expect(onSave).not.toHaveBeenCalled();
    });

    it('calls onActivate when opened and onDeactivate when closed', async () => {
      const onActivate = vi.fn();
      const onDeactivate = vi.fn();
      const { container } = render(InPlaceSelectEdit, {
        props: {
          ...BASE_PROPS,
          onSave: vi.fn().mockResolvedValue(undefined),
          onActivate,
          onDeactivate
        }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      expect(onActivate).toHaveBeenCalledOnce();

      await fireEvent.keyDown(document, { key: 'Escape' });
      expect(onDeactivate).toHaveBeenCalledOnce();
    });

    it('shows an error message if onSave rejects', async () => {
      const onSave = vi.fn().mockRejectedValue(new Error('save failed'));
      const { container } = render(InPlaceSelectEdit, { props: { ...BASE_PROPS, onSave } });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);

      const metalOption = Array.from(container.querySelectorAll('[role="option"]')).find((el) =>
        el.textContent?.includes('Metal')
      ) as HTMLElement;
      await fireEvent.click(metalOption);

      // Wait for the async onSave to reject and error to render
      await vi.waitFor(() => {
        expect(container.querySelector('[role="alert"]')).not.toBeNull();
      });
      expect(container.querySelector('[role="alert"]')?.textContent).toContain('Failed to save');
    });
  });
});
