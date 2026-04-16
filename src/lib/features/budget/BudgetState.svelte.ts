/**
 * Budget State Controller
 * * Svelte 5 logic for managing budget configuration and records.
 * Provides pre-formatted strings and derived safety checks for the UI.
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
// TYPES & INTERFACES
// ─────────────────────────────────────────────────────────────

export interface EnhancedMonthlyRecord extends MonthlyBudgetRecordDto {
  formattedBase: string;
  formattedExtra: string;
  formattedRolloverIn: string;
  formattedAvailable: string;
  formattedSpent: string;
  formattedRemaining: string;
  formattedRolloverOut: string;
  remainingPercentage: number;
  statusLabel: string;
}

const STATE_KEY = Symbol('budget-state');

// ─────────────────────────────────────────────────────────────
// STATE CLASS
// ─────────────────────────────────────────────────────────────

export class BudgetState {
  #service: BudgetService;

  // Cached formatter — recomputes only when currency changes
  #formatter = $derived.by(
    () =>
      new Intl.NumberFormat(undefined, {
        style: 'currency',
        currency: this.#service.config?.currency ?? 'EUR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      })
  );

  // Memoized enhanced records — recomputes only when monthlyRecords changes
  #enhancedRecords = $derived.by(() =>
    this.#service.monthlyRecords.map((record) => ({
      ...record,
      formattedBase: this.#formatAmount(record.baseBudget),
      formattedExtra: this.#formatAmount(record.extraBudget),
      formattedRolloverIn: this.#formatAmount(record.rolloverIn),
      formattedAvailable: this.#formatAmount(record.available),
      formattedSpent: this.#formatAmount(record.actualSpend),
      formattedRemaining: this.#formatAmount(record.remaining),
      formattedRolloverOut: this.#formatAmount(record.rolloverOut),
      remainingPercentage:
        record.available > 0
          ? Math.max(0, Math.min(100, (record.remaining / record.available) * 100))
          : 0,
      statusLabel: this.#mapStatus(record.status)
    }))
  );

  constructor(service: BudgetService) {
    this.#service = service;
  }

  // ───────────────────────────────────────────────────────────
  // REACTIVE GETTERS (Proxied from Service)
  // ───────────────────────────────────────────────────────────

  get config(): BudgetConfigDto | null {
    return this.#service.config;
  }
  get isLoading(): boolean {
    return this.#service.isLoading;
  }
  get isTransitioning(): boolean {
    return this.#service.isTransitioning;
  }
  get hasConfig(): boolean {
    return this.#service.hasConfig;
  }
  get dashboardSummary(): BudgetDashboardSummary | null {
    return this.#service.dashboardSummary;
  }
  get extraBudgets(): ExtraBudgetDto[] {
    return this.#service.extraBudgets;
  }
  get quarterlySummaries(): QuarterlySummary[] {
    return this.#service.quarterlySummaries;
  }

  get monthlyRecords(): MonthlyBudgetRecordDto[] {
    return this.#service.monthlyRecords;
  }

  // ───────────────────────────────────────────────────────────
  // THE ENHANCED RECORD MAPPER
  // ───────────────────────────────────────────────────────────

  /**
   * Returns records with pre-formatted currency and calculated safety values.
   * This is the primary data source for the Budget Table.
   * Memoized via $derived — only recomputes when monthlyRecords changes.
   */
  get enhancedMonthlyRecords(): EnhancedMonthlyRecord[] {
    return this.#enhancedRecords;
  }

  get hasRecords(): boolean {
    return this.#service.monthlyRecords.length > 0;
  }

  // ───────────────────────────────────────────────────────────
  // DERIVED CALCULATIONS (Financial)
  // ───────────────────────────────────────────────────────────

  get currency(): string {
    return this.config?.currency ?? 'EUR';
  }

  get monthlyBudget(): number {
    return this.config?.monthlyAmount ?? 0;
  }
  get yearlyBudget(): number {
    return this.config?.yearlyAmount ?? 0;
  }

  get formattedMonthlyBudget(): string {
    return this.formatAmount(this.monthlyBudget);
  }
  get formattedYearlyBudget(): string {
    return this.formatAmount(this.yearlyBudget);
  }

  get modeLabel(): string {
    if (!this.config) return '';
    return this.config.mode === 'YEARLY' ? m.budget_mode_yearly() : m.budget_mode_monthly();
  }

  // ───────────────────────────────────────────────────────────
  // COMMAND METHODS
  // ───────────────────────────────────────────────────────────

  async load(): Promise<void> {
    await this.#service.loadConfig();
  }

  async loadBootstrap(year?: number): Promise<void> {
    await this.#service.loadBootstrap(year);
  }

  async loadMonthlyRecords(year?: number): Promise<void> {
    await this.#service.loadMonthlyRecords(year);
  }

  async loadQuarterlySummaries(year?: number, currency?: string): Promise<void> {
    await this.#service.loadQuarterlySummaries(year, currency);
  }

  async loadDashboard(): Promise<void> {
    await this.#service.loadDashboard();
  }

  async addExtraBudget(args: AddExtraBudgetArgs): Promise<ExtraBudgetDto> {
    return await this.#service.addExtraBudget(args);
  }

  async save(mode: BudgetMode, baseAmount: number, currency?: string): Promise<void> {
    await this.#service.setBudgetConfig({ mode, baseAmount, currency });
  }

  reset(): void {
    this.#service.reset();
  }

  // ───────────────────────────────────────────────────────────
  // PRIVATE UTILITIES
  // ───────────────────────────────────────────────────────────

  /**
   * Formats minor units (cents) into a localized currency string.
   * Reuses the cached #formatter derived field.
   */
  public formatAmount(minorUnits: number): string {
    return this.#formatter.format(minorUnits / 100);
  }

  #formatAmount(minorUnits: number): string {
    return this.#formatter.format(minorUnits / 100);
  }

  #mapStatus(status: string): string {
    switch (status) {
      case 'COMPLETED':
        return m.budget_status_completed();
      case 'IN_PROGRESS':
        return m.budget_status_in_progress();
      default:
        return m.budget_status_projected();
    }
  }
}

// ─────────────────────────────────────────────────────────────
// CONTEXT HELPERS
// ─────────────────────────────────────────────────────────────

export function createBudgetState(service: BudgetService): BudgetState {
  const state = new BudgetState(service);
  setContext(STATE_KEY, state);
  return state;
}

export function getBudgetState(): BudgetState {
  const state = getContext<BudgetState>(STATE_KEY);
  if (!state) {
    throw new Error('BudgetState not found in context. Did you call createBudgetState()?');
  }
  return state;
}
