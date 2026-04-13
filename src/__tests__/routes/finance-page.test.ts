import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { cleanup, render, screen, waitFor } from '@testing-library/svelte';

// ── Mocks ────────────────────────────────────────────────────

vi.mock('@tauri-apps/api/core', () => ({ invoke: vi.fn() }));

vi.mock('$lib/paraglide/messages.js', async (importOriginal) => {
  const actual = (await importOriginal()) as Record<string, unknown>;
  return Object.fromEntries(
    Object.entries(actual).map(([k, v]) => [k, typeof v === 'function' ? () => k : v])
  );
});

vi.mock('$lib/toaster', () => ({
  toaster: { success: vi.fn(), error: vi.fn(), loading: vi.fn() }
}));

// Mutable state shared across tests
const { mockBudgetState, mockBudgetService } = vi.hoisted(() => ({
  mockBudgetState: {
    isLoading: false,
    hasConfig: false,
    currency: 'EUR',
    modeLabel: 'budget_mode_monthly',
    formattedMonthlyBudget: '€100.00',
    formattedYearlyBudget: '€1,200.00',
    dashboardSummary: {
      remainingAmount: 9000,
      remainingPercentage: 75,
      totalAvailable: 12000,
      currency: 'EUR',
      monthlySpending: [],
      monthlyGoal: 10000,
      quarterlyActivity: []
    },
    monthlyRecords: [] as unknown[],
    enhancedMonthlyRecords: [] as unknown[],
    load: vi.fn().mockResolvedValue(undefined),
    loadDashboard: vi.fn().mockResolvedValue(undefined),
    loadMonthlyRecords: vi.fn().mockResolvedValue(undefined),
    save: vi.fn().mockResolvedValue(undefined),
    formatAmount: vi.fn((minorUnits: number) => `€${(minorUnits / 100).toFixed(2)}`)
  },
  mockBudgetService: {
    isLoading: false
  }
}));

vi.mock('$lib/features/budget/services/BudgetService.svelte', () => ({
  createBudgetService: vi.fn(() => mockBudgetService),
  BudgetService: vi.fn(() => mockBudgetService)
}));

vi.mock('$lib/features/budget/BudgetState.svelte', () => ({
  createBudgetState: vi.fn(() => mockBudgetState),
  BudgetState: vi.fn(() => mockBudgetState)
}));

// Stub heavy children
vi.mock('$lib/features/budget/components/BudgetConfigSheet.svelte', () => ({
  default: function BudgetConfigSheetStub() {}
}));
vi.mock('$lib/features/budget/components/BudgetMonthRow.svelte', () => ({
  default: function BudgetMonthRowStub() {}
}));
vi.mock('$lib/features/budget/components/ExtraBudgetModal.svelte', () => ({
  default: function ExtraBudgetModalStub() {}
}));
vi.mock('$lib/components/PageHeader.svelte', () => ({
  default: function PageHeaderStub() {}
}));

// ── Test target ───────────────────────────────────────────────

import FinancePage from '../../routes/finance/+page.svelte';

describe('routes/finance/+page.svelte', () => {
  afterEach(() => {
    cleanup();
  });

  beforeEach(() => {
    vi.clearAllMocks();
    window.localStorage.clear();
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    mockBudgetState.modeLabel = 'budget_mode_monthly';
    mockBudgetState.dashboardSummary = {
      remainingAmount: 9000,
      remainingPercentage: 75,
      totalAvailable: 12000,
      currency: 'EUR',
      monthlySpending: [],
      monthlyGoal: 10000,
      quarterlyActivity: []
    };
    mockBudgetState.monthlyRecords = [];
    mockBudgetState.enhancedMonthlyRecords = [];
    mockBudgetState.load = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadDashboard = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadMonthlyRecords = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.formatAmount = vi.fn(
      (minorUnits: number) => `€${(minorUnits / 100).toFixed(2)}`
    );
  });

  it('renders without throwing', () => {
    expect(() => render(FinancePage)).not.toThrow();
  });

  it('shows a loading spinner when isLoading is true', () => {
    mockBudgetState.isLoading = true;
    const { container } = render(FinancePage);
    const spinner = container.querySelector('.animate-spin');
    expect(spinner).not.toBeNull();
  });

  it('shows loading message text when isLoading', () => {
    mockBudgetState.isLoading = true;
    render(FinancePage);
    expect(screen.getByText('budget_loading')).toBeInTheDocument();
  });

  it('shows "budget_empty_state_title" and message when hasConfig is false', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('budget_empty_state_title')).toBeInTheDocument();
      expect(screen.getByText('budget_empty_state_message')).toBeInTheDocument();
    });
  });

  it('shows the "dashboard_chart_budget_set_cta" button when no config exists', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('dashboard_chart_budget_set_cta')).toBeInTheDocument();
    });
  });

  it('shows monthly allocation card when hasConfig is true', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('budget_summary_monthly_allocation')).toBeInTheDocument();
      expect(screen.getByText('budget_summary_yearly_forecast')).toBeInTheDocument();
    });
  });

  it('shows the formatted monthly budget when hasConfig is true', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    mockBudgetState.formattedMonthlyBudget = '€100.00';
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('€100.00')).toBeInTheDocument();
    });
  });

  it('renders a year selector with current year', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    const currentYear = new Date().getFullYear().toString();
    return waitFor(() => {
      expect(screen.getByDisplayValue(currentYear)).toBeInTheDocument();
    });
  });

  it('calls budgetState.load on mount', async () => {
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.load).toHaveBeenCalledOnce();
    });
  });

  it('calls budgetState.loadDashboard on mount', async () => {
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.loadDashboard).toHaveBeenCalledOnce();
    });
  });

  it('calls loadMonthlyRecords when hasConfig becomes true after load', async () => {
    mockBudgetState.load = vi.fn().mockImplementation(async () => {
      mockBudgetState.hasConfig = true;
    });
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.loadMonthlyRecords).toHaveBeenCalled();
    });
  });
});
