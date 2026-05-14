import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import CardHarness from '../../../../stubs/CardHarness.svelte';

describe('Card composition', () => {
  it('renders header, title, description, content and footer in one composed card', () => {
    render(CardHarness);

    const title = screen.getByText('Plan');
    const description = screen.getByText('Current sprint');
    const content = screen.getByText('Body content');
    const action = screen.getByRole('button', { name: 'Save' });

    expect(title.className).toContain('custom-title');
    expect(description.className).toContain('custom-description');
    expect(content.parentElement?.className).toContain('custom-content');
    expect(action.closest('div')?.className).toContain('custom-footer');
  });

  it('omits footer when composition disables it', () => {
    render(CardHarness, { props: { includeFooter: false } });

    expect(screen.queryByRole('button', { name: 'Save' })).not.toBeInTheDocument();
  });
});
