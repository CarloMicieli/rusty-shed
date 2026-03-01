import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/svelte';

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
    formattedMonthlyBudget: '€100.00',
    formattedYearlyBudget: '€1,200.00',
    enhancedMonthlyRecords: [] as unknown[],
    load: vi.fn().mockResolvedValue(undefined),
    loadMonthlyRecords: vi.fn().mockResolvedValue(undefined),
    save: vi.fn().mockResolvedValue(undefined)
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

vi.mock('$lib/stores/modal', () => ({
  getModalStore: vi.fn(() => ({
    trigger: vi.fn(),
    close: vi.fn()
  }))
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
  beforeEach(() => {
    vi.clearAllMocks();
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    mockBudgetState.enhancedMonthlyRecords = [];
    mockBudgetState.load = vi.fn().mockResolvedValue(undefined);
    mockBudgetState.loadMonthlyRecords = vi.fn().mockResolvedValue(undefined);
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

  it('shows "NO_BUDGET_CONFIG_FOUND" when hasConfig is false', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    expect(screen.getByText('NO_BUDGET_CONFIG_FOUND')).toBeInTheDocument();
  });

  it('shows the "Initialize System Budget" button when no config exists', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = false;
    render(FinancePage);
    expect(screen.getByText('Initialize System Budget')).toBeInTheDocument();
  });

  it('shows monthly allocation card when hasConfig is true', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    expect(screen.getByText('Monthly Allocation')).toBeInTheDocument();
    expect(screen.getByText('Yearly Forecast')).toBeInTheDocument();
  });

  it('shows the formatted monthly budget when hasConfig is true', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    mockBudgetState.formattedMonthlyBudget = '€100.00';
    render(FinancePage);
    expect(screen.getByText('€100.00')).toBeInTheDocument();
  });

  it('renders a year selector with current year', () => {
    mockBudgetState.isLoading = false;
    mockBudgetState.hasConfig = true;
    render(FinancePage);
    const currentYear = new Date().getFullYear().toString();
    expect(screen.getByText(currentYear)).toBeInTheDocument();
  });

  it('calls budgetState.load on mount', async () => {
    render(FinancePage);
    await waitFor(() => {
      expect(mockBudgetState.load).toHaveBeenCalledOnce();
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
