import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, fireEvent, screen, cleanup } from '@testing-library/svelte';
import BudgetConfigForm from '$lib/features/budget/components/BudgetConfigForm.svelte';
import * as m from '$lib/paraglide/messages';

describe('BudgetConfigForm.svelte', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('renders correctly with default props', () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        currency: 'USD'
      }
    });

    const amountInput = screen.getByLabelText(new RegExp(m.budget_config_amount_label()));
    expect(amountInput).toBeInTheDocument();
    expect((amountInput as HTMLInputElement).value).toBe('0.00');
    expect(screen.getAllByText(m.budget_config_mode_monthly()).length).toBeGreaterThan(0);
  });

  it('populates initial amount based on baseAmount (minor units to major units) using $effect', () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        baseAmount: 15000 // 150.00 in major units
      }
    });

    const amountInput = screen.getByDisplayValue('150.00');
    expect(amountInput).toBeInTheDocument();
  });

  it('updates derived monthly and yearly display correctly when typing amount in MONTHLY mode', async () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        mode: 'MONTHLY'
      }
    });

    const input = screen.getByLabelText(new RegExp(m.budget_config_amount_label()));
    await fireEvent.input(input, { target: { value: '100' } });

    const allText = document.body.textContent;
    // In MONTHLY mode: monthlyDisplay is 100, yearlyDisplay is 1200
    expect(allText).toContain('100.00');
    expect(allText).toMatch(/1[,.\\s]*200/);
  });

  it('updates derived monthly and yearly display correctly when typing amount in YEARLY mode', async () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        mode: 'YEARLY'
      }
    });

    const input = screen.getByLabelText(new RegExp(m.budget_config_amount_label()));
    await fireEvent.input(input, { target: { value: '1200' } });

    const allText = document.body.textContent;
    expect(allText).toContain('100.00');
    expect(allText).toMatch(/1[,.\\s]*200/);
  });

  it('submits form converting major units back to minor units', async () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        mode: 'MONTHLY'
      }
    });

    const input = screen.getByLabelText(new RegExp(m.budget_config_amount_label()));
    await fireEvent.input(input, { target: { value: '45.67' } });

    const form = input.closest('form');
    expect(form).not.toBeNull();
    await fireEvent.submit(form!);

    expect(onsubmitMock).toHaveBeenCalledWith('MONTHLY', 4567);
  });

  it('displays saving state', () => {
    const onsubmitMock = vi.fn();
    render(BudgetConfigForm, {
      props: {
        onsubmit: onsubmitMock,
        saving: true
      }
    });

    expect(screen.getByText(m.budget_config_saving_status())).toBeInTheDocument();

    const button = screen.getByRole('button', {
      name: new RegExp(m.budget_config_saving_button(), 'i')
    });
    expect(button).toBeDisabled();

    const amountInput = screen.getByLabelText(new RegExp(m.budget_config_amount_label()));
    expect(amountInput).toBeDisabled();
  });
});
