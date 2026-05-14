import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import FormButton from '$lib/components/ui/form/form-button.svelte';

describe('form-button', () => {
  it('renders a submit button by default', () => {
    render(FormButton, {
      props: {
        'aria-label': 'save-form'
      }
    });

    const button = screen.getByRole('button', { name: 'save-form' });
    expect(button).toHaveAttribute('type', 'submit');
  });

  it('keeps an explicit button type override', () => {
    render(FormButton, {
      props: {
        type: 'button',
        disabled: true,
        'aria-label': 'cancel-form'
      }
    });

    const button = screen.getByRole('button', { name: 'cancel-form' });
    expect(button).toHaveAttribute('type', 'button');
    expect(button).toBeDisabled();
  });
});
