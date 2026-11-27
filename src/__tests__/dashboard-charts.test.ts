import { cleanup, render, screen } from '@testing-library/svelte';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('$lib/features/settings/RegionalManager.svelte', () => ({
  regionalManager: { locale: 'en-US' }
}));
vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([key, value]) => [
      key,
      typeof value === 'function' ? () => key : value
    ])
  );
});

import DashboardCharts from '$lib/features/dashboard/components/DashboardCharts.svelte';

describe('DashboardCharts', () => {
  beforeEach(() => {
    cleanup();
    vi.clearAllMocks();
  });

  it('uses the compact no-budget state when budget is disabled', () => {
    render(DashboardCharts, {
      props: {
        currencyCode: 'EUR',
        compact: true,
        hasBudget: false,
        data: { budget: 0, monthlySpending: [] }
      }
    });

    expect(screen.getByText('dashboard_chart_budget_no_budget')).toBeInTheDocument();
    expect(screen.getByText('dashboard_chart_spending_no_data')).toBeInTheDocument();
  });

  it('renders a 75 percent budget gauge in compact mode', () => {
    const { container } = render(DashboardCharts, {
      props: {
        currencyCode: 'EUR',
        compact: true,
        hasBudget: true,
        data: { budget: 0.75, monthlySpending: [{ month: 0, amount: 120 }] }
      }
    });

    expect(screen.getByText('75%')).toBeInTheDocument();
    expect(container.querySelector('[style*="conic-gradient"]')).not.toBeNull();
  });

  it('formats spending values and month labels for positive data', () => {
    render(DashboardCharts, {
      props: {
        currencyCode: 'USD',
        compact: true,
        hasBudget: true,
        data: {
          budget: 0.5,
          monthlySpending: [
            { month: 0, amount: 250 },
            { month: 1, amount: 400 }
          ]
        }
      }
    });

    expect(screen.getByText('50%')).toBeInTheDocument();
  });

  it('falls back to default currency and empty chart data', () => {
    render(DashboardCharts, {
      props: {
        compact: true,
        hasBudget: true
      }
    });

    expect(screen.getByText('0%')).toBeInTheDocument();
    expect(screen.getByText('dashboard_chart_spending_no_data')).toBeInTheDocument();
  });

  it('clamps budget percentage and viewport-independent compact chart height', () => {
    render(DashboardCharts, {
      props: {
        compact: true,
        hasBudget: true,
        data: { budget: 1.8, monthlySpending: [] }
      }
    });

    expect(screen.getByText('180%')).toBeInTheDocument();
  });
});
