/**
 * Budget Service - Manages budget configuration and state.
 *
 * This service provides:
 * - Budget configuration CRUD operations
 * - Budget state management
 * - Optimistic updates with rollback
 * - Error handling with retry logic
 */

import { setContext, getContext } from 'svelte';
import { SvelteDate } from 'svelte/reactivity';
import { toaster } from '$lib/toaster';
import * as m from '$lib/paraglide/messages.js';
import { safeInvoke } from '$lib/shared/services/TauriAdapter';
import { getErrorMessage } from '$lib/shared/domain/errors';

// ─────────────────────────────────────────────────────────────
// TYPES (Matching Rust DTOs until bindings are generated)
// ─────────────────────────────────────────────────────────────

export type BudgetMode = 'YEARLY' | 'MONTHLY';
export type MonthStatus = 'PROJECTED' | 'IN_PROGRESS' | 'COMPLETED';
export type BudgetQuarter = 'Q1' | 'Q2' | 'Q3' | 'Q4';
export type SpendingLevel = 'NONE' | 'LOW' | 'MEDIUM' | 'HIGH';

export interface BudgetConfigDto {
  id: number;
  mode: BudgetMode;
  baseAmount: number;
  monthlyAmount: number;
  yearlyAmount: number;
  currency: string;
  lastResetYear: number;
  createdAt: string;
  updatedAt: string;
  version: number;
}

export interface SetBudgetConfigArgs {
  mode: BudgetMode;
  baseAmount: number;
  currency?: string;
}

export interface MonthlyBudgetRecordDto {
  year: number;
  month: number;
  baseBudget: number;
  extraBudget: number;
  actualSpend: number;
  rolloverIn: number;
  rolloverOut: number;
  available: number;
  remaining: number;
  remainingPercentage: number;
  status: MonthStatus;
  currency: string;
}

export interface GetMonthlyBudgetRecordsArgs {
  year?: number;
}

export interface MonthlySpendingPoint {
  month: number;
  amount: number;
  currency: string;
}

export interface QuarterlyActivityPoint {
  year: number;
  quarter: BudgetQuarter;
  spendingLevel: SpendingLevel;
  amount: number;
}

export interface BudgetDashboardSummary {
  remainingAmount: number;
  remainingPercentage: number;
  totalAvailable: number;
  currency: string;
  monthlySpending: MonthlySpendingPoint[];
  monthlyGoal: number;
  quarterlyActivity: QuarterlyActivityPoint[];
}

export interface ExtraBudgetDto {
  id: string;
  year: number;
  month: number;
  amount: number;
  currency: string;
  reason: string | null;
  createdAt: string;
  version: number;
}

export interface AddExtraBudgetArgs {
  year: number;
  month: number;
  amount: number;
  currency?: string;
  reason?: string;
}

export interface RemoveExtraBudgetArgs {
  id: string;
}

export interface GetExtraBudgetsArgs {
  year: number;
}

export interface CategorySpending {
  category: string;
  amount: {
    amount: number;
    currency: string;
  };
  percentage: number;
}

export interface QuarterlySummary {
  year: number;
  quarter: BudgetQuarter;
  totalSpending: {
    amount: number;
    currency: string;
  };
  categoryBreakdown: CategorySpending[];
}

export interface GetQuarterlySummariesArgs {
  year?: number;
  currency?: string;
}

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY (for Dependency Injection)
// ─────────────────────────────────────────────────────────────
const SERVICE_KEY = Symbol('budget-service');

// ─────────────────────────────────────────────────────────────
// SERVICE CLASS
// ─────────────────────────────────────────────────────────────
export class BudgetService {
  // Private reactive state
  #config = $state<BudgetConfigDto | null>(null);
  #monthlyRecords = $state<MonthlyBudgetRecordDto[]>([]);
  #dashboardSummary = $state<BudgetDashboardSummary | null>(null);
  #extraBudgets = $state<ExtraBudgetDto[]>([]);
  #quarterlySummaries = $state<QuarterlySummary[]>([]);
  #isLoading = $state(false);
  #snapshot: BudgetConfigDto | null = null;

  // Public readonly getters (defensive encapsulation)
  get config(): BudgetConfigDto | null {
    return this.#config;
  }

  get monthlyRecords(): MonthlyBudgetRecordDto[] {
    return this.#monthlyRecords;
  }

  get dashboardSummary(): BudgetDashboardSummary | null {
    return this.#dashboardSummary;
  }

  get extraBudgets(): ExtraBudgetDto[] {
    return this.#extraBudgets;
  }

  get quarterlySummaries(): QuarterlySummary[] {
    return this.#quarterlySummaries;
  }

  get isLoading(): boolean {
    return this.#isLoading;
  }

  get hasConfig(): boolean {
    return this.#config !== null;
  }

  // ───────────────────────────────────────────────────────────
  // INITIALIZATION
  // ───────────────────────────────────────────────────────────

  /**
   * Load the budget configuration from the backend.
   */
  async loadConfig(): Promise<void> {
    if (this.#isLoading) return;

    this.#isLoading = true;
    try {
      const result = await safeInvoke<BudgetConfigDto | null>('get_budget_config');

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      this.#config = result.data;
    } catch (error) {
      const message = error instanceof Error ? error.message : m.budget_error_load_failed();
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  // ───────────────────────────────────────────────────────────
  // COMMAND OPERATIONS
  // ───────────────────────────────────────────────────────────

  /**
   * Load monthly budget records for a specific year.
   * Defaults to the current year if not specified.
   */
  async loadMonthlyRecords(year?: number): Promise<void> {
    if (this.#isLoading) return;

    this.#isLoading = true;
    try {
      const args: GetMonthlyBudgetRecordsArgs = { year };
      const result = await safeInvoke<MonthlyBudgetRecordDto[]>('get_monthly_budget_records', {
        args
      });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      this.#monthlyRecords = result.data;
    } catch (error) {
      const message =
        error instanceof Error ? error.message : 'Failed to load monthly budget records';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Load budget dashboard summary for widgets.
   */
  async loadDashboard(): Promise<void> {
    if (this.#isLoading) return;

    this.#isLoading = true;
    try {
      const result = await safeInvoke<BudgetDashboardSummary | null>('get_budget_dashboard');

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      this.#dashboardSummary = result.data;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to load budget dashboard';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Set or update the budget configuration.
   * Uses optimistic updates with automatic rollback on failure.
   */
  async setBudgetConfig(args: SetBudgetConfigArgs): Promise<void> {
    if (this.#isLoading) return;

    // Create snapshot for rollback
    this.#snapshot = this.#config;

    // Optimistic update (create a temporary config for UI)
    const optimisticConfig: BudgetConfigDto = {
      id: this.#config?.id ?? 1,
      mode: args.mode,
      baseAmount: args.baseAmount,
      monthlyAmount: args.mode === 'MONTHLY' ? args.baseAmount : Math.floor(args.baseAmount / 12),
      yearlyAmount: args.mode === 'YEARLY' ? args.baseAmount : args.baseAmount * 12,
      currency: args.currency ?? this.#config?.currency ?? 'EUR',
      lastResetYear: new SvelteDate().getFullYear(),
      createdAt: this.#config?.createdAt ?? new SvelteDate().toISOString(),
      updatedAt: new SvelteDate().toISOString(),
      version: (this.#config?.version ?? 0) + 1
    };

    this.#config = optimisticConfig;
    this.#isLoading = true;

    try {
      const result = await safeInvoke<BudgetConfigDto>('set_budget_config', { args });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      // Update with actual server response
      this.#config = result.data;
      this.#snapshot = null;

      toaster.success({ title: m.budget_config_saved(), duration: 2000 });
    } catch (error) {
      // Rollback optimistic update
      this.#config = this.#snapshot;
      this.#snapshot = null;

      const message = error instanceof Error ? error.message : m.budget_error_save_failed();
      toaster.error({ title: message, duration: 5000 });

      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Add a one-time budget injection to a specific month.
   */
  async addExtraBudget(args: AddExtraBudgetArgs): Promise<ExtraBudgetDto> {
    if (this.#isLoading) {
      throw new Error('Operation already in progress');
    }

    this.#isLoading = true;
    try {
      const result = await safeInvoke<ExtraBudgetDto>('add_extra_budget', { args });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      // Reload monthly records to reflect the new extra budget
      await this.loadMonthlyRecords(args.year);

      toaster.success({ title: 'Extra budget added successfully', duration: 2000 });
      return result.data;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to add extra budget';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Remove an extra budget entry.
   */
  async removeExtraBudget(id: string, year: number): Promise<void> {
    if (this.#isLoading) {
      throw new Error('Operation already in progress');
    }

    this.#isLoading = true;
    try {
      const args: RemoveExtraBudgetArgs = { id };
      const result = await safeInvoke<void>('remove_extra_budget', { args });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      // Reload monthly records to reflect the removed extra budget
      await this.loadMonthlyRecords(year);

      toaster.success({ title: 'Extra budget removed successfully', duration: 2000 });
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to remove extra budget';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Load all extra budgets for a specific year.
   */
  async loadExtraBudgets(year: number): Promise<void> {
    if (this.#isLoading) return;

    this.#isLoading = true;
    try {
      const args: GetExtraBudgetsArgs = { year };
      const result = await safeInvoke<ExtraBudgetDto[]>('get_extra_budgets', { args });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      this.#extraBudgets = result.data;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to load extra budgets';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  /**
   * Load quarterly summaries with category breakdown for a specific year.
   */
  async loadQuarterlySummaries(year?: number, currency?: string): Promise<void> {
    if (this.#isLoading) return;

    this.#isLoading = true;
    try {
      const args: GetQuarterlySummariesArgs = { year, currency };
      const result = await safeInvoke<QuarterlySummary[]>('get_quarterly_summaries', { args });

      if (!result.ok) {
        throw new Error(getErrorMessage(result.error));
      }

      this.#quarterlySummaries = result.data;
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Failed to load quarterly summaries';
      toaster.error({ title: message, duration: 5000 });
      throw error;
    } finally {
      this.#isLoading = false;
    }
  }

  // ───────────────────────────────────────────────────────────
  // UTILITY METHODS
  // ───────────────────────────────────────────────────────────

  /**
   * Reset the service state (useful for testing or logout scenarios).
   */
  reset(): void {
    this.#config = null;
    this.#monthlyRecords = [];
    this.#dashboardSummary = null;
    this.#extraBudgets = [];
    this.#quarterlySummaries = [];
    this.#isLoading = false;
    this.#snapshot = null;
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS
// ─────────────────────────────────────────────────────────────

/**
 * Create and register the budget service in Svelte context.
 * Call this at the root of your feature or layout.
 */
export function createBudgetService(): BudgetService {
  const service = new BudgetService();
  setContext(SERVICE_KEY, service);
  return service;
}

/**
 * Retrieve the budget service from Svelte context.
 * Throws an error if called outside the context tree.
 */
export function getBudgetService(): BudgetService {
  const service = getContext<BudgetService>(SERVICE_KEY);
  if (!service) {
    throw new Error('BudgetService not found in context. Did you call createBudgetService()?');
  }
  return service;
}
