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
});
