/**
 * Budget State Controller
 *
 * Manages budget configuration state and derived calculations.
 * This is a lightweight controller that wraps BudgetService
 * and provides computed values for the UI.
 */

import { setContext, getContext } from 'svelte';
import {
  BudgetService,
  type AddExtraBudgetArgs,
  type BudgetConfigDto,
  type BudgetDashboardSummary,
  type BudgetMode,
  type ExtraBudgetDto,
  type MonthlyBudgetRecordDto,
  type QuarterlySummary
} from './services/BudgetService.svelte';
import * as m from '$lib/paraglide/messages.js';

// ─────────────────────────────────────────────────────────────
// CONTEXT KEY
// ─────────────────────────────────────────────────────────────
const STATE_KEY = Symbol('budget-state');

// ─────────────────────────────────────────────────────────────
// STATE CLASS
// ─────────────────────────────────────────────────────────────
export class BudgetState {
  #service: BudgetService;

  constructor(service: BudgetService) {
    this.#service = service;
  }

  // ───────────────────────────────────────────────────────────
  // REACTIVE GETTERS (Proxied from service)
  // ───────────────────────────────────────────────────────────

  get config(): BudgetConfigDto | null {
    return this.#service.config;
  }

  get isLoading(): boolean {
    return this.#service.isLoading;
  }

  get hasConfig(): boolean {
    return this.#service.hasConfig;
  }

  get monthlyRecords(): MonthlyBudgetRecordDto[] {
    return this.#service.monthlyRecords;
  }

  get hasRecords(): boolean {
    return this.monthlyRecords.length > 0;
  }

  get dashboardSummary(): BudgetDashboardSummary | null {
    return this.#service.dashboardSummary;
  }

  get hasDashboard(): boolean {
    return this.dashboardSummary !== null;
  }

  get extraBudgets(): ExtraBudgetDto[] {
    return this.#service.extraBudgets;
  }

  get hasExtraBudgets(): boolean {
    return this.extraBudgets.length > 0;
  }

  get quarterlySummaries(): QuarterlySummary[] {
    return this.#service.quarterlySummaries;
  }

  get hasQuarterlySummaries(): boolean {
    return this.quarterlySummaries.length > 0;
  }

  // ───────────────────────────────────────────────────────────
  // DERIVED CALCULATIONS
  // ───────────────────────────────────────────────────────────

  /**
   * Get the monthly budget amount in minor units (cents).
   */
  get monthlyBudget(): number {
    return this.config?.monthlyAmount ?? 0;
  }

  /**
   * Get the yearly budget amount in minor units (cents).
   */
  get yearlyBudget(): number {
    return this.config?.yearlyAmount ?? 0;
  }

  /**
   * Get the currency code.
   */
  get currency(): string {
    return this.config?.currency ?? 'EUR';
  }

  /**
   * Get the base amount in major units (e.g., dollars, euros).
   */
  get baseAmountMajor(): number {
    if (!this.config) return 0;
    return this.config.baseAmount / 100;
  }

  /**
   * Get the monthly amount in major units.
   */
  get monthlyBudgetMajor(): number {
    return this.monthlyBudget / 100;
  }

  /**
   * Get the yearly amount in major units.
   */
  get yearlyBudgetMajor(): number {
    return this.yearlyBudget / 100;
  }

  /**
   * Get formatted base amount with currency symbol.
   */
  get formattedBaseAmount(): string {
    if (!this.config) return this.formatAmount(0);
    return this.formatAmount(this.config.baseAmount);
  }

  /**
   * Get formatted monthly budget with currency symbol.
   */
  get formattedMonthlyBudget(): string {
    return this.formatAmount(this.monthlyBudget);
  }

  /**
   * Get formatted yearly budget with currency symbol.
   */
  get formattedYearlyBudget(): string {
    return this.formatAmount(this.yearlyBudget);
  }

  /**
   * Get the current budget mode display label.
   */
  get modeLabel(): string {
    if (!this.config) return '';
    return this.config.mode === 'YEARLY' ? m.budget_mode_yearly() : m.budget_mode_monthly();
  }

  // ───────────────────────────────────────────────────────────
  // COMMAND METHODS
  // ───────────────────────────────────────────────────────────

  /**
   * Load the budget configuration from the backend.
   */
  async load(): Promise<void> {
    await this.#service.loadConfig();
  }

  /**
   * Load monthly budget records for a specific year.
   */
  async loadMonthlyRecords(year?: number): Promise<void> {
    await this.#service.loadMonthlyRecords(year);
  }

  /**
   * Load budget dashboard summary.
   */
  async loadDashboard(): Promise<void> {
    await this.#service.loadDashboard();
  }

  /**
   * Load extra budgets for a specific year.
   */
  async loadExtraBudgets(year: number): Promise<void> {
    await this.#service.loadExtraBudgets(year);
  }

  /**
   * Load quarterly summaries with category breakdown.
   */
  async loadQuarterlySummaries(year?: number, currency?: string): Promise<void> {
    await this.#service.loadQuarterlySummaries(year, currency);
  }

  /**
   * Add a one-time budget injection to a specific month.
   */
  async addExtraBudget(args: AddExtraBudgetArgs): Promise<ExtraBudgetDto> {
    return await this.#service.addExtraBudget(args);
  }

  /**
   * Remove an extra budget entry.
   */
  async removeExtraBudget(id: string, year: number): Promise<void> {
    await this.#service.removeExtraBudget(id, year);
  }

  /**
   * Save or update the budget configuration.
   */
  async save(mode: BudgetMode, baseAmount: number, currency?: string): Promise<void> {
    await this.#service.setBudgetConfig({
      mode,
      baseAmount,
      currency
    });
  }

  /**
   * Reset the state (for testing or logout).
   */
  reset(): void {
    this.#service.reset();
  }

  // ───────────────────────────────────────────────────────────
  // UTILITY METHODS
  // ───────────────────────────────────────────────────────────

  /**
   * Format an amount in minor units to a currency string.
   */
  private formatAmount(minorUnits: number): string {
    const major = minorUnits / 100;
    return new Intl.NumberFormat(undefined, {
      style: 'currency',
      currency: this.currency,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2
    }).format(major);
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS
// ─────────────────────────────────────────────────────────────

/**
 * Create and register BudgetState in Svelte context.
 */
export function createBudgetState(service: BudgetService): BudgetState {
  const state = new BudgetState(service);
  setContext(STATE_KEY, state);
  return state;
}

/**
 * Retrieve BudgetState from Svelte context.
 */
export function getBudgetState(): BudgetState {
  const state = getContext<BudgetState>(STATE_KEY);
  if (!state) {
    throw new Error('BudgetState not found in context. Did you call createBudgetState()?');
  }
  return state;
}
