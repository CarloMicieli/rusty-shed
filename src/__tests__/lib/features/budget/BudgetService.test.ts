import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mocks must be declared before imports
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('$lib/toaster', () => ({
  toaster: {
    loading: vi.fn(),
    success: vi.fn(),
    error: vi.fn()
  }
}));

vi.mock('$lib/paraglide/messages.js', () => ({
  budget_error_load_failed: () => 'Failed to load budget configuration',
  budget_config_saved: () => 'Budget configuration saved',
  budget_error_save_failed: () => 'Failed to save budget configuration'
}));

import { invoke } from '@tauri-apps/api/core';
import { toaster } from '$lib/toaster';
import { BudgetService } from '$lib/features/budget/services/BudgetService.svelte';
import type {
  BudgetConfigDto,
  MonthlyBudgetRecordDto,
  BudgetDashboardSummary,
  ExtraBudgetDto,
  QuarterlySummary
} from '$lib/features/budget/services/BudgetService.svelte';

const mockInvoke = vi.mocked(invoke);
const mockToaster = vi.mocked(toaster);

// ── Helpers ──────────────────────────────────────────────────────────────────

/** Route invoke calls by command name — like tauriMock from wishlist tests */
function setupInvokeMock(handlers: Record<string, () => unknown>): void {
  mockInvoke.mockImplementation(async (cmd) => {
    const key = String(cmd);
    if (key in handlers) return handlers[key]() as never;
    throw new Error(`Unmocked command: ${key}`);
  });
}

function makeConfig(overrides?: Partial<BudgetConfigDto>): BudgetConfigDto {
  return {
    id: 1,
    mode: 'MONTHLY',
    baseAmount: 20000,
    monthlyAmount: 20000,
    yearlyAmount: 240000,
    currency: 'EUR',
    lastResetYear: 2026,
    createdAt: '2026-01-01T00:00:00Z',
    updatedAt: '2026-01-01T00:00:00Z',
    version: 1,
    ...overrides
  };
}

function makeMonthlyRecord(overrides?: Partial<MonthlyBudgetRecordDto>): MonthlyBudgetRecordDto {
  return {
    year: 2026,
    month: 1,
    baseBudget: 20000,
    extraBudget: 0,
    actualSpend: 5000,
    rolloverIn: 0,
    rolloverOut: 0,
    available: 20000,
    remaining: 15000,
    remainingPercentage: 75,
    status: 'IN_PROGRESS',
    currency: 'EUR',
    ...overrides
  };
}

function makeDashboard(overrides?: Partial<BudgetDashboardSummary>): BudgetDashboardSummary {
  return {
    remainingAmount: 15000,
    remainingPercentage: 75,
    totalAvailable: 20000,
    currency: 'EUR',
    monthlySpending: [],
    monthlyGoal: 20000,
    quarterlyActivity: [],
    ...overrides
  };
}

function makeExtraBudget(overrides?: Partial<ExtraBudgetDto>): ExtraBudgetDto {
  return {
    id: 'extra-1',
    year: 2026,
    month: 3,
    amount: 5000,
    currency: 'EUR',
    reason: 'Birthday gift',
    createdAt: '2026-03-01T00:00:00Z',
    version: 1,
    ...overrides
  };
}

function makeQuarterlySummary(overrides?: Partial<QuarterlySummary>): QuarterlySummary {
  return {
    year: 2026,
    quarter: 'Q1',
    totalSpending: { amount: 12000, currency: 'EUR' },
    categoryBreakdown: [],
    ...overrides
  };
}

// ── Tests ─────────────────────────────────────────────────────────────────────

describe('BudgetService', () => {
  let service: BudgetService;

  beforeEach(() => {
    service = new BudgetService();
    // resetAllMocks clears implementations AND once-queues, preventing cross-test contamination
    vi.resetAllMocks();
  });

  // ── Initial state ──────────────────────────────────────────────────────────

  describe('initial state', () => {
    it('has null config', () => {
      expect(service.config).toBeNull();
    });

    it('has empty monthlyRecords', () => {
      expect(service.monthlyRecords).toEqual([]);
    });

    it('has null dashboardSummary', () => {
      expect(service.dashboardSummary).toBeNull();
    });

    it('has empty extraBudgets', () => {
      expect(service.extraBudgets).toEqual([]);
    });

    it('has empty quarterlySummaries', () => {
      expect(service.quarterlySummaries).toEqual([]);
    });

    it('is not loading', () => {
      expect(service.isLoading).toBe(false);
    });

    it('hasConfig is false', () => {
      expect(service.hasConfig).toBe(false);
    });
  });

  // ── reset() ───────────────────────────────────────────────────────────────

  describe('reset()', () => {
    it('clears all state back to initial values', async () => {
      setupInvokeMock({ get_budget_config: () => makeConfig() });
      await service.loadConfig();

      service.reset();

      expect(service.config).toBeNull();
      expect(service.monthlyRecords).toEqual([]);
      expect(service.dashboardSummary).toBeNull();
      expect(service.extraBudgets).toEqual([]);
      expect(service.quarterlySummaries).toEqual([]);
      expect(service.isLoading).toBe(false);
      expect(service.hasConfig).toBe(false);
    });
  });

  // ── loadConfig() ──────────────────────────────────────────────────────────

  describe('loadConfig()', () => {
    it('sets config on success', async () => {
      const config = makeConfig();
      setupInvokeMock({ get_budget_config: () => config });

      await service.loadConfig();

      expect(service.config).toEqual(config);
      expect(service.hasConfig).toBe(true);
      expect(service.isLoading).toBe(false);
    });

    it('sets config to null when backend returns null', async () => {
      setupInvokeMock({ get_budget_config: () => null });

      await service.loadConfig();

      expect(service.config).toBeNull();
      expect(service.hasConfig).toBe(false);
    });

    it('toasts error and rethrows when backend fails', async () => {
      mockInvoke.mockRejectedValue(new Error('DB error'));

      await expect(service.loadConfig()).rejects.toThrow('DB error');

      expect(mockToaster.error).toHaveBeenCalledWith(
        expect.objectContaining({ title: 'DB error' })
      );
    });

    it('resets isLoading to false after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadConfig()).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });

    it('does not re-enter if already loading', async () => {
      // First call: never resolves (simulates in-flight request)
      mockInvoke.mockReturnValue(new Promise(() => {}));

      const first = service.loadConfig();
      // #isLoading is now true — second call should be a no-op
      void service.loadConfig();

      // First call is still pending — mockInvoke was only called once
      expect(mockInvoke).toHaveBeenCalledTimes(1);

      // Cleanup: we can't await first since it never resolves, but that's fine for this assertion
      first.catch(() => {}); // prevent unhandled rejection warning
    });
  });

  // ── loadMonthlyRecords() ──────────────────────────────────────────────────

  describe('loadMonthlyRecords()', () => {
    it('sets monthlyRecords on success', async () => {
      const records = [makeMonthlyRecord({ month: 1 }), makeMonthlyRecord({ month: 2 })];
      setupInvokeMock({ get_monthly_budget_records: () => records });

      await service.loadMonthlyRecords(2026);

      expect(service.monthlyRecords).toEqual(records);
      expect(service.isLoading).toBe(false);
    });

    it('passes year in args', async () => {
      setupInvokeMock({ get_monthly_budget_records: () => [] });

      await service.loadMonthlyRecords(2025);

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_monthly_budget_records',
        expect.objectContaining({ args: { year: 2025 } })
      );
    });

    it('passes undefined year when not provided', async () => {
      setupInvokeMock({ get_monthly_budget_records: () => [] });

      await service.loadMonthlyRecords();

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_monthly_budget_records',
        expect.objectContaining({ args: { year: undefined } })
      );
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('network error'));

      await expect(service.loadMonthlyRecords(2026)).rejects.toThrow('network error');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadMonthlyRecords()).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });

    it('does not re-enter if already loading', async () => {
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.loadMonthlyRecords(2026);
      void service.loadMonthlyRecords(2026); // second call is a no-op

      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });
  });

  // ── loadDashboard() ───────────────────────────────────────────────────────

  describe('loadDashboard()', () => {
    it('sets dashboardSummary on success', async () => {
      const dashboard = makeDashboard();
      setupInvokeMock({ get_budget_dashboard: () => dashboard });

      await service.loadDashboard();

      expect(service.dashboardSummary).toEqual(dashboard);
      expect(service.isLoading).toBe(false);
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadDashboard()).rejects.toThrow('fail');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadDashboard()).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });

    it('does not re-enter if already loading', async () => {
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.loadDashboard();
      void service.loadDashboard(); // no-op

      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });
  });

  // ── setBudgetConfig() ─────────────────────────────────────────────────────

  describe('setBudgetConfig()', () => {
    it('applies optimistic update before awaiting backend', async () => {
      let capturedOptimistic: BudgetConfigDto | null = null;
      mockInvoke.mockImplementation(async () => {
        capturedOptimistic = service.config;
        return makeConfig({ mode: 'YEARLY', baseAmount: 120000 }) as never;
      });

      await service.setBudgetConfig({ mode: 'YEARLY', baseAmount: 120000, currency: 'EUR' });

      expect(capturedOptimistic).not.toBeNull();
      expect(capturedOptimistic!.mode).toBe('YEARLY');
      expect(capturedOptimistic!.baseAmount).toBe(120000);
    });

    it('replaces optimistic config with server response on success', async () => {
      const serverConfig = makeConfig({ id: 42, version: 2 });
      setupInvokeMock({ set_budget_config: () => serverConfig });

      await service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 20000 });

      expect(service.config).toEqual(serverConfig);
    });

    it('toasts success on completion', async () => {
      setupInvokeMock({ set_budget_config: () => makeConfig() });

      await service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 20000 });

      expect(mockToaster.success).toHaveBeenCalledWith(
        expect.objectContaining({ title: 'Budget configuration saved' })
      );
    });

    it('rolls back to previous config on failure', async () => {
      const original = makeConfig({ baseAmount: 10000 });
      setupInvokeMock({ get_budget_config: () => original });
      await service.loadConfig();

      mockInvoke.mockRejectedValue(new Error('save failed'));

      await expect(
        service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 30000 })
      ).rejects.toThrow();

      expect(service.config).toEqual(original);
    });

    it('toasts error on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('save error'));

      await expect(
        service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 20000 })
      ).rejects.toThrow();

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('calculates monthlyAmount for YEARLY mode', async () => {
      let capturedOptimistic: BudgetConfigDto | null = null;
      mockInvoke.mockImplementation(async () => {
        capturedOptimistic = service.config;
        return makeConfig({ mode: 'YEARLY', baseAmount: 120000 }) as never;
      });

      await service.setBudgetConfig({ mode: 'YEARLY', baseAmount: 120000 });

      expect(capturedOptimistic!.monthlyAmount).toBe(Math.floor(120000 / 12));
      expect(capturedOptimistic!.yearlyAmount).toBe(120000);
    });

    it('calculates yearlyAmount for MONTHLY mode', async () => {
      let capturedOptimistic: BudgetConfigDto | null = null;
      mockInvoke.mockImplementation(async () => {
        capturedOptimistic = service.config;
        return makeConfig({ mode: 'MONTHLY', baseAmount: 20000 }) as never;
      });

      await service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 20000 });

      expect(capturedOptimistic!.monthlyAmount).toBe(20000);
      expect(capturedOptimistic!.yearlyAmount).toBe(20000 * 12);
    });

    it('resets isLoading after success', async () => {
      setupInvokeMock({ set_budget_config: () => makeConfig() });

      await service.setBudgetConfig({ mode: 'MONTHLY', baseAmount: 20000 });

      expect(service.isLoading).toBe(false);
    });
  });

  // ── addExtraBudget() ──────────────────────────────────────────────────────

  describe('addExtraBudget()', () => {
    it('returns the created extra budget on success', async () => {
      const extra = makeExtraBudget();
      // Note: loadMonthlyRecords is called internally but skipped (isLoading guard)
      setupInvokeMock({ add_extra_budget: () => extra });

      const result = await service.addExtraBudget({
        year: 2026,
        month: 3,
        amount: 5000,
        currency: 'EUR',
        reason: 'Birthday gift'
      });

      expect(result).toEqual(extra);
    });

    it('sets isLoading to false after success', async () => {
      setupInvokeMock({ add_extra_budget: () => makeExtraBudget() });

      await service.addExtraBudget({ year: 2026, month: 3, amount: 5000 });

      expect(service.isLoading).toBe(false);
    });

    it('toasts success', async () => {
      setupInvokeMock({ add_extra_budget: () => makeExtraBudget() });

      await service.addExtraBudget({ year: 2026, month: 1, amount: 1000 });

      expect(mockToaster.success).toHaveBeenCalled();
    });

    it('throws immediately if already loading', async () => {
      // First call never resolves — keeps #isLoading = true
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.addExtraBudget({ year: 2026, month: 1, amount: 1000 });

      // Second call throws synchronously because isLoading is true
      await expect(service.addExtraBudget({ year: 2026, month: 2, amount: 2000 })).rejects.toThrow(
        'Operation already in progress'
      );
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('insert failed'));

      await expect(service.addExtraBudget({ year: 2026, month: 1, amount: 1000 })).rejects.toThrow(
        'insert failed'
      );

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(
        service.addExtraBudget({ year: 2026, month: 1, amount: 1000 })
      ).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });
  });

  // ── removeExtraBudget() ───────────────────────────────────────────────────

  describe('removeExtraBudget()', () => {
    it('calls remove_extra_budget with the correct id', async () => {
      // Note: loadMonthlyRecords is called internally but skipped (isLoading guard)
      setupInvokeMock({ remove_extra_budget: () => undefined });

      await service.removeExtraBudget('extra-1', 2026);

      expect(mockInvoke).toHaveBeenCalledWith(
        'remove_extra_budget',
        expect.objectContaining({ args: { id: 'extra-1' } })
      );
    });

    it('resets isLoading after success', async () => {
      setupInvokeMock({ remove_extra_budget: () => undefined });

      await service.removeExtraBudget('extra-1', 2026);

      expect(service.isLoading).toBe(false);
    });

    it('toasts success', async () => {
      setupInvokeMock({ remove_extra_budget: () => undefined });

      await service.removeExtraBudget('extra-1', 2026);

      expect(mockToaster.success).toHaveBeenCalled();
    });

    it('throws immediately if already loading', async () => {
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.removeExtraBudget('extra-1', 2026);

      await expect(service.removeExtraBudget('extra-2', 2026)).rejects.toThrow(
        'Operation already in progress'
      );
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('delete failed'));

      await expect(service.removeExtraBudget('extra-1', 2026)).rejects.toThrow('delete failed');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.removeExtraBudget('extra-1', 2026)).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });
  });

  // ── loadExtraBudgets() ────────────────────────────────────────────────────

  describe('loadExtraBudgets()', () => {
    it('sets extraBudgets on success', async () => {
      const extras = [makeExtraBudget(), makeExtraBudget({ id: 'extra-2', month: 6 })];
      setupInvokeMock({ get_extra_budgets: () => extras });

      await service.loadExtraBudgets(2026);

      expect(service.extraBudgets).toEqual(extras);
      expect(service.isLoading).toBe(false);
    });

    it('passes year in args', async () => {
      setupInvokeMock({ get_extra_budgets: () => [] });

      await service.loadExtraBudgets(2025);

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_extra_budgets',
        expect.objectContaining({ args: { year: 2025 } })
      );
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('query failed'));

      await expect(service.loadExtraBudgets(2026)).rejects.toThrow('query failed');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadExtraBudgets(2026)).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });

    it('does not re-enter if already loading', async () => {
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.loadExtraBudgets(2026);
      void service.loadExtraBudgets(2026); // no-op

      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });
  });

  // ── loadQuarterlySummaries() ──────────────────────────────────────────────

  describe('loadQuarterlySummaries()', () => {
    it('sets quarterlySummaries on success', async () => {
      const summaries = [makeQuarterlySummary(), makeQuarterlySummary({ quarter: 'Q2' })];
      setupInvokeMock({ get_quarterly_summaries: () => summaries });

      await service.loadQuarterlySummaries(2026, 'EUR');

      expect(service.quarterlySummaries).toEqual(summaries);
      expect(service.isLoading).toBe(false);
    });

    it('passes year and currency in args', async () => {
      setupInvokeMock({ get_quarterly_summaries: () => [] });

      await service.loadQuarterlySummaries(2025, 'USD');

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_quarterly_summaries',
        expect.objectContaining({ args: { year: 2025, currency: 'USD' } })
      );
    });

    it('passes undefined when year/currency omitted', async () => {
      setupInvokeMock({ get_quarterly_summaries: () => [] });

      await service.loadQuarterlySummaries();

      expect(mockInvoke).toHaveBeenCalledWith(
        'get_quarterly_summaries',
        expect.objectContaining({ args: { year: undefined, currency: undefined } })
      );
    });

    it('toasts error and rethrows on failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadQuarterlySummaries(2026)).rejects.toThrow('fail');

      expect(mockToaster.error).toHaveBeenCalled();
    });

    it('resets isLoading after failure', async () => {
      mockInvoke.mockRejectedValue(new Error('fail'));

      await expect(service.loadQuarterlySummaries()).rejects.toThrow();

      expect(service.isLoading).toBe(false);
    });

    it('does not re-enter if already loading', async () => {
      mockInvoke.mockReturnValue(new Promise(() => {}));

      void service.loadQuarterlySummaries(2026);
      void service.loadQuarterlySummaries(2026); // no-op

      expect(mockInvoke).toHaveBeenCalledTimes(1);
    });
  });

  // ── hasConfig derived ─────────────────────────────────────────────────────

  describe('hasConfig', () => {
    it('is false initially', () => {
      expect(service.hasConfig).toBe(false);
    });

    it('is true after loading a non-null config', async () => {
      setupInvokeMock({ get_budget_config: () => makeConfig() });
      await service.loadConfig();
      expect(service.hasConfig).toBe(true);
    });

    it('is false after loading null config', async () => {
      setupInvokeMock({ get_budget_config: () => null });
      await service.loadConfig();
      expect(service.hasConfig).toBe(false);
    });

    it('becomes false after reset', async () => {
      setupInvokeMock({ get_budget_config: () => makeConfig() });
      await service.loadConfig();
      service.reset();
      expect(service.hasConfig).toBe(false);
    });
  });
});
