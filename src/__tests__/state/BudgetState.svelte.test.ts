import { describe, it, expect, vi } from 'vitest';

vi.mock('$lib/paraglide/messages.js', () => ({
  budget_mode_yearly: () => 'Yearly',
  budget_mode_monthly: () => 'Monthly',
  budget_status_completed: () => 'Completed',
  budget_status_in_progress: () => 'In Progress',
  budget_status_projected: () => 'Projected',
  budget_error_load_failed: () => 'Load failed',
  budget_error_save_failed: () => 'Save failed',
  budget_config_saved: () => 'Config saved'
}));

import { BudgetState } from '$lib/features/budget/BudgetState.svelte';
import type {
  BudgetConfigDto,
  BudgetService,
  MonthlyBudgetRecordDto
} from '$lib/features/budget/services/BudgetService.svelte';

// ─── mock service factory ──────────────────────────────────────────────────

function makeMockService(overrides: Partial<BudgetService> = {}): BudgetService {
  return {
    config: null,
    isLoading: false,
    hasConfig: false,
    dashboardSummary: null,
    extraBudgets: [],
    quarterlySummaries: [],
    monthlyRecords: [],
    loadConfig: vi.fn().mockResolvedValue(undefined),
    loadMonthlyRecords: vi.fn().mockResolvedValue(undefined),
    loadQuarterlySummaries: vi.fn().mockResolvedValue(undefined),
    loadDashboard: vi.fn().mockResolvedValue(undefined),
    addExtraBudget: vi.fn().mockResolvedValue({}),
    setBudgetConfig: vi.fn().mockResolvedValue(undefined),
    reset: vi.fn(),
    ...overrides
  } as unknown as BudgetService;
}

function makeConfig(overrides: Partial<BudgetConfigDto> = {}): BudgetConfigDto {
  return {
    id: 1,
    mode: 'MONTHLY',
    baseAmount: 5000, // 50.00 EUR in cents
    monthlyAmount: 5000,
    yearlyAmount: 60000,
    currency: 'EUR',
    lastResetYear: 2026,
    createdAt: '2026-01-01T00:00:00Z',
    updatedAt: '2026-01-01T00:00:00Z',
    version: 1,
    ...overrides
  };
}

function makeRecord(overrides: Partial<MonthlyBudgetRecordDto> = {}): MonthlyBudgetRecordDto {
  return {
    year: 2026,
    month: 1,
    baseBudget: 5000,
    extraBudget: 0,
    actualSpend: 2500,
    rolloverIn: 0,
    rolloverOut: 2500,
    available: 5000,
    remaining: 2500,
    remainingPercentage: 50,
    status: 'IN_PROGRESS',
    currency: 'EUR',
    ...overrides
  };
}

// ─── tests ─────────────────────────────────────────────────────────────────

describe('BudgetState', () => {
  describe('currency', () => {
    it('returns currency from config', () => {
      const service = makeMockService({ config: makeConfig({ currency: 'USD' }) });
      const state = new BudgetState(service);
      expect(state.currency).toBe('USD');
    });

    it('defaults to EUR when config is null', () => {
      const service = makeMockService({ config: null });
      const state = new BudgetState(service);
      expect(state.currency).toBe('EUR');
    });
  });

  describe('monthlyBudget / yearlyBudget', () => {
    it('returns monthlyAmount from config', () => {
      const service = makeMockService({ config: makeConfig({ monthlyAmount: 8000 }) });
      const state = new BudgetState(service);
      expect(state.monthlyBudget).toBe(8000);
    });

    it('returns yearlyAmount from config', () => {
      const service = makeMockService({ config: makeConfig({ yearlyAmount: 96000 }) });
      const state = new BudgetState(service);
      expect(state.yearlyBudget).toBe(96000);
    });

    it('returns 0 when config is null', () => {
      const service = makeMockService({ config: null });
      const state = new BudgetState(service);
      expect(state.monthlyBudget).toBe(0);
      expect(state.yearlyBudget).toBe(0);
    });
  });

  describe('formatAmount', () => {
    it('converts minor units (cents) to currency string', () => {
      const service = makeMockService({ config: makeConfig({ currency: 'EUR' }) });
      const state = new BudgetState(service);
      const formatted = state.formatAmount(5000);
      // Should contain "50" and the EUR symbol
      expect(formatted).toMatch(/50/);
    });

    it('handles zero value', () => {
      const service = makeMockService({ config: makeConfig({ currency: 'EUR' }) });
      const state = new BudgetState(service);
      const formatted = state.formatAmount(0);
      expect(formatted).toMatch(/0/);
    });

    it('handles large amounts', () => {
      const service = makeMockService({ config: makeConfig({ currency: 'EUR' }) });
      const state = new BudgetState(service);
      const formatted = state.formatAmount(1000000); // 10,000.00 EUR
      expect(formatted).toMatch(/10/);
    });
  });

  describe('modeLabel', () => {
    it('returns yearly label for YEARLY mode', () => {
      const service = makeMockService({ config: makeConfig({ mode: 'YEARLY' }) });
      const state = new BudgetState(service);
      expect(state.modeLabel).toBe('Yearly');
    });

    it('returns monthly label for MONTHLY mode', () => {
      const service = makeMockService({ config: makeConfig({ mode: 'MONTHLY' }) });
      const state = new BudgetState(service);
      expect(state.modeLabel).toBe('Monthly');
    });

    it('returns empty string when config is null', () => {
      const service = makeMockService({ config: null });
      const state = new BudgetState(service);
      expect(state.modeLabel).toBe('');
    });
  });

  describe('hasRecords', () => {
    it('returns true when monthlyRecords is non-empty', () => {
      const service = makeMockService({ monthlyRecords: [makeRecord()] });
      const state = new BudgetState(service);
      expect(state.hasRecords).toBe(true);
    });

    it('returns false when monthlyRecords is empty', () => {
      const service = makeMockService({ monthlyRecords: [] });
      const state = new BudgetState(service);
      expect(state.hasRecords).toBe(false);
    });
  });

  describe('enhancedMonthlyRecords', () => {
    it('adds formatted fields to each record', () => {
      const record = makeRecord({ baseBudget: 5000, currency: 'EUR' });
      const service = makeMockService({
        config: makeConfig({ currency: 'EUR' }),
        monthlyRecords: [record]
      });
      const state = new BudgetState(service);
      const enhanced = state.enhancedMonthlyRecords;

      expect(enhanced).toHaveLength(1);
      expect(enhanced[0].formattedBase).toMatch(/50/);
      expect(enhanced[0].formattedSpent).toMatch(/25/);
      expect(enhanced[0].formattedRemaining).toMatch(/25/);
    });

    it('calculates remainingPercentage correctly', () => {
      const record = makeRecord({ available: 10000, remaining: 7500 }); // 75%
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      const enhanced = state.enhancedMonthlyRecords;

      expect(enhanced[0].remainingPercentage).toBeCloseTo(75, 0);
    });

    it('clamps remainingPercentage to 0 when remaining is negative (overspent)', () => {
      const record = makeRecord({ available: 5000, remaining: -1000 }); // overspent
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      const enhanced = state.enhancedMonthlyRecords;

      expect(enhanced[0].remainingPercentage).toBe(0);
    });

    it('clamps remainingPercentage to 100 maximum', () => {
      const record = makeRecord({ available: 5000, remaining: 6000 }); // rollover > available
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      const enhanced = state.enhancedMonthlyRecords;

      expect(enhanced[0].remainingPercentage).toBe(100);
    });

    it('returns 0 remaining percentage when available is 0', () => {
      const record = makeRecord({ available: 0, remaining: 0 });
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      const enhanced = state.enhancedMonthlyRecords;

      expect(enhanced[0].remainingPercentage).toBe(0);
    });

    it('maps IN_PROGRESS status to label', () => {
      const record = makeRecord({ status: 'IN_PROGRESS' });
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      expect(state.enhancedMonthlyRecords[0].statusLabel).toBe('In Progress');
    });

    it('maps COMPLETED status to label', () => {
      const record = makeRecord({ status: 'COMPLETED' });
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      expect(state.enhancedMonthlyRecords[0].statusLabel).toBe('Completed');
    });

    it('maps PROJECTED status to label', () => {
      const record = makeRecord({ status: 'PROJECTED' });
      const service = makeMockService({ config: makeConfig(), monthlyRecords: [record] });
      const state = new BudgetState(service);
      expect(state.enhancedMonthlyRecords[0].statusLabel).toBe('Projected');
    });

    it('returns empty array when no records exist', () => {
      const service = makeMockService({ monthlyRecords: [] });
      const state = new BudgetState(service);
      expect(state.enhancedMonthlyRecords).toHaveLength(0);
    });
  });

  describe('proxy getters', () => {
    it('proxies isLoading from service', () => {
      const service = makeMockService({ isLoading: true });
      const state = new BudgetState(service);
      expect(state.isLoading).toBe(true);
    });

    it('proxies hasConfig from service', () => {
      const service = makeMockService({ hasConfig: true });
      const state = new BudgetState(service);
      expect(state.hasConfig).toBe(true);
    });

    it('proxies dashboardSummary from service', () => {
      const summary = {
        remainingAmount: 5000,
        remainingPercentage: 50,
        totalAvailable: 10000,
        currency: 'EUR',
        monthlySpending: [],
        monthlyGoal: 5000,
        quarterlyActivity: []
      };
      const service = makeMockService({ dashboardSummary: summary });
      const state = new BudgetState(service);
      expect(state.dashboardSummary).toBe(summary);
    });
  });

  describe('command delegation', () => {
    it('load() calls service.loadConfig()', async () => {
      const service = makeMockService();
      const state = new BudgetState(service);
      await state.load();
      expect(service.loadConfig).toHaveBeenCalledOnce();
    });

    it('loadMonthlyRecords() calls service.loadMonthlyRecords()', async () => {
      const service = makeMockService();
      const state = new BudgetState(service);
      await state.loadMonthlyRecords(2026);
      expect(service.loadMonthlyRecords).toHaveBeenCalledWith(2026);
    });

    it('reset() calls service.reset()', () => {
      const service = makeMockService();
      const state = new BudgetState(service);
      state.reset();
      expect(service.reset).toHaveBeenCalledOnce();
    });
  });
});
