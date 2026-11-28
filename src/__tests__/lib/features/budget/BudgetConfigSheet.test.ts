import { cleanup, fireEvent, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import * as m from '$lib/paraglide/messages';
import BudgetConfigSheet from '$lib/features/budget/components/BudgetConfigSheet.svelte';

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { locale: 'en-US' }
}));

describe('BudgetConfigSheet', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders and updates derived monthly/yearly values when switching mode', async () => {
    render(BudgetConfigSheet, {
      props: {
        open: true,
        mode: 'YEARLY',
        baseAmount: 120000,
        currency: 'EUR',
        onsubmit: vi.fn()
      }
    });

    expect(screen.getByText(m.budget_config_title())).toBeInTheDocument();
    expect(screen.getByText('€1,200.00')).toBeInTheDocument();
    expect(screen.getByText('€100.00')).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: m.budget_config_mode_monthly() }));

    expect(screen.getByText('€1,200.00')).toBeInTheDocument();
  });

  it('submits major units as minor units and calls onSave', async () => {
    const onsubmit = vi.fn();
    const onSave = vi.fn();

    render(BudgetConfigSheet, {
      props: {
        open: true,
        mode: 'MONTHLY',
        baseAmount: 0,
        currency: 'EUR',
        onsubmit,
        onSave
      }
    });

    const amountInput = screen.getByPlaceholderText(m.placeholder_amount());
    await fireEvent.input(amountInput, { target: { value: '45.67' } });

    await fireEvent.submit(document.querySelector('form') as HTMLFormElement);

    expect(onsubmit).toHaveBeenCalledWith('MONTHLY', 4567);
    expect(onSave).toHaveBeenCalledOnce();
  });

  it('disables controls and shows saving state while saving', () => {
    render(BudgetConfigSheet, {
      props: {
        open: true,
        mode: 'MONTHLY',
        baseAmount: 1000,
        saving: true,
        onsubmit: vi.fn()
      }
    });

    expect(screen.getByRole('button', { name: m.budget_config_saving_button() })).toBeDisabled();
    expect(screen.getByPlaceholderText(m.placeholder_amount())).toBeDisabled();
  });
});
