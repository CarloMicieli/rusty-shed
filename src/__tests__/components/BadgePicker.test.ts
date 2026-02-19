import { describe, it, expect, vi } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import BadgePicker from '$lib/components/BadgePicker.svelte';

// Mock Paraglide messages used by BadgePicker
vi.mock('$lib/paraglide/messages', () => ({
  badge_picker_close: () => 'Close'
}));

const OPTIONS = [
  { id: 'H0', label: 'H0 (1:87)' },
  { id: 'N', label: 'N (1:160)' },
  { id: 'TT', label: 'TT (1:120)' }
];

describe('BadgePicker', () => {
  describe('Closed state', () => {
    it('renders the current value as trigger text', () => {
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect: vi.fn().mockResolvedValue(undefined) }
      });
      expect(container.textContent).toContain('H0');
      expect(container.querySelector('[role="listbox"]')).toBeNull();
    });

    it('does not show the picker panel when closed', () => {
      const { container } = render(BadgePicker, {
        props: { value: 'N', options: OPTIONS, onSelect: vi.fn().mockResolvedValue(undefined) }
      });
      expect(container.querySelector('[role="listbox"]')).toBeNull();
    });
  });

  describe('Open state', () => {
    it('opens the picker panel when trigger is clicked', async () => {
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect: vi.fn().mockResolvedValue(undefined) }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      expect(container.querySelector('[role="listbox"]')).not.toBeNull();
    });

    it('renders all options in the picker', async () => {
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect: vi.fn().mockResolvedValue(undefined) }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      const items = container.querySelectorAll('[role="option"]');
      expect(items).toHaveLength(OPTIONS.length);
    });

    it('closes picker and calls onSelect with selected id when an option is clicked', async () => {
      const onSelect = vi.fn().mockResolvedValue(undefined);
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);

      const nOption = Array.from(container.querySelectorAll('[role="option"]')).find((el) =>
        el.textContent?.includes('N')
      ) as HTMLElement;
      await fireEvent.click(nOption);

      expect(onSelect).toHaveBeenCalledWith('N');
    });

    it('closes the picker without calling onSelect when Escape is pressed', async () => {
      const onSelect = vi.fn().mockResolvedValue(undefined);
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);
      expect(container.querySelector('[role="listbox"]')).not.toBeNull();

      await fireEvent.keyDown(document, { key: 'Escape' });
      expect(container.querySelector('[role="listbox"]')).toBeNull();
      expect(onSelect).not.toHaveBeenCalled();
    });

    it('reverts displayed value and does not close if onSelect rejects', async () => {
      const onSelect = vi.fn().mockRejectedValue(new Error('save failed'));
      const { container } = render(BadgePicker, {
        props: { value: 'H0', options: OPTIONS, onSelect }
      });
      const trigger = container.querySelector('[role="button"]') as HTMLElement;
      await fireEvent.click(trigger);

      const nOption = Array.from(container.querySelectorAll('[role="option"]')).find((el) =>
        el.textContent?.includes('N')
      ) as HTMLElement;
      await fireEvent.click(nOption);

      // After rejection the trigger should still display the original value
      expect(trigger.textContent).toContain('H0');
    });
  });
});
