import { describe, it, expect } from 'vitest';
import { render, fireEvent } from '@testing-library/svelte';
import SearchableSelectTest from './SearchableSelectTest.svelte';

describe('SearchableSelect with Snippets', () => {
  it('renders a custom trigger using the trigger snippet', () => {
    const { getByTestId } = render(SearchableSelectTest);
    const trigger = getByTestId('custom-trigger');
    expect(trigger).toBeDefined();
    expect(trigger.textContent).toBe('Italy');
  });

  it('renders custom items using the item snippet when opened', async () => {
    const { getByRole, getByTestId } = render(SearchableSelectTest);
    const triggerBtn = getByRole('button');
    await fireEvent.click(triggerBtn);

    const itItem = getByTestId('custom-item-IT');
    const deItem = getByTestId('custom-item-DE');

    expect(itItem.textContent).toBe('Italy (IT)');
    expect(deItem.textContent).toBe('Germany (DE)');
  });
});
