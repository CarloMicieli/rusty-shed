import { describe, expect, it, vi } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import SelectTrigger from '$lib/components/ui/select/select-trigger.svelte';

vi.mock('bits-ui', async () => {
  const module = await import('../../../../stubs/SelectTriggerPrimitiveStub.svelte');
  return {
    Select: {
      Trigger: module.default
    }
  };
});

describe('select-trigger', () => {
  it('sets data-size to sm and forwards custom class names', () => {
    render(SelectTrigger, {
      props: {
        size: 'sm',
        class: 'custom-trigger'
      }
    });

    const trigger = screen.getByTestId('select-trigger-primitive');
    expect(trigger).toHaveAttribute('data-size', 'sm');
    expect(trigger).toHaveAttribute('data-slot', 'select-trigger');
    expect(trigger.className).toContain('custom-trigger');
  });

  it('uses default size when no size prop is passed', () => {
    render(SelectTrigger);

    const trigger = screen.getByTestId('select-trigger-primitive');
    expect(trigger).toHaveAttribute('data-size', 'default');
  });
});
