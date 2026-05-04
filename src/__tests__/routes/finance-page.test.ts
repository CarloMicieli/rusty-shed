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
const { mockBudgetState } = vi.hoisted(() => ({
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
    hasWarmFinanceState: vi.fn(() => false),
    loadBootstrap: vi.fn().mockResolvedValue(undefined),
    load: vi.fn().mockResolvedValue(undefined),
    loadDashboard: vi.fn().mockResolvedValue(undefined),
    loadMonthlyRecords: vi.fn().mockResolvedValue(undefined),
    save: vi.fn().mockResolvedValue(undefined),
    formatAmount: vi.fn((minorUnits: number) => `€${(minorUnits / 100).toFixed(2)}`)
  }
}));

const { mockFinanceState } = vi.hoisted(() => ({
  mockFinanceState: {
    data: {
      remainingAmount: 9000,
      remainingPercentage: 75,
      totalAvailable: 12000,
      currency: 'EUR',
      monthlySpending: [],
      monthlyGoal: 10000,
      quarterlyActivity: []
    },
    loading: false,
    error: null as string | null,
    hasFetched: false,
    ensureLoaded: vi.fn().mockResolvedValue(undefined),
    refresh: vi.fn().mockResolvedValue(undefined),
    startListening: vi.fn().mockResolvedValue(undefined),
    stopListening: vi.fn()
  }
}));

vi.mock('$lib/features/budget/BudgetState.svelte', () => ({
  getBudgetState: vi.fn(() => mockBudgetState),
  BudgetState: vi.fn(() => mockBudgetState)
}));

vi.mock('$lib/state/finance.svelte', () => ({
  financeState: mockFinanceState
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
    mockBudgetState.hasWarmFinanceState = vi.fn(() => false);
    mockBudgetState.loadBootstrap = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.load = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadDashboard = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadMonthlyRecords = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.formatAmount = vi.fn(
      (minorUnits: number) => `€${(minorUnits / 100).toFixed(2)}`
    );

    mockFinanceState.loading = false;
    mockFinanceState.error = null;
    mockFinanceState.hasFetched = false;
    mockFinanceState.data = {
      remainingAmount: 9000,
      remainingPercentage: 75,
      totalAvailable: 12000,
      currency: 'EUR',
      monthlySpending: [],
      monthlyGoal: 10000,
      quarterlyActivity: []
    };
    mockFinanceState.ensureLoaded = vi.fn().mockResolvedValue(undefined);
    mockFinanceState.refresh = vi.fn().mockResolvedValue(undefined);
  });

  it('renders without throwing', () => {
    expect(() => render(FinancePage)).not.toThrow();
  });

  it('shows the Finance skeleton shell before initialization completes', () => {
    const { container } = render(FinancePage);
    expect(container.querySelector('[aria-busy="true"]')).not.toBeNull();
    expect(container.querySelectorAll('[data-slot="skeleton"]').length).toBeGreaterThan(0);
  });

  it('exposes the loading label during the skeleton state', () => {
    render(FinancePage);
    expect(screen.getByLabelText('budget_loading')).toBeInTheDocument();
  });

  it('shows "budget_empty_state_title" and message when hasConfig is false', () => {
    mockFinanceState.hasFetched = true;
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('budget_empty_state_title')).toBeInTheDocument();
      expect(screen.getByText('budget_empty_state_message')).toBeInTheDocument();
    });
  });

  it('shows the "dashboard_chart_budget_set_cta" button when no config exists', () => {
    mockFinanceState.hasFetched = true;
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('dashboard_chart_budget_set_cta')).toBeInTheDocument();
    });
  });

  it('shows monthly allocation card when hasConfig is true', () => {
    mockFinanceState.hasFetched = true;
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('budget_summary_monthly_allocation')).toBeInTheDocument();
      expect(screen.getByText('budget_summary_yearly_forecast')).toBeInTheDocument();
    });
  });

  it('shows the formatted monthly budget when hasConfig is true', () => {
    mockFinanceState.hasFetched = true;
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    mockBudgetState.formattedMonthlyBudget = '€100.00';
    render(FinancePage);
    return waitFor(() => {
      expect(screen.getByText('€100.00')).toBeInTheDocument();
    });
  });

  it('renders a year selector with current year', () => {
    mockFinanceState.hasFetched = true;
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    const currentYear = new Date().getFullYear().toString();
    return waitFor(() => {
      expect(screen.getByDisplayValue(currentYear)).toBeInTheDocument();
    });
  });

  it('does not call budgetState.loadBootstrap on mount', async () => {
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.loadBootstrap).not.toHaveBeenCalled();
    });
  });

  it('does not orchestrate budget loading on mount', async () => {
    window.localStorage.setItem('finance:selected-year', '2024');
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.loadBootstrap).not.toHaveBeenCalled();
      expect(mockBudgetState.load).not.toHaveBeenCalled();
      expect(mockBudgetState.loadDashboard).not.toHaveBeenCalled();
      expect(mockBudgetState.loadMonthlyRecords).not.toHaveBeenCalled();
    });
  });
});
