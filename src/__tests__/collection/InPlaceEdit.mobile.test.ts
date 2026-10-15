import { describe, expect, it, vi } from 'vitest';
import { fireEvent, render } from '@testing-library/svelte';
import InPlaceEdit from '$lib/components/InPlaceEdit.svelte';

vi.mock('$lib/paraglide/messages', () => ({
  edit_field_placeholder_empty: () => 'Click to add...',
  edit_save_error: () => 'Failed to save'
}));

function mockMobile() {
  Object.defineProperty(window, 'matchMedia', {
    configurable: true,
    writable: true,
    value: (query: string): MediaQueryList => ({
      matches: query.includes('max-width: 767px'),
      media: query,
      onchange: null,
      addEventListener: () => {},
      removeEventListener: () => {},
      addListener: () => {},
      removeListener: () => {},
      dispatchEvent: () => false
    })
  });
}

describe('InPlaceEdit mobile suppression', () => {
  it('delegates to mobile sheet request instead of inline edit when disabled', async () => {
    mockMobile();

    const onRequestMobileEdit = vi.fn();
    const onSave = vi.fn().mockResolvedValue(undefined);

    const { container } = render(InPlaceEdit, {
      props: {
        value: 'Notes',
        onSave,
        disableInlineOnMobile: true,
        onRequestMobileEdit
      }
    });

    const trigger = container.querySelector('button') as HTMLButtonElement;
    await fireEvent.click(trigger);

    expect(onRequestMobileEdit).toHaveBeenCalledTimes(1);
    expect(container.querySelector('input')).toBeNull();
    expect(container.querySelector('textarea')).toBeNull();
  });
});
